use crate::{
    // entity::{prelude::*, user},
    models::{game_result::GameResult, generic::GenericResponse},
    server::server_state::ServerState,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

pub async fn post_game_result(
    State(state): State<ServerState>,
    Json(game_result): Json<GameResult>,
) -> (StatusCode, Result<(), Json<GenericResponse>>) {
    (
        StatusCode::BAD_REQUEST,
        Err(Json(GenericResponse {
            status: "Oopsie we had a fucky wucky".to_owned(),
        })),
    )

    // if (game_result.loser == "ferris"){
    //     throw 406 "Not Acceptable; Ferris cannot lose"
    // }
}
