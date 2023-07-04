use regex::Regex;
use std::convert::TryFrom;
use std::fmt;
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Aircraft {
    pub registration_number: String,
    pub model: String,
    pub competition_number: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseAircraftError {
    #[error("invalid format")]
    InvalidFormat,
}

impl fmt::Display for Aircraft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.competition_number {
            Some(c_number) => write!(f, "{} {} - {}", self.registration_number, self.model, c_number),
            None => write!(f, "{} {}", self.registration_number, self.model),
        }
    }
}

impl TryFrom<&str> for Aircraft {
    type Error = ParseAircraftError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(\w+-\w+)\s+(\w+)\s*(-\s*(\w+))?").unwrap();
        match re.captures(value.trim()) {
            Some(captures) => {
                let registration_number = captures.get(1).unwrap().as_str().to_string();
                let model = captures.get(2).unwrap().as_str().to_string();
                let competition_number = captures.get(4).map(|m| m.as_str().to_string());

                Ok(Aircraft {
                    registration_number,
                    model,
                    competition_number,
                })
            },
            None => Err(ParseAircraftError::InvalidFormat),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_aircraft() {
        let test_cases = vec![
            ("HB-3187 LS4 - 9F", Aircraft {
                registration_number: "HB-3187".to_string(),
                model: "LS4".to_string(),
                competition_number: Some("9F".to_string()),
            }),
            ("HB-2505 DG1001M - FB", Aircraft {
                registration_number: "HB-2505".to_string(),
                model: "DG1001M".to_string(),
                competition_number: Some("FB".to_string()),
            }),
            ("HB-3370 ASW28", Aircraft {
                registration_number: "HB-3370".to_string(),
                model: "ASW28".to_string(),
                competition_number: None,
            }),
            ("HB-3284 LS8 - XF", Aircraft {
                registration_number: "HB-3284".to_string(),
                model: "LS8".to_string(),
                competition_number: Some("XF".to_string()),
            }),
            ("HB-3360 LS8 - 12F", Aircraft {
                registration_number: "HB-3360".to_string(),
                model: "LS8".to_string(),
                competition_number: Some("12F".to_string()),
            }),
            ("HB-3473 DG1001C - BF3", Aircraft {
                registration_number: "HB-3473".to_string(),
                model: "DG1001C".to_string(),
                competition_number: Some("BF3".to_string()),
            }),
            ("HB-2087 G109", Aircraft {
                registration_number: "HB-2087".to_string(),
                model: "G109".to_string(),
                competition_number: None,
            }),
            ("HB-3472 DG1001C- BF2", Aircraft {
                registration_number: "HB-3472".to_string(),
                model: "DG1001C".to_string(),
                competition_number: Some("BF2".to_string()),
            }),
            ("HB-212 S18", Aircraft {
                registration_number: "HB-212".to_string(),
                model: "S18".to_string(),
                competition_number: None,
            }),
        ];

        for (input, expected) in test_cases {
            let aircraft = Aircraft::try_from(input).unwrap();
            assert_eq!(aircraft, expected);
        }
    }

    #[test]
    fn test_invalid_format() {
        let invalid_input = "invalid format";
        match Aircraft::try_from(invalid_input) {
            Ok(_) => panic!("Expected error, but got ok"),
            Err(e) => assert_eq!(e, ParseAircraftError::InvalidFormat),
        }
    }
}
