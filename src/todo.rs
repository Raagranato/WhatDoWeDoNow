use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]//save
pub struct Todo{
    pub tarefa:String,//Se for arq diferentes tem q add o pub sempre
    pub feito:bool,
}


