fn fac(n: u128) -> u128 {
    if n > 1 {
        n * fac(n-1)
    } else {
        n
    }
}

fn main() {
    println!("The result is {}", fac(10));
}
