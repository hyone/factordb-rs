use reqwest;
use serde_json;
use std::iter;
use url::Url;
#[cfg(test)]
use mockito;

use self::errors::{Result, ResultExt};

#[cfg(not(test))]
const ENDPOINT: &'static str = "https://factordb.com/api";

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct FactorResult {
    id: String,
    status: String,
    factors: Vec<(String, usize)>,
}

pub fn request(num: u64) -> Result<FactorResult> {
    let params = [("query", num.to_string())];
    let uri = Url::parse_with_params(ENDPOINT, &params)
        .chain_err(|| "Failed to parse URL")?;

    reqwest::get(uri)
        .and_then(|r| r.error_for_status())
        .chain_err(|| "Failed to request")?
        .json::<FactorResult>()
        .chain_err(|| "Failed to parse JSON")
}

impl FactorResult {
    pub fn factor_list(&self) -> Vec<u64> {
        self.factors
            .iter()
            .map(|&(ref num_str, count)| {
                let num = num_str.parse::<u64>().unwrap();
                iter::repeat(num).take(count)
            })
            .flat_map(|v| v)
            .collect()
    }

    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub mod errors {
    error_chain!{}
}

#[cfg(test)]
const ENDPOINT: &'static str = mockito::SERVER_URL;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn test_request() {
        let _m = mock("GET", "/?query=336")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"336","status":"FF","factors":[["2",4],["3",1],["7",1]]}"#)
            .create();

        assert_eq!(
            request(336).expect("succeed to request"),
            FactorResult {
                id: "336".into(),
                status: "FF".into(),
                factors: vec![
                    ("2".into(), 4),
                    ("3".into(), 1),
                    ("7".into(), 1),
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "Failed to request")]
    fn test_request_failure() {
        let _m = mock("GET", "/?query=336")
            .with_status(500)
            .create();

        request(336).unwrap_or_else(|err| {
            panic!(err.description().to_owned());
        });
    }

    #[test]
    fn test_factor_list() {
        let result = FactorResult {
            id: "336".into(),
            status: "FF".into(),
            factors: vec![
                ("2".into(), 4),
                ("3".into(), 1),
                ("7".into(), 1),
            ]
        };
        assert_eq!(result.factor_list(), vec![2, 2, 2, 2, 3, 7]);
    }

    #[test]
    fn test_json() {
        let result = FactorResult {
            id: "336".into(),
            status: "FF".into(),
            factors: vec![
                ("2".into(), 4),
                ("3".into(), 1),
                ("7".into(), 1),
            ]
        };
        assert_eq!(
            result.json(),
            r#"{"id":"336","status":"FF","factors":[["2",4],["3",1],["7",1]]}"#
        );
    }
}
