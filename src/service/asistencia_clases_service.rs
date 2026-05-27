use sqlx::PgPool;
use crate::models::asistencia_clases::{
    ActualizarAsistenciaClase, AsistenciaClase, NuevaAsistenciaClase,
};
use crate::repository::asistencia_clases_repository::AsistenciaRepository;

pub async fn obtener_asistencias(pool: &PgPool) -> Result<Vec<AsistenciaClase>, sqlx::Error> {
    let repo = AsistenciaRepository::new(pool.clone());
    repo.obtener_asistencias().await
}

pub async fn obtener_asistencia_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<AsistenciaClase>, sqlx::Error> {
    let repo = AsistenciaRepository::new(pool.clone());
    match repo.obtener_asistencia_por_id(id).await {
        Ok(asistencia) => Ok(Some(asistencia)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn crear_asistencia(
    pool: &PgPool,
    nueva_asistencia: NuevaAsistenciaClase,
) -> Result<AsistenciaClase, sqlx::Error> {
    let repo = AsistenciaRepository::new(pool.clone());
    repo.crear_asistencia(nueva_asistencia).await
}

pub async fn actualizar_asistencia(
    pool: &PgPool,
    id: i32,
    asistencia_actualizada: ActualizarAsistenciaClase,
) -> Result<Option<AsistenciaClase>, sqlx::Error> {
    let repo = AsistenciaRepository::new(pool.clone());
    match repo.actualizar_asistencia(id, asistencia_actualizada).await {
        Ok(asistencia) => Ok(Some(asistencia)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn eliminar_asistencia(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
    let repo = AsistenciaRepository::new(pool.clone());
    match repo.eliminar_asistencia(id).await {
        Ok(_) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(e) => Err(e),
    }
}