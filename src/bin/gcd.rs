use numeric::arithmetic::arithmetic_gcd;

use structopt::StructOpt;

/// Command-line arguments for the GCD program.
#[derive(StructOpt)]
struct Cli {
    /// The first number.
    a: u32,
    /// The second number.
    b: u32,
}

fn main() {
    let args = Cli::from_args();

    // Print the greatest common divisor of the two numbers.
    println!("{}", arithmetic_gcd(args.a, args.b));
}
