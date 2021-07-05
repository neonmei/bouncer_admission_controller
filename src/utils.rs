use kube::core::{DynamicObject, GroupVersionKind};

use crate::error::ValidationError;
use serde_json::Value;
use tracing::*;

/// Extracts container data from a known kind of kubernetes object.
pub fn extract_containers<'a>(
    kind: &GroupVersionKind,
    reviewed_object: &'a DynamicObject,
) -> Result<&'a Vec<Value>, ValidationError> {
    let query = match kind.kind.as_str() {
        "Deployment" | "ReplicaSet" => "/spec/template/spec/containers",
        "Pod" => "/spec/containers",
        k => {
            let ve = ValidationError::UnsupportedKind(k.into());
            info!(%ve);
            return Err(ve);
        }
    };

    let containers: &Value = match reviewed_object.data.pointer(query) {
        Some(obj) => obj,
        None => {
            let ve = ValidationError::NoContainers;
            info!(%ve);
            return Err(ve);
        }
    };

    containers
        .as_array()
        .ok_or_else(|| ValidationError::InvalidType("Array".to_owned()))
}

/// Extract executable name.
pub fn get_program_name() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}
