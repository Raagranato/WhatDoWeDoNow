mod todo;//mod so aparece em um arquivo o resto pode usar usando crate sem usar mod
use crate::todo::Todo;
mod tarefas;
use crate::tarefas::*;//ou {imprime_tarefas, adiciona_tarefa, marcar_feito,remove_tarefa,remove_all};(list td, q sucks)
mod utils;
use crate::utils::*;
use std::io::Write;//biblioteca pra da flush no buffer
use std::{io};

//TODO:Arummar formataçao na hora de limpar o terminal, dar esso se n digitar direito
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
    loop {
        
        print!("> ");
        io::stdout().flush().unwrap();//limpa buffer
        let mut opc = String::new();
        io::stdin().read_line(&mut opc).expect("Erro ao ler");
        let (command, args) = filter_command(&opc);

        // match command.trim() {
        //     "menu"=> menu(),
        //     "add"|"Add"=>add_tarefa(&mut tarefas,&mut args),//Passar parematro para remover maybe? done [parametro], sem precisa rpedir qual
        //     "done"=>marcar_feito(&mut tarefas),//Passar parematro para remover maybe? done [parametro], sem precisa rpedir qual
        //     "remove"|"Remove"|"r"=>remove_tarefa(&mut tarefas),//Passar parematro para remover maybe? remove [parametro], sem precisa rpedir qual
        //     "clear"|"Clear"=>remove_all(&mut tarefas),
        //     "exit"=>break,
        //     _ => println!("Not possible")
        // }
        
        match (command.as_str(), args.is_some()){
            ("menu",_)=>menu(),
            ("add"|"Add",true)=> add_tarefa(&mut tarefas, &args.unwrap()),
            ("add"|"Add",false)=>println!("Structure: add 'Compile code!'"),
            //("done"|"Done",true)=>marcar_feito(&mut tarefas),
            //("done"|"Done",_)=>println!("Structure: done 'Compile code!'"),//TODO: Identificar palavras iguais- rapidfuzz
            //("remove"|"Remove"|"r"|"R",true)=>remove_tarefa(&mut tarefas),
            //("remove"|"Remove"|"r"|"R",false)=>println!("Structure: remove 'Compile code!'"),
            ("list",_)=>imprime_tarefas(&tarefas),
            ("clear"|"Clear",_)=>limpa_terminal(),
            ("delete"|"Delete",_)=>remove_all(&mut tarefas),//TODO: Confirmar o delete apertando enter talvez igual em atualizacoes (Y/n) e o enter e padrao maiusculo
            ("exit",_)=>break,
            (&_, _) => println!("Not a command!"),
            //(_,_)=>println!("Not a command\n"),
        }

        
    }

}