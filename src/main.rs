struct Todo{
    tarefa:String,
    feito:bool,
}
fn main(){
    println!("Bem vindo ao WhatDoWe-Now");
    /*--PASSOS PRA FAZER TODO LIST--
    -Imprime lista
    -Adiciona item
    -Marcar como feito*/
    menu();
    let mut tarefas : Vec<Todo>= Vec::new();//->permite muda a variavel -mut

    let t = Todo{tarefa: String::from("Lavar carro"),feito: false,};
    tarefas.push(t);

    imprime_tarefas(&tarefas);
    


}

fn menu(){
    println!("Menu:\n 1 - Add\n 2- Mark done\n 3 - Remove");
}


fn imprime_tarefas(tarefas:&Vec<Todo>){
    if tarefas.is_empty() {
        print!("Tudo em dia");
    }
    for x in tarefas{
        print!("Tarefa: {}    Feito:{}\n", x.tarefa, x.feito);
    }
}
