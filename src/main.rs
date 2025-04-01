//! WebSearch_rust é um motor de busca em linha de comando que utiliza o Mojeek como fonte de dados.
//! 
//! Este programa permite aos usuários realizar buscas na web e receber resultados formatados
//! incluindo título, descrição, link, fonte e data de cada resultado encontrado.

mod models;
mod crawler;

use std::{time::Instant, io};
use models::format_results;
use crawler::WebCrawler;

/// Função principal que gerencia a interface do usuário e o fluxo do programa.
/// 
/// # Errors
/// Retorna um erro se houver falha na inicialização do crawler ou na execução da busca.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n");
    println!("██╗    ██╗███████╗██████╗  ██████╗██████╗  █████╗ ██╗    ██╗██╗     ███████╗██████╗ ");
    println!("██║    ██║██╔════╝██╔══██╗██╔════╝██╔══██╗██╔══██╗██║    ██║██║     ██╔════╝██╔══██╗");
    println!("██║ █╗ ██║█████╗  ██████╔╝██║     ██████╔╝███████║██║ █╗ ██║██║     █████╗  ██████╔╝");
    println!("██║███╗██║██╔══╝  ██╔══██╗██║     ██╔══██╗██╔══██║██║███╗██║██║     ██╔══╝  ██╔══██╗");
    println!("╚███╔███╔╝███████╗██████╔╝╚██████╗██║  ██║██║  ██║╚███╔███╔╝███████╗███████╗██║  ██║");
    println!(" ╚══╝╚══╝ ╚══════╝╚═════╝  ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚══════╝╚══════╝╚═╝  ╚═╝");
    println!("====================================== 2025 ========================================");
    println!("================================== marioCodeLabs ===================================");
    
    println!("Digite o termo que deseja pesquisar:");
    let mut query = String::new();
    io::stdin().read_line(&mut query)?;
    let query = query.trim();

    println!("Iniciando busca no Mojeek pelo termo: {}", query);
    println!("\n");

    let crawler = WebCrawler::new()?;
    let start_time = Instant::now();
    
    match crawler.search(query) {
        Ok(results) => {
            println!("Resultados da pesquisa:");
            println!("{}", format_results(&results));
        },
        Err(e) => {
            eprintln!("Erro durante a busca: {}", e);
            return Err(e);
        }
    }

    let duration = start_time.elapsed();
    println!("\nTempo total da busca: {:.2} segundos", duration.as_secs_f64());

    Ok(())
}