use rocket::{fairing::{Fairing, Info, Kind, self}, Rocket, Build, tokio};
use tokio_postgres::{Client, Connection, Socket, tls::NoTlsStream, Error, NoTls};

pub struct RepositoryFairing;

pub mod transactions_repository;
pub mod history_repository;

const POSTGRES_HOST: &str       = dotenv!("POSTGRES_HOST");
const POSTGRES_PORT: &str       = dotenv!("POSTGRES_PORT");
const POSTGRES_USER: &str       = dotenv!("POSTGRES_USER");
const POSTGRES_PASSWORD: &str   = dotenv!("POSTGRES_PASSWORD");
const POSTGRES_DB: &str         = dotenv!("POSTGRES_DB");


pub async fn get_conn() -> Result<(Client, Connection<Socket, NoTlsStream>), Error>{
    let host =      if POSTGRES_HOST.is_empty() { "localhost" } else { POSTGRES_HOST };
    let port =      if POSTGRES_PORT.is_empty() { "5432" } else { POSTGRES_PORT };
    let user =      if POSTGRES_USER.is_empty() { "postgres" } else { POSTGRES_USER };
    let password =  if POSTGRES_PASSWORD.is_empty() { "password" } else { POSTGRES_PASSWORD };
    let db =        if POSTGRES_DB.is_empty() { "postgres" } else { POSTGRES_DB };

    let params = &format!("host={} port={} user={} password={} dbname={}", host, port, user, password, db)[..];

    tokio_postgres::connect(params, NoTls).await
}

#[rocket::async_trait]
impl Fairing for RepositoryFairing {
    fn info(&self) -> Info {
        Info {
            name: "Liftoff Db Check",
            kind: Kind::Ignite
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        match get_conn().await {
            Ok((client, conn)) => {
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        eprintln!("{}", e);
                    }
                });

                client.execute("CREATE TABLE IF NOT EXISTS transactions (
                    id varchar NOT NULL,
                    banco_org varchar NOT NULL,
                    agencia_org varchar NOT NULL,
                    conta_org varchar NOT NULL,
                    banco_dest varchar NOT NULL,
                    agencia_dest varchar NOT NULL,
                    conta_dest varchar NOT NULL,
                    valor varchar NOT NULL,
                    data varchar NOT NULL,
                    PRIMARY KEY (id)
                  );", &[]).await.unwrap();

                  client.execute("CREATE TABLE IF NOT EXISTS import_history (
                    id varchar NOT NULL,
                    data_transacoes varchar NOT NULL,
                    data_importacao varchar NOT NULL,
                    PRIMARY KEY(id)
                  );", &[]).await.unwrap();
    
                println!("Returning Ok");
                return Result::Ok(rocket)
            },
            Err(_) => {
                eprintln!("Unable to connect to database.");
                return Err(rocket)
            },
        }
    }
}