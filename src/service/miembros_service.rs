use sqlx::PgPool;
use crate::models::miembros::{Miembros, NuevoMiembro, ActualizarMiembro};
use crate::repository::miembros_repository::MiembrosRepository;

pub async fn obtener_miembros(pool: &PgPool) -> Result<Vec<Miembros>, sqlx::Error> {
    let repo = MiembrosRepository::new(pool.clone());
    repo.obtener_miembros().await
}

pub async fn crear_miembro(pool: &PgPool, nuevo_miembro: NuevoMiembro) -> Result<Miembros, sqlx::Error> {
    let repo = MiembrosRepository::new(pool.clone());
    repo.crear_miembro(&nuevo_miembro).await
}

pub async fn actualizar_miembro_por_id(pool: &PgPool, id_miembro: i32, miembro_actualizado: ActualizarMiembro) -> Result<Miembros, sqlx::Error> {
    let repo = MiembrosRepository::new(pool.clone());
    repo.actualizar_miembro(id_miembro, miembro_actualizado).await
}

pub async fn eliminar_miembro_por_id(pool: &PgPool, id_miembro: i32) -> Result<(), sqlx::Error> {
    let repo = MiembrosRepository::new(pool.clone());
    repo.eliminar_miembro(id_miembro).await
}