use numeric::arithmetic;

use structopt::StructOpt;

#[derive(StructOpt)]
/// compute the gcd of a and b.
struct Cli {
    /// number one
    a: u32,
    /// number two
    b: u32,
}

fn main() {
    let args = Cli::from_args();

    println!("{}", arithmetic::arithmetic_gcd(args.a, args.b));
}
