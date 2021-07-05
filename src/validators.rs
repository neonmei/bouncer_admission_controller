/// This module contains the logic for the different Validating Admission Controllers
use kube::core::{
    admission::{AdmissionRequest, AdmissionResponse},
    DynamicObject,
};

use crate::{error::ValidationError, utils};
use tracing::*;

/// Validate that manifest with containers has liveness and readiness probes declared.
#[tracing::instrument(
    name = "Validating probes",
    level = "debug",
    skip(request),
    fields(
        manifest.group = request.request_resource.as_ref().map_or("", |gvr| &gvr.group),
        manifest.version = request.request_resource.as_ref().map_or("", |gvr| &gvr.version),
        manifest.kind = request.request_resource.as_ref().map_or("", |gvr| &gvr.resource))
)]
pub fn require_probes(
    request: AdmissionRequest<DynamicObject>,
) -> Result<AdmissionResponse, ValidationError> {
    let mut response = AdmissionResponse::from(&request);

    let object = match request.object.as_ref() {
        Some(ro) => ro,
        None => {
            info!("Request Empty, faulty ValidatingWebhookConfiguration?");
            return Err(ValidationError::EmptyObject);
        }
    };

    let container_list = utils::extract_containers(&request.kind, object)?;
    response.allowed = true;

    for container in container_list.iter() {
        let liveness = container.get("livenessProbe");
        let readiness = container.get("readinessProbe");

        if readiness.is_none() && liveness.is_none() {
            let reason = format!(
                "Missing livenessProbe or readinessProbe for container: {}",
                container["name"]
            );
            info!("{}", reason);
            response.allowed = false;
            response = response.deny(reason);
        }
    }

    Ok(response)
}
