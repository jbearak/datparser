//! Text that is not UTF-8, read as Windows-1252 (see ADR 0004 of
//! dta-parser, where this code comes from).

/// `bytes` decoded as Windows-1252.
pub(crate) fn windows_1252(bytes: &[u8]) -> String {
    if bytes.is_ascii() {
        // ASCII is valid UTF-8.
        return String::from_utf8_lossy(bytes).into_owned();
    }
    bytes
        .iter()
        .map(|&b| match b {
            0x80..=0x9f => WINDOWS_1252[(b - 0x80) as usize],
            _ => b as char,
        })
        .collect()
}

/// Windows-1252 bytes 0x80 to 0x9f. The five bytes the code page leaves
/// undefined map to the C1 control of the same number, as Latin-1 does.
const WINDOWS_1252: [char; 32] = [
    '\u{20ac}', '\u{81}', '\u{201a}', '\u{192}', '\u{201e}', '\u{2026}', '\u{2020}', '\u{2021}',
    '\u{2c6}', '\u{2030}', '\u{160}', '\u{2039}', '\u{152}', '\u{8d}', '\u{17d}', '\u{8f}',
    '\u{90}', '\u{2018}', '\u{2019}', '\u{201c}', '\u{201d}', '\u{2022}', '\u{2013}', '\u{2014}',
    '\u{2dc}', '\u{2122}', '\u{161}', '\u{203a}', '\u{153}', '\u{9d}', '\u{17e}', '\u{178}',
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_windows_1252() {
        assert_eq!(windows_1252(&[b'a', 0xe9, 0x80]), "aé€");
        assert_eq!(windows_1252(b"plain"), "plain");
    }
}
