fn towersolve(n: u16, from: char, to: char, other: char) {
    if n == 1 {
        println!("Moving disk 1 from rod {from} to rod {to}");
        return;
    }
    towersolve(n-1, from, other, to);
    println!("Moving disk {n} from rod {from} to rod {to}");
    towersolve(n-1, other, to, from);
}

fn main() {
    towersolve(4, 'A', 'B', 'C');
}
