//! Which page of Help explains which failure message in Folders. A message

use std::io;

use super::archive::{self, Trouble};
use super::remote::{Error, Scheme};
use super::work::{Extra, Kind};

pub const ARCHIVE_PASSWORD: &str = "folders#archive-password";
pub const ARCHIVE_NOT_OPENED: &str = "folders#an-archive-will-not-open";
pub const ARCHIVE_NO_ROOM: &str = "folders#an-archive-will-not-unpack-here";
pub const WORK_NOT_DONE: &str = "folders#work-that-did-not-finish";
pub const NOT_KEPT: &str = "folders#choices-that-were-not-kept";
pub const NOT_STARTED: &str = "folders#a-program-would-not-start";
pub const PART_MISSING: &str = "folders#a-part-is-not-on-this-computer";
pub const DRIVE: &str = "folders#a-drive-would-not-open-or-be-put-away";

pub const SERVER_REACH: &str = "folders#a-server-would-not-answer";
pub const SERVER_LOGIN: &str = "folders#a-server-would-not-let-you-in";
pub const SERVER_REFUSED: &str = "folders#a-server-would-not-do-that";
pub const PHONE: &str = "folders#a-phone-would-not-show-its-files";

pub fn remote_page(e: &Error, phone: bool) -> &'static str {
    if phone {
        return PHONE;
    }
    match e {
        Error::BadAddress(_) | Error::Unreachable(_) | Error::Lost(_) => SERVER_REACH,
        Error::NeedsPassword | Error::LoginRefused | Error::HostKeyUnknown(_) | Error::HostKeyChanged(_) | Error::CertUnknown { .. }
        | Error::CertChanged { .. } | Error::Key(_) | Error::Heard { .. } => SERVER_LOGIN,
        _ => SERVER_REFUSED,
    }
}

pub fn server_page(path: &std::path::Path) -> &'static str {
    match super::far::address(path) {
        Some(a) if a.scheme == Scheme::Mtp => PHONE,
        _ => SERVER_REFUSED,
    }
}

pub fn server_work_page(places: &[&std::path::Path]) -> Option<&'static str> {
    places.iter().find(|p| super::far::is_far(p)).map(|p| server_page(p))
}

pub const SHARE_LOGIN: &str = "folders#a-share-asks-for-a-login";
pub const SHARE_NOT_OPENED: &str = "folders#a-share-would-not-open";

pub fn share_page(p: &crate::common::smb::Problem, nfs: bool) -> &'static str {
    use crate::common::smb::Problem;
    match p {
        Problem::NeedsLogin | Problem::WrongLogin | Problem::Account | Problem::Denied if !nfs => SHARE_LOGIN,
        _ => SHARE_NOT_OPENED,
    }
}

pub const SHARING_FOLDER: &str = "folders#a-folder-could-not-be-shared";

pub const FILE_UNREAD: &str = "folders#a-file-could-not-be-read";
pub const DETAILS_NOT_CHANGED: &str = "folders#details-could-not-be-changed";

pub const PLACE_UNREAD: &str = "folders#a-folder-would-not-open";
pub const COLOURS_LOST: &str = "folders#colours-and-emblems-did-not-follow";

pub const DRAG: &str = "folders#a-drag-from-or-to-another-program-did-not-finish";
pub const MAKE_FAILED: &str = "folders#something-could-not-be-made-here";

pub const COMPARE_STOPPED: &str = "folders#a-comparison-or-search-stopped";

pub const NAME_REFUSED: &str = "folders#a-name-was-refused";

#[cfg(test)]
pub const PAGES: &[&str] = &[ARCHIVE_PASSWORD, ARCHIVE_NOT_OPENED, ARCHIVE_NO_ROOM, WORK_NOT_DONE, NOT_KEPT, NOT_STARTED, PART_MISSING, DRIVE,
                             SERVER_REACH, SERVER_LOGIN, SERVER_REFUSED, PHONE, NAME_REFUSED, SHARE_LOGIN, SHARE_NOT_OPENED, SHARING_FOLDER, FILE_UNREAD, DETAILS_NOT_CHANGED, PLACE_UNREAD, COLOURS_LOST, DRAG, MAKE_FAILED, COMPARE_STOPPED];

pub fn archive_page(e: &io::Error) -> &'static str {
    if let Some(t) = archive::trouble_of(e) {
        return match t {
            Trouble::Locked | Trouble::WrongKey | Trouble::WrongOrBad | Trouble::LockedRar | Trouble::LockedForeign => ARCHIVE_PASSWORD,
            Trouble::TooMany | Trouble::NoRoom | Trouble::GaveUp => ARCHIVE_NO_ROOM,
            Trouble::Unknown | Trouble::Foreign | Trouble::NoReader | Trouble::Parts => ARCHIVE_NOT_OPENED,
        };
    }
    match e.kind() {
        io::ErrorKind::StorageFull | io::ErrorKind::OutOfMemory | io::ErrorKind::PermissionDenied | io::ErrorKind::ReadOnlyFilesystem => ARCHIVE_NO_ROOM,
        _ => ARCHIVE_NOT_OPENED,
    }
}

pub fn work_page(kind: &Kind) -> &'static str {
    match kind {
        Kind::Other(Extra::Unpack(_) | Extra::TakeOut(_)) => ARCHIVE_NOT_OPENED,
        _ => WORK_NOT_DONE,
    }
}

#[cfg(test)]
#[cfg(test)]
pub const MESSAGES: &[super::super::common::more::Entry] = &[
    ("folders-archive-trouble-locked", ARCHIVE_PASSWORD),
    ("folders-archive-trouble-wrong-key", ARCHIVE_PASSWORD),
    ("folders-archive-trouble-wrong-or-bad", ARCHIVE_PASSWORD),
    ("folders-archive-trouble-locked-rar", ARCHIVE_PASSWORD),
    ("folders-archive-trouble-locked-foreign", ARCHIVE_PASSWORD),
    ("folders-locked-open-again", ARCHIVE_PASSWORD),
    ("folders-archive-trouble-unknown", ARCHIVE_NOT_OPENED),
    ("folders-archive-trouble-foreign", ARCHIVE_NOT_OPENED),
    ("folders-archive-trouble-no-reader", ARCHIVE_NOT_OPENED),
    ("folders-archive-trouble-parts", ARCHIVE_NOT_OPENED),
    ("folders-archive-why-damaged", ARCHIVE_NOT_OPENED),
    ("folders-archive-why-gone", ARCHIVE_NOT_OPENED),
    ("folders-archive-why-other", ARCHIVE_NOT_OPENED),
    ("folders-inside-unread", ARCHIVE_NOT_OPENED),
    ("folders-inside-failed", ARCHIVE_NOT_OPENED),
    ("folders-locked-could-not-read", ARCHIVE_NOT_OPENED),
    ("folders-archive-trouble-too-many", ARCHIVE_NO_ROOM),
    ("folders-archive-trouble-no-room", ARCHIVE_NO_ROOM),
    ("folders-archive-trouble-gave-up", ARCHIVE_NO_ROOM),
    ("folders-archive-why-no-room", ARCHIVE_NO_ROOM),
    ("folders-archive-why-read-only", ARCHIVE_NO_ROOM),
    ("folders-archive-why-memory", ARCHIVE_NO_ROOM),
    ("folders-archive-7z-memory", ARCHIVE_NO_ROOM),
    ("folders-inside-refused", ARCHIVE_NO_ROOM),
    ("folders-act-failed", WORK_NOT_DONE),
    ("folders-act-no-start", WORK_NOT_DONE),
    ("kit-work-not-looked", WORK_NOT_DONE),
    ("kit-work-too-big", WORK_NOT_DONE),
    ("kit-work-changed-since", WORK_NOT_DONE),
    ("kit-work-taken", WORK_NOT_DONE),
    ("kit-work-no-trash", WORK_NOT_DONE),
    ("kit-work-stayed", WORK_NOT_DONE),
    ("kit-work-not-kept", WORK_NOT_DONE),
    ("kit-work-not-all", WORK_NOT_DONE),
    ("kit-work-inside-replaced", WORK_NOT_DONE),
    ("kit-work-copy-made-it", WORK_NOT_DONE),
    ("kit-work-copy-wrong", WORK_NOT_DONE),
    ("folders-app-panes-not-kept", NOT_KEPT),
    ("folders-app-link-not-kept", NOT_KEPT),
    ("folders-app-view-not-kept", NOT_KEPT),
    ("folders-app-grouping-not-kept", NOT_KEPT),
    ("folders-zoom-not-kept", NOT_KEPT),
    ("folders-panel-keep-failed", NOT_KEPT),
    ("folders-heading-widths-not-kept", NOT_KEPT),
    ("folders-menu-columns-not-saved", NOT_KEPT),
    ("folders-menu-ring-not-saved", NOT_KEPT),
    ("folders-dialogs-columns-failed", NOT_KEPT),
    ("folders-dialogs-colour-failed", NOT_KEPT),
    ("folders-dialogs-usual-failed", NOT_KEPT),
    ("folders-app-not-started", NOT_STARTED),
    ("folders-app-no-window", NOT_STARTED),
    ("folders-dialogs-start-failed", NOT_STARTED),
    ("folders-menu-no-new-window", NOT_STARTED),
    ("folders-menu-no-admin-window", NOT_STARTED),
    ("folders-menu-wallpaper-failed", NOT_STARTED),
    ("folders-alone-no-share", PART_MISSING),
    ("folders-alone-no-mount", PART_MISSING),
    ("folders-alone-no-nfs", PART_MISSING),
    ("folders-alone-no-wallpaper", PART_MISSING),
    ("folders-alone-no-root", PART_MISSING),
    ("folders-alone-no-ai", PART_MISSING),
    ("folders-alone-no-terminal", PART_MISSING),
    ("folders-drives-open-failed", DRIVE),
    ("folders-drives-away-failed", DRIVE),
    ("folders-drives-busy", DRIVE),
    ("folders-drives-unreachable", DRIVE),
    ("folders-remote-ftp-no-server", SERVER_REACH),
    ("folders-remote-ftp-turned-away", SERVER_REACH),
    ("folders-remote-ftp-no-answer", SERVER_REACH),
    ("folders-remote-ftp-no-way", SERVER_REACH),
    ("folders-remote-ftp-stopped", SERVER_REACH),
    ("folders-remote-ftp-lost", SERVER_REACH),
    ("folders-remote-ftp-closed", SERVER_REACH),
    ("folders-remote-sftp-no-server", SERVER_REACH),
    ("folders-remote-sftp-turned-away", SERVER_REACH),
    ("folders-remote-sftp-no-answer", SERVER_REACH),
    ("folders-remote-sftp-no-way", SERVER_REACH),
    ("folders-remote-sftp-lost", SERVER_REACH),
    ("folders-remote-sftp-nonsense", SERVER_REACH),
    ("folders-remote-dav-not-found", SERVER_REACH),
    ("folders-remote-dav-no-address", SERVER_REACH),
    ("folders-remote-tls-bad-name", SERVER_REACH),
    ("folders-far-no-server", SERVER_REACH),
    ("folders-far-address-unknown", SERVER_REACH),
    ("folders-remote-mod-bad-address", SERVER_REACH),
    ("folders-remote-mod-unreachable", SERVER_REACH),
    ("folders-remote-address-no-scheme", SERVER_REACH),
    ("folders-remote-address-bad-scheme", SERVER_REACH),
    ("folders-remote-address-no-bracket", SERVER_REACH),
    ("folders-remote-address-no-server", SERVER_REACH),
    ("folders-remote-address-bad-name", SERVER_REACH),
    ("folders-remote-address-line-break", SERVER_REACH),
    ("folders-remote-address-after-name", SERVER_REACH),
    ("folders-remote-address-not-port", SERVER_REACH),
    ("folders-remote-mod-needs-password", SERVER_LOGIN),
    ("folders-remote-mod-login-refused", SERVER_LOGIN),
    ("folders-remote-mod-host-key-unknown", SERVER_LOGIN),
    ("folders-remote-mod-host-key-changed", SERVER_LOGIN),
    ("folders-remote-mod-cert-unknown", SERVER_LOGIN),
    ("folders-remote-mod-cert-changed", SERVER_LOGIN),
    ("folders-remote-mod-heard-card", SERVER_LOGIN),
    ("folders-remote-mod-heard-stopped", SERVER_LOGIN),
    ("folders-remote-sftp-no-key", SERVER_LOGIN),
    ("folders-remote-tls-no-roots", SERVER_LOGIN),
    ("folders-remote-tls-unknown-issuer", SERVER_LOGIN),
    ("folders-remote-tls-expired", SERVER_LOGIN),
    ("folders-remote-tls-not-yet", SERVER_LOGIN),
    ("folders-remote-tls-wrong-name", SERVER_LOGIN),
    ("folders-remote-tls-revoked", SERVER_LOGIN),
    ("folders-remote-tls-failed-checks", SERVER_LOGIN),
    ("folders-far-connect-new-host", SERVER_LOGIN),
    ("folders-far-connect-host-changed", SERVER_LOGIN),
    ("folders-far-connect-needs-password", SERVER_LOGIN),
    ("folders-far-connect-needs-key", SERVER_LOGIN),
    ("folders-far-connect-cert-new", SERVER_LOGIN),
    ("folders-far-connect-cert-changed", SERVER_LOGIN),
    ("folders-remote-mod-not-found", SERVER_REFUSED),
    ("folders-remote-mod-exists", SERVER_REFUSED),
    ("folders-remote-mod-denied", SERVER_REFUSED),
    ("folders-remote-mod-busy", SERVER_REFUSED),
    ("folders-remote-mod-not-supported", SERVER_REFUSED),
    ("folders-remote-mod-cancelled", SERVER_REFUSED),
    ("folders-remote-mod-copy-share", SERVER_REFUSED),
    ("folders-remote-mod-copy-nfs", SERVER_REFUSED),
    ("folders-remote-ftp-no-protect", SERVER_REFUSED),
    ("folders-remote-ftp-no-welcome", SERVER_REFUSED),
    ("folders-remote-ftp-no-secure", SERVER_REFUSED),
    ("folders-remote-ftp-accounts", SERVER_REFUSED),
    ("folders-remote-ftp-data-failed", SERVER_REFUSED),
    ("folders-remote-ftp-that-here", SERVER_REFUSED),
    ("folders-remote-ftp-could-not", SERVER_REFUSED),
    ("folders-remote-ftp-not-reply", SERVER_REFUSED),
    ("folders-remote-sftp-no-ssh", SERVER_REFUSED),
    ("folders-remote-sftp-ssh-failed", SERVER_REFUSED),
    ("folders-remote-sftp-old", SERVER_REFUSED),
    ("folders-remote-sftp-that-here", SERVER_REFUSED),
    ("folders-remote-sftp-could-not", SERVER_REFUSED),
    ("folders-remote-sftp-could-not-said", SERVER_REFUSED),
    ("folders-remote-sftp-none-here", SERVER_REFUSED),
    ("folders-remote-sftp-ssh-said", SERVER_REFUSED),
    ("folders-remote-dav-changed-at-removal", SERVER_REFUSED),
    ("folders-remote-dav-chunk", SERVER_REFUSED),
    ("folders-remote-dav-is-file", SERVER_REFUSED),
    ("folders-remote-dav-file-ended", SERVER_REFUSED),
    ("folders-remote-dav-unknown-login", SERVER_REFUSED),
    ("folders-remote-dav-no-listing", SERVER_REFUSED),
    ("folders-remote-dav-not-web", SERVER_REFUSED),
    ("folders-remote-dav-full", SERVER_REFUSED),
    ("folders-remote-dav-status", SERVER_REFUSED),
    ("folders-remote-dav-listing-unreadable", SERVER_REFUSED),
    ("folders-remote-dav-cut-short", SERVER_REFUSED),
    ("folders-remote-dav-not-empty", SERVER_REFUSED),
    ("folders-remote-tls-setup-failed", SERVER_REFUSED),
    ("folders-remote-tls-start-failed", SERVER_REFUSED),
    ("folders-remote-tls-stopped", SERVER_REFUSED),
    ("folders-remote-tls-closed", SERVER_REFUSED),
    ("folders-remote-tls-could-not-set-up", SERVER_REFUSED),
    ("folders-far-failed-why", SERVER_REFUSED),
    ("folders-far-not-all-arrived", SERVER_REFUSED),
    ("folders-far-not-followed", SERVER_REFUSED),
    ("folders-far-not-sent", SERVER_REFUSED),
    ("folders-far-not-told", SERVER_REFUSED),
    ("folders-far-cannot-do", SERVER_REFUSED),
    ("folders-far-not-looked-at", SERVER_REFUSED),
    ("folders-far-not-opened", SERVER_REFUSED),
    ("folders-far-inside-itself", SERVER_REFUSED),
    ("folders-far-taken-by-folder", SERVER_REFUSED),
    ("folders-far-taken-by-file", SERVER_REFUSED),
    ("folders-far-copy-wrong-length", SERVER_REFUSED),
    ("folders-far-only-server-file", SERVER_REFUSED),
    ("folders-far-aside-both", SERVER_REFUSED),
    ("folders-far-aside-kept", SERVER_REFUSED),
    ("folders-far-aside-kept-why", SERVER_REFUSED),
    ("folders-far-not-empty", SERVER_REFUSED),
    ("folders-far-cannot-fetch", SERVER_REFUSED),
    ("folders-far-not-saved", SERVER_REFUSED),
    ("folders-remote-mtp-nonsense", PHONE),
    ("folders-remote-mtp-full", PHONE),
    ("folders-remote-mtp-the-phone", PHONE),
    ("folders-remote-mtp-that-here", PHONE),
    ("folders-remote-mtp-refused", PHONE),
    ("folders-remote-mtp-not-plugged", PHONE),
    ("folders-remote-mtp-shorter", PHONE),
    ("folders-remote-mtp-only-storage", PHONE),
    ("folders-remote-mtp-not-folder", PHONE),
    ("folders-remote-mtp-is-folder", PHONE),
    ("folders-remote-mtp-not-empty", PHONE),
    ("folders-remote-mtp-remove-storage", PHONE),
    ("folders-remote-mtp-rename-storage", PHONE),
    ("folders-remote-mtp-no-storage", PHONE),
    ("folders-remote-usb-denied", PHONE),
    ("folders-remote-usb-unplugged", PHONE),
    ("folders-remote-usb-busy", PHONE),
    ("folders-remote-usb-stopped", PHONE),
    ("folders-remote-usb-refused", PHONE),
    ("folders-remote-usb-part", PHONE),
    ("folders-far-phones", PHONE),
    ("folders-far-unsafe-name", NAME_REFUSED),
    ("folders-far-unsafe-names", NAME_REFUSED),
    ("folders-remote-screen-not-a-name", NAME_REFUSED),
    ("folders-remote-screen-no-name", NAME_REFUSED),
    ("folders-act-name-empty", NAME_REFUSED),
    ("folders-act-name-slash", NAME_REFUSED),
    ("folders-act-name-long", NAME_REFUSED),
    ("folders-act-name-kept", NAME_REFUSED),
    ("folders-inplace-taken", NAME_REFUSED),
    ("folders-shares-held", SHARE_LOGIN),
    ("folders-shares-nowhere", SHARE_LOGIN),
    ("folders-shares-lapsed", SHARE_LOGIN),
    ("folders-shares-unknown", SHARE_LOGIN),
    ("folders-shares-unshown", SHARE_LOGIN),
    ("folders-shares-again", SHARE_LOGIN),
    ("folders-shares-keyring-unreachable", SHARE_LOGIN),
    ("folders-shares-unkept", SHARE_LOGIN),
    ("folders-shares-nfs-denied", SHARE_NOT_OPENED),
    ("folders-shares-nfs-no-share", SHARE_NOT_OPENED),
    ("folders-shares-nfs-version", SHARE_NOT_OPENED),
    ("folders-shares-nfs-opened-unknown", SHARE_NOT_OPENED),
    ("folders-shares-opened-unknown", SHARE_NOT_OPENED),
    ("folders-shares-bad-computer", SHARE_NOT_OPENED),
    ("folders-shares-bad-path", SHARE_NOT_OPENED),
    ("folders-shares-close-failed", SHARE_NOT_OPENED),
    ("folders-sharing-cannot-read", SHARING_FOLDER),
    ("folders-sharing-only-folder", SHARING_FOLDER),
    ("folders-sharing-only-yours", SHARING_FOLDER),
    ("folders-sharing-name-taken", SHARING_FOLDER),
    ("folders-sharing-password-short", SHARING_FOLDER),
    ("folders-viewer-no-ffmpeg", FILE_UNREAD),
    ("folders-viewer-unplayable", FILE_UNREAD),
    ("folders-viewer-picture-unread", FILE_UNREAD),
    ("folders-viewer-gone", FILE_UNREAD),
    ("folders-viewer-denied", FILE_UNREAD),
    ("folders-viewer-read-failed", FILE_UNREAD),
    ("folders-preview-page-failed", FILE_UNREAD),
    ("folders-preview-rest-failed", FILE_UNREAD),
    ("folders-preview-fetch-failed", FILE_UNREAD),
    ("folders-preview-broken", FILE_UNREAD),
    ("folders-preview-too-large-pixels", FILE_UNREAD),
    ("folders-preview-too-large-bytes", FILE_UNREAD),
    ("folders-preview-unread", FILE_UNREAD),
    ("folders-preview-unplayable", FILE_UNREAD),
    ("folders-panel-broken", FILE_UNREAD),
    ("folders-panel-too-large", FILE_UNREAD),
    ("folders-panel-read-failed", FILE_UNREAD),
    ("folders-dialogs-sums-failed", FILE_UNREAD),
    ("folders-fonts-too-large", FILE_UNREAD),
    ("folders-fonts-unreadable", FILE_UNREAD),
    ("folders-details-change-failed", DETAILS_NOT_CHANGED),
    ("folders-details-date-failed", DETAILS_NOT_CHANGED),
    ("folders-details-song-failed", DETAILS_NOT_CHANGED),
    ("folders-details-photo-failed", DETAILS_NOT_CHANGED),
    ("folders-details-taken-bad", DETAILS_NOT_CHANGED),
    ("folders-meta-link", DETAILS_NOT_CHANGED),
    ("folders-meta-damaged", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-out-of-reach", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-block-past-end", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-no-tiff", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-too-big", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-jpeg-parts", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-jpeg-past-end", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-png-past-end", DETAILS_NOT_CHANGED),
    ("folders-meta-bad-png-empty", DETAILS_NOT_CHANGED),
    ("folders-meta-refuse-date", DETAILS_NOT_CHANGED),
    ("folders-meta-refuse-nul", DETAILS_NOT_CHANGED),
    ("folders-meta-refuse-way-up", DETAILS_NOT_CHANGED),
    ("folders-meta-refuse-too-long", DETAILS_NOT_CHANGED),
    ("folders-meta-only-jpeg-png", DETAILS_NOT_CHANGED),
    ("folders-meta-no-room", DETAILS_NOT_CHANGED),
    ("folders-meta-tag-too-big", DETAILS_NOT_CHANGED),
    ("folders-imaging-no-turn", WORK_NOT_DONE),
    ("folders-imaging-unreadable", WORK_NOT_DONE),
    ("folders-imaging-no-write", WORK_NOT_DONE),
    ("folders-tools-rename-stopped", WORK_NOT_DONE),
    ("kit-proof-not-looked-through", WORK_NOT_DONE),
    ("kit-proof-not-taken-away", WORK_NOT_DONE),
    ("kit-proof-not-to-trash", WORK_NOT_DONE),
    ("kit-proof-not-moved-said", WORK_NOT_DONE),
    ("kit-proof-not-moved-reason", WORK_NOT_DONE),
    ("kit-proof-unreadable", WORK_NOT_DONE),
    ("kit-trash-drive-unsafe", WORK_NOT_DONE),
    ("kit-trash-nothing-to-move", WORK_NOT_DONE),
    ("kit-trash-too-many-names", WORK_NOT_DONE),
    ("folders-places-unread-denied", PLACE_UNREAD),
    ("folders-places-unread-gone", PLACE_UNREAD),
    ("folders-places-unread-other", PLACE_UNREAD),
    ("folders-app-nothing-at", PLACE_UNREAD),
    ("kit-reach-not-answering", PLACE_UNREAD),
    ("folders-act-colours-lost", COLOURS_LOST),
    ("folders-outside-gone", DRAG),
    ("folders-outside-inside-itself", DRAG),
    ("folders-outside-not-answering", DRAG),
    ("folders-outside-unshown", DRAG),
    ("folders-outside-short-move", DRAG),
    ("folders-outside-short-copy", DRAG),
    ("folders-outside-stayed-one", DRAG),
    ("folders-outside-stayed-changed", DRAG),
    ("folders-outside-stayed-many", DRAG),
    ("folders-outside-stayed-each", DRAG),
    ("folders-act-make-failed", MAKE_FAILED),
    ("folders-act-links-failed", MAKE_FAILED),
    ("folders-menu-wallpaper-refused", MAKE_FAILED),
    ("folders-menu-link-failed", MAKE_FAILED),
    ("folders-fonts-install-failed", MAKE_FAILED),
    ("folders-tools-diff-failed", COMPARE_STOPPED),
    ("folders-tools-dupes-stopped", COMPARE_STOPPED),
    ("folders-tools-compare-stopped", COMPARE_STOPPED),
    ("folders-tools-dupes-unread", COMPARE_STOPPED),
    ("folders-settings-not-saved", NOT_KEPT),
    ("folders-settings-not-saved-because", NOT_KEPT),
    ("folders-settings-not-read", NOT_KEPT),
    ("folders-settings-forget-failed", SHARE_LOGIN),
    ("folders-app-pins-not-saved", NOT_KEPT),
    ("folders-menu-ring-full", NOT_KEPT),
    ("folders-menu-ring-unkept", NOT_KEPT),
    ("folders-inplace-not-renamed", NAME_REFUSED),
    ("folders-archive-damaged-7z", ARCHIVE_NOT_OPENED),
    ("folders-archive-damaged-zip", ARCHIVE_NOT_OPENED),
    ("folders-archive-inside-too-big", ARCHIVE_NOT_OPENED),
    ("folders-archive-inside-missing", ARCHIVE_NOT_OPENED),
    ("folders-unarchive-damaged", ARCHIVE_NOT_OPENED),
    ("folders-archive-pack-grew", WORK_NOT_DONE),
    ("folders-archive-pack-no-name", WORK_NOT_DONE),
    ("folders-archive-pack-cannot-make", WORK_NOT_DONE),
    ("folders-archive-7z-a-lock-too-slow-to-open", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-lock-with-no-settings", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-lock-with-short-settings", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-lock-with-no-key", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-locked-block-is-not-whole", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-the-header-ends-too-soon", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-count-is-far-too-big", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-has-no-packed-bytes", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-is-out-of-reach", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-too-short", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-no-7z-mark", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-the-start-header-s-check-does-not-match", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-the-header-is-out-of-reach", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-the-header-s-check-does-not-match", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-packed-header-with-no-block", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-the-header-is-far-too-big", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-the-packed-header-s-check-does-not-match", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-an-unknown-header", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-headers-packed-too-many-times-over", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-an-unknown-part-in-the-header", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-an-unknown-part-in-the-streams", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-an-unknown-part-in-the-packed-streams", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-packed-streams-with-no-sizes", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-no-list-of-blocks", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-blocks-kept-elsewhere", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-blocks-with-no-sizes", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-an-unknown-part-in-the-blocks", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-with-no-sensible-coders", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-coder-with-too-many-ends", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-s-coders-do-not-join-up", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-far-too-many-files", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-files-in-a-block-with-no-sizes", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-files-bigger-than-their-block", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-an-unknown-part-in-the-files-sizes", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-part-is-far-too-big", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-names-kept-elsewhere", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-too-few-names", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-times-kept-elsewhere", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-attributes-kept-elsewhere", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-runs-past-the-end", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-s-coders-go-round-in-a-loop", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-lzma-with-no-settings", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-lzma2-with-no-settings", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-lzma2-with-impossible-settings", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-would-not-open", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-block-ends-too-soon", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-file-ends-too-soon", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-a-file-s-check-does-not-match", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-files-sizes-and-checks-do-not-match-up", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-more-files-than-blocks-hold", ARCHIVE_NOT_OPENED),
    ("folders-archive-7z-more-files-than-sizes", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-no-end-record", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-no-zip64-end-record", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-directory-out-of-place", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-directory-too-big", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-record-runs-past-the-directory", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-no-local-header", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-entry-runs-past-the-end", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-an-entry-s-check-does-not-match", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-an-entry-is-longer-than-it-says", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-a-locked-entry-is-cut-short", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-a-lock-with-a-key-of-no-known-length", ARCHIVE_NOT_OPENED),
    ("folders-archive-zip-a-locked-entry-s-check-does-not-match", ARCHIVE_NOT_OPENED),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::app::{Files, Msg};
    use crate::files::dialogs::Dialog;
    use crate::files::locked::Then;
    use crate::toolkit::ui::App;

    fn more() -> String {
        format!("button: {}", crate::toolkit::t!("common-more"))
    }

    #[test]
    fn every_page_is_used_and_every_message_names_one() {
        for page in PAGES.iter().copied() {
            assert!(MESSAGES.iter().any(|(_, a)| *a == page), "{page} has no message");
        }
        for (key, address) in MESSAGES {
            assert!(PAGES.contains(address), "{key} names {address}");
        }
    }

    #[test]
    fn every_archive_trouble_has_a_page() {
        use Trouble::*;
        for (t, page) in [(Locked, ARCHIVE_PASSWORD), (WrongKey, ARCHIVE_PASSWORD), (WrongOrBad, ARCHIVE_PASSWORD), (LockedRar, ARCHIVE_PASSWORD), (LockedForeign, ARCHIVE_PASSWORD),
                          (Unknown, ARCHIVE_NOT_OPENED), (Foreign, ARCHIVE_NOT_OPENED), (NoReader, ARCHIVE_NOT_OPENED), (Parts, ARCHIVE_NOT_OPENED),
                          (TooMany, ARCHIVE_NO_ROOM), (NoRoom, ARCHIVE_NO_ROOM), (GaveUp, ARCHIVE_NO_ROOM)] {
            assert_eq!(archive_page(&archive::trouble(t)), page, "{t:?}");
        }
        for (kind, page) in [(io::ErrorKind::InvalidData, ARCHIVE_NOT_OPENED), (io::ErrorKind::UnexpectedEof, ARCHIVE_NOT_OPENED), (io::ErrorKind::NotFound, ARCHIVE_NOT_OPENED),
                             (io::ErrorKind::Other, ARCHIVE_NOT_OPENED), (io::ErrorKind::StorageFull, ARCHIVE_NO_ROOM), (io::ErrorKind::OutOfMemory, ARCHIVE_NO_ROOM),
                             (io::ErrorKind::PermissionDenied, ARCHIVE_NO_ROOM), (io::ErrorKind::ReadOnlyFilesystem, ARCHIVE_NO_ROOM)] {
            assert_eq!(archive_page(&io::Error::new(kind, "x")), page, "{kind:?}");
        }
    }

    #[test]
    fn work_that_did_not_finish_has_the_page_for_its_kind() {
        use super::super::archive::Place;
        use super::super::imaging::Work;
        let archives = [Kind::Other(Extra::Unpack(Place::Here)), Kind::Other(Extra::TakeOut("a.zip".into()))];
        let others = [Kind::Copy, Kind::Move, Kind::Trash, Kind::Delete, Kind::Restore, Kind::Undo(Vec::new()), Kind::Other(Extra::Wipe(1)),
                      Kind::Other(Extra::Pack(archive::Format::Zip, "x".into())), Kind::Other(Extra::Picture(Work::Turn(super::super::imaging::Turn::Left)))];
        assert!(archives.iter().all(|k| work_page(k) == ARCHIVE_NOT_OPENED));
        assert!(others.iter().all(|k| work_page(k) == WORK_NOT_DONE));
    }

    #[test]
    fn a_failure_shows_a_more_that_opens_its_page_and_the_next_saying_takes_it_away() {
        let dir = std::env::temp_dir().join(format!("trindle-files-help-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("fake.rar"), b"not a rar").unwrap();
        let mut f = Files::new(vec![dir.join("fake.rar")]);
        f.load(0, 0);
        assert!(f.bad);
        assert_eq!(f.said_help, Some(ARCHIVE_NOT_OPENED));
        assert!(super::super::help_cases::window_has_more(&super::super::view::view(&f).said()), "{:?}", f.said);
        f.update(Msg::Help(ARCHIVE_NOT_OPENED));
        assert_eq!(f.helped, [ARCHIVE_NOT_OPENED]);
        f.say("Copied.", false);
        assert_eq!(f.said_help, None);
        assert!(!super::super::help_cases::window_has_more(&super::super::view::view(&f).said()));
        f.say("Something else.", true);
        assert_eq!(f.said_help, None);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_password_card_has_a_more_beside_a_wrong_password() {
        let dir = std::env::temp_dir().join(format!("trindle-files-help-card-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let mut f = Files::new(vec![dir.clone()]);
        for (wrong, has) in [(None, false), (Some(Trouble::WrongKey), true), (Some(Trouble::WrongOrBad), true)] {
            f.dialog = None;
            f.unlock_ask(dir.join("a.zip"), Then::List, wrong);
            let Some(Dialog::Unlock(u)) = &f.dialog else { panic!("no card") };
            assert_eq!(super::super::locked::view(u, f.turn).said().contains(&more()), has, "{wrong:?}");
        }
    }

    #[test]
    fn a_missing_part_and_a_program_that_would_not_start_each_have_their_page() {
        let mut f = Files::new(vec![std::env::temp_dir()]);
        f.say_help(super::super::alone::no_terminal(), PART_MISSING);
        assert_eq!((f.said_help, f.bad), (Some(PART_MISSING), true));
        f.say_help("x", NOT_STARTED);
        assert_eq!(f.said_help, Some(NOT_STARTED));
        f.say_help("x", DRIVE);
        assert_eq!(f.said_help, Some(DRIVE));
        f.say_help("x", NOT_KEPT);
        assert_eq!(f.said_help, Some(NOT_KEPT));
    }
}
