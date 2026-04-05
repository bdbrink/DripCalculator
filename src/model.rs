use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawRecord {
    #[serde(rename = "Symbol")]
    pub symbol: Option<String>,

    #[serde(rename = "Quantity")]
    pub quantity: Option<f64>,

    #[serde(rename = "Last Price")]
    pub price: Option<f64>,

    #[serde(rename = "Market Value")]
    pub market_value: Option<f64>,
}