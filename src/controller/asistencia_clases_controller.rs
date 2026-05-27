use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::asistencia_clases::{ActualizarAsistenciaClase, NuevaAsistenciaClase};
use crate::service::asistencia_clases_service as service;

pub async fn get_all(pool: web::Data<PgPool>) -> impl Responder {
    match service::obtener_asistencias(pool.get_ref()).await {
        Ok(asistencias) => HttpResponse::Ok().json(asistencias),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_by_id(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match service::obtener_asistencia_por_id(pool.get_ref(), id).await {
        Ok(Some(asistencia)) => HttpResponse::Ok().json(asistencia),
        Ok(None) => HttpResponse::NotFound().body("Asistencia no encontrada"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn create(
    pool: web::Data<PgPool>,
    body: web::Json<NuevaAsistenciaClase>,
) -> impl Responder {
    if let Err(e) = body.validar() {
        return HttpResponse::BadRequest().body(e);
    }

    match service::crear_asistencia(pool.get_ref(), body.into_inner()).await {
        Ok(asistencia) => HttpResponse::Created().json(asistencia),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    body: web::Json<ActualizarAsistenciaClase>,
) -> impl Responder {
    if let Err(e) = body.validar() {
        return HttpResponse::BadRequest().body(e);
    }

    let id = path.into_inner();
    match service::actualizar_asistencia(pool.get_ref(), id, body.into_inner()).await {
        Ok(Some(asistencia)) => HttpResponse::Ok().json(asistencia),
        Ok(None) => HttpResponse::NotFound().body("Asistencia no encontrada"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn delete(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match service::eliminar_asistencia(pool.get_ref(), id).await {
        Ok(true) => HttpResponse::Ok().body("Asistencia eliminada"),
        Ok(false) => HttpResponse::NotFound().body("Asistencia no encontrada"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api/asistencias-clases")
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete)),
    );
}
