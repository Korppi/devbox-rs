use std::io::{Result, Write};

use chrono::{Date, DateTime, Local, Utc};
use chrono_tz::Tz;

pub fn run(
    iso: bool,
    utc: bool,
    tz: Option<String>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()> {
    let now_utc = Utc::now();

    if let Some(tz_str) = tz {
        // TODO: actually use tz...
        writeln!(out, "{}", now_utc.to_rfc3339())?;
    } else if utc {
        writeln!(out, "{}", now_utc.to_rfc3339())?;
    } else {
        // paikallinen aika
        let dt_local = now_utc.with_timezone(&Local);
        writeln!(out, "{}", dt_local)?;
    }
    Ok(())
}