use crate::tarefas::*;
use crate::todo::Todo;
use colored::Colorize;
use std::fs;
use serde_json;

//use std::{io};
pub fn clean_terminal() {
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
    println!("{}", "=== WhatDoWe-Now ===".bold());
    println!("  {} [Task]     — add Task", "add".green());
    println!("  {} [Task]    — conclude Task", "done".green());
    println!("  {} [Task]  — remove Task", "remove".green());
    println!("  {}              — list everything", "list".green());
    println!("  {}            — delete everything", "delete".green());
    println!("  {}              — clean terminal", "clear".green());
    println!("  {}              — exit aplication", "exit".red());
}

pub fn examine_opc(command:&String,args: &mut Option<String>,tarefas: &mut Vec<Todo>){
        match (command.as_str(), args.is_some()){
            ("menu",_)=>menu(),
            ("add"|"Add",true)=> add_task(tarefas, &args.as_ref().unwrap()),
            ("add"|"Add",false)=>println!("Structure: add 'Compile code!'"),
            ("done"|"Done",true)=>mark_done(tarefas, &args.as_ref().unwrap()),//TODO: Identificar palavras iguais- rapidfuzz
            ("done"|"Done",_)=>println!("Structure: done 'Compile code!'"),
            ("remove"|"Remove"|"r"|"R",true)=>remove_task(tarefas,&args.as_ref().unwrap()),
            ("remove"|"Remove"|"r"|"R",false)=>println!("Structure: remove 'Compile code!'"),
            ("list",_)=>print_tasks(&tarefas),
            ("clear"|"Clear",_)=>clean_terminal(),
            ("delete"|"Delete",_)=>remove_all(tarefas),//TODO: Confirmar o delete apertando enter talvez igual em atualizacoes (Y/n) e o enter e padrao maiusculo
            (&_, _) => println!("Not a command!"),
        }
}



pub fn save(tarefas: &Vec<Todo>) {
    let json = serde_json::to_string(tarefas).unwrap();
    fs::write("data/Tasks.json", json).unwrap();
}

pub fn load() -> Vec<Todo> {
    let json = fs::read_to_string("data/Tasks.json").unwrap_or(String::from("[]"));
    serde_json::from_str(&json).unwrap()
}