
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

\Users\AMD\dev\rustgrep_std> rustgrep_std

PS C:\Users\AMD\dev\rustgrep_std> git rm --cached src/main.exe src/main.pdb
rm 'src/main.exe'
rm 'src/main.pdb'
PS C:\Users\AMD\dev\rustgrep_std> 
