use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct AsistenciaClase {
    pub id_asistencia: i32,
    pub id_miembro: Option<i32>,
    pub id_clase: Option<i32>,
    pub fecha_asistencia: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevaAsistenciaClase {
    pub id_miembro: Option<i32>,
    pub id_clase: Option<i32>,
    pub fecha_asistencia: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarAsistenciaClase {
    pub id_miembro: Option<i32>,
    pub id_clase: Option<i32>,
    pub fecha_asistencia: NaiveDate,
}