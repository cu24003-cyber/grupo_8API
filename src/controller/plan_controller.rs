use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::models::plan::NuevoPlan;
use crate::service::plan_service;

// Obtener todos los planes
pub async fn obtener_todos_los_planes(
    pool: web::Data<PgPool>,
) -> impl Responder {

    match plan_service::obtener_todos_los_planes(pool.get_ref()).await {

        Ok(planes) => HttpResponse::Ok().json(planes),

        Err(error) => {
            HttpResponse::InternalServerError()
                .body(format!("Error al obtener planes: {}", error))
        }
    }
}

// Obtener plan por ID
pub async fn obtener_plan_por_id(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {

    match plan_service::obtener_plan_por_id(
        pool.get_ref(),
        id.into_inner(),
    )
    .await
    {
        Ok(Some(plan)) => HttpResponse::Ok().json(plan),

        Ok(None) => {
            HttpResponse::NotFound()
                .body("Plan no encontrado")
        }

        Err(error) => {
            HttpResponse::InternalServerError()
                .body(format!("Error al obtener plan: {}", error))
        }
    }
}

// Crear plan
pub async fn crear_plan(
    pool: web::Data<PgPool>,
    nuevo_plan: web::Json<NuevoPlan>,
) -> impl Responder {

    match plan_service::crear_plan(
        pool.get_ref(),
        nuevo_plan.into_inner(),
    )
    .await
    {
        Ok(plan) => HttpResponse::Created().json(plan),

        Err(error) => {
            HttpResponse::InternalServerError()
                .body(format!("Error al crear plan: {}", error))
        }
    }
}

// Actualizar plan
pub async fn actualizar_plan(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    plan_actualizado: web::Json<NuevoPlan>,
) -> impl Responder {

    match plan_service::actualizar_plan(
        pool.get_ref(),
        id.into_inner(),
        plan_actualizado.into_inner(),
    )
    .await
    {
        Ok(Some(plan)) => HttpResponse::Ok().json(plan),

        Ok(None) => {
            HttpResponse::NotFound()
                .body("Plan no encontrado")
        }

        Err(error) => {
            HttpResponse::InternalServerError()
                .body(format!("Error al actualizar plan: {}", error))
        }
    }
}

// Eliminar plan
pub async fn eliminar_plan(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {

    match plan_service::eliminar_plan(
        pool.get_ref(),
        id.into_inner(),
    )
    .await
    {
        Ok(true) => {
            HttpResponse::Ok()
                .body("Plan eliminado correctamente")
        }

        Ok(false) => {
            HttpResponse::NotFound()
                .body("Plan no encontrado")
        }

        Err(error) => {
            HttpResponse::InternalServerError()
                .body(format!("Error al eliminar plan: {}", error))
        }
    }
}

// Configuración de rutas
pub fn config(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("api/planes")
            .route("", web::get().to(obtener_todos_los_planes))
            .route("/{id}", web::get().to(obtener_plan_por_id))
            .route("", web::post().to(crear_plan))
            .route("/{id}", web::put().to(actualizar_plan))
            .route("/{id}", web::delete().to(eliminar_plan))
    );
}