
extern crate chrono;
extern crate clap;

use std::{ops::Add, process::exit};

use chrono::{DateTime, Datelike, Duration, Local, TimeZone};
use clap::{App, Arg};

/// Calcola i giorni lavorativi tra due date
/// # Parametri
/// * `from` data di partenza
/// * `to` data di arrivo
///
/// # Examples
///
/// ```rust
/// let ferie = weekday_count(Local::now(),  Local.ymd(2021, 07, 19));
/// println!("{}", ferie);
/// ```
///
fn weekday_count(from: DateTime<Local>, to: DateTime<Local>) -> Duration {
    let mut today = from.add(Duration::days(1));
    let mut day_count = 0;
    if from > to {
      return Duration::days(0);
    }  
    loop {
        if today.weekday().number_from_monday() < 6 {
            day_count += 1;
        }
        today = today.add(Duration::days(1));
        if today > to {
            break;
        }
    }
    Duration::days(day_count)
}

fn main() {
    let matches = App::new("Ferie")
        .version("1.0")
        .author("Andrea Forlin <andrea.forlin@gmail.com>")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Set format output")
                .takes_value(true),
        )
        .get_matches();

    let ferie: DateTime<Local> = Local.ymd(2022, 07, 18).and_hms(4, 0, 0);

    let format = match matches.value_of("format").unwrap_or("text") {
        fmt @ "text" => fmt,
        fmt @ "json" => fmt,
        e @ _ => {
            println!("Valore format \"{}\" sconosciuto", e);
            matches.usage();
            exit(2);
        }
    };

    let day_count = weekday_count(Local::now(), ferie);

    match format {
        "text" => println!("{}", day_count.num_days()),
        "json" => println!(r#"{{"days":{}}}"#, day_count.num_days(),),
        _ => exit(2),
    };
}
