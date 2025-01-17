mod duration_hand;
mod duration_nom;

use std::str::FromStr;

const SECS_PER_MINUTE: u64 = 60;
const SECS_PER_HOUR: u64 = 60 * SECS_PER_MINUTE;
const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;
const SECS_PER_WEEK: u64 = 7 * SECS_PER_DAY;

pub use self::duration_hand::parse;
pub use self::duration_nom::parse as parse_nom;

/// AsRef<str> but implementable on nom types
/// Workaround for https://github.com/Geal/nom/pull/753
trait Strable {
    fn as_str(&self) -> &str;
}

impl<'s> Strable for &'s str {
    fn as_str(&self) -> &str {
        self
    }
}

impl Strable for dyn AsRef<str> {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

fn to_nanos<S: Strable>(s: S) -> Option<u32> {
    let s = s.as_str();

    const NANO_DIGITS: usize = 9;
    if s.len() > NANO_DIGITS {
        return None;
    }

    let extra_zeros = (NANO_DIGITS - s.len()) as u32;
    let mul = 10u32.pow(extra_zeros);
    match u32::from_str(s) {
        Ok(num) => Some(num * mul),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nanos() {
        use super::to_nanos;
        assert_eq!(0, to_nanos("0").unwrap());
        assert_eq!(0, to_nanos("000").unwrap());

        assert_eq!(1, to_nanos("000000001").unwrap());
        assert_eq!(10, to_nanos("00000001").unwrap());
        assert_eq!(100, to_nanos("0000001").unwrap());
        assert_eq!(1000, to_nanos("000001").unwrap());
        assert_eq!(10000, to_nanos("00001").unwrap());
        assert_eq!(100000, to_nanos("0001").unwrap());
        assert_eq!(1000000, to_nanos("001").unwrap());
        assert_eq!(10000000, to_nanos("01").unwrap());
        assert_eq!(100000000, to_nanos("1").unwrap());

        assert_eq!(7_010, to_nanos("00000701").unwrap());
    }
}
