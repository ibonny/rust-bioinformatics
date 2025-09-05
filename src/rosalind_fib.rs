use std::env;

fn mortal_fibonacci(n: usize, k: u128) -> u128 {
    let mut sequence = vec![0; n + 1];

    sequence[1] = 1;

    if n >= 2 {
        sequence[2] = 1;
    }

    for i in 3..=n {
        sequence[i] = sequence[i - 1] + k * sequence[i - 2];
    }

    sequence[n]
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut n = 5;
    let mut k = 3;

    if args.len() >= 2 {
        n = args[1].parse().expect("Not a valid n value.");
        k = args[2].parse().expect("Not a valid k value.");
    }

    println!("{}", mortal_fibonacci(n, k));
}
