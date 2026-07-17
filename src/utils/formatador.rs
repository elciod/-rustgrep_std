  /// Envolve um texto com a cor Vermelha em Negrito para dar destaque (Padrão Grep)
pub fn colorir_termo(texto: &str) -> String {
    format!("\x1b[1;31m{}\x1b[0m", texto)
}

/// Formata o cabeçalho do arquivo em Azul Brilhante
pub fn formatar_cabecalho_arquivo(caminho: &str) -> String {
    format!("\x1b[1;34m--- Buscando em: {} ---\x1b[0m", caminho)
}

/// Formata o nome do arquivo em Amarelo Claro para o relatório de contagem (-c)
pub fn formatar_nome_contagem(caminho: &str) -> String {
    format!("\x1b[33m{}:\x1b[0m", caminho)
}
