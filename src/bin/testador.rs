
use std::process::Command;

fn main() {
    // Cores ANSI para relatórios visuais
    let azul = "\x1b[1;34m";
    let sem_cor = "\x1b[0m";

    println!("{}=== Iniciando Testes do Rustgrep ==={}", azul, sem_cor);

    // 1. Compilação
    if !Command::new("cargo").arg("build").status().unwrap().success() {
        println!("Erro na compilação.");
        return;
    }

    // 2. Cenários de Teste
    let testes = vec![
        (vec!["-n", "pub", "src/main.rs"], "Busca com Número de Linha"),
        (vec!["-r", "-c", "struct", "src"], "Busca Recursiva e Contagem"),
    ];

    // 3. Execução Automatizada
    for (args, descricao) in testes {
        println!("\n[Teste] {}: cargo run -- {}", descricao, args.join(" "));
        Command::new("cargo")
            .arg("run")
            .arg("--")
            .args(args)
            .status()
            .expect("Falha ao rodar teste");
    }
}
