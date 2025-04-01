use serde::Serialize;
use chrono::{DateTime, NaiveDateTime, Utc};

/// Representa um resultado individual de busca com todas as informações relevantes.
/// 
/// Esta estrutura armazena os detalhes de um resultado de busca, incluindo
/// a fonte do conteúdo, título, descrição, link e data de publicação.
#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub fonte: String,
    pub titulo: String,
    pub descricao: String,
    pub link: String,
    pub data: String,
}

impl SearchResult {
    /// Converte o resultado da busca em uma string formatada para exibição.
    /// 
    /// # Returns
    /// Uma string formatada contendo todos os campos do resultado.
    pub fn to_string_format(&self) -> String {
        format!(
            "Fonte: {}\nTítulo: {}\nDescrição: {}\nLink: {}\nData: {}",
            self.fonte,
            self.titulo,
            self.descricao,
            self.link,
            self.data
        )
    }

    /// Converte uma string de data em um objeto DateTime.
    /// 
    /// Suporta múltiplos formatos de data e retorna uma data padrão em caso de erro.
    /// 
    /// # Arguments
    /// * `date_str` - A string contendo a data a ser convertida
    /// 
    /// # Returns
    /// Um DateTime<Utc> representando a data parseada
    pub fn parse_date(date_str: &str) -> DateTime<Utc> {
        NaiveDateTime::parse_from_str(date_str, "%d %b %Y %H:%M")
            .or_else(|_| NaiveDateTime::parse_from_str(date_str, "%d/%m/%Y %H:%M"))
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
            .unwrap_or_else(|_| DateTime::<Utc>::from_timestamp(0, 0).unwrap())
    }
}

/// Formata uma coleção de resultados de busca em uma única string.
/// 
/// # Arguments
/// * `results` - Um vetor de resultados de busca
/// 
/// # Returns
/// Uma string contendo todos os resultados formatados
pub fn format_results(results: &Vec<SearchResult>) -> String {
    results
        .iter()
        .map(|result| result.to_string_format())
        .collect::<Vec<String>>()
        .join("\n\n")
}