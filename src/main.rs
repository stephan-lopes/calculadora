use std::io;
use std::io::prelude::*;

fn soma(x: f32, y: f32) -> f32 {
    return x + y;
}

fn subtracao(x: f32, y: f32) -> f32 {
    return x - y;
}

fn divisao(x: f32, y: f32) -> f32 {
    return x / y;
}

fn multiplicacao(x: f32, y: f32) -> f32 {
    return x * y;
}

fn main() {

    let mut number_one = String::new();
    let mut number_two = String::new();
    
    println!("===== Calculadora =====");
    
    print!("Digite o primeiro número: ");
    io::stdout()
        .flush()
        .ok()
        .expect("Não possuí saída no buffer");
    
    io::stdin()
        .read_line(&mut number_one)
        .expect("Erro");

    print!("Digite o segundo número: ");
    
    io::stdout()
        .flush()
        .ok()
        .expect("Não possuí saída no buffer");

    io::stdin()
        .read_line(&mut number_two)
        .expect("Erro");
    
    println!(""); //Quebra de linha

    let number_one: f32 = number_one.trim().parse().expect("A informação não é um número.");
    let number_two: f32 = number_two.trim().parse().expect("A informação não é um número.");

    println!("Soma: {}", soma(number_one, number_two));
    println!("Subtração: {}", subtracao(number_one, number_two));
    println!("Multiplicação: {}", multiplicacao(number_one, number_two));
    println!("Divisão: {}", divisao(number_one, number_two));
}