#[derive(Debug)]

pub struct Agendamento {
    pub nome: String,
    pub cpf: String,
    pub celular: String,
    pub nome_pet: String,
    pub motivo: String,
}

impl Agendamento {
    pub fn new(nome: String, cpf: String, celular: String, nome_pet: String, motivo: String) -> Self {
        Agendamento {
            nome,
            cpf,
            celular,
            nome_pet,
            motivo,
        }
    }
}