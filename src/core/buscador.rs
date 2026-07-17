


use crate::core::args::Argumentos;
use crate::io::arquivos::{ler_conteudo_arquivo, eh_diretorio};
use crate::utils::formatador::{colorir_termo, formatar_cabecalho_arquivo, formatar_nome_contagem};
use std::fs;
use std::path::Path;

/// PONTO DE ENTRADA PRINCIPAL: Coordena o fluxo inicial da busca.
pub fn executar_busca(args: &Argumentos) -> Result<(), String> {
    let mut total_global = 0;          // Acumulador de todas as ocorrências do projeto todo
    let mut arquivos_processados = 0;   // Contador para saber quantos arquivos físicos foram lidos

    // LOOP INICIAL: Varre a lista de alvos (arquivos/pastas) digitados pelo usuário
    for caminho in &args.arquivos {
        let path = Path::new(caminho);

        // FLUXO DE SEGURANÇA: Se o arquivo ou pasta não existir, avisa e vai para o próximo
        if !path.exists() {
            eprintln!("Erro: O caminho '{}' não existe.", caminho);
            continue;
        }

        // FLUXO DE DECISÃO (Usando o arquivos.rs): Verifica se o alvo atual é uma pasta
        if eh_diretorio(path) {
            if args.recursivo {
                // Se a flag -r estiver ativa, chama a função recursiva de diretório
                match buscar_em_diretorio(path, args, &mut arquivos_processados) {
                    Ok(total) => total_global += total, // Soma as ocorrências achadas na pasta no total geral
                    Err(e) => eprint!("Erro ao ler diretório '{}': {}", caminho, e),
                }
            } else {
                // Se passou pasta mas esqueceu o -r, dispara o aviso preventivo
                eprintln!(
                    "Aviso: '{}' é um diretório. Use a flag '-r' para buscar dentro dele.",
                    caminho
                );
            }
        // Se for um arquivo direto (ex: src/main.rs), direciona para a busca direta
        } else {
            arquivos_processados += 1;
            total_global += buscar_em_arquivo(path, args); // Soma os matches deste arquivo
        }
    }

    // SAÍDA DA FLAG -c: Se processou mais de 1 arquivo, exibe a moldura verde com o Total Global
    if args.contagem && arquivos_processados > 1 {
        println!("\n\x1b[1;32m=====================================");
        println!("TOTAL GLOBAL DE OCORRÊNCIAS: {}", total_global);
        println!("=====================================\x1b[0m");
    }

    Ok(())
}

/// FLUXO RECURSIVO: Navega por pastas abrindo arquivos e subpastas de forma contínua.
fn buscar_em_diretorio(dir: &Path, args: &Argumentos, qtd_arquivos: &mut usize) -> std::io::Result<usize> {
    let mut total_diretorio = 0;

    if dir.is_dir() {
        // Varre os itens da pasta atual
        for entrada in fs::read_dir(dir)? {
            let entrada = entrada?;
            let caminho = entrada.path();

            // Se achar outra pasta lá dentro, faz a RECURSÃO (chama a si mesma para entrar nela)
            if eh_diretorio(&caminho) {
                total_diretorio += buscar_em_diretorio(&caminho, args, qtd_arquivos)?;
            } else {
                // Se for arquivo, incrementa o marcador e executa a busca textual nele
                *qtd_arquivos += 1;
                total_diretorio += buscar_em_arquivo(&caminho, args);
            }
        }
    }
    Ok(total_diretorio) // Devolve a soma acumulada da pasta para o loop principal
}

/// FLUXO DE LEITURA E FILTRAGEM: Onde a análise textual da linha acontece de verdade.
fn buscar_em_arquivo(path: &Path, args: &Argumentos) -> usize {
    let caminho_str = path.to_string_lossy();

    // LEITURA SEGURA (Usando o arquivos.rs): Extrai o conteúdo ou descarta em caso de erro (ex: binários)
    let conteudo = match ler_conteudo_arquivo(path) {
        Some(c) => c,
        None => return 0,
    };

    let mut total_ocorrencias = 0; // Registra os matches específicos deste arquivo
    
    // NORMALIZAÇÃO -i: Se ativo, cria o padrão de busca todo em letras minúsculas
    let padrao_busca = if args.ignorar_caso {
        args.padrao.to_lowercase()
    } else {
        args.padrao.clone()
    };

    let mut linhas_encontradas = String::new(); // Buffer para salvar as linhas antes de dar print

    // LOOP DE LINHAS: Varre o arquivo linha por linha quebrando os blocos de texto
    for (indice, linha) in conteudo.lines().enumerate() {
        let numero_linha = indice + 1; // Ajusta o contador para começar em 1

        // NORMALIZAÇÃO -i: Se ativo, cria uma cópia da linha toda em minúsculo para comparar
        let linha_comparacao = if args.ignorar_caso {
            linha.to_lowercase()
        } else {
            linha.to_string()
        };

        // FILTRAGEM TEXTUAL: Verifica se a linha contém o que o usuário quer
        if linha_comparacao.contains(&padrao_busca) {
            total_ocorrencias += 1;

            // Se for apenas contagem (-c), não precisa formatar linhas nem cores agora
            if !args.contagem {
                let mut linha_colorida = String::new(); // Nova linha com as cores ANSI injetadas
                let mut restante_linha_ref = linha;      // Ponteiro móvel da linha original

                // FLUXO DE PINTURA INTELIGENTE: Pinta todos os matches mantendo maiúsculas/minúsculas
                while let Some(pos) = restante_linha_ref.to_lowercase().find(&padrao_busca) {
                    let pos_fim = pos + padrao_busca.len();
                    
                    // 1. Copia o trecho que estava antes da palavra encontrada
                    linha_colorida.push_str(&restante_linha_ref[..pos]);
                    
                    // 2. Isola o termo original e passa pelo formatador.rs para envelopar em Vermelho
                    let termo_original = &restante_linha_ref[pos..pos_fim];
                    linha_colorida.push_str(&colorir_termo(termo_original));
                    
                    // 3. Move o ponteiro para frente, descartando o pedaço já pintado
                    restante_linha_ref = &restante_linha_ref[pos_fim..];
                    
                    // Otimização: Se for busca exata e não houver mais termos, para o loop de pintura
                    if !args.ignorar_caso && !restante_linha_ref.contains(&args.padrao) {
                        break;
                    }
                }
                // Adiciona o fim da linha que sobrou após a última palavra pintada
                linha_colorida.push_str(restante_linha_ref);

                // Monta a string final aplicando a flag -n (número da linha)
                if args.numero_linha {
                    linhas_encontradas.push_str(&format!("{}: {}\n", numero_linha, linha_colorida));
                } else {
                    linhas_encontradas.push_str(&format!("{}\n", linha_colorida));
                }
            }
        }
    }

    // FLUXO DE IMPRESSÃO VISUAL: Só executa se houver matchings no arquivo
    if total_ocorrencias > 0 {
        if args.contagem {
            // Se for flag -c: Usa formatador.rs para exibir o nome do arquivo em Amarelo + Total
            println!("{} {}", formatar_nome_contagem(&caminho_str), total_ocorrencias);
        } else {
            // Se for busca normal: Usa formatador.rs para exibir o cabeçalho em Azul + Linhas
            println!("\n{}", formatar_cabecalho_arquivo(&caminho_str));
            print!("{}", linhas_encontradas);
        }
    }

    total_ocorrencias // Devolve a quantidade para o somador global
}
