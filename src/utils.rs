use crate::tarefas::*;
use crate::todo::Todo;

//use std::{io};
pub fn limpa_terminal() {
    match cfg!(windows) {
        true  => {std::process::Command::new("cmd").args(["/C", "cls"]).status().unwrap();},//gambiarra pra rodar na windowslop
        false => { std::process::Command::new("clear").status().unwrap(); },
    }
}

pub fn filter_command(main_input: &String)-> (String,Option<String>){//Option -> ou retorna none(==null) ou algo
    
    let parts: Vec<&str> = main_input.trim().split(' ').collect();//collect faz um vetor dq foi separado no split
    if parts.len() == 1{
        return (parts[0].to_string(), None);//Usar argumento.is_none() ou .is_some
    }
    else{
        return (parts[0].to_string(), Some(parts[1..].join(" ").to_string()));
    }
}

pub fn menu(){
    println!(" menu\n add\n done\n remove\n list\n delete\n clear\n undo \nexit" );
}

pub fn examine_opc(command:&String,args: &mut Option<String>,tarefas: &mut Vec<Todo>){
        match (command.as_str(), args.is_some()){
            ("menu",_)=>menu(),
            ("add"|"Add",true)=> add_tarefa(tarefas, &args.as_ref().unwrap()),
            ("add"|"Add",false)=>println!("Structure: add 'Compile code!'"),
            ("done"|"Done",true)=>marcar_feito(tarefas, &args.as_ref().unwrap()),//TODO: Identificar palavras iguais- rapidfuzz
            ("done"|"Done",_)=>println!("Structure: done 'Compile code!'"),
            ("remove"|"Remove"|"r"|"R",true)=>remove_tarefa(tarefas,&args.as_ref().unwrap()),
            ("remove"|"Remove"|"r"|"R",false)=>println!("Structure: remove 'Compile code!'"),
            ("list",_)=>imprime_tarefas(&tarefas),
            ("clear"|"Clear",_)=>limpa_terminal(),
            ("delete"|"Delete",_)=>remove_all(tarefas),//TODO: Confirmar o delete apertando enter talvez igual em atualizacoes (Y/n) e o enter e padrao maiusculo
            (&_, _) => println!("Not a command!"),
        }
}