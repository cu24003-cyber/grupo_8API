use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Clase {
    pub id_clase: i32,
    pub nombre_clase: String,
    pub id_instructor: Option<i32>,
    pub horario: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateClase {
    pub nombre_clase: String,
    pub id_instructor: Option<i32>,
    pub horario: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClase {
    pub nombre_clase: Option<String>,
    pub id_instructor: Option<i32>,
    pub horario: Option<String>
}