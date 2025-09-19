use std::env;
use std::collections::HashMap;

fn fibonacci(memoize: &mut HashMap<u32, u128>, n: u32) -> u128 {
    if let Some(&value) = memoize.get(&n) {
        return value;
    }
    
    let result = match n {
        1 | 2 => 1,
        n => {
            let result = fibonacci(memoize, n-1) + fibonacci(memoize, n-2);
            memoize.insert(n, result);
            result
        }
    };
    
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let n_value;

    if args.len() == 2 {
        let n_value_result = args[1].parse::<u32>();

        n_value = match n_value_result {
            Ok(v) => v,
            Err(_) => {
                println!("Error parsing value.");

                return;
            }
        };
    } else {
        println!("\n  Unknown or invalid first arg.\n");

        return;
    }
    
    let mut memoize: HashMap<u32, u128> = HashMap::new();

    println!("{}", fibonacci(&mut memoize, n_value));
}