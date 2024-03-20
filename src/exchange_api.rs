use anyhow::{anyhow, Context, Result};
use reqwest::{self, blocking::Response};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;

/// Wrapper for the Rate JSON API response.
#[derive(Serialize, Deserialize)]
struct RatesJSON {
    rates: BTreeMap<String, f32>,
}

/// Get the api_id from the OS environement variables.
///
/// Note that the varname should be `API_ID`.
fn get_api_id() -> Result<String> {
    env::var("API_ID").with_context(|| "The `API_ID` environment variable is not well configured.")
}

/// Checks the status of the API response.
fn check_response_status(resp: &Response) -> Result<()> {
    if resp.status().is_success() {
        Ok(())
    } else if resp.status().as_u16() == 401 {
        Err(anyhow!("Invalid App ID provided."))?
    } else if resp.status().as_u16() == 403 {
        Err(anyhow!("Access restricted for repeated over-use."))?
    } else {
        Err(anyhow!(
            "Unexpected api response. Status: {:?}",
            resp.status()
        ))?
    }
}

/// Fetchs the list of available currencies and their current exchange rates.
pub fn fetch_available_currencies() -> Result<BTreeMap<String, f32>> {
    let session = reqwest::blocking::Client::new();
    let resp = session
        .get("https://openexchangerates.org/api/latest.json")
        .query(&[("app_id", get_api_id()?)])
        .send()?;
    check_response_status(&resp)?;

    let rates_json: RatesJSON = resp
        .json()
        .with_context(|| "Failed to read the API response.")?;

    Ok(rates_json.rates)
}
