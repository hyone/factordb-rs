#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate url;
#[cfg(test)]
extern crate mockito;

mod client;

use clap::{ App, Arg };
use client::errors::Result;

fn run() -> Result<()> {
    let validate_number = |s: String| s.parse::<u64>().map(|_| ()).map_err(|e| e.to_string());

    let args = App::new("factordb")
        .version(crate_version!())
        .author(crate_authors!())
        .about("factordb get the factorization of the number from factordb.com")
        .arg(Arg::with_name("raw")
             .short("r")
             .long("raw")
             .help("Display the answer as raw json format"))
        .arg(Arg::with_name("NUM")
             .index(1)
             .validator(validate_number)
             .required(true)
             .help("Number to be factored")
             )
        .get_matches();

    let num: u64 = args.value_of("NUM")
        .and_then(|n| n.parse::<u64>().ok())
        .unwrap();
    let result = client::request(num)?;

    if args.is_present("raw") {
        println!("{}", result.json())
    } else {
        let output = result.factor_list()
            .into_iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", output);
    }

    Ok(())
}

quick_main!(run);
