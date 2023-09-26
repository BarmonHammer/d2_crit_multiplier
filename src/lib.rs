use serde::{Deserialize, Serialize};
use thiserror::Error;

///This struct is a wrapper over Mossy's crit multiplier math.
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case", try_from = "Option<i32>")]
pub struct CritMultiplier(Option<i32>);

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
        if let Some(x) = value {
            if !(-25..=99).contains(&x) {
                return Err(CritMultiplierErr::OutOfRange(x));
            }
        }
        Ok(CritMultiplier(value))
    }
}

impl From<CritMultiplier> for f64 {
    fn from(value: CritMultiplier) -> Self {
        if let Some(x) = value.0 {
            return x as f64 / 51.0 + 1.5;
        }
        1.0
    }
}

#[cfg(test)]
mod test {
    use crate::CritMultiplier;

    #[test]
    fn serialize() {
        let x = CritMultiplier::try_from(Some(2)).unwrap();
        let test = serde_json::to_string_pretty(&x).unwrap();
        assert_eq!("2", test.as_str());
    }
    #[test]
    fn null_deserialize() {
        let a: CritMultiplier = serde_json::from_str("null").unwrap();
        let b = CritMultiplier::try_from(None).unwrap();

        let x: f64 = a.into();
        let y: f64 = b.into();

        assert_eq!(x, y);
    }

    #[test]
    fn value_deserialize() {
        let a: CritMultiplier = serde_json::from_str("5").unwrap();
        let b = CritMultiplier::try_from(Some(5)).unwrap();

        let x: f64 = a.into();
        let y: f64 = b.into();

        assert_eq!(x, y);
    }

    #[test]
    #[should_panic]
    fn lower_bounds_deserialize() {
        let _: CritMultiplier = serde_json::from_str("-26").unwrap();
    }

    #[test]
    #[should_panic]
    fn upper_bounds_deserialize() {
        let _: CritMultiplier = serde_json::from_str("100").unwrap();
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
}
