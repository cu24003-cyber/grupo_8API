use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::miembros::{NuevoMiembro, ActualizarMiembro};
use crate::service::miembros_service as service;

pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match service::obtener_miembros(pool.get_ref()).await {
        Ok(miembros) => HttpResponse::Ok().json(miembros),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<NuevoMiembro>,
) -> impl Responder {
    match service::crear_miembro(pool.get_ref(), body.into_inner()).await {
        Ok(miembro) => HttpResponse::Created().json(miembro),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<ActualizarMiembro>,
) -> impl Responder {
    let id = path.into_inner();
    match service::actualizar_miembro_por_id(pool.get_ref(), id, body.into_inner()).await {
        Ok(miembro) => HttpResponse::Ok().json(miembro),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match service::eliminar_miembro_por_id(pool.get_ref(), id).await {
        Ok(_) => HttpResponse::Ok().body("Miembro eliminado"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api/miembros")
            .route("", web::get().to(get_all))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete)),
    );
}