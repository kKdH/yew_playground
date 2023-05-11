use futures::TryFutureExt;
use reqwasm::http::{Request, Response};

use yew_playground_model::PlantWateringHistory;

#[derive(thiserror::Error, Debug, Clone)]
pub enum ApiError {

    #[error("Unknown plant: {plant}")]
    UnknownPlant {
        plant: String
    },

    #[error("ParsingFailure: {message}")]
    ParsingFailure { message: String },

    #[error("Failed to request")]
    RequestFailure
}

pub fn clear_watering_history<F>(name: String, handle: F)
where F: Fn(Result<(), ApiError>) -> () + 'static {
    wasm_bindgen_futures::spawn_local(async move {
        let plants_endpoint = format!("/api/plant/{}/watering_history", name);
        let result = Request::delete(&plants_endpoint).send().await;
        match result {
            Ok(_) => {
                handle(Ok(()));
            }
            Err(_) => {
                handle(Err(ApiError::UnknownPlant {
                    plant: name
                }));
            },
        }
    });
}

pub fn do_watering<F>(name: String, handle: F)
where F: Fn(Result<(), ApiError>) -> () + 'static {
    wasm_bindgen_futures::spawn_local(async move {
        let plants_endpoint = format!("/api/plant/{}/watering", name);
        let result = Request::post(&plants_endpoint).send().await;
        match result {
            Ok(_) => {
                handle(Ok(()));
            }
            Err(_) => {
                handle(Err(ApiError::UnknownPlant {
                    plant: name
                }));
            },
        }
    });
}

pub async fn get_watering_history(name: String) -> Result<PlantWateringHistory, ApiError> {
    let plants_endpoint = format!("/api/plant/{}/watering_history", name);
    let response: Response = Request::get(&plants_endpoint)
        .send()
        .map_err(|_| ApiError::RequestFailure)
        .await?;
    match response.status() {
        200 => {
            response
                .json::<PlantWateringHistory>()
                .map_err(|cause| { ApiError::ParsingFailure { message: cause.to_string() } })
                .await
        },
        404 => {
            Err(ApiError::UnknownPlant {
                plant: name
            })
        }
        _ => {
            Err(ApiError::RequestFailure)
        }
    }
}
