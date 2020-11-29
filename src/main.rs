use dotenv::dotenv;
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::convert::Infallible;
use std::env;
use tokio_postgres::NoTls;
use warp::{Filter, Rejection};

mod data;
mod db;
mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    if let Ok(_) = dotenv() {
        // use the returned value
        log::info!("Loaded environment .env file");
    }

    let db_pool = db::create_pool().expect("database pool can be created");

    // db::init_db(&db_pool)
    //     .await
    //     .expect("database can be initialized");

    let health_route = warp::path!("health")
        .and(with_db(db_pool.clone()))
        .and_then(handler::health_handler);

    let todo = warp::path("todo");
    let todo_routes = todo
        .and(warp::get())
        .and(warp::query())
        .and(with_db(db_pool.clone()))
        .and_then(handler::list_todos_handler)
        .or(todo
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::create_todo_handler))
        .or(todo
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db_pool.clone()))
            .and_then(handler::update_todo_handler))
        .or(todo
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db_pool.clone()))
            .and_then(handler::delete_todo_handler));

    let routes = health_route
        .or(todo_routes)
        .with(warp::cors().allow_any_origin())
        .recover(error::handle_rejection);

    // let host = env::var("HOST").expect("Please set host in .env");
    let port = env::var("PORT")
        .expect("Port should be set")
        .parse::<u16>()
        .unwrap();
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    // warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

fn with_db(db_pool: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
