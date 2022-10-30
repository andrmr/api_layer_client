// https://apilayer.com/marketplace/currency_data-api
// Available Endpoints
// /list Get all available currencies.
// /live Get the most recent exchange rate data.
// /convert Convert one currency to another.
// /historical Get historical rates for a specific day.
// /timeframe Request exchange rates for a specific period of time.
// /change Request any currency's change parameters (margin, percentage).

use std::collections::HashMap;

use anyhow::{Context, Ok, bail};
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub type ApiResult<R> = anyhow::Result<R>;

/// Client for API Layer currency endpoints
pub struct ApiLayerClient {
    client: reqwest::Client,
    api_key: String,
}

impl ApiLayerClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    /// Get all available currencies.
    pub async fn list(&self) -> ApiResult<HashMap<String, String>> {
        let url = reqwest::Url::parse("https://api.apilayer.com/currency_data/list")?;
        let json = self.get_json(url).await?;
        from_json::<HashMap<String, String>>(&json, "currencies")
    }

    /// Get the most recent exchange rate data.
    pub async fn live(&self, source: &str, currencies: Option<&Vec<String>>) -> ApiResult<HashMap<String, f64>> {
        let mut params = HashMap::from([
            ("source", source.to_string()),
        ]);

        if let Some(currencies) = currencies {
            params.insert("currencies", currencies.join(","));
        }

        let url = reqwest::Url::parse_with_params("https://api.apilayer.com/currency_data/live", &params)?;
        let json = self.get_json(url).await?;
        let quotes = from_json::<HashMap<String, f64>>(&json, "quotes")?
            .into_iter()
            .map(|(symbol, value)| {
                (
                    // strip source symbol added as prefix to keys e.g. USDEUR
                    symbol.strip_prefix(source).unwrap_or(&symbol).into(),
                    value
                )
            })
            .collect();
        
        Ok(quotes)
    }

    async fn get_json(&self, url: reqwest::Url) -> ApiResult<Value> {
        let res = self.client
            .get(url)
            .header("apikey", HeaderValue::from_str(&self.api_key)?)
            .send().await?;
    
        if !res.status().is_success() {
            bail!("http: not successful {:?}", res);
        }

        Ok(res.json::<serde_json::Value>().await?)
    }
}

fn from_json<T>(json: &Value, key: &str) -> ApiResult<T>
where
    T: DeserializeOwned
{
    let success = json
        .get("success")
        .context("json: missing key 'success'")?
        .as_bool()
        .context("json: unable to parse 'success'")?;

    if !success {
        bail!("json: not successful {:?}", json);
    }

    let json = json.get(key)
        .context(format!("json: missing key {key}"))?;

    serde_json::from_value::<T>(json.clone())
        .context(format!("json: unable to parse {key} as type {}", stringify!(T)))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_from_json_float() {
        let val: f64 = 1.333;

        let json = json!({
            "success": true,
            "expected": val
        });
        
        let expected = from_json::<f64>(&json, "expected");
        assert!(expected.is_ok(), "json not parsed");

        // todo: make sure this compares floats properly
        assert_eq!(expected.unwrap(), val, "vals not equal")
    }

    #[test]
    fn test_from_json_hash_map() {
        let key = "EUR";
        let val = "Euro";

        let json = json!({
            "success": true,
            "symbols": {
                key: val
            }
        });

        let hash_map = from_json::<HashMap<String, String>>(&json, "symbols");
        assert!(hash_map.is_ok(), "json not parsed");
        
        let hash_map = hash_map.unwrap();
        let expected = hash_map.get(key);
        assert!(expected.is_some(), "key not found");
        
        assert_eq!(expected.unwrap(), val, "vals not equal")
    }
}
