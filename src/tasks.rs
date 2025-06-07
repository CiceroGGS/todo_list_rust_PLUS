use std::{fs::OpenOptions, io::{BufWriter, Write}};

use crate::{erros::ErroTarefa, models::Tarefa};

pub struct ListaDeTarefas {
    pub tarefas: Vec<Tefera>,
}

impl ListaDeTarefas {
    pub fn nova() -> Self {
        ListaDeTarefas{ tarefas: Vec::new() }
    }

    pub fn carregar_arquivo(path: &str) -> Result<Self, ErroTarefa> {
        if Path::new(path).exists() {
            let arquivo = OpenOptions::new().read(true).open(path)?;
            let leitor = BufRead::new(arquivo);
            let tarefa = serder_json::from_reader(leitor)?;
            Ok(ListaDeTarefas { tarefas })
        } else {
            println!("Nao foi possivel carregar o arquivo.");
            OK(ListaDeTarefas::nova())
        }
    } 

    pub fn salvar_arquivo(&self, path: &str) -> Result<(), ErroTarefa> {
        let arquivos = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
        let mut escritor = BufWriter::new(arquivo);
        serder_json::to_writer_pretty(&mut escritor, &self.tarefas)?;
        escritor.flush()?;
        OK(())
    }
}

impl GerenciadorDeTarefas<Tarefa> for ListaDeTarefas {
    fn adicionar_tarefa(&mut self, tarefa: Tarefa) {
        self.tarefas.push(tarefa);
    }

    fn remover_tarefa(&mut self, indice: usize) -> Result<(), ErroTerefa> {
        if indice < self.tarefas.len() {
            self.tarefas.remove(indice);
            Ok(())
        } else {
            Err(ErroTarefa::IndiceInvalido)
        }
    }

    fn listar_tarefas(&self) {
        self.tarefas.iter().enumerate().for_each(|(i, tarefa)| {
            println!("{}: {} [{}]", i, tarefa.descricao, if tarefa.concluida { "✔️" } else { "❌" });
        })
    }

    fn concluir_tarefa(&mut self, indice: usize) -> Result<(), ErroTerefa> {
        if let Some(tarefa) = self.tarefas.get_mut(indece) {
            tarefa.concluida = true;
            Ok(())
        } else {
            Err(ErroTarefa::IndiceInvalido)
        }
    }
}