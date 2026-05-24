use std::io;//biblioteca para receber entrada e saida
use crate::utils::{limpa_terminal};
use crate::todo::Todo;

pub fn imprime_tarefas(tarefas:&Vec<Todo>){//So pega emprestado
    limpa_terminal();
    if tarefas.is_empty() {
        print!("Tudo em dia");
    }
    for x in tarefas{
        print!("Tarefa: {}    Feito:{}\n", x.tarefa, x.feito);//TODO: ficar formatado a saida do feito um embaixo do outro
    }
}

pub fn adiciona_tarefa(tarefas:&mut Vec<Todo>){//Pega emprestado e pode mudar
    print!("Nova tarefa:\n");//TODO:receber novas tarefas
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler");

    let nvtarefa = Todo{tarefa:input.trim().to_string(),feito: false,};

    tarefas.push(nvtarefa);
}


pub fn marcar_feito(tarefas: &mut Vec<Todo>){
    imprime_tarefas(&tarefas);
    let mut input = String::new();
    print!("\nDigite o [index] da q vc quer conluir\n");
    io::stdin().read_line(&mut input).expect("Erro ao ler");
    let index: usize = input.trim().parse().expect("Digite um número");//parce-> converte pro tipo declarado, q é isize(index)
    tarefas[index].feito = true;
}