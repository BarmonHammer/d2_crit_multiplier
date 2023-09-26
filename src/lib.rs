#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

///This struct is a wrapper over Mossy's crit multiplier math.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(rename_all = "snake_case", try_from = "Option<i32>")
)]
#[derive(Debug, Copy, Clone)]
pub struct CritMultiplier(#[cfg_attr(feature = "serde", serde(default))] Option<i32>);

#[derive(Error, Debug)]
pub enum CritMultiplierErr {
    #[error("CritMultiplier must be within -25 and 99, value provided was {}", .0)]
    OutOfRange(i32),
}

///Converts a i32 to the CritMultiplier wrapper
/// Will return an error if the values is not between -25 and 99
impl TryFrom<Option<i32>> for CritMultiplier {
    type Error = CritMultiplierErr;
    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(CritMultiplier(value)),
            Some(x) if (-25..=99).contains(&x) => Ok(CritMultiplier(value)),
            Some(x) => Err(CritMultiplierErr::OutOfRange(x)),
        }
    }
}

impl From<CritMultiplier> for f64 {
    fn from(value: CritMultiplier) -> Self {
        if let Some(x) = value.0 {
            x as f64 / 51.0 + 1.5
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::CritMultiplier;

    #[test]
    #[cfg(feature = "serde")]
    fn serialize_value() {
        let x = CritMultiplier::try_from(Some(2)).unwrap();
        let test = serde_json::to_string_pretty(&x).unwrap();
        assert_eq!("2", test.as_str());
    }
    #[test]
    fn serialize_none() {
        let x = CritMultiplier::try_from(None).unwrap();
        let test = serde_json::to_string_pretty(&x).unwrap();
        assert_eq!("null", test.as_str());
    }
    #[test]
    #[cfg(feature = "serde")]
    fn null_deserialize() {
        let a: CritMultiplier = serde_json::from_str("null").unwrap();
        let b = CritMultiplier::try_from(None).unwrap();

        let x: f64 = a.into();
        let y: f64 = b.into();

        assert_eq!(x, y);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn value_deserialize() {
        let a: CritMultiplier = serde_json::from_str("5").unwrap();
        let b = CritMultiplier::try_from(Some(5)).unwrap();

        let x: f64 = a.into();
        let y: f64 = b.into();

        assert_eq!(x, y);
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "serde")]
    fn lower_bounds_deserialize() {
        let _: CritMultiplier = serde_json::from_str("-26").unwrap();
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "serde")]
    fn upper_bounds_deserialize() {
        let _: CritMultiplier = serde_json::from_str("100").unwrap();
    }

    #[test]
    fn normal_use() {
        let x = CritMultiplier::try_from(Some(0)).unwrap();
        let y: f64 = x.into();
        assert_eq!(y, 1.5);
    }
    #[test]
    fn none_use() {
        let x = CritMultiplier::try_from(None).unwrap();
        let y: f64 = x.into();
        assert_eq!(y, 1.0);
    }

    #[test]
    #[should_panic]
    fn lower_bounds() {
        let _ = CritMultiplier::try_from(Some(-26)).unwrap();
    }

    #[test]
    #[should_panic]
    fn upper_bounds() {
        let _ = CritMultiplier::try_from(Some(100)).unwrap();
    }

    #[test]
    fn test_constructor() {
        let _ = CritMultiplier(Some(2));
        let _ = CritMultiplier(None);
    }
}
