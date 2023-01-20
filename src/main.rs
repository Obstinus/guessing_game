use std::io;
use rand::Rng;

fn main() {
    println!("Descubra o número secreto!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("O numéro secreto é: {secret_number}");

    println!("Por favor, escreva seu palpite.");

    let mut guess = String::new(); // variável mutável
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao ler a linha");

    println!("Você palpitou: {guess}");
}
