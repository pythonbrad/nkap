mod cache;
mod exchange_api;
pub mod interactive;

use anyhow::{anyhow, Result};
use cache::Cache;
use std::collections::BTreeMap;

/// Offline check the currency code format.
///
/// The check is based on ISO 4217 currency codes.
pub fn check_currency_format(code: &str) -> Result<()> {
    if code.len() == 3 {
        Ok(())
    } else {
        Err(anyhow!("Invalid currency format `{code}`"))
    }
}

/// Converts an amount from a source currency to a target currency.
///
/// The exchange rate is fetch in real time.
pub fn convert(from: &str, to: &str, amount: f32) -> Result<(f32, f32)> {
    check_currency_format(from)?;
    check_currency_format(to)?;

    // We prevent useless api request.
    if amount == 0.0f32 {
        return Ok((0.0, 0.0));
    }

    let data = get_available_currencies()?;
    let source_rate = data.get(from).ok_or(anyhow!("Invalid currency `{from}`"))?;
    let target_rate = data.get(to).ok_or(anyhow!("Invalid currency `{to}`"))?;
    let conv_rate = target_rate / source_rate;

    Ok((amount * conv_rate, conv_rate))
}

/// Returns list of available currencies and their current exchange rates.
///
/// A cache is used internally to decrease the frequency of API call.
/// In case of cache error, the data will be fetch from the API.
pub fn get_available_currencies() -> Result<BTreeMap<String, f32>> {
    let data = Cache::load(".currencies").and_then(|data| data.unwrap());
    let data = data.or_else(|_| -> Result<_> {
        let data = exchange_api::fetch_available_currencies()?;
        Cache::new(&data)?.dump(".currencies")?;

        Ok(data)
    })?;

    Ok(data)
}

/// Print API information on stdout.
pub fn print_request(request: &str) -> Result<()> {
    match request {
        "currency-list" => {
            let currencies = get_available_currencies()?;

            currencies.iter().for_each(|(currency, rate)| {
                println!("{currency} -> {rate}");
            });
        }
        _ => {
            return Err(anyhow!("unknow print request `{request}`"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{check_currency_format, convert};

    #[test]
    fn test_check_currency_format() {
        assert!(check_currency_format("AB").is_err());
        assert!(check_currency_format("ABC").is_ok());
        assert!(check_currency_format("ABCD").is_err());
    }

    #[test]
    fn test_convert() {
        use crate::Cache;
        use std::collections::BTreeMap;
        use std::fs;

        // Generate a test cache for a offline test.
        Cache::new(&BTreeMap::from([
            ("USD".to_owned(), 1.0f32),
            ("XAF".to_owned(), 500.0f32),
        ]))
        .unwrap()
        .dump(".currencies")
        .unwrap();

        assert!(convert("US", "EUR", 100.0).is_err());
        assert!(convert("ABC", "XYZ", 100.0).is_err());
        assert_eq!(convert("XAF", "USD", 500.0).unwrap(), (1.0, 0.002));
        assert_eq!(convert("USD", "XAF", 100.0).unwrap(), (50000.0, 500.0));

        // We delete the cache to evict confusions in manual test.
        fs::remove_file(".currencies").unwrap();
    }
}
