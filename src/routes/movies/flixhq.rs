use crate::models::{FlixhqInfo, FlixhqSearch, FlixhqServer, FlixhqSource, ProviderInfo};
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use consumet::{
    models::{IEpisodeServer, IMovieResult, ISearch, ISource},
    providers::movies::{FlixHQ, FlixHQInfo},
};

pub async fn mount() -> Router {
    Router::new()
        .route("/", get(flixhq_home))
        .route("/search", get(flixhq_search))
        .route("/recent-shows", get(flixhq_recent_shows))
        .route("/recent-movies", get(flixhq_recent_movies))
        .route("/trending-shows", get(flixhq_trending_shows))
        .route("/trending-movies", get(flixhq_trending_movies))
        .route("/info", get(flixhq_info))
        .route("/servers", get(flixhq_server))
        .route("/sources", get(flixhq_sources))
}

pub async fn flixhq_home() -> (StatusCode, Json<ProviderInfo>) {
    (
        StatusCode::OK,
        Json(ProviderInfo {
            intro: String::from(
                r#"Welcome to the flixhq provider: check out the provider's website @ https://flixhq.to/"#,
            ),
            routes: vec![
                String::from("/search"),
                String::from("/info"),
                String::from("/watch"),
            ],
            documentation: String::from("https://docs.consumet.org/#tag/flixhq"),
        }),
    )
}

pub async fn flixhq_search(
    route_params: Query<FlixhqSearch>,
) -> (StatusCode, Json<ISearch<IMovieResult>>) {
    let search = FlixHQ
        .search(&route_params.query, route_params.page)
        .await
        .unwrap();

    (StatusCode::OK, Json(search))
}

pub async fn flixhq_info(route_params: Query<FlixhqInfo>) -> (StatusCode, Json<FlixHQInfo>) {
    let info = FlixHQ.info(&route_params.id).await.unwrap();
    (StatusCode::OK, Json(info))
}

pub async fn flixhq_server(
    route_params: Query<FlixhqServer>,
) -> (StatusCode, Json<Vec<IEpisodeServer>>) {
    let server = FlixHQ
        .servers(&route_params.episode_id, &route_params.media_id)
        .await
        .unwrap();

    (StatusCode::OK, Json(server))
}

pub async fn flixhq_sources(route_params: Query<FlixhqSource>) -> (StatusCode, Json<ISource>) {
    let sources = FlixHQ
        .sources(
            &route_params.episode_id,
            &route_params.media_id,
            route_params.server,
        )
        .await
        .unwrap();

    (StatusCode::OK, Json(sources))
}

pub async fn flixhq_recent_movies() -> (StatusCode, Json<Vec<IMovieResult>>) {
    let recent_movies = FlixHQ.recent_movies().await.unwrap();

    (StatusCode::OK, Json(recent_movies))
}

pub async fn flixhq_recent_shows() -> (StatusCode, Json<Vec<IMovieResult>>) {
    let recent_shows = FlixHQ.recent_shows().await.unwrap();

    (StatusCode::OK, Json(recent_shows))
}

pub async fn flixhq_trending_movies() -> (StatusCode, Json<Vec<IMovieResult>>) {
    let trending_movies = FlixHQ.recent_shows().await.unwrap();

    (StatusCode::OK, Json(trending_movies))
}

pub async fn flixhq_trending_shows() -> (StatusCode, Json<Vec<IMovieResult>>) {
    let trending_shows = FlixHQ.trending_shows().await.unwrap();

    (StatusCode::OK, Json(trending_shows))
}
