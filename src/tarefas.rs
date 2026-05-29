use std::{io};//biblioteca para receber entrada e saida
use std::io::Write;
use colored::Colorize;
//use crate::tarefas;
use crate::{todo::Todo};

pub fn print_tasks(tarefas:&Vec<Todo>){//So pega emprestado
    if tarefas.is_empty() {
        print!("Everything done! :)\n");
    }
    for x in tarefas{
        print!("Task: {}\t",done(&x));

    }
    let total = tarefas.len();
    let feitas = tarefas.iter().filter(|t| t.feito).count();
    println!("{}/{} Done", feitas, total);
}

pub fn add_task(tarefas:&mut Vec<Todo>,args :&String){//Pega emprestado e pode mudar
    let mut chars = args.trim().chars();
    let capitalizado = match chars.next() 
    {
    Some(primeiro) => primeiro.to_uppercase().to_string() + chars.as_str(),
    None => String::new(),
    };
    let nvtarefa = Todo { tarefa: capitalizado, feito: false };

    tarefas.push(nvtarefa);
    println!("[OK]");

}


pub fn mark_done(tarefas: &mut Vec<Todo>,args:&String){ 

    let encontradas: Vec<usize> = tarefas.iter().enumerate().filter(|(_, t)| t.tarefa.to_lowercase()
    .contains(args.as_str())).map(|(i, _)| i).collect();//Da um Vec so co as tarefas que batem
    match encontradas.len(){
        0 => println!("task not found!"),
        1 => {
            tarefas[encontradas[0]].feito = true;
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

pub fn remove_task(tarefas: &mut Vec<Todo>,args:&String){
    let encontradas: Vec<usize> = tarefas.iter().enumerate().filter(|(_, t)| t.tarefa.to_lowercase()
    .contains(args.as_str())).map(|(i, _)| i).collect();//Da um Vec so co as tarefas que batem
    match encontradas.len(){
        0 => println!("Task not Found!"),
        1 => {
            tarefas.remove(encontradas[0]);
            println!("[OK]");
            },
        _=>{ 
            println!("Witch one did you mean? Especify");
            for i in encontradas{

                println!("{}: {}", i, tarefas[i].tarefa);
            }
        }
    }
}

pub fn remove_all(tarefas: &mut Vec<Todo>){
    print!("Delete everything?(Y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read");
    if input.trim().is_empty()||input.trim() == "y"||input.trim() == "Y"{
    tarefas.clear();
    println!("[Cleared]");
    }
}


fn done(x: &Todo) -> String{//Retornar se true retorna X e n retorna ' '(vazio)
    match x.feito {
        true  =>x.tarefa.green().strikethrough().to_string(),
        false => x.tarefa.clone(),
    }
}

// pub fn have_this(tarefas: &Vec<Todo>,x: &Todo) -> bool{
//     tarefas.iter().any(|t| t.tarefa == x.tarefa)//retorna true se tiver presente no vetor
// }
