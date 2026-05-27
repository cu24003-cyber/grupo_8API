use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::clase::{ CreateClase, UpdateClase};
use crate::service::clase_service as service;

pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match service::obtener_todos(pool.get_ref()).await {
        Ok(clases) => HttpResponse::Ok().json(clases),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_by_id(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match service::obtener_por_id(pool.get_ref(), id).await {
        Ok(Some(clase)) => HttpResponse::Ok().json(clase),
        Ok(None) => HttpResponse::NotFound().body("Clase no encontrada"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<CreateClase>,
) -> impl Responder {
    match service::crear(pool.get_ref(), body.into_inner()).await {
        Ok(clase) => HttpResponse::Created().json(clase),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateClase>,
) -> impl Responder {
    let id = path.into_inner();
    match service::actualizar(pool.get_ref(), id, body.into_inner()).await {
        Ok(Some(clase)) => HttpResponse::Ok().json(clase),
        Ok(None) => HttpResponse::NotFound().body("Clase no encontrada"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let id = path.into_inner();
    match service::eliminar(pool.get_ref(), id).await {
        Ok(true) => HttpResponse::Ok().body("Clase eliminada"),
        Ok(false) => HttpResponse::NotFound().body("Clase no encontrada"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/clases")
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
    );
}