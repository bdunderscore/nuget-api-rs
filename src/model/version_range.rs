use std::str::FromStr;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::{AsBytes, IResult};
use nom::branch::alt;
use nom::sequence::tuple;

use crate::Error;

#[derive(Clone, Eq, PartialEq)]
pub enum RangeSpecifier {
    Open,
    Inclusive(String),
    Exclusive(String),
}

#[derive(Clone, Eq, PartialEq)]
pub struct VersionRange {
    pub low: RangeSpecifier,
    pub high: RangeSpecifier
}

fn version_ident(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::take_while1(|c| (c as char).is_ascii_alphanumeric() || c == b'-' || c == b'.' || c == b'+')(input)
}

fn whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::take_while(|c| (c as char).is_ascii_whitespace())(input)
}

fn full_specifier(input: &[u8]) -> IResult<&[u8], VersionRange> {
    let (remain, (_, lo_tag, _, lo, _, _, _, hi, _, hi_tag, _)) = tuple((
        whitespace,
        alt((tag("["), tag("("))),
        whitespace,
        opt(version_ident),
        whitespace,
        tag(b","),
        whitespace,
        opt(version_ident),
        whitespace,
        alt((tag("]"), tag(")"))),
        whitespace
    ))(input.as_bytes())?;

    let lo = match (lo_tag, lo) {
        (_, None) => RangeSpecifier::Open,
        (b"[", Some(v)) => RangeSpecifier::Inclusive(String::from_utf8(v.to_owned()).unwrap()),
        (b"(", Some(v)) => RangeSpecifier::Exclusive(String::from_utf8(v.to_owned()).unwrap()),
        _ => unreachable!()
    };

    let hi = match (hi_tag, hi) {
        (_, None) => RangeSpecifier::Open,
        (b"]", Some(v)) => RangeSpecifier::Inclusive(String::from_utf8(v.to_owned()).unwrap()),
        (b")", Some(v)) => RangeSpecifier::Exclusive(String::from_utf8(v.to_owned()).unwrap()),
        _ => unreachable!()
    };

    Ok((remain, VersionRange {
        low: lo,
        high: hi
    }))
}

fn exact_match(input: &[u8]) -> IResult<&[u8], VersionRange> {
    let (remain, (_, _, _, version, _, _, _)) = tuple((
        whitespace,
        tag(b"["),
        whitespace,
        version_ident,
        whitespace,
        tag(b"]"),
        whitespace
    ))(input)?;

    Ok((remain, VersionRange {
        low: RangeSpecifier::Inclusive(String::from_utf8(version.to_owned()).unwrap()),
        high: RangeSpecifier::Inclusive(String::from_utf8(version.to_owned()).unwrap())
    }))
}

fn min_version_bare(input: &[u8]) -> IResult<&[u8], VersionRange> {
    let (remain, version) = version_ident(input)?;

    Ok((remain, VersionRange {
        low: RangeSpecifier::Inclusive(String::from_utf8(version.to_owned()).unwrap()),
        high: RangeSpecifier::Open
    }))
}

impl FromStr for VersionRange {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = alt((full_specifier, exact_match, min_version_bare))(s.as_bytes());

        match result {
            Ok((b"", v)) => Ok(v),
            _ => Err(Error::InvalidVersionRange(s.to_owned())),
        }
    }
}

impl std::fmt::Debug for VersionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "VersionRange({})", self)
    }
}

impl std::fmt::Display for VersionRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.low {
            RangeSpecifier::Open => write!(f, "[")?,
            RangeSpecifier::Inclusive(v) => write!(f, "[{} ", v)?,
            RangeSpecifier::Exclusive(v) => write!(f, "({} ", v)?,
        }

        write!(f, ", ")?;

        match &self.high {
            RangeSpecifier::Open => write!(f, "]")?,
            RangeSpecifier::Inclusive(v) => write!(f, "{}]", v)?,
            RangeSpecifier::Exclusive(v) => write!(f, "{})", v)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::model::{RangeSpecifier, VersionRange};

    #[test]
    fn test_vecs() {
        assert_eq!("1.0".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Inclusive("1.0".to_owned()),
            high: RangeSpecifier::Open
        });

        assert_eq!("[1.0]".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Inclusive("1.0".to_owned()),
            high: RangeSpecifier::Inclusive("1.0".to_owned())
        });

        assert_eq!("[1.0, )".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Inclusive("1.0".to_owned()),
            high: RangeSpecifier::Open
        });
        assert_eq!("  [ 1.0  ,)  ".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Inclusive("1.0".to_owned()),
            high: RangeSpecifier::Open
        });
        assert_eq!("(1.0, )".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Exclusive("1.0".to_owned()),
            high: RangeSpecifier::Open
        });
        assert_eq!("(,1.0]".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Open,
            high: RangeSpecifier::Inclusive("1.0".to_owned())
        });
        assert_eq!("(,1.0)".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Open,
            high: RangeSpecifier::Exclusive("1.0".to_owned())
        });
        assert_eq!("[1.0,2.0]".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Inclusive("1.0".to_owned()),
            high: RangeSpecifier::Inclusive("2.0".to_owned())
        });
        assert_eq!("(1.0,2.0)".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Exclusive("1.0".to_owned()),
            high: RangeSpecifier::Exclusive("2.0".to_owned())
        });
        assert_eq!("[1.0,2.0)".parse::<VersionRange>().unwrap(), VersionRange {
            low: RangeSpecifier::Inclusive("1.0".to_owned()),
            high: RangeSpecifier::Exclusive("2.0".to_owned())
        });
        assert!(VersionRange::from_str("(1.0)").is_err());
    }
}