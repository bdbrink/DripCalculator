use csv::ReaderBuilder;
use anyhow::Result;

pub fn load_portfolio(path: &str) -> Result<Vec<RawRecord>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(path)?;

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: RawRecord = match result {
            Ok(r) => r,
            Err(_) => continue, // skip garbage rows
        };

        // basic filtering
        if record.symbol.is_some() && record.quantity.unwrap_or(0.0) > 0.0 {
            records.push(record);
        }
    }

    Ok(records)
}