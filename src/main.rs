
// // mod core {
// //     pub mod args;
// // }

// // use crate::core::args::Argumentos;

// // fn main() {
// //     match Argumentos::novo() {
// //         Ok(args) => {
// //             println!("========== RustGrep STD ==========");
// //             println!("Padrão........: {}", args.padrao);
// //             println!("Arquivos......: {:?}", args.arquivos);
// //             println!("Ignorar caso..: {}", args.ignorar_caso);
// //             println!("Recursivo.....: {}", args.recursivo);
// //             println!("Contagem......: {}", args.contagem);
// //             println!("Número linha..: {}", args.numero_linha);
// //         }

// //         Err(erro) => {
// //             eprintln!("Erro: {}", erro);
// //             Argumentos::ajuda();
// //             std::process::exit(1);
// //         }
// //     }
// // }






// mod core;
// mod io;
// mod utils;

// use core::args::Argumentos;
// use core::buscador::executar_busca;

// fn main() {
//     match Argumentos::novo() {
//         Ok(args) => {
//             println!("========== RustGrep STD ==========");
//             println!("Padrão........: {}", args.padrao);
//             println!("Arquivos......: {:?}", args.arquivos);
//             println!("Ignorar caso..: {}", args.ignorar_caso);
//             println!("Recursivo.....: {}", args.recursivo);
//             println!("Contagem......: {}", args.contagem);
//             println!("Número linha..: {}", args.numero_linha);

//             // Chama o motor de busca passando os argumentos validados
//             if let Err(e) = executar_busca(&args) {
//                 eprintln!("Erro na execução: {}", e);
//             }
//         }
//         Err(e) => {
//             eprintln!("Erro: {}", e);
//             Argumentos::ajuda();
//         }
//     }
// }



use rustgrep_std::core::args::Argumentos;
use rustgrep_std::core::buscador::executar_busca;

fn main() {
    match Argumentos::novo() {
        Ok(args) => {
            if let Err(e) = executar_busca(&args) {
                eprintln!("Erro na busca: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Erro: {}", e);
            Argumentos::ajuda();
        }
    }
}
