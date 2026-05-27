mod todo;//mod so aparece em um arquivo o resto pode usar usando crate sem usar mod
use crate::todo::Todo;
mod tarefas;
use crate::tarefas::*;//ou {imprime_tarefas, adiciona_tarefa, marcar_feito,remove_tarefa,remove_all};(list td, q sucks)
mod utils;
use crate::utils::*;
use std::io::Write;//biblioteca pra da flush no buffer
use std::{io};

/*TODO:
+Marcar digitando parte da string com o maximo de erro de seila? 3
[OK]Tarefas concluídas com strikethrough em verde, pendentes em branco
+Um contador no topo tipo 3/5 tarefas concluídas
+Separador visual entre as tarefas
+Fazer teste automatizados

UX:

+help mostrando todos os comandos com exemplo de uso
+Mensagens de erro mais claras (did you mean: done lavar carro?)
+Confirmação antes do delete — Tem certeza? (s/n)

Funcional:

+Salvar as tarefas em arquivo pra persistir entre sessões
+undo pra desfazer a última ação
*/
fn main(){
    println!("Welcome to WhatDoWe-Now");

    let mut tarefas : Vec<Todo>= Vec::new();//->permite muda a variavel -> mut
    
    menu();
    loop {
        
        print!("> ");
        io::stdout().flush().unwrap();//limpa buffer
        let mut opc = String::new();
        io::stdin().read_line(&mut opc).expect("Erro ao ler");
        let (command, args) = filter_command(&opc);
        if command == "exit"{
            break;
        }
        // match command.trim() {
        //     "menu"=> menu(),
        //     "add"|"Add"=>add_tarefa(&mut tarefas,&mut args),//Passar parematro para remover maybe? done [parametro], sem precisa rpedir qual
        //     "done"=>marcar_feito(&mut tarefas),//Passar parematro para remover maybe? done [parametro], sem precisa rpedir qual
        //     "remove"|"Remove"|"r"=>remove_tarefa(&mut tarefas),//Passar parematro para remover maybe? remove [parametro], sem precisa rpedir qual
        //     "clear"|"Clear"=>remove_all(&mut tarefas),
        //     "exit"=>break,
        //     _ => println!("Not possible")
        // }
        
        // match (command.as_str(), args.is_some()){
        //     ("menu",_)=>menu(),
        //     ("add"|"Add",true)=> add_tarefa(&mut tarefas, &args.unwrap()),
        //     ("add"|"Add",false)=>println!("Structure: add 'Compile code!'"),
        //     ("done"|"Done",true)=>marcar_feito(&mut tarefas, &args.unwrap()),//TODO: Identificar palavras iguais- rapidfuzz
        //     ("done"|"Done",_)=>println!("Structure: done 'Compile code!'"),
        //     ("remove"|"Remove"|"r"|"R",true)=>remove_tarefa(&mut tarefas,&args.unwrap()),
        //     ("remove"|"Remove"|"r"|"R",false)=>println!("Structure: remove 'Compile code!'"),
        //     ("list",_)=>imprime_tarefas(&tarefas),
        //     ("clear"|"Clear",_)=>limpa_terminal(),
        //     ("delete"|"Delete",_)=>remove_all(&mut tarefas),//TODO: Confirmar o delete apertando enter talvez igual em atualizacoes (Y/n) e o enter e padrao maiusculo
        //     ("exit",_)=>break,
        //     (&_, _) => println!("Not a command!"),
        // }
        examine_opc(&command,&mut args,&mut tarefas);
        
    }

}