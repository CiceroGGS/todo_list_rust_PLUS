use serde::{Serialize, Deserialize};
use crate::erros::ErroTarefa;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarefa {
    pub descricao: String,
    pub concluida: bool
}

impl Tarefa {
    pub fn nova(descricao: &str) -> Self {
        Tarefa {
            descricao: descricao.to_string(),
            concluida: false
        }
    }
}

pub trait GerenciadorDeTarefas<T> {
    fn adicionar_tarefa(&mut self, terefa: T);
    fn remover_tarefa(&mut self, indice:usize) -> Result<(), ErroTarefa>;
    fn listar_tarefas(&self);
    fn concluir_tarefa(&mut self, indice: usize) -> Result<(), ErroTarefa>;
}