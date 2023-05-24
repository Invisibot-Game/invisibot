use invisibot_common::coordinate::Coordinate;
use sqlx::{types::Uuid, Transaction};

use crate::{postgres_error::PostgresResult, tables::starting_position::StartingPosition};

use super::DB;

pub async fn insert(
    transaction: &mut Transaction<'_, DB>,
    map_id: Uuid,
    coord: Coordinate,
) -> PostgresResult<StartingPosition> {
    Ok(sqlx::query_as!(
        StartingPosition,
        r#"
INSERT INTO map_starting_position(map_id, x,  y)
VALUES                           ($1,     $2, $3)
RETURNING map_id, x, y
        "#,
        map_id,
        coord.x as i32,
        coord.y as i32,
    )
    .fetch_one(transaction)
    .await?)
}

pub async fn get_by_map_id(
    transaction: &mut Transaction<'_, DB>,
    map_id: Uuid,
) -> PostgresResult<Vec<StartingPosition>> {
    Ok(sqlx::query_as!(
        StartingPosition,
        r#"
SELECT map_id, x, y
FROM map_starting_position
WHERE map_id = $1
        "#,
        map_id
    )
    .fetch_all(transaction)
    .await?)
}
