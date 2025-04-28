//! Módulo responsável pela implementação do crawler ‘web’ que interage com o Mojeek.
//! 
//! Este módulo fornece funcionalidades para realizar buscas ‘web’, processar
//! os resultados e extrair informações relevantes das páginas.

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector};
use rand::prelude::IndexedRandom;
use crate::models::SearchResult;

/// Lista de ‘User’-Agents utilizados para rotação nas requisições.
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/91.0.864.59",
];

const BASE_URL: &str = "https://www.mojeek.com/search";

/// Estrutura principal do crawler que gerencia as requisições ‘web’.
/// 
/// Contém um cliente HTTP configurado com headers apropriados
/// para realizar as requisições ao Mojeek.
pub struct WebCrawler {
    client: Client,
}

impl WebCrawler {
    /// Cria uma instância do WebCrawler com configurações padrão.
    /// 
    /// Configura headers HTTP apropriados e inicializa o cliente HTTP.
    /// 
    /// # Errors
    /// Retorna erro se houver falha na criação do cliente HTTP.
    pub fn new() -> Result<Self, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENTS.choose(&mut rand::rng()).unwrap()));
        headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"));
        headers.insert("Accept-Language", HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert("Connection", HeaderValue::from_static("keep-alive"));

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(WebCrawler { client })
    }

    /// Realiza uma busca no Mojeek com o termo fornecido.
    /// 
    /// # Arguments
    /// * 'query' - O termo de busca
    /// 
    /// # Returns
    /// Um vetor de resultados de busca ordenados por data
    /// 
    /// # Errors
    /// Retorna erro se houver falha na requisição ou no processamento dos resultados
    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
        let encoded_query = query.replace(" ", "+");
        let url = format!("{base}?q={query}&date=1&si=2&since=month",
            base = BASE_URL,
            query = encoded_query
        );

        let response = self.client.get(&url).send()?;
        let html = response.text()?;
        let document = Html::parse_document(&html);

        let results_selector = Selector::parse("li").unwrap();
        let link_selector = Selector::parse("a.ob").unwrap();
        let title_selector = Selector::parse("h2 a.title").unwrap();
        let desc_selector = Selector::parse("p.s").unwrap();
        let date_selector = Selector::parse("div.serp-meta span.mdate").unwrap();

        let mut results = Vec::new();
        for element in document.select(&results_selector) {
            if let (Some(link_elem), Some(title_elem)) = (
                element.select(&link_selector).next(),
                element.select(&title_selector).next()
            ) {
                let link = link_elem.value().attr("href").unwrap_or("").to_string();
                let title = title_elem.text().collect::<String>();
                let description = element
                    .select(&desc_selector)
                    .next()
                    .map(|desc| desc.text().collect::<String>())
                    .unwrap_or_default();

                let data = element
                    .select(&date_selector)
                    .next()
                    .map(|date| date.text().collect::<String>())
                    .unwrap_or_default();

                let fonte = link
                    .split("/")
                    .nth(2)
                    .unwrap_or("")
                    .replace("www.", "")
                    .to_string();

                results.push(SearchResult {
                    fonte,
                    titulo: title,
                    descricao: description,
                    link,
                    data,
                });
            }
        }

        results.sort_by(|a, b| {
            let date_a = SearchResult::parse_date(&a.data);
            let date_b = SearchResult::parse_date(&b.data);
            date_b.cmp(&date_a)
        });

        Ok(results)
    }
}