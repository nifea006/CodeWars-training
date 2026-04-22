fn main() {
    println!("{}", dna_strand("AAAA"));
}

fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|chr| match chr {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => todo!(),
        })
        .collect()
}
