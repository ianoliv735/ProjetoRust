use std::io;
mod models;
use models::Agendamento;

fn main() {
    
    let agendamento = Agendamento::new(
        "Bela".to_string(),
        "01/01/2024".to_string(),
        "10:00".to_string(),
        "Banho".to_string(),
        "jujjj".to_string(),
    );

    println!("Agendamento: {:?}", agendamento);
    
}


// println!("Informe seu nome: ");

//let mut nome = String::new();
//io::stdin().read_line(&mut nome);

//println!("Informe o cpf: ");
//let mut cpf = String::new() ;
//io::stdin().read_line(&mut cpf);

//println!("Informe seu Numero de celular: ");
//let mut celular = String::new();
//io::stdin().read_line(&mut celular);

//println!("Nome do Pet: ");
//let mut nome_pet = String::new();
//io::stdin().read_line(&mut nome_pet);

//println!("Descreva com poucas palavras o motivo da consulta: ");
//let mut motivo = String::new();
//io::stdin().read_line(&mut motivo);

//println!("Voce digitou: {}", nome); 
