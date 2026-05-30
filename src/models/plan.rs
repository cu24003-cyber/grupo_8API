use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Plan {
    pub id_plan: i32,
    pub nombre_plan: String,
    pub precio_mensual: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoPlan {
    pub nombre_plan: String,
    pub precio_mensual: Decimal,
}