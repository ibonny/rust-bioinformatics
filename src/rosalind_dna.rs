use std::env;

fn main() {
    let mut dna_string = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        dna_string = args[1].as_str();
    }

    let a_count = dna_string.chars().filter(|c| *c == 'A').count();
    let c_count = dna_string.chars().filter(|c| *c == 'C').count();
    let g_count = dna_string.chars().filter(|c| *c == 'G').count();
    let t_count = dna_string.chars().filter(|c| *c == 'T').count();

    println!("{} {} {} {}", a_count, c_count, g_count, t_count);
}
