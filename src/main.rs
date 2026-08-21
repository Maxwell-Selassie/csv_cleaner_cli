use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct RawRecords {
    region: String,
    year: String,
    tonnes: String,
    price_usd: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ReadRecords {
    region: String,
    year: String,
    tonnes: String,
    price_usd: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
struct CleanRecords {
    region: String,
    year: u32,
    tonnes: f64,
    price_usd: f64,
}

fn write_data() -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("cocoa_exports.csv")?;

    let data = vec![
        RawRecords {
            region: "Ashanti".to_string(),
            year: "2021".to_string(),
            tonnes: "150000".to_string(),
            price_usd: "12350.00".to_string(),
        },
        RawRecords {
            region: "Western".to_string(),
            year: "2023".to_string(),
            tonnes: "96500".to_string(),
            price_usd: "8350.50".to_string(),
        },
        RawRecords {
            region: "Upper East".to_string(),
            year: "2025".to_string(),
            tonnes: "10000".to_string(),
            price_usd: "1000.00".to_string(),
        },
        RawRecords {
            region: "Volta".to_string(),
            year: "2025".to_string(),
            tonnes: "38000".to_string(),
            price_usd: "1900.00".to_string(),
        },
        RawRecords {
            region: "Central".to_string(),
            year: "2026".to_string(),
            tonnes: "57300".to_string(),
            price_usd: "7050.80".to_string(),
        },
        RawRecords {
            region: "Eastern".to_string(),
            year: "2026".to_string(),
            tonnes: "45000".to_string(),
            price_usd: "bad_value".to_string(),
        },
    ];

    for record in data {
        writer.serialize(record)?
    }

    writer.flush()?;

    Ok(())
}

fn clean(raw: &ReadRecords) -> Result<CleanRecords, Box<dyn Error>> {
    let region = raw.region.trim().to_string();
    let year = raw
        .year
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("bad year value '{}': {e}", raw.year))?;
    let tonnes = raw
        .tonnes
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("bad tonnes value '{}': {e}", raw.tonnes))?;
    let price_usd = raw
        .price_usd
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("bad price value '{}': {e}", raw.price_usd))?;

    Ok(CleanRecords {
        region,
        year,
        tonnes,
        price_usd,
    })
}

fn read_data() -> Result<Vec<ReadRecords>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("cocoa_exports.csv")?;
    let mut records = Vec::new();

    for record in reader.deserialize::<ReadRecords>() {
        let raw = record?;
        records.push(raw);
    }
    Ok(records)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut kept = Vec::new();

    write_data()?;
    let records = read_data()?;
    for record in records {
        match clean(&record) {
            Ok(_clean_data) => {
                kept.push(_clean_data);
            }
            Err(e) => {
                println!("Error parsing row: {}", e);
            }
        }
    }
    let mut writer = csv::Writer::from_path("cleaned_cocoa_export.csv")?;

    for record in kept {
        writer.serialize(record)?
    }

    writer.flush()?;
    Ok(())
}
