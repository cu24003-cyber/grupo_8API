use sqlx::{PgPool, Row};
use crate::models::miembros::{Miembros, NuevoMiembro, ActualizarMiembro};

pub struct MiembrosRepository {
    pool: PgPool,
}

impl MiembrosRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn obtener_miembros(&self) -> sqlx::Result<Vec<Miembros>> {   
        let filas = sqlx::query("SELECT id_miembro, nombre, fecha_inscripcion, id_plan, estado_membresia FROM miembros")
        .fetch_all(&self.pool)
        .await?;

    let miembros = filas.into_iter().map(|fila|{
        Miembros {
            id_miembro: fila.get("id_miembro"),
            nombre: fila.get("nombre"),
            fecha_inscripcion: fila.get("fecha_inscripcion"),
            id_plan: fila.get("id_plan"),
            estado_membresia: fila.get("estado_membresia"),
        }
    }).collect();
    Ok(miembros)
}

pub async fn crear_miembro(&self, nuevo_miembro: &NuevoMiembro) -> sqlx::Result<Miembros>{
    let fila = sqlx::query("INSERT INTO miembros (nombre, fecha_inscripcion, id_plan, estado_membresia) VALUES ($1, $2, $3, $4) RETURNING id_miembro, nombre, fecha_inscripcion, id_plan, estado_membresia")
    .bind(&nuevo_miembro.nombre)
    .bind(&nuevo_miembro.fecha_inscripcion)
    .bind(&nuevo_miembro.id_plan)
    .bind(&nuevo_miembro.estado_membresia)
    .fetch_one(&self.pool)
    .await?;
    Ok(Miembros{
        id_miembro: fila.get("id_miembro"),
        nombre: fila.get("nombre"),
        fecha_inscripcion: fila.get("fecha_inscripcion"),
        id_plan: fila.get("id_plan"),
        estado_membresia: fila.get("estado_membresia"),
    })
}

pub async fn actualizar_miembro(&self, id_miembro: i32, miembro_actualizado: ActualizarMiembro) -> sqlx::Result<Miembros>{
    let fila = sqlx::query("UPDATE miembros SET nombre = $1, fecha_inscripcion = $2, id_plan = $3, estado_membresia = $4 WHERE id_miembro = $5 RETURNING id_miembro, nombre, fecha_inscripcion, id_plan, estado_membresia")
    .bind(&miembro_actualizado.nombre)
    .bind(&miembro_actualizado.fecha_inscripcion)
    .bind(&miembro_actualizado.id_plan)
    .bind(&miembro_actualizado.estado_membresia)
    .bind(&id_miembro)
    .fetch_one(&self.pool)
    .await?;
    Ok(Miembros{
        id_miembro: fila.get("id_miembro"),
        nombre: fila.get("nombre"),
        fecha_inscripcion: fila.get("fecha_inscripcion"),
        id_plan: fila.get("id_plan"),
        estado_membresia: fila.get("estado_membresia"),
    })
}

pub async fn eliminar_miembro(&self, id_miembro: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM miembros WHERE id_miembro = $1")
    .bind(id_miembro)
    .execute(&self.pool)
    .await?;
    Ok(())
}
}