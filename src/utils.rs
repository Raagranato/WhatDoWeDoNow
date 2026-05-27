pub fn limpa_terminal() {
    match cfg!(windows) {
        true  => {std::process::Command::new("cmd").args(["/C", "cls"]).status().unwrap();},//gambiarra pra rodar na windowslop
        false => { std::process::Command::new("clear").status().unwrap(); },
    }
}

pub fn menu(){
    println!(" Menu\n Add\n Mark done\n Remove\n Clear\n exit" );//TODO:Formatar p colocar a primeira letra maiuscula
}