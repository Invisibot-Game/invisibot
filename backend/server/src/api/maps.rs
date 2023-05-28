use std::fs;

use super::response::GameResponse;

#[get("/maps")]
pub async fn get_maps() -> GameResponse<Vec<String>> {
    GameResponse::ok(get_map_urls())
}

fn get_map_urls() -> Vec<String> {
    fs::read_dir("resources/maps")
        .expect("Failed to read directory")
        .map(|dir| {
            let dir = dir.expect("Failed to read directory");
            let file_name = dir.file_name();
            file_name.to_str().unwrap().to_string()
        })
        .collect()
}
