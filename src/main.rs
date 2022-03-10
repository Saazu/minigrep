use clap::{App, Arg};
use regex::Regex;

fn main() {
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    let args = App::new("grep-lite")
        .version("0.1")
        .about("A mini grep utility built with Rust")
        .arg(
            Arg::with_name("pattern")
                .help("Pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
