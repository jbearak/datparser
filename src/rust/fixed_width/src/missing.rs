//! Stata's numeric missing values, which SAS's `.` and `.A` to `.Z`
//! become.
//!
//! A *missing code* is a `u8`: 0 for a value that is not missing, 1 for
//! system missing `.`, and 2 to 27 for `.a` to `.z`.

pub const NOT_MISSING: u8 = 0;
pub const SYSTEM: u8 = 1;
/// The largest code, `.z`.
pub const LAST: u8 = 27;

/// The name of a missing code: `.`, `.a`, ... `.z`.
pub fn name(code: u8) -> Option<String> {
    match code {
        SYSTEM => Some(".".into()),
        2..=LAST => Some(format!(".{}", (b'a' + code - 2) as char)),
        _ => None,
    }
}

/// The code of a name such as `.` or `.q`.
pub fn from_name(name: &str) -> Option<u8> {
    match name.as_bytes() {
        b"." => Some(SYSTEM),
        [b'.', c @ b'a'..=b'z'] => Some(c - b'a' + 2),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for code in SYSTEM..=LAST {
            assert_eq!(from_name(&name(code).unwrap()), Some(code));
        }
        assert_eq!(name(2).unwrap(), ".a");
        assert_eq!(name(27).unwrap(), ".z");
        assert_eq!(from_name(".A"), None);
    }
}
