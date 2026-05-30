use sqlx::PgPool;
use crate::models::plan::{Plan, NuevoPlan};

// Obtener todos los planes
pub async fn obtener_todos(
    pool: &PgPool,
) -> Result<Vec<Plan>, sqlx::Error> {

    sqlx::query_as::<_, Plan>(
        "SELECT id_plan, nombre_plan, precio_mensual FROM planes"
    )
    .fetch_all(pool)
    .await
}

// Obtener por ID
pub async fn obtener_plan_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Plan>, sqlx::Error> {

    sqlx::query_as::<_, Plan>(
        "SELECT id_plan, nombre_plan, precio_mensual
         FROM planes
         WHERE id_plan = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

// Crear
pub async fn crear_plan(
    pool: &PgPool,
    nuevo_plan: NuevoPlan,
) -> Result<Plan, sqlx::Error> {

    sqlx::query_as::<_, Plan>(
        "INSERT INTO planes (nombre_plan, precio_mensual)
         VALUES ($1, $2)
         RETURNING id_plan, nombre_plan, precio_mensual"
    )
    .bind(nuevo_plan.nombre_plan.trim()) // ✔ mejora ligera
    .bind(nuevo_plan.precio_mensual)
    .fetch_one(pool)
    .await
}

// Actualizar
pub async fn actualizar_plan(
    pool: &PgPool,
    id: i32,
    plan_actualizado: NuevoPlan,
) -> Result<Option<Plan>, sqlx::Error> {

    sqlx::query_as::<_, Plan>(
        "UPDATE planes
         SET nombre_plan = $1,
             precio_mensual = $2
         WHERE id_plan = $3
         RETURNING id_plan, nombre_plan, precio_mensual"
    )
    .bind(plan_actualizado.nombre_plan.trim()) // ✔ evita espacios basura
    .bind(plan_actualizado.precio_mensual)
    .bind(id)
    .fetch_optional(pool)
    .await
}

// Eliminar
pub async fn eliminar_plan(
    pool: &PgPool,
    id: i32,
) -> Result<bool, sqlx::Error> {

    let result = sqlx::query(
        "DELETE FROM planes WHERE id_plan = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}