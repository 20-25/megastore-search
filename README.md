# Sistema de Busca Otimizado para Catalogo de Produtos - MegaStore

## Descricao do projeto

Este projeto apresenta um sistema de busca otimizado para o catalogo de produtos da empresa ficticia MegaStore. A solucao foi desenvolvida em Rust e utiliza tabelas hash para indexar produtos por nome, marca e categoria, permitindo buscas rapidas e filtros por categoria, marca e preco.

## Objetivo

Melhorar a experiencia de busca em um catalogo de produtos, reduzindo lentidao, aumentando a precisao dos resultados e tornando a solucao mais escalavel.

## Tecnologias utilizadas

- Rust
- Cargo
- HashMap
- HashSet
- Testes com `cargo test`

## Estrutura do projeto

```text
megastore-search/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── search_tests.rs
└── documentacao/
    └── documentacao.txt
```

## Como executar

```bash
cargo run
```

## Como executar os testes

```bash
cargo test
```

## Arquitetura do sistema

- `src/lib.rs`: contem as estruturas `Product`, `SearchFilters` e `SearchEngine`.
- `src/main.rs`: contem um exemplo de execucao do sistema com produtos cadastrados.
- `tests/search_tests.rs`: contem testes unitarios e de integracao.

## Estruturas de dados utilizadas

A principal estrutura utilizada foi a tabela hash por meio de `HashMap`, que permite acesso eficiente aos dados. O indice invertido associa cada palavra-chave a um conjunto de identificadores de produtos. Tambem foi utilizado `HashSet` para evitar duplicidade.

## Consideracoes sobre desempenho e escalabilidade

A utilizacao de tabelas hash reduz a necessidade de percorrer todo o catalogo a cada consulta. Essa estrategia melhora o tempo de resposta e permite maior escalabilidade.

## Exemplo de solucao existente

Solucoes de e-commerce como Amazon, Mercado Livre e Magazine Luiza utilizam mecanismos de busca e recomendacao capazes de indexar produtos, considerar categorias, marcas, historico de navegacao e termos pesquisados.

## Licenca

Projeto academico para fins educacionais.
