use dotenv;
use tide::{Server, StatusCode, Request, Response};
use serde_json::json;
use sqlx::{PgPool, Pool, query};

#[async_std::main]
async fn main() {
    let app = server().await;
    app.listen("127.0.0.1:8080").await.unwrap();
}

async fn server() -> Server<State> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool: PgPool = Pool::new(&db_url).await.unwrap();

    let mut app: Server<State> = Server::with_state(State{ db_pool });

    app.at("/").get(|req: Request<State>| async move {
        /* let db_pool = &req.state().db_pool;
        let rows = query!("select 1 as one where 1 = 2").fetch_one(db_pool).await.unwrap();
        dbg!(rows);
        Ok("Hello, world!") */
        let json = json!([1, 2, 3]);
        Ok(Response::new(StatusCode::Ok).body_json(&json)?)
    });

    app
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}
