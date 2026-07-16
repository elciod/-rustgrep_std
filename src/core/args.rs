


use std::env;

#[derive(Debug)]
pub struct Argumentos {
    pub padrao: String,
    pub arquivos: Vec<String>,
    pub ignorar_caso: bool,
    pub recursivo: bool,
    pub contagem: bool,
    pub numero_linha: bool,
}

impl Argumentos {
    pub fn novo() -> Result<Self, String> {
        let mut ignorar_caso = false;
        let mut recursivo = false;
        let mut contagem = false;
        let mut numero_linha = false;

        let mut argumentos = env::args().skip(1);

        let mut padrao = None;
        let mut arquivos = Vec::new();

        while let Some(arg) = argumentos.next() {
            match arg.as_str() {
                "-i" | "--ignore-case" => ignorar_caso = true,

                "-r" | "--recursive" => recursivo = true,

                "-c" | "--count" => contagem = true,

                "-n" | "--line-number" => numero_linha = true,

                _ => {
                    if padrao.is_none() {
                        padrao = Some(arg);
                    } else {
                        arquivos.push(arg);
                    }
                }
            }
        }

        let padrao = match padrao {
            Some(p) => p,
            None => return Err("Informe o padrão de busca.".into()),
        };

        if arquivos.is_empty() {
            return Err("Informe pelo menos um arquivo.".into());
        }

        Ok(Self {
            padrao,
            arquivos,
            ignorar_caso,
            recursivo,
            contagem,
            numero_linha,
        })
    }

    pub fn ajuda() {
        println!();
        println!("RustGrep STD");
        println!();
        println!("Uso:");
        println!("    rustgrep_std [opções] <padrao> <arquivo...>");
        println!();
        println!("Opções:");
        println!("    -i   Ignorar maiúsculas/minúsculas");
        println!("    -r   Buscar recursivamente");
        println!("    -c   Contar ocorrências");
        println!("    -n   Mostrar número da linha");
    }
}





