const USAGE: &str = "
Usage: p2 [--dt N] [--plots]
       p2 --help
Problem 2 in project euler. https://projecteuler.net/problem=2
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

    let mut sum_of_even_fibanocci = 2;
    let mut fib_n_2 = 2;
    let mut fib_n_1 = 3;
    // this is a dummy value
    let mut fib_n = 0;

    while fib_n < 4000000 {
        fib_n = fib_n_1 + fib_n_2;

        if fib_n % 2 == 0 {
            sum_of_even_fibanocci += fib_n;
        }
        fib_n_2 = fib_n_1;
        fib_n_1 = fib_n;
    }

    println!("The sum of fibanocci less than 4 millioin is --> {:?}", sum_of_even_fibanocci);

}
