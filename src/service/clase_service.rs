use sqlx::PgPool;
use crate::models::clase::{Clase, CreateClase, UpdateClase};
use crate::repository::clase_repository as repository;
pub async fn obtener_todos(pool: &PgPool) -> Result<Vec<Clase>, sqlx::Error> {
    repository::get_all(pool).await
}
pub async fn obtener_por_id(pool: &PgPool, id: i32) -> Result<Option<Clase>, sqlx::Error> {
    repository::get_by_id(pool, id).await
}
pub async fn crear(pool: &PgPool, data: CreateClase) -> Result<Clase, sqlx::Error> {
    repository::create(pool, data).await
}
pub async fn actualizar(pool: &PgPool, id: i32, data: UpdateClase) -> Result<Option<Clase>, sqlx::Error> {
    repository::update(pool, id, data).await
}
pub async fn eliminar(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    repository::delete(pool, id).await
}