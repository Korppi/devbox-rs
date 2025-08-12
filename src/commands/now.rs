use std::io::{Result, Write};

use chrono::{DateTime, Datelike, Local, Utc};
use chrono_tz::Tz;

use std::str::FromStr;

pub fn run(
    iso: bool,
    utc: bool,
    week: bool,
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
                print_datetime(out, local, iso, week);
            }
            Err(e) => {
                writeln!(
                    err,
                    "Error with timezone \"{tz_str}\": {e}. Use IANA name, for example. Europe/Helsinki"
                );
            }
        }
    } else if utc {
        print_datetime(out, now_utc, iso, week);
    } else {
        // paikallinen aika
        let dt_local = now_utc.with_timezone(&Local);
        print_datetime(out, dt_local, iso, week);
    }
    Ok(())
}

fn print_datetime<W: Write>(
    mut out: W,
    dt: DateTime<impl chrono::TimeZone>,
    iso: bool,
    week: bool,
) {
    if iso {
        write!(out, "{}", dt.to_rfc3339());
    } else {
        write!(out, "{}", dt.naive_local());
    }
    if week {
        write!(out, " | Week: {}", dt.iso_week().week());
    }
    writeln!(out, "");
}
