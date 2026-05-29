mod todo;//mod so aparece em um arquivo o resto pode usar usando crate sem usar mod
use crate::todo::Todo;
mod tarefas;
//use crate::tarefas::*;//ou {imprime_tarefas, adiciona_tarefa, marcar_feito,remove_tarefa,remove_all};(list td, q sucks)
mod utils;
use crate::utils::*;
use std::io::Write;//biblioteca pra da flush no buffer
use std::{io};

/*TODO:
+Fazer taferas dependerem de uma ou varia fazerem parte de uma
+Marcar digitando parte da string com o maximo de erro de seila? 3
[OK]Tarefas concluídas com strikethrough em verde, pendentes em branco
[OK]Um contador no topo tipo 3/5 tarefas concluídas
+Separador visual entre as tarefas
[OK]Fazer teste automatizados

UX:

+help mostrando todos os comandos com exemplo de uso
[OK]Confirmação antes do delete — Tem certeza? (s/n)

Funcional:

+Salvar as tarefas em arquivo pra persistir entre sessões
+undo pra desfazer a última ação
*/
fn main(){
    println!("Welcome to WhatDoWe-Now");
    

    let mut tarefas : Vec<Todo>= load();//->permite muda a variavel -> mut
    
    menu();
    loop {
        
        print!("> ");
        io::stdout().flush().unwrap();//limpa buffer
        let mut opc = String::new();
        io::stdin().read_line(&mut opc).expect("Erro ao ler");
        let (command,mut args) = filter_command(&opc);
        if command == "exit"{
            break;
        }
        
        examine_opc(&command,&mut args,&mut tarefas);
        save(&tarefas);
    }
    
}