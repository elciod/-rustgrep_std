

use std::fs;
use std::path::Path;

/// Tenta ler o conteúdo de um arquivo e retornar como String.
/// Se falhar (como em arquivos binários), retorna None de forma segura.
pub fn ler_conteudo_arquivo(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

/// Verifica se o caminho informado é uma pasta/diretório
pub fn eh_diretorio(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}
