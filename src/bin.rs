use econ::convert;
use structopt::StructOpt;

/// The Elemental Converter
#[derive(StructOpt)]
#[structopt(name = "E-Con")]
struct Opt {
    /// Print all possible solutions
    #[structopt(short, long)]
    all: bool,

    /// Text to convert
    input: String,
}

fn main() {
    let opt = Opt::from_args();

    let results = convert(&opt.input);

    if results.len() > 0 {
        if opt.all {
            for result in results {
                println!("{}", result);
            }
        } else {
            println!("{}", results[0]);
        }
    } else {
        println!("Sorry, your text couldn't be converted into chemical symbols.");
    }
}
