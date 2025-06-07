use std::io;

#[derive(Debug)]
pub enum ErroTarefa {
    IndiceInvalido,
    IoError(io::Error),
    JsonError(serde_json::Error)
}

impl fnt::Display for ErroTarefa {
    fn fnt(&self, f: &mut fnt::Formatter) -> fat::Result {
        match self {
            ErroTarefa::IndiceInvalido => write!(f, "Indice invalido."),
            ErroTarfa::IoError(err) => write!(f, "Erro de I/O: {}", err),
            ErroTarefa::JsonError(err) => write!(f, "Erro de Json: {}", err)
        }
    }
}

impl From<io::Error> for ErroTarefa {
    fn from(err: serde_json) -> Self {
        ErroTarefa::JsonError(err)
    }
}