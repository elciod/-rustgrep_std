# rustgrep_std 🚧

Este projeto está atualmente em **desenvolvimento ativo**.

O objetivo é o desenvolvimento de uma ferramenta de busca textual parecida com o comando `grep` do Linux, construída inteiramente em **Rust** e com o grande diferencial de utilizar apenas a **biblioteca padrão (std)**, sem o uso de bibliotecas (crates) externas como `clap`, `regex` ou `walkdir`.

---

## 🛠️ Como Clonar e Inicializar a Estrutura

Caso precise recriar a estrutura de pastas e arquivos modulares no Windows (PowerShell), utilize os comandos abaixo:

```powershell
# Criar os Diretórios
New-Item -ItemType Directory src\core
New-Item -ItemType Directory src\io
New-Item -ItemType Directory src\utils

# Criar os Arquivos Base
New-Item src\core\mod.rs -ItemType File
New-Item src\core\args.rs -ItemType File
New-Item src\core\buscador.rs -ItemType File
New-Item src\io\mod.rs -ItemType File
New-Item src\io\arquivos.rs -ItemType File
New-Item src\utils\mod.rs -ItemType File
New-Item src\utils\formatador.rs -ItemType File
```

---

## 🌲 Árvore do Projeto

A arquitetura do software foi dividida seguindo o princípio de responsabilidade única:

```text
rustgrep_std/
│
├── Cargo.toml
├── README.md
│
└── src/
    ├── main.rs
    │
    ├── core/
    │   ├── mod.rs          # Expõe o módulo core
    │   ├── args.rs         # Analisador (Parser) de argumentos da CLI
    │   └── buscador.rs     # Motor principal de varredura textual
    │
    ├── io/
    │   ├── mod.rs          # Expõe o módulo io
    │   └── arquivos.rs     # Interação segura com o sistema de arquivos
    │
    └── utils/
        ├── mod.rs          # Expõe o módulo utils
        └── formatador.rs   # Gerenciador de cores e estilização ANSI
```

---

## 🎛️ Flags Suportadas

O utilitário aceita tanto a sintaxe curta quanto a sintaxe longa para os parâmetros:

| Curta | Longa | Descrição |
| :---: | :--- | :--- |
| `-n` | `--line-number` | Exibe o número correspondente da linha no arquivo |
| `-i` | `--ignore-case` | Executa a busca ignorando maiúsculas e minúsculas |
| `-c` | `--count` | Exibe apenas a contagem de ocorrências por arquivo e o total global |
| `-r` | `--recursive` | Realiza a varredura recursiva dentro de pastas e diretórios |

---

## 🧪 Roteiro Prático de Testes

Execute estes comandos no seu terminal moderno (**Windows Terminal** ou **WSL**) para validar o comportamento do sistema:

### 1. Testes de Flags Individuais
```powershell
# Exibição de Linhas (-n)
cargo run -- -n "pub" src/core/args.rs

# Ignorar Caixa / Case-Insensitive (-i)
cargo run -- -i "ARGUMENTOS" src/core/args.rs

# Relatório de Contagem Simples (-c)
cargo run -- -c "let" src/core/args.rs
```

### 2. Testes Combinados
```powershell
# Case-Insensitive + Número de Linha (-i -n)
cargo run -- -i -n "struct" src/core/args.rs

# Contagem + Case-Insensitive (-c -i)
cargo run -- -c -i "fn" src/core/args.rs
```

### 3. Testes com Múltiplos Arquivos
```powershell
# Busca textual em múltiplos arquivos soltos
cargo run -- -n "pub" src/core/args.rs src/main.rs

# Contagem individual e acumulada de múltiplos arquivos
cargo run -- -c "impl" src/core/args.rs src/core/buscador.rs
```

### 4. Testes do Motor Recursivo em Diretórios
```powershell
# Aviso de Segurança (Tentar passar pasta sem a flag -r)
cargo run -- "pub" src

# Varredura Recursiva Completa com Linhas e Cores (-r -n -i)
cargo run -- -r -n -i "Argumentos" src

# Tabela Amarela de Contagem + Total Global em Verde (-r -c)
cargo run -- -r -c "pub" src

# Contagem Geral com Case-Insensitive no Projeto Todo (-r -c -i)
cargo run -- -r -c -i "struct" src
```

### 5. Tratamento de Erros e Proteção
```powershell
# Forçar caminho inexistente (Validação preventiva)
cargo run -- "teste" pasta_que_nao_existe

# Misturar arquivo inexistente com um existente (Continuidade de fluxo)
cargo run -- -n "pub" arquivo_fantasma.txt src/main.rs
```
