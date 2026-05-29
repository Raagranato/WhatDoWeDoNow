use std::{io};//biblioteca para receber entrada e saida
//use crate::utils::{limpa_terminal};
//use std::io::Write;
use colored::Colorize;
use crate::{todo::Todo};

pub fn imprime_tarefas(tarefas:&Vec<Todo>){//So pega emprestado
    //limpa_terminal();
    if tarefas.is_empty() {
        print!("Tudo em dia\n");
    }
    for x in tarefas{
        print!("Tarefa: {}\t",feito(&x));

    }
    let total = tarefas.len();
    let feitas = tarefas.iter().filter(|t| t.feito).count();
    println!("{}/{} tarefas concluídas", feitas, total);
}

pub fn add_tarefa(tarefas:&mut Vec<Todo>,args :&String){//Pega emprestado e pode mudar
    let mut chars = args.trim().chars();
    let capitalizado = match chars.next() 
    {
    Some(primeiro) => primeiro.to_uppercase().to_string() + chars.as_str(),
    None => String::new(),
    };
    let nvtarefa = Todo { tarefa: capitalizado, feito: false };

    tarefas.push(nvtarefa);
}


pub fn marcar_feito(tarefas: &mut Vec<Todo>,args:&String){ 
    //imprime_tarefas(&tarefas);
    let encontradas: Vec<usize> = tarefas.iter().enumerate().filter(|(_, t)| t.tarefa.to_lowercase()
    .contains(args.as_str())).map(|(i, _)| i).collect();//Da um Vec so co as tarefas que batem
    match encontradas.len(){
        0 => println!("Nenhuma tarefa encontrada!"),
        1 => {
            tarefas[encontradas[0]].feito = true;
            println!("[OK]");
            },
        _=>{ 
            println!("Witch one? Especify");
            for i in encontradas{

                println!("{}: {}", i, tarefas[i].tarefa);
            }
            // print!("> ");
            // io::stdout().flush().unwrap();//limpa buffer
            // let mut args_aux = String::new();
            // io::stdin().read_line(&mut args_aux).expect("Erro ao ler");
            // marcar_feito(tarefas, &args_aux);

            // let mut input = String::new();
            // print!("\nDigite o [index] da q vc quer remover\n");
            // io::stdin().read_line(&mut input).expect("Erro ao ler");
            // let index: usize = input.trim().parse().expect("Digite um número");
            // tarefas[index].feito = true;
        }
    } 

}

pub fn remove_tarefa(tarefas: &mut Vec<Todo>,args:&String){
    let encontradas: Vec<usize> = tarefas.iter().enumerate().filter(|(_, t)| t.tarefa.to_lowercase()
    .contains(args.as_str())).map(|(i, _)| i).collect();//Da um Vec so co as tarefas que batem
    match encontradas.len(){
        0 => println!("Nenhuma tarefa encontrada!"),
        1 => {
            tarefas.remove(encontradas[0]);
            println!("[OK]");
            },
        _=>{ 
            println!("Witch one? Especify");
            for i in encontradas{

                println!("{}: {}", i, tarefas[i].tarefa);
            }
        }
    }
}

pub fn remove_all(tarefas: &mut Vec<Todo>){
    print!("Delete everything?(Y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler");
    if input.trim().is_empty()||input.trim() == "y"||input.trim() == "Y"{
    tarefas.clear();
    print!("[Cleared]");
    }
}


fn feito(x: &Todo) -> String{//Retornar se true retorna X e n retorna ' '(vazio)
    match x.feito {
        true  =>x.tarefa.green().strikethrough().to_string(),
        false => x.tarefa.clone(),
    }
}

pub fn have_this(tarefas: &Vec<Todo>,x: &Todo) -> bool{
    tarefas.iter().any(|t| t.tarefa == x.tarefa)//retorna true se tiver presente no vetor
}
//fn save(tarefa: &Vec<Todo>){
//}