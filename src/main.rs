mod todo;//mod so aparece em um arquivo o resto pode usar usando crate sem usar mod
use crate::todo::Todo;
mod tarefas;
//use crate::tarefas::*;//ou {imprime_tarefas, adiciona_tarefa, marcar_feito,remove_tarefa,remove_all};(list td, q sucks)
mod utils;
use crate::utils::*;
use std::io::Write;//biblioteca pra da flush no buffer
use std::{io};

fn main(){
    println!("Welcome to WhatDoWe-Now");
    

    let mut tarefas : Vec<Todo>= load();//->permite muda a variavel -> mut
    
    menu();
    loop {
        
        print!("> ");
        io::stdout().flush().unwrap();//limpa buffer
        let mut opc = String::new();
        io::stdin().read_line(&mut opc).expect("Erro ao ler");
        let (command,mut args) = filter_command(&opc);
        if command == "exit"{
            break;
        }
        
        examine_opc(&command,&mut args,&mut tarefas);
        save(&tarefas);
    }
    
}