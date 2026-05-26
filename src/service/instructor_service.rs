use sqlx::PgPool;
use crate::models::instructor::{Instructor, CreateInstructor, UpdateInstructor};
use crate::repository::instructor_repository as repository;

pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Instructor>, sqlx::Error> {
    repository::get_all(pool).await
}

pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Instructor>, sqlx::Error> {
    repository::get_by_id(pool, id).await
}

pub async fn crear(pool: &PgPool, data: CreateInstructor) -> Result<Instructor, sqlx::Error> {
    repository::create(pool, data).await
}

pub async fn actualizar(pool: &PgPool, id: i32, data: UpdateInstructor) -> Result<Option<Instructor>, sqlx::Error> {
    repository::update(pool, id, data).await
}

pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    repository::delete(pool, id).await
}