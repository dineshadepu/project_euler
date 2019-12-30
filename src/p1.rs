const USAGE: &str = "
Usage: p1 [--dt N] [--plots]
       p1 --help
Problem 1 in project euler. https://projecteuler.net/problem=1
Options:
    -h, --help      Show this message.
";

// -------------------------------------------------
// external crate imports
use docopt::Docopt;
// -------------------------------------------------


#[derive(Deserialize, Debug)]
pub struct Args {
}


pub fn main(args: &[String]) {
    // --------------------------------------
    // GET THE COMMAND LINE VARIABLES
    // --------------------------------------
    let _args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(args).deserialize())
        .unwrap_or_else(|e| e.exit());
    // println!("{:?}", args);
    // --------------------------------------
    // GET THE COMMAND LINE VARIABLES ENDS
    // --------------------------------------

    let mut sum_of_3_or_5_multiples = 0;

    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0{
            sum_of_3_or_5_multiples = sum_of_3_or_5_multiples + i;
        }
    }

    println!("The sum of multiples of 3 or 5 below 1000 is --> {:?}", sum_of_3_or_5_multiples);
}
