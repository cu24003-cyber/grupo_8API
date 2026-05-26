use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::instructor::{CreateInstructor, UpdateInstructor};
use crate::service::instructor_service as service;

pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match service::obtener_todos(pool.get_ref()).await {
        Ok(instructores) => HttpResponse::Ok().json(instructores),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_by_id(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match service::obtener_por_id(pool.get_ref(), id).await {
        Ok(Some(instructor)) => HttpResponse::Ok().json(instructor),
        Ok(None) => HttpResponse::NotFound().body("Instructor no encontrado"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<CreateInstructor>,
) -> impl Responder {
    match service::crear(pool.get_ref(), body.into_inner()).await {
        Ok(instructor) => HttpResponse::Created().json(instructor),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<UpdateInstructor>,
) -> impl Responder {
    let id = path.into_inner();
    match service::actualizar(pool.get_ref(), id, body.into_inner()).await {
        Ok(Some(instructor)) => HttpResponse::Ok().json(instructor),
        Ok(None) => HttpResponse::NotFound().body("Instructor no encontrado"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match service::eliminar(pool.get_ref(), id).await {
        Ok(true) => HttpResponse::Ok().body("Instructor eliminado"),
        Ok(false) => HttpResponse::NotFound().body("Instructor no encontrado"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/instructores")
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete)),
    );
}