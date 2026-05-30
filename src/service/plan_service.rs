use sqlx::PgPool;
use rust_decimal::Decimal;

use crate::models::plan::{Plan, NuevoPlan};
use crate::repository::plan_repository;


// VALIDACIÓN CENTRAL

fn validar_plan(nombre: &str, precio: Decimal) -> Result<(), String> {
    if nombre.trim().is_empty() {
        return Err("El nombre del plan no puede estar vacío".to_string());
    }

    if precio <= Decimal::from(0) {
        return Err("El precio debe ser mayor a 0".to_string());
    }

    Ok(())
}


// OBTENER TODOS

pub async fn obtener_todos_los_planes(
    pool: &PgPool,
) -> Result<Vec<Plan>, sqlx::Error> {
    plan_repository::obtener_todos(pool).await
}


// OBTENER POR ID (AQUÍ ESTÁ)

pub async fn obtener_plan_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Option<Plan>, sqlx::Error> {
    plan_repository::obtener_plan_por_id(pool, id).await
}


// CREAR PLAN (CON VALIDACIÓN)

pub async fn crear_plan(
    pool: &PgPool,
    nuevo_plan: NuevoPlan,
) -> Result<Plan, String> {

    validar_plan(
        &nuevo_plan.nombre_plan,
        nuevo_plan.precio_mensual,
    )?;

    plan_repository::crear_plan(pool, nuevo_plan)
        .await
        .map_err(|e| e.to_string())
}


// ACTUALIZAR PLAN (CON VALIDACIÓN)

pub async fn actualizar_plan(
    pool: &PgPool,
    id: i32,
    plan_actualizado: NuevoPlan,
) -> Result<Option<Plan>, String> {

    validar_plan(
        &plan_actualizado.nombre_plan,
        plan_actualizado.precio_mensual,
    )?;

    plan_repository::actualizar_plan(pool, id, plan_actualizado)
        .await
        .map_err(|e| e.to_string())
}


// ELIMINAR PLAN

pub async fn eliminar_plan(
    pool: &PgPool,
    id: i32,
) -> Result<bool, sqlx::Error> {
    plan_repository::eliminar_plan(pool, id).await
}