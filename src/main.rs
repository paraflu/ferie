extern crate chrono;
extern crate clap;

use std::{ops::Add, process::exit};

use chrono::{ NaiveDate, Datelike, Duration, Local};
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
fn weekday_count(from: NaiveDate, to: NaiveDate) -> Duration {
    let mut today = from.add(Duration::days(1));
    //println!("from {:?} to {:?}", today, to);
    let mut day_count = 0;
    loop {
    //println!("{} {}", today, day_count);
        if today.weekday().number_from_monday() < 6 {
            day_count += 1;
        }
        today = today.add(Duration::days(1));
        if today >= to {
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

    let ferie: NaiveDate = NaiveDate::parse_from_str("2021-07-19", "%Y-%m-%d").unwrap();

    let format = match matches.value_of("format").unwrap_or("text") {
        fmt @ "text" => fmt,
        fmt @ "json" => fmt,
        e @ _ => {
            println!("Valore format \"{}\" sconosciuto", e);
            matches.usage();
            exit(2);
        }
    };

    let day_count = weekday_count(Local::today().naive_local(),ferie);

    match format {
        "text" => println!("{}", day_count.num_days()),
        "json" => println!(r#"{{"days":{}}}"#, day_count.num_days(),),
        _ => exit(2),
    };
}
