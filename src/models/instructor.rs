use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Instructor {
    pub id_instructor: i32,
    pub nombre: String,
    pub especialidad: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInstructor {
    pub nombre: String,
    pub especialidad: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInstructor {
    pub nombre: Option<String>,
    pub especialidad: Option<String>,
}