#!/usr/bin/env python3
# Makes the password-locked zip and 7z test archives kept beside this file.

import hashlib
import hmac
import lzma
import struct
import subprocess
import sys
import zlib

PASSWORD = "john-smith-test"
HELLO = b"Hello from John Smith.\n"
NOTE = b"Second note, a little longer than the first one.\n" * 2
WHEN = 1767323045
DOS_TIME = (3 << 11) | (4 << 5) | (5 // 2)
DOS_DATE = ((2026 - 1980) << 9) | (1 << 5) | 2


def salt(tag, n):
    return hashlib.sha256(b"trindle test salt " + tag).digest()[:n]


def openssl(cipher, key, data, iv=None, decrypt=False):
    cmd = ["openssl", "enc", "-" + cipher, "-K", key.hex(), "-nopad"]
    if iv is not None:
        cmd += ["-iv", iv.hex()]
    if decrypt:
        cmd.append("-d")
    return subprocess.run(cmd, input=data, capture_output=True, check=True).stdout



CRC_TABLE = []
for i in range(256):
    c = i
    for _ in range(8):
        c = (c >> 1) ^ 0xEDB88320 if c & 1 else c >> 1
    CRC_TABLE.append(c)


def old_lock(data, check, tag):
    """Zip's first lock: twelve bytes in front, the last a check."""
    k = [0x12345678, 0x23456789, 0x34567890]

    def stir(b):
        k[0] = CRC_TABLE[(k[0] ^ b) & 0xFF] ^ (k[0] >> 8)
        k[1] = ((k[1] + (k[0] & 0xFF)) * 134775813 + 1) & 0xFFFFFFFF
        k[2] = CRC_TABLE[(k[2] ^ (k[1] >> 24)) & 0xFF] ^ (k[2] >> 8)

    for b in PASSWORD.encode():
        stir(b)
    out = bytearray()
    for b in salt(tag, 11) + bytes([check]) + data:
        t = (k[2] | 2) & 0xFFFF
        out.append(b ^ (((t * (t ^ 1)) >> 8) & 0xFF))
        stir(b)
    return bytes(out)


def aes_lock(data, strength, tag):
    """WinZip's AES: salt, two check bytes, the bytes, ten bytes of HMAC."""
    salt_len, key_len = {1: (8, 16), 2: (12, 24), 3: (16, 32)}[strength]
    s = salt(tag, salt_len)
    made = hashlib.pbkdf2_hmac("sha1", PASSWORD.encode(), s, 1000, 2 * key_len + 2)
    key, mac_key, fits = made[:key_len], made[key_len:2 * key_len], made[2 * key_len:]
    blocks = (len(data) + 15) // 16
    counters = b"".join((i + 1).to_bytes(16, "little") for i in range(blocks))
    pad = openssl("aes-%d-ecb" % (key_len * 8), key, counters) if blocks else b""
    locked = bytes(a ^ b for a, b in zip(data, pad))
    return s + fits + locked + hmac.new(mac_key, locked, hashlib.sha1).digest()[:10]


def deflate(data):
    z = zlib.compressobj(9, zlib.DEFLATED, -15)
    return z.compress(data) + z.flush()


def zip_of(entries):
    """entries: (name, mode, data, how) where how is None for open, "old",
    "old-late" (its CRC written after its bytes) or ("aes", strength, form)."""
    out = bytearray()
    central = bytearray()
    for name, mode, data, how in entries:
        raw = name.encode()
        folder = name.endswith("/")
        crc = zlib.crc32(data)
        method = 0 if folder or len(data) < 40 else 8
        body = deflate(data) if method == 8 else data
        flags, extra, said_crc, said_method = 0x800, b"", crc, method
        if how == "old":
            flags |= 1
            body = old_lock(body, crc >> 24, raw)
        elif how == "old-late":
            flags |= 1 | 8
            body = old_lock(body, DOS_TIME >> 8, raw)
        elif how:
            _, strength, form = how
            flags |= 1
            body = aes_lock(body, strength, raw)
            extra = struct.pack("<HHH2sBH", 0x9901, 7, form, b"AE", strength, method)
            said_method = 99
            if form == 2:
                said_crc = 0
        late = flags & 8 != 0
        at = len(out)
        out += struct.pack("<IHHHHHIIIHH", 0x04034B50, 20, flags, said_method, DOS_TIME, DOS_DATE,
                           0 if late else said_crc, 0 if late else len(body), 0 if late else len(data), len(raw), len(extra))
        out += raw + extra + body
        if late:
            out += struct.pack("<IIII", 0x08074B50, said_crc, len(body), len(data))
        central += struct.pack("<IHHHHHHIIIHHHHHII", 0x02014B50, 0x031E, 20, flags, said_method, DOS_TIME, DOS_DATE,
                               said_crc, len(body), len(data), len(raw), len(extra), 0, 0, 0, mode << 16, at)
        central += raw + extra
    out += central + struct.pack("<IHHHHIIH", 0x06054B50, 0, 0, len(entries), len(entries), len(central), len(out), 0)
    return bytes(out)



AES_ID = bytes([0x06, 0xF1, 0x07, 0x01])


def num(v):
    for i in range(8):
        if v < 1 << (7 * i + 7):
            first = ((0xFF << (8 - i)) & 0xFF) | (v >> (8 * i))
            return bytes([first]) + (v & ((1 << (8 * i)) - 1)).to_bytes(i, "little")
    return b"\xff" + v.to_bytes(8, "little")


def sz_key(power, s):
    typed = PASSWORD.encode("utf-16-le")
    h = hashlib.sha256()
    for i in range(1 << power):
        h.update(s + typed + i.to_bytes(8, "little"))
    return h.digest()


def sz_lock(data, power, s, iv):
    """7z's lock: AES-256 in CBC, padded with noughts; and its settings."""
    padded = data + bytes(-len(data) % 16)
    locked = openssl("aes-256-cbc", sz_key(power, s), padded, iv + bytes(16 - len(iv)))
    first = power | (0x80 if s else 0) | (0x40 if iv else 0)
    props = bytes([first])
    if s or iv:
        props += bytes([((len(s) - 1 if s else 0) << 4) | (len(iv) - 1 if iv else 0)]) + s + iv
    return locked, props


def coder(ident, props):
    return bytes([len(ident) | (0x20 if props else 0)]) + ident + (num(len(props)) + props if props else b"")


def streams(pack_pos, pack_size, coders, sizes, crc, files=None):
    """The part of a header that says where one block is and how it is
    packed. coders run from the one giving the plain bytes to the lock,
    which reads the packed bytes."""
    out = b"\x06" + num(pack_pos) + num(1) + b"\x09" + num(pack_size) + b"\x00"
    out += b"\x07\x0b" + num(1) + b"\x00" + num(len(coders)) + b"".join(coders)
    for i in range(len(coders) - 1):
        out += num(i) + num(i + 1)
    out += b"\x0c" + b"".join(num(n) for n in sizes)
    if files is None:
        out += b"\x0a\x01" + struct.pack("<I", crc)
    out += b"\x00"
    if files is not None:
        out += b"\x08\x0d" + num(len(files))
        if len(files) > 1:
            out += b"\x09" + b"".join(num(len(f)) for f in files[:-1])
        out += b"\x0a\x01" + b"".join(struct.pack("<I", zlib.crc32(f)) for f in files) + b"\x00"
    return out + b"\x00"


def bits(flags):
    out = bytearray((len(flags) + 7) // 8)
    for i, on in enumerate(flags):
        if on:
            out[i // 8] |= 0x80 >> (i % 8)
    return bytes(out)


def sevenz(things, squeeze, power, lock_names, odd=False):
    """things: (name, mode, data or None for a folder). The files with bytes
    go in one locked block, in the order given."""
    full = [t for t in things if t[2]]
    empty = [t for t in things if not t[2]]
    order = full + empty
    plain = b"".join(t[2] for t in full)
    if squeeze:
        packed = lzma.compress(plain, lzma.FORMAT_RAW, filters=[{"id": lzma.FILTER_LZMA2, "dict_size": 1 << 16}])
        first = coder(bytes([0x03, 0x04, 0x01]), bytes(5)) if odd else coder(bytes([0x21]), bytes([8]))
    else:
        packed, first = plain, None
    locked, props = sz_lock(packed, power, b"", salt(b"data iv", 8))
    coders = ([first] if first else []) + [coder(AES_ID, props)]
    sizes = ([len(plain)] if first else []) + [len(packed)]
    head = b"\x01\x04" + streams(0, len(locked), coders, sizes, None, [t[2] for t in full])
    props_out = b""

    def prop(kind, data):
        return bytes([kind]) + num(len(data)) + data

    if empty:
        props_out += prop(0x0E, bits([not t[2] for t in order]))
        props_out += prop(0x0F, bits([t[2] is not None for t in empty]))
    props_out += prop(0x11, b"\x00" + b"".join(t[0].encode("utf-16-le") + b"\x00\x00" for t in order))
    stamp = struct.pack("<Q", (WHEN + 11644473600) * 10_000_000)
    props_out += prop(0x14, b"\x01\x00" + stamp * len(order))
    attrib = b"".join(struct.pack("<I", 0x8000 | (t[1] << 16) | (0x10 if t[1] & 0o170000 == 0o040000 else 0x20)) for t in order)
    props_out += prop(0x15, b"\x01\x00" + attrib)
    head += b"\x05" + num(len(order)) + props_out + b"\x00\x00"
    body = locked
    if lock_names:
        lc_lp_pb = {"lc": 3, "lp": 0, "pb": 2}
        small = lzma.compress(head, lzma.FORMAT_RAW, filters=[{"id": lzma.FILTER_LZMA1, "dict_size": 1 << 16, **lc_lp_pb}])
        shut, props = sz_lock(small, power, salt(b"head salt", 4), salt(b"head iv", 16))
        coders = [coder(bytes([0x03, 0x01, 0x01]), bytes([0x5D]) + struct.pack("<I", 1 << 16)), coder(AES_ID, props)]
        outer = b"\x17" + streams(len(body), len(shut), coders, [len(head), len(small)], zlib.crc32(head))
        body += shut
        head = outer
    start = struct.pack("<QQI", len(body), len(head), zlib.crc32(head))
    return b"7z\xbc\xaf\x27\x1c\x00\x04" + struct.pack("<I", zlib.crc32(start)) + start + body + head


FILE, FOLDER, LINK = 0o100644, 0o040755, 0o120777


def main(out):
    plain = [("hello.txt", FILE, HELLO), ("docs/note.txt", FILE, NOTE), ("docs/blank.txt", FILE, b""), ("docs", FOLDER, None)]
    made = {
        "locked-old.zip": zip_of([("docs/", FOLDER, b"", None), ("docs/note.txt", FILE, NOTE, "old-late"),
                                  ("hello.txt", FILE, HELLO, "old"), ("open.txt", FILE, b"Not locked.\n", None)]),
        "locked-aes.zip": zip_of([("docs/", FOLDER, b"", None), ("docs/note.txt", FILE, NOTE, ("aes", 1, 1)),
                                  ("hello.txt", FILE, HELLO, ("aes", 3, 2)), ("docs/copy.txt", FILE, NOTE + HELLO, ("aes", 2, 2))]),
        "locked.7z": sevenz(plain, True, 19, False),
        "locked-names.7z": sevenz(plain, True, 8, True),
        "locked-odd.7z": sevenz(plain, True, 6, False, odd=True),
        "locked-evil.7z": sevenz([("../escape.txt", FILE, b"out"), ("/abs.txt", FILE, b"abs"), ("ok.txt", FILE, b"fine"),
                                  ("link-out", LINK, b"../../outside"), ("sub/tool", 0o104755, b"#!/bin/sh\n"),
                                  ("sub", FOLDER, None)], False, 6, True),
    }
    for name, data in made.items():
        with open(out + "/" + name, "wb") as f:
            f.write(data)
        print(name, len(data))


main(sys.argv[1])
