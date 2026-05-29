use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow, PartialEq)]
pub struct Miembros {
    pub id_miembro: i32,
    pub nombre: String,
    pub fecha_inscripcion: String,
    pub id_plan: i32,
    pub estado_membresia: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct NuevoMiembro {
        pub nombre: String,
        pub fecha_inscripcion: String,
        pub id_plan: i32,
        pub estado_membresia: bool,
    }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ActualizarMiembro {
    pub nombre: String,
    pub fecha_inscripcion: String,
    pub id_plan: i32,
    pub estado_membresia: bool,
}