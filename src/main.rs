use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: find-pep <input_fasta.fasta> <input_list.txt>");
        std::process::exit(1);
    }

    let fasta_content = fs::read_to_string(&args[1])?;
    let peptides_content = fs::read_to_string(&args[2])?;

    let proteins = parse_fasta(&fasta_content);
    let peptides = peptides_content.lines().collect::<Vec<_>>();

    for peptide in peptides {
        let mut found = false;
        for (accession, sequence) in &proteins {
            let count = sequence.matches(peptide).count();
            if count > 0 {
                println!("{}:{}:{}", peptide, accession, count);
                found = true;
            }
        }
        if !found {
            // If you want to print peptides with no matches, uncomment the next line
            // println!("{}:0", peptide);
        }
    }

    Ok(())
}

fn parse_fasta(content: &str) -> HashMap<String, String> {
    let mut proteins = HashMap::new();
    let mut accession = String::new();
    let mut sequence = String::new();

    for line in content.lines() {
        if line.starts_with('>') {
            if !accession.is_empty() {
                proteins.insert(accession.clone(), sequence.clone());
                sequence.clear();
            }
            accession = line.split('|').nth(1).unwrap_or("").to_string();
        } else {
            sequence.push_str(line);
        }
    }
    if !accession.is_empty() {
        proteins.insert(accession, sequence);
    }

    proteins
}
