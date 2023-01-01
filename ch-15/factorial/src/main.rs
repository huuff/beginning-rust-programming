use clap::Parser;

#[derive(Parser)]
struct Args {
    num: u128,
}

fn fac(n: u128) -> u128 {
    if n > 1 {
        n * fac(n-1)
    } else {
        n
    }
}

fn main() {
    let args = Args::parse();
    println!("The result is {}", fac(args.num));
}
