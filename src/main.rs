use dotenv;
use sqlx::PgPool;
use sqlx::Pool;
use tide::Server;

#[async_std::main]

async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool: PgPool = Pool::connect(&db_url).await.unwrap();

    let mut app: Server<State> = Server::with_state(State { db_pool });

    app.at("/").get(|_| async move { Ok("Hello, world") });

    app.listen("127.0.0.1:8080").await.unwrap();
}

#[derive(Clone, Debug)]
struct State {
    db_pool: PgPool,
}
