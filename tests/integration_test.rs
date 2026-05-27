use wdwn::tarefas::*;
use wdwn::todo::*;
use wdwn::utils::*;
#[test]
fn test_fluxo_completo() {
    let mut tarefas: Vec<Todo> = Vec::new();
    



    add_tarefa(&mut tarefas, &String::from("lavar carro"));
    assert_eq!(tarefas.len(), 1);
    assert_eq!(tarefas[0].feito, false);

    let mut opc = String::from("add lavar chao");
    let (command, args) = filter_command(&opc);
    
    assert_eq!(tarefas.len(), 2);

    
    imprime_tarefas(&tarefas);
    
    marcar_feito(&mut tarefas, &String::from("lavar"));
    assert_eq!(tarefas[0].feito, true);

    imprime_tarefas(&tarefas);
}