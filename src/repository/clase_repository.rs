use sqlx::PgPool;
use crate::models::clase::{Clase, CreateClase, UpdateClase};

pub async fn get_all(pool: &PgPool) -> Result<Vec<Clase>, sqlx::Error> {
    let clases = sqlx::query_as::<_, Clase>(
        "SELECT id_clase, nombre, id_instructor, horario FROM clases"
    )
    .fetch_all(pool)
    .await?;

    Ok(clases)
}
pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Clase>, sqlx::Error> {
    let clase = sqlx::query_as::<_, Clase>(
        "SELECT id_clase, nombre, id_instructor, horario FROM clases WHERE id_clase = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(clase)
}
pub async fn create(pool: &PgPool, data: CreateClase) -> Result<Clase, sqlx::Error> {
    let clase = sqlx::query_as::<_, Clase>(
        "INSERT INTO clases (nombre, id_instructor, horario)
         VALUES ($1, $2, $3)
         RETURNING id_clase, nombre, id_instructor, horario"
    )
    .bind(&data.nombre)
    .bind(data.id_instructor)
    .bind(&data.horario)
    .fetch_one(pool)
    .await?;

    Ok(clase)
}
pub async fn update(pool: &PgPool, id: i32, data: UpdateClase) -> Result<Option<Clase>, sqlx::Error> {
    let actual = get_by_id(pool, id).await?;
    let actual = match actual {
        Some(c) => c,
        None => return Ok(None),
    };

    let nombre = data.nombre.unwrap_or(actual.nombre);
    let id_instructor = data.id_instructor.or(actual.id_instructor);
    let horario = data.horario.unwrap_or(actual.horario);

    let clase = sqlx::query_as::<_, Clase>(
        "UPDATE clases SET nombre = $1, id_instructor = $2, horario = $3
         WHERE id_clase = $4
         RETURNING id_clase, nombre, id_instructor, horario"
    )
    .bind(nombre)
    .bind(id_instructor)
    .bind(horario)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(Some(clase))
}
pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM clases WHERE id_clase = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}