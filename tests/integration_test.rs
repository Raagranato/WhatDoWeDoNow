use wdwn::tarefas::*;
use wdwn::todo::*;
use wdwn::utils::*;
#[test]
fn test_fluxo_completo() {
    let mut tarefas: Vec<Todo> = Vec::new();
    

    let mut opc = String::from("add lavar chao");
    let (command,mut args) = filter_command(&opc);
    examine_opc(&command, &mut args, &mut tarefas);
    assert_eq!(tarefas[0].feito, false);
    assert_eq!(tarefas.len(), 1);
    assert_eq!(have_this(&tarefas,&tarefas[0]), true);

    opc = String::from("add lavar carro");
    let (command,mut args) = filter_command(&opc);
    examine_opc(&command, &mut args, &mut tarefas);


    opc = String::from("add lavar geladeira");
    let (command,mut args) = filter_command(&opc);
    examine_opc(&command, &mut args, &mut tarefas);
    assert_eq!(tarefas[2].feito, false);
    

    opc = String::from("add compilar codigo");
    let (command,mut args) = filter_command(&opc);
    examine_opc(&command, &mut args, &mut tarefas);
    assert_eq!(tarefas[3].feito, false);

    let nvtarefa = Todo{tarefa:"lavar carro".to_string(),feito:false,};
    opc = String::from("remove lavar car");
    let (command,mut args) = filter_command(&opc);
    examine_opc(&command, &mut args, &mut tarefas);
    assert_eq!(have_this(&tarefas,&nvtarefa), false);



    assert_eq!(tarefas.len(), 3);
    let (command,mut args) = filter_command(&opc);
    examine_opc(&command, &mut args, &mut tarefas);


    assert_eq!(tarefas.len(), 3);

    
    marcar_feito(&mut tarefas, &String::from("lavar c"));
    assert_eq!(tarefas[0].feito, true);

}