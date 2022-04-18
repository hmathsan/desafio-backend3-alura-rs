# Desafio Backend #3 - Alura

Aplicação criada para o [desafio backend #3](https://www.alura.com.br/challenges/back-end-3) da [Alura](https://www.alura.com.br/).

## Tecnologias utilizadas

- [Rust](https://www.rust-lang.org/pt-BR)
  - [Tokio](https://tokio.rs/) (para operações assíncronas)
  - [Serde](https://serde.rs/) (para serialização e deserialização)
- [Rocket](https://rocket.rs/)
- [Tera](https://tera.netlify.app/)

## Como rodar

### Build local

Após clonar o repositório e com o docker iniciado execute `docker-compose up -d` para iniciar o container do Postgres.

Depois que o container do Postgres estiver rodando, execute `cargo run`. (Tenha certeza de ter o Rust instalado e atualizado).

Acesse `http://localhost:8080/` para utilizar a aplicação.