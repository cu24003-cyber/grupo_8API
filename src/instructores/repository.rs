use sqlx::PgPool;
use crate::instructores::model::{Instructor, CreateInstructor, UpdateInstructor};

pub async fn get_all(pool: &PgPool) -> Result<Vec<Instructor>, sqlx::Error> {
    let instructores = sqlx::query_as::<_, Instructor>(
        "SELECT id_instructor, nombre, especialidad FROM instructores"
    )
    .fetch_all(pool)
    .await?;

    Ok(instructores)
}

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Instructor>, sqlx::Error> {
    let instructor = sqlx::query_as::<_, Instructor>(
        "SELECT id_instructor, nombre, especialidad FROM instructores WHERE id_instructor = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(instructor)
}

pub async fn create(pool: &PgPool, data: CreateInstructor) -> Result<Instructor, sqlx::Error> {
    let instructor = sqlx::query_as::<_, Instructor>(
        "INSERT INTO instructores (nombre, especialidad)
         VALUES ($1, $2)
         RETURNING id_instructor, nombre, especialidad"
    )
    .bind(&data.nombre)
    .bind(&data.especialidad)
    .fetch_one(pool)
    .await?;

    Ok(instructor)
}

pub async fn update(pool: &PgPool, id: i32, data: UpdateInstructor) -> Result<Option<Instructor>, sqlx::Error> {
    let actual = get_by_id(pool, id).await?;
    let actual = match actual {
        Some(i) => i,
        None => return Ok(None),
    };

    let nombre = data.nombre.unwrap_or(actual.nombre);
    let especialidad = data.especialidad.unwrap_or(actual.especialidad);

    let instructor = sqlx::query_as::<_, Instructor>(
        "UPDATE instructores SET nombre = $1, especialidad = $2
         WHERE id_instructor = $3
         RETURNING id_instructor, nombre, especialidad"
    )
    .bind(&nombre)
    .bind(&especialidad)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(Some(instructor))
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM instructores WHERE id_instructor = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}