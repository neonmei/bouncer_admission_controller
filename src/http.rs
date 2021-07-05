use serde_json::json;
use std::convert::TryInto;
/// This module contains the http-related processing.
use tracing::*;
use uuid::Uuid;

use crate::validators;

use actix_web::{
    dev::RequestHead, get, http::header, post, web, HttpRequest, HttpResponse, Responder,
};

use kube::core::{
    admission::{AdmissionRequest, AdmissionResponse, AdmissionReview},
    DynamicObject,
};

/// Actix guard for checking proper Content-Type header
fn guard_content_type(http_request: &RequestHead) -> bool {
    match http_request.headers.get(header::CONTENT_TYPE) {
        Some(content_type) => content_type == "application/json",
        None => false,
    }
}

#[get("/health")]
#[tracing::instrument(name = "HEALTHCHECK", level = "debug")]
/// Service health status.
pub async fn get_healthcheck() -> impl Responder {
    HttpResponse::Ok()
        .append_header((header::CONTENT_TYPE, "application/json"))
        .json(json!({"message": "ok"}))
        .await
}

#[post("/v0/validate/probes", guard = "guard_content_type")]
#[tracing::instrument(
    name = "probes validation",
    level = "info",
    skip(http_request, body_json),
    fields(
        request_id = %Uuid::new_v4(),
        method = %http_request.method(),
        uri = %http_request.uri()
    )
)]
/// Admission Controller: readiness/liveness probes HTTP request.
pub async fn post_probes_validation(
    http_request: HttpRequest,
    body_json: web::Json<AdmissionReview<DynamicObject>>,
) -> Result<HttpResponse, HttpResponse> {
    let request: AdmissionRequest<_> = match body_json.into_inner().try_into() {
        Ok(r) => r,
        Err(err) => {
            let reason = format!("Ill-formed AdmissionRequest: {}", err);
            error!("{}", reason);
            return Err(
                HttpResponse::BadRequest().json(&AdmissionResponse::invalid(reason).into_review())
            );
        }
    };

    match validators::require_probes(request) {
        Ok(admission_response) => Ok(HttpResponse::Ok().json(admission_response.into_review())),
        Err(validation_error) => Err(HttpResponse::InternalServerError()
            .json(&AdmissionResponse::invalid(validation_error).into_review())),
    }
}
