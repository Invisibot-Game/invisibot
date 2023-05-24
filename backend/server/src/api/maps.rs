use super::response::GameResponse;

#[get("/maps")]
pub async fn get_maps() -> GameResponse<Vec<()>> {
    GameResponse::ok(vec![])
}
