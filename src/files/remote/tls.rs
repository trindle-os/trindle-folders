//! TLS for secure FTP, by rustls on ring's cryptography. A certificate is

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::client::WebPkiServerVerifier;
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{CertificateError, ClientConfig, ClientConnection, DigitallySignedStruct, RootCertStore, SignatureScheme, StreamOwned};
use sha2::{Digest, Sha256};

use super::Error;
use crate::toolkit::t;

pub enum Link {
    Plain(TcpStream),
    Tls(Box<StreamOwned<ClientConnection, TcpStream>>),
}

impl Link {
    pub fn tcp(&self) -> &TcpStream {
        match self {
            Link::Plain(s) => s,
            Link::Tls(t) => t.get_ref(),
        }
    }

    pub fn is_tls(&self) -> bool {
        matches!(self, Link::Tls(_))
    }

    pub fn finish_writing(&mut self) {
        if let Link::Tls(t) = self {
            t.conn.send_close_notify();
            let _ = t.flush();
        }
        let _ = self.tcp().shutdown(std::net::Shutdown::Write);
    }
}

impl Read for Link {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Link::Plain(s) => s.read(buf),
            Link::Tls(t) => match t.read(buf) {
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(0),
                r => r,
            },
        }
    }
}

impl Write for Link {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Link::Plain(s) => s.write(buf),
            Link::Tls(t) => t.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Link::Plain(s) => s.flush(),
            Link::Tls(t) => t.flush(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Found {
    Unknown { fingerprint: String, why: String },
    Changed { fingerprint: String },
}

#[derive(Debug)]
struct Check {
    usual: Option<Arc<WebPkiServerVerifier>>,
    provider: Arc<CryptoProvider>,
    remembered: Option<String>,
    trusted: Option<String>,
    found: Mutex<Option<Found>>,
    used_trust: Mutex<bool>,
}

impl ServerCertVerifier for Check {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        server_name: &ServerName<'_>,
        ocsp: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let why = match &self.usual {
            Some(v) => match v.verify_server_cert(end_entity, intermediates, server_name, ocsp, now) {
                Ok(ok) => return Ok(ok),
                Err(e) => words(&e),
            },
            None => t!("folders-remote-tls-no-roots").into(),
        };
        let fingerprint = fingerprint(end_entity);
        if self.remembered.as_deref() == Some(fingerprint.as_str()) {
            return Ok(ServerCertVerified::assertion());
        }
        if self.trusted.as_deref() == Some(fingerprint.as_str()) {
            *lock(&self.used_trust) = true;
            return Ok(ServerCertVerified::assertion());
        }
        *lock(&self.found) = Some(match self.remembered {
            Some(_) => Found::Changed { fingerprint },
            None => Found::Unknown { fingerprint, why },
        });
        Err(rustls::Error::InvalidCertificate(CertificateError::ApplicationVerificationFailure))
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider.signature_verification_algorithms.supported_schemes()
    }
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}

fn words(e: &rustls::Error) -> String {
    use CertificateError as C;
    match e {
        rustls::Error::InvalidCertificate(c) => match c {
            C::UnknownIssuer => t!("folders-remote-tls-unknown-issuer"),
            C::Expired | C::ExpiredContext { .. } => t!("folders-remote-tls-expired"),
            C::NotValidYet | C::NotValidYetContext { .. } => t!("folders-remote-tls-not-yet"),
            C::NotValidForName | C::NotValidForNameContext { .. } => t!("folders-remote-tls-wrong-name"),
            C::Revoked => t!("folders-remote-tls-revoked"),
            _ => t!("folders-remote-tls-failed-checks"),
        },
        _ => t!("folders-remote-tls-failed-checks"),
    }
    .into()
}

pub fn fingerprint(cert: &[u8]) -> String {
    let d = Sha256::digest(cert);
    d.iter().map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(":")
}

fn roots() -> Arc<RootCertStore> {
    static ROOTS: OnceLock<Arc<RootCertStore>> = OnceLock::new();
    ROOTS
        .get_or_init(|| {
            let mut store = RootCertStore::empty();
            if !cfg!(test) {
                let found = rustls_native_certs::load_native_certs();
                store.add_parsable_certificates(found.certs);
            }
            Arc::new(store)
        })
        .clone()
}

pub struct Secure {
    config: Arc<ClientConfig>,
    name: ServerName<'static>,
    check: Arc<Check>,
    key: String,
    host: String,
}

impl Secure {
    pub fn new(host: &str, port: u16, trusted: Option<&str>) -> Result<Secure, Error> {
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let roots = roots();
        let usual = if roots.is_empty() {
            None
        } else {
            WebPkiServerVerifier::builder_with_provider(roots, provider.clone()).build().ok()
        };
        let key = format!("{host}:{port}");
        let check = Arc::new(Check {
            usual,
            provider: provider.clone(),
            remembered: remembered(&key),
            trusted: trusted.map(str::to_string),
            found: Mutex::new(None),
            used_trust: Mutex::new(false),
        });
        let config = ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .map_err(|e| Error::Failed(t!("folders-remote-tls-setup-failed", why = e.to_string())))?
            .dangerous()
            .with_custom_certificate_verifier(check.clone())
            .with_no_client_auth();
        let name = ServerName::try_from(host.to_string()).map_err(|_| Error::BadAddress(t!("folders-remote-tls-bad-name", host = host)))?;
        Ok(Secure { config: Arc::new(config), name, check, key, host: host.to_string() })
    }

    pub fn wrap(&self, mut tcp: TcpStream) -> Result<Link, Error> {
        let mut conn = ClientConnection::new(self.config.clone(), self.name.clone())
            .map_err(|e| Error::Failed(t!("folders-remote-tls-start-failed", why = e.to_string())))?;
        while conn.is_handshaking() {
            match conn.complete_io(&mut tcp) {
                Ok((0, 0)) => return Err(self.why(io::ErrorKind::UnexpectedEof.into())),
                Ok(_) => {}
                Err(e) => return Err(self.why(e)),
            }
        }
        if *lock(&self.check.used_trust) {
            remember(&self.key, self.check.trusted.as_deref().unwrap_or(""));
        }
        Ok(Link::Tls(Box::new(StreamOwned::new(conn, tcp))))
    }

    fn why(&self, e: io::Error) -> Error {
        match lock(&self.check.found).take() {
            Some(Found::Unknown { fingerprint, why }) => Error::CertUnknown { host: self.host.clone(), fingerprint, why },
            Some(Found::Changed { fingerprint }) => Error::CertChanged { host: self.host.clone(), fingerprint },
            None => match e.kind() {
                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => Error::Failed(t!("folders-remote-tls-stopped").into()),
                io::ErrorKind::UnexpectedEof | io::ErrorKind::ConnectionReset | io::ErrorKind::BrokenPipe => {
                    Error::Failed(t!("folders-remote-tls-closed").into())
                }
                _ => Error::Failed(t!("folders-remote-tls-could-not-set-up", why = e.to_string())),
            },
        }
    }
}

fn store() -> PathBuf {
    if cfg!(test) {
        std::env::temp_dir().join(format!("trindle-files-ftps-{}", std::process::id())).join("trusted")
    } else {
        crate::common::files::trindle_config().join("folders-trusted-certificates")
    }
}

fn remembered(key: &str) -> Option<String> {
    let text = crate::common::files::read_string(store()).ok()?;
    text.lines().find_map(|l| l.split_once(' ').filter(|(k, _)| *k == key).map(|(_, f)| f.trim().to_string()))
}

fn remember(key: &str, fingerprint: &str) {
    remember_in(&store(), key, fingerprint);
}

fn remember_in(path: &Path, key: &str, fingerprint: &str) {
    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
    let _held = lock(&ONE_AT_A_TIME);
    let Ok(old) = crate::common::files::read_small(path) else { return };
    let mut text: String = old.lines().filter(|l| l.split_once(' ').map(|(k, _)| k) != Some(key)).flat_map(|l| [l, "\n"]).collect();
    text.push_str(&format!("{key} {fingerprint}\n"));
    let _ = crate::common::files::write_whole(path, text.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprints_and_the_kept_list() {
        assert_eq!(fingerprint(b"").len(), 32 * 3 - 1);
        assert!(fingerprint(b"abc").starts_with("BA:78:16:BF"));
        remember("kept.example:21", "AA:BB");
        remember("other.example:990", "CC");
        assert_eq!(remembered("kept.example:21").as_deref(), Some("AA:BB"));
        remember("kept.example:21", "DD");
        assert_eq!(remembered("kept.example:21").as_deref(), Some("DD"));
        assert_eq!(remembered("other.example:990").as_deref(), Some("CC"));
        assert_eq!(remembered("kept.example:2121"), None);
    }

    #[test]
    fn a_list_of_certificates_that_cannot_be_read_is_never_written_over() {
        let dir = std::env::temp_dir().join(format!("trindle-files-ftps-unread-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("trusted");
        let was = b"kept.example:21 AA:BB\nother.example:990 CC\n\xff\n".to_vec();
        std::fs::write(&path, &was).unwrap();
        remember_in(&path, "new.example:21", "EE");
        assert_eq!(std::fs::read(&path).unwrap(), was);
        assert!(!dir.join("trusted.new").exists());
        std::fs::write(&path, "kept.example:21 AA:BB\n").unwrap();
        remember_in(&path, "new.example:21", "EE");
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "kept.example:21 AA:BB\nnew.example:21 EE\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    fn rss() -> u64 {
        let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
        s.lines().find_map(|l| l.strip_prefix("VmRSS:")).and_then(|v| v.trim().trim_end_matches("kB").trim().parse().ok()).unwrap_or(0)
    }

    #[test]
    #[ignore = "needs a real network server: run by hand"]
    fn what_tls_weighs() {
        let start = rss();
        let mut store = RootCertStore::empty();
        let found = rustls_native_certs::load_native_certs();
        let (added, _) = store.add_parsable_certificates(found.certs);
        drop(store);
        let after_roots = rss();
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
        let fp = fingerprint(cert.cert.der());
        let key = rustls::pki_types::PrivateKeyDer::Pkcs8(cert.key_pair.serialize_der().into());
        let server = Arc::new(
            rustls::ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
                .with_safe_default_protocol_versions()
                .unwrap()
                .with_no_client_auth()
                .with_single_cert(vec![cert.cert.der().clone()], key)
                .unwrap(),
        );
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let t = std::thread::spawn(move || {
            let (s, _) = listener.accept().unwrap();
            let mut tls = rustls::StreamOwned::new(rustls::ServerConnection::new(server).unwrap(), s);
            let mut b = [0u8; 5];
            io::Read::read_exact(&mut tls, &mut b).unwrap();
        });
        let before_link = rss();
        let secure = Secure::new("localhost", port, Some(&fp)).unwrap();
        let mut link = secure.wrap(TcpStream::connect(("127.0.0.1", port)).unwrap()).unwrap();
        io::Write::write_all(&mut link, b"hello").unwrap();
        io::Write::flush(&mut link).unwrap();
        let with_link = rss();
        t.join().unwrap();
        println!("roots: {added} certificates, {} kB; one connection with its server: {} kB", after_roots.saturating_sub(start), with_link.saturating_sub(before_link));
    }
}
