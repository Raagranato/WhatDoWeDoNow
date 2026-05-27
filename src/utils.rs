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
    println!(" Menu\n Add\n Mark done\n Remove\n Clear\n exit" );//TODO:Formatar p colocar a primeira letra maiuscula
}