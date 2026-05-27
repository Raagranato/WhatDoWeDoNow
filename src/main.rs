mod todo;//mod so aparece em um arquivo o resto pode usar usando crate sem usar mod
use crate::todo::Todo;
mod tarefas;
use crate::tarefas::*;//ou {imprime_tarefas, adiciona_tarefa, marcar_feito,remove_tarefa,remove_all};(list td, q sucks)
mod utils;
use crate::utils::*;
use std::{io};


fn main(){
    println!("Welcome to WhatDoWe-Now");
    /*--PASSOS PRA FAZER TODO LIST--
    -Imprime lista
    -Adiciona item
    -Marcar como feito*/    
    let mut tarefas : Vec<Todo>= Vec::new();//->permite muda a variavel -> mut

    let t = Todo{tarefa: String::from("Lavar carro"),feito: false,};
    tarefas.push(t);
    menu();
    let mut opc=String::new();
    while opc!="5" {
        
        io::stdin().read_line(&mut opc).expect("Erro ao ler");


        match opc.trim() {
            "menu"=> menu(),
            "add"|"Add"=>add_tarefa(&mut tarefas),//Passar parematro para remover maybe? done [parametro], sem precisa rpedir qual
            "done"=>marcar_feito(&mut tarefas),//Passar parematro para remover maybe? done [parametro], sem precisa rpedir qual
            "remove"|"Remove"|"r"|"delete"=>remove_tarefa(&mut tarefas),//Passar parematro para remover maybe? remove [parametro], sem precisa rpedir qual
            "clear"|"Clear"=>remove_all(&mut tarefas),
            "exit"=>break,
            _ => println!("Not possible")
        }
        imprime_tarefas(&tarefas);
    }

}