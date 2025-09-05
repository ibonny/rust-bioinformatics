use std::env;

fn main() {
    let mut rna_string = "GATGGAACTTGACTACGTAAATT";

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        rna_string = args[1].as_str();
    }

    let new_rna = rna_string.replace("T", "U");

    println!("{}", new_rna);
}
