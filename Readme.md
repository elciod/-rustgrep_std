
>estrutura do projeto!

# Diretorios

New-Item -ItemType Directory src\core
New-Item -ItemType Directory src\io
New-Item -ItemType Directory src\utils

>Arquivos

New-Item src\core\args.rs -ItemType File
New-Item src\core\buscador.rs -ItemType File
New-Item src\io\arquivos.rs -ItemType File
New-Item src\utils\formatador.rs -ItemType File

# Arvore do Projeto:

rustgrep_std/
│
├── Cargo.toml
├── README.md
│
└── src/
    │
    ├── main.rs
    │
    ├── core/
    │   ├── args.rs
    │   └── buscador.rs
    │
    ├── io/
    │   └── arquivos.rs
    │
    └── utils/
        └── formatador.rs

# ##########################################

-n
-i
-c
-r

---------------------------------------------------------

--numero-linha
--ignore-case
--count
--recursive

cargo run -- -n "pub" src/core/args.rs

cargo run -- -i "ARGUMENTOS" src/core/args.rs

cargo run -- -c "let" src/core/args.rs

cargo run -- -i -n "struct" src/core/args.rs

cargo run -- -c -i "fn" src/core/args.rs

cargo run -- -n "pub" src/core/args.rs src/main.rs

cargo run -- -c "impl" src/core/args.rs src/core/buscador.rs


cargo run -- "pub" src

cargo run -- -r -c -i "struct" src


cargo run -- -r -n -i "Argumentos" src


cargo run -- "teste" pasta_que_nao_existe

cargo run -- -n "pub" arquivo_fantasma.txt src/main.rs
