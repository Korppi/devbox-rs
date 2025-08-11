use std::io::{Result, Write};

use chrono::{Local, Utc};
use chrono_tz::Tz;

use std::str::FromStr;

pub fn run(
    iso: bool,
    utc: bool,
    tz: Option<String>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<()> {
    let now_utc = Utc::now();
    // TODO: decide default format?
    if let Some(tz_str) = tz {
        let tz_result = Tz::from_str(&tz_str);
        
        match tz_result {
            Ok(tz) => {
                let local = now_utc.with_timezone(&tz);
                if iso {
                    writeln!(out, "{}", local.to_rfc3339())?;
                } else {
                    writeln!(out, "{}", local)?;
                }
            }
            Err(e) => {
                writeln!(err, "Error with timezone \"{}\": {}. Use IANA name, for example. Europe/Helsinki", tz_str, e);
            }
        }
    } else if utc {
        if iso {
            writeln!(out, "{}", now_utc.to_rfc3339())?;
        } else {
            writeln!(out, "{}", now_utc)?;
        }
    } else {
        // paikallinen aika
        let dt_local = now_utc.with_timezone(&Local); 
        if iso {
            writeln!(out, "{}", dt_local.to_rfc3339())?;
        } else {
            writeln!(out, "{}", dt_local)?;
        }
         
    }
    Ok(())
}