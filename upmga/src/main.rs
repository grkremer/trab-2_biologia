fn calcula_score(seq1: &str, seq2: &str) {
    let gap = -2;
    let mismatch = -1;
    let matc = 1;
    let seq1 = format!(" {}", seq1);
    let seq2 = format!(" {}", seq2);
    let mut linha_anterior: Vec<i32> = (0..seq1.len()).map(|i| (i as i32) * gap).collect();
    let mut i = 1;
    while i < seq1.len() {
        let mut atual: Vec<i32> = vec![(i as i32) * gap];
        for j in 1..seq2.len() {
            let current_score;
            if seq1.chars().nth(i).unwrap() == seq2.chars().nth(j).unwrap() {
                current_score = std::cmp::max(
                    linha_anterior[j - 1] + matc,
                    std::cmp::max(linha_anterior[j] + gap, atual[j - 1] + gap),
                );
            } else {
                current_score = std::cmp::max(
                    linha_anterior[j - 1] + mismatch,
                    std::cmp::max(linha_anterior[j] + gap, atual[j - 1] + gap),
                );
            }
            atual.push(current_score);
        }
        linha_anterior = atual;
        i += 1;
    }
    println!("{}", linha_anterior.last().unwrap());
}
use std::time::{Duration, Instant};
fn main() {
    let start = Instant::now();
    
    let seq1 = std::fs::read_to_string(r"src\fonte\seq_1.fna").unwrap();
    let seq2 = std::fs::read_to_string(r"src\fonte\seq_2.fna").unwrap();
    let seq1 = &seq1[..1000];
    let seq2 = &seq2[..1000];
    calcula_score(seq1, seq2);
    let duration = start.elapsed();
    println!("Time elapsed in calcula_score() is: {:?}", duration);
}