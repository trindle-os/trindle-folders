//! The DOS code page and dates that zip names and times files in.

use crate::common::clock::Local;

pub(in crate::files::archive) const CP437_HIGH: &str = "ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■\u{a0}";

pub(in crate::files::archive) fn cp437(b: u8) -> char {
    if b < 128 { char::from(b) } else { CP437_HIGH.chars().nth(usize::from(b - 128)).unwrap_or('?') }
}

pub(in crate::files::archive) fn from_dos(time: u16, date: u16) -> Option<i64> {
    let (y, m, d) = (i64::from(date >> 9) + 1980, i64::from((date >> 5) & 15), i64::from(date & 31));
    if !(1..=12).contains(&m) || d == 0 {
        return None;
    }
    let naive = days_from_civil(y, m, d) * 86_400 + i64::from(time >> 11) * 3600 + i64::from((time >> 5) & 63) * 60 + i64::from(time & 31) * 2;
    let guess = naive - Local::at(naive).utc_offset();
    Some(naive - Local::at(guess).utc_offset())
}

pub(in crate::files::archive) fn to_dos(unix: i64) -> (u16, u16) {
    let t = Local::at(unix);
    if t.year() < 1980 {
        return (0, (1 << 5) | 1);
    }
    if t.year() > 2107 {
        return ((23 << 11) | (59 << 5) | 29, (127 << 9) | (12 << 5) | 31);
    }
    let time = (t.hour() << 11) | (t.minute() << 5) | (t.second() / 2);
    let date = ((t.year() - 1980) << 9) | (t.month() << 5) | t.day();
    (time as u16, date as u16)
}

pub(in crate::files::archive) fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}
