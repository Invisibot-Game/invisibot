use invisibot_common::coordinate::Coordinate;
use sqlx::{types::Uuid, Transaction};

use crate::{postgres_error::PostgresResult, tables::map_wall::MapWall};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    map_id: Uuid,
    coordinate: Coordinate,
) -> PostgresResult<MapWall> {
    Ok(sqlx::query_as!(
        MapWall,
        r#"
INSERT INTO map_wall (map_id, x, y)
VALUES               ($1,     $2,    $3)
RETURNING map_id, x, y
        "#,
        map_id,
        coordinate.x as i32,
        coordinate.y as i32,
    )
    .fetch_one(&mut **transaction)
    .await?)
}

pub async fn get_by_map_id(
    transaction: &mut Transaction<'_, DB>,
    map_id: Uuid,
) -> PostgresResult<Vec<MapWall>> {
    Ok(sqlx::query_as!(
        MapWall,
        r#"
SELECT map_id, x, y
FROM map_wall
WHERE map_id = $1
        "#,
        map_id
    )
    .fetch_all(&mut **transaction)
    .await?)
}
