use std::{io, process};

fn main() {
    let mut arg = String::new();

    println!("Provide the amount of disks:");

    io::stdin().read_line(&mut arg).unwrap_or_else(|err| {
        println!("Error reading argument: {err}");
        process::exit(1)
    });

    let n = arg.trim().parse().unwrap_or_else(|_| {
        println!("Value must be a positive number");
        process::exit(1)
    });

    tower_of_hanoi(n, 'A', 'C', 'B')
}

fn tower_of_hanoi(n: u32, from: char, to: char, aux: char) {
    if n == 0 {
        return;
    }

    tower_of_hanoi(n - 1, from, aux, to);

    println!("Move disk {n} from rod {from} to rod {to}");

    tower_of_hanoi(n - 1, aux, to, from)
}
