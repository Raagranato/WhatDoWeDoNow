mod todo;//mod so aparece em um arquivo o resto pode usar usando crate sem usar mod
use crate::todo::Todo;
mod tarefas;
use crate::tarefas::{imprime_tarefas, adiciona_tarefa, marcar_feito};
mod utils;


fn main(){
    println!("Bem vindo ao WhatDoWe-Now");
    /*--PASSOS PRA FAZER TODO LIST--
    -Imprime lista
    -Adiciona item
    -Marcar como feito*/
    //menu();
    let mut tarefas : Vec<Todo>= Vec::new();//->permite muda a variavel -mut

    let t = Todo{tarefa: String::from("Lavar carro"),feito: false,};
    tarefas.push(t);

    adiciona_tarefa(&mut tarefas);

    imprime_tarefas(&tarefas);
    
    marcar_feito(&mut tarefas);

    imprime_tarefas(&tarefas);
}