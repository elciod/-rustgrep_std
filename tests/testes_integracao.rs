


use rustgrep_std::core::args::Argumentos;
use rustgrep_std::core::buscador::executar_busca;

#[test]
fn teste_verificacao_estruturas_base() {
    // Simula a struct de argumentos preenchida diretamente via código
    let args = Argumentos {
        padrao: "pub".to_string(),
        arquivos: vec!["src/core/args.rs".to_string()],
        ignorar_caso: false,
        recursivo: false,
        contagem: false,
        numero_linha: true,
    };

    // Executa o motor direto pelo código. Se ele rodar sem pânicos, o teste passa!
    let resultado = executar_busca(&args);
    assert!(resultado.is_ok());
}

#[test]
fn teste_modo_contagem_recursiva() {
    let args = Argumentos {
        padrao: "struct".to_string(),
        arquivos: vec!["src".to_string()],
        ignorar_caso: true,
        recursivo: true,
        contagem: true,
        numero_linha: false,
    };

    let resultado = executar_busca(&args);
    assert!(resultado.is_ok());
}
