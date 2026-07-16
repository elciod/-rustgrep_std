
mod core {
    pub mod args;
}

use crate::core::args::Argumentos;

fn main() {
    match Argumentos::novo() {
        Ok(args) => {
            println!("========== RustGrep STD ==========");
            println!("Padrão........: {}", args.padrao);
            println!("Arquivos......: {:?}", args.arquivos);
            println!("Ignorar caso..: {}", args.ignorar_caso);
            println!("Recursivo.....: {}", args.recursivo);
            println!("Contagem......: {}", args.contagem);
            println!("Número linha..: {}", args.numero_linha);
        }

        Err(erro) => {
            eprintln!("Erro: {}", erro);
            Argumentos::ajuda();
            std::process::exit(1);
        }
    }
}



