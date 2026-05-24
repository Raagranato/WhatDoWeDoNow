pub fn limpa_terminal() {
    match cfg!(windows) {
        true  => {std::process::Command::new("cmd").args(["/C", "cls"]).status().unwrap();},//gambiarra pra rodar na windowslop
        false => { std::process::Command::new("clear").status().unwrap(); },
    }
}

pub fn menu(){
    println!("Menu:\n 1 - Add\n 2- Mark done\n 3 - Remove");
}