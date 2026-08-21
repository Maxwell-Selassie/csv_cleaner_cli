# 🦀 CSV Cleaner CLI

Project 1 of 26 in a series of progressively harder Rust projects, built while learning Rust as a working data engineer. Each project adds a new concept on top of the last, and (where relevant) leans on Ghana/Germany-flavored sample data.

What it does
---

Generates a sample raw CSV of Ghana cocoa export records (region, year, tonnes, price in USD), then runs a validation pipeline over it:

`Write` creates cocoa_exports.csv with a mix of clean and intentionally bad rows (one row has a corrupted price_usd value).

`Read` parses the CSV into a Vec<ReadRecords>, where every field is read as a raw String.

`Clean` attempts to convert each field into its proper type (u32 for year, f64 for tonnes/price). Rows that fail validation are rejected with a specific error message (not silently dropped) explaining which field failed and why.

`Write clean output` all rows that pass validation are serialized to cleaned_cocoa_export.csv.

Why this project
---

The point wasn't just "parse a CSV", it was to practice the core loop every data pipeline needs: ingest untrusted data, validate it explicitly, and make failures visible instead of swallowing them. Rust's Result type turned out to be a great forcing function for that.

What I learned
---

- Ownership and borrowing across function boundaries
- Result<T, E> vs Option<T>, and why they aren't interchangeable with ?
-The ? operator's propagation rules (it needs the error type to match the function's own return type)
- serde for (de)serializing structs to/from CSV
- Why silent error handling (Option-based, "just drop it") hides real data bugs — found this the hard way when my own test data was invalid (used _ as a thousands separator, which Rust only allows in source-code numeric literals, not when parsing strings at runtime)

Stack
---

- Rust
- csv — CSV reading/writing
- serde — struct (de)serialization

Running it
---

```
    cargo fmt && cargo run --release
```

This will:
---

1. generate cocoa_exports.csv (raw/messy data)
2. print an error message for any row that fails validation
3. write the valid rows to cleaned_cocoa_export.csv

`Sample output:`
Error parsing row: bad price value 'bad_value': invalid float literal

`cleaned_cocoa_export.csv` will contain 5 clean rows; the row with the corrupted price is excluded and logged.

Part of a series
---

This is Project 1 of 26 in a progressive Rust learning series, moving from language fundamentals through to databases, pipelines, and streaming, shaped around my work in data engineering. Follow along for the rest.