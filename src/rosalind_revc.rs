use std::env;

struct DNA {
    input: String,
}

impl DNA {
    fn new(str: String) -> DNA {
        return DNA { input: str };
    }

    fn complement(self) -> String {
        let mut temp = self.input.chars().rev().collect::<String>();

        temp = temp
            .chars()
            .map(|c| match c {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => ' ',
            })
            .collect::<String>();

        return temp;
    }
}

fn main() {
    let mut main_dna = DNA::new(String::from("AAAACCCGGT"));

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let str = &args[1];

        main_dna = DNA::new(str.clone());
    }

    println!("{}", main_dna.complement());
}
