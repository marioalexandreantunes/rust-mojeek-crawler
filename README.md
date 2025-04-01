# rust-mojeek-crawler - Mojeek Search

Um motor de busca em Rust que utiliza o Mojeek como fonte de dados. Este projeto permite realizar buscas na web de forma eficiente e organizada, apresentando resultados com informações detalhadas como título, descrição, link, fonte e data.

## Funcionalidades

- Busca web através do Mojeek
- Resultados ordenados por data
- Apresentação formatada dos resultados
- Rotação automática de User-Agents
- Tratamento de erros robusto
- Interface de linha de comando intuitiva

## Requisitos

- Rust (última versão estável)
- Cargo (gerenciador de pacotes do Rust)

## Instalação

1. Clone o repositório:
```bash
git clone [URL_DO_REPOSITÓRIO]
cd rust-mojeek-crawler
```

2. Compile o projeto:
```bash
cargo build --release
```
## Problemas Comuns

### Erro de Acesso ao Ficheiro Durante a Compilação

Se encontrar o seguinte erro durante a compilação:
Este erro pode ocorrer porque o Antivírus do seu computador está a bloquear o acesso ao ficheiro. Pode tentar:

1. Desativar temporariamente o antivírus durante a compilação
2. Adicionar a directoria do projeto às exceções do antivírus
3. Se estiver a utilizar o Windows Defender, verificar se o ficheiro está a ser bloqueado e permitir o acesso

## Uso

1. Execute o programa:
```bash
cargo run
```

2. Digite o termo que deseja pesquisar quando solicitado

3. Os resultados serão exibidos no formato:
   - Fonte
   - Título
   - Descrição
   - Link
   - Data

## Estrutura do Projeto

- `src/main.rs`: Ponto de entrada do programa e interface do usuário
- `src/crawler.rs`: Implementação do WebCrawler e lógica de busca
- `src/models.rs`: Definição das estruturas de dados e formatação
- `src/mod.rs`: Declaração dos módulos

## Dependências Principais

- `reqwest`: Cliente HTTP para fazer requisições web
- `scraper`: Parser HTML para extrair dados
- `serde`: Serialização/deserialização de dados
- `chrono`: Manipulação de datas e horários
- `rand`: Geração de números aleatórios
- `urlencoding`: Codificação e decodificação de URLs

## Licença

Este projeto está licenciado sob a licença GNU - veja o arquivo LICENSE para detalhes.