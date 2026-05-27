use sqlx::{PgPool, Row};
use crate::models::asistencia_clases::{ActualizarAsistenciaClase, AsistenciaClase, NuevaAsistenciaClase};
pub struct AsistenciaRepository {
    pool: PgPool,
}
impl AsistenciaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // OBTENER TODAS LAS ASISTENCIAS
    pub async fn obtener_asistencias(&self) -> sqlx::Result<Vec<AsistenciaClase>> {
        let filas = sqlx::query(
            "SELECT id_asistencia, id_miembro, id_clase, fecha_asistencia FROM asistencia_clases",
        )
        .fetch_all(&self.pool)
        .await?;
        let asistencias = filas
            .into_iter()
            .map(|fila| AsistenciaClase {
                id_asistencia: fila.get("id_asistencia"),
                id_miembro: fila.get("id_miembro"),
                id_clase: fila.get("id_clase"),
                fecha_asistencia: fila.get("fecha_asistencia"),
            })
            .collect();

        Ok(asistencias)
    }

    // OBTENER UNA ASISTENCIA POR ID
    pub async fn obtener_asistencia_por_id(&self, id: i32) -> sqlx::Result<AsistenciaClase> {
        let fila = sqlx::query(
            "SELECT id_asistencia, id_miembro, id_clase, fecha_asistencia
             FROM asistencia_clases
             WHERE id_asistencia = $1",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(AsistenciaClase {
            id_asistencia: fila.get("id_asistencia"),
            id_miembro: fila.get("id_miembro"),
            id_clase: fila.get("id_clase"),
            fecha_asistencia: fila.get("fecha_asistencia"),
        })
    }

    // CREAR ASISTENCIA
    pub async fn crear_asistencia(
        &self,
        nueva_asistencia: NuevaAsistenciaClase,
    ) -> sqlx::Result<AsistenciaClase> {
        let fila = sqlx::query(
            "INSERT INTO asistencia_clases
            (id_miembro, id_clase, fecha_asistencia)
            VALUES ($1, $2, $3)
            RETURNING id_asistencia, id_miembro, id_clase, fecha_asistencia",
        )
        .bind(nueva_asistencia.id_miembro)
        .bind(nueva_asistencia.id_clase)
        .bind(nueva_asistencia.fecha_asistencia)
        .fetch_one(&self.pool)
        .await?;

        Ok(AsistenciaClase {
            id_asistencia: fila.get("id_asistencia"),
            id_miembro: fila.get("id_miembro"),
            id_clase: fila.get("id_clase"),
            fecha_asistencia: fila.get("fecha_asistencia"),
        })
    }

    // ACTUALIZAR ASISTENCIA
    pub async fn actualizar_asistencia(
        &self,
        id: i32,
        asistencia_actualizada: ActualizarAsistenciaClase,
    ) -> sqlx::Result<AsistenciaClase> {
        let fila = sqlx::query(
            "UPDATE asistencia_clases
            SET
                id_miembro = $1,
                id_clase = $2,
                fecha_asistencia = $3
            WHERE id_asistencia = $4
            RETURNING id_asistencia, id_miembro, id_clase, fecha_asistencia",
        )
        .bind(asistencia_actualizada.id_miembro)
        .bind(asistencia_actualizada.id_clase)
        .bind(asistencia_actualizada.fecha_asistencia)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(AsistenciaClase {
            id_asistencia: fila.get("id_asistencia"),
            id_miembro: fila.get("id_miembro"),
            id_clase: fila.get("id_clase"),
            fecha_asistencia: fila.get("fecha_asistencia"),
        })
    }

    // ELIMINAR ASISTENCIA
    pub async fn eliminar_asistencia(&self, id: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM asistencia_clases WHERE id_asistencia = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
