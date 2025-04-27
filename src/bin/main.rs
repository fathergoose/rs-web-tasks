use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;

use dotenvy::dotenv;
use rs_web_tasks::{models::*, schema::things};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    let app = Router::new()
        .route("/things", get(get_things).post(create_thing))
        .route("/things/{id}", get(get_thing))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_things(
    State(pool): State<Pool<Manager<PgConnection>>>,
) -> Result<Json<Vec<Thing>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| things::table.select(Thing::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

async fn get_thing(
    Path(id): Path<String>,
    State(pool): State<Pool<Manager<PgConnection>>>,
) -> Result<Json<Vec<Thing>>, (StatusCode, String)> {
    use rs_web_tasks::schema::things::dsl::id as row_id;

    let id = id.parse::<i32>().map_err(bad_request_error)?;
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            things::table
                .filter(row_id.eq(id))
                .select(Thing::as_select())
                .load(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

#[debug_handler]
async fn create_thing(
    State(pool): State<Pool<Manager<PgConnection>>>,
    Json(new_thing): Json<&'static NewThing<'static>>,
) -> Result<Json<Vec<Thing>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(move |conn| {
            diesel::insert_into(things::table)
                .values(new_thing)
                .load(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn bad_request_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::BAD_REQUEST, err.to_string())
}
