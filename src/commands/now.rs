use std::io::{Result, Write};

use chrono::{DateTime, Local, Utc};
use chrono_tz::Tz;

use std::str::FromStr;

pub fn run(
    iso: bool,
    utc: bool,
    week: bool,
    weekday: bool,
    tz: Option<String>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()> {
    let now_utc = Utc::now();
    if let Some(tz_str) = tz {
        let tz_result = Tz::from_str(&tz_str);

        match tz_result {
            Ok(tz) => {
                let local = now_utc.with_timezone(&tz);
                let _ = print_datetime(out, local, iso);
            }
            Err(e) => {
                writeln!(
                    err,
                    "Error with timezone \"{tz_str}\": {e}. Use IANA name, for example. Europe/Helsinki"
                );
            }
        }
    } else if utc {
        let _ = print_datetime(out, now_utc, iso);
    } else {
        // paikallinen aika
        let dt_local = now_utc.with_timezone(&Local);
       let _ = print_datetime(out, dt_local, iso);
    }
    Ok(())
}

fn print_datetime<W: Write>(
    mut out: W,
    dt: DateTime<impl chrono::TimeZone>,
    iso: bool,
) -> Result<()> {
    if iso {
        writeln!(out, "{}", dt.to_rfc3339())
    } else {
        writeln!(out, "{}", dt.naive_local())
    }
}