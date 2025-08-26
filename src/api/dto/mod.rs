use serde::Serialize;
use validator::ValidationErrors;

pub mod appointment_request;
pub mod appointment_response;

#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    errors: std::collections::HashMap<String, Vec<String>>,
}

pub fn format_validation_errors(e: ValidationErrors) -> ValidationErrorResponse {
    let mut map = std::collections::HashMap::new();

    for (field, errors) in e.field_errors().iter() {
        let messages: Vec<String> = errors
            .iter()
            .map(|err| {
                if let Some(message) = &err.message {
                    message.clone().into_owned()
                } else {
                    "Invalid value".into()
                }
            })
            .collect();
        map.insert(field.to_string(), messages);
    }

    ValidationErrorResponse { errors: map }
}
