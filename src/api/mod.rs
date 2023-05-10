use reqwasm::http::Request;
use thiserror::Error;
use yew_playground_model::PlantWateringHistory;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Unknown plant: {plant}")]
    UnknownPlant {
        plant: String
    },
    #[error("Failed to parse json")]
    ParsingFailure
}

pub fn clear_watering_history<F>(name: String, handle: F)
where F: Fn(Result<(), ApiError>) -> () + 'static {
    wasm_bindgen_futures::spawn_local(async move {
        let plants_endpoint = format!("/api/plant/{}/watering_history", name);
        let result = Request::delete(&plants_endpoint).send().await;
        match result {
            Ok(response) => {
                handle(Ok(()));
            }
            Err(e) => {
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
            Ok(response) => {
                handle(Ok(()));
            }
            Err(e) => {
                handle(Err(ApiError::UnknownPlant {
                    plant: name
                }));
            },
        }
    });
}

pub fn get_history<F>(name: String, handle: F)
where F: Fn(Result<PlantWateringHistory, ApiError>) -> () + 'static {
    wasm_bindgen_futures::spawn_local(async move {
        let plants_endpoint = format!("/api/plant/{}/watering_history", name);
        let result = Request::get(&plants_endpoint).send().await;

        match result {
            Ok(response) => {
                let json: Result<PlantWateringHistory, _> = response.json().await;
                match json {
                    Ok(watering_history) => {
                        handle(Ok(watering_history));
                    }
                    Err(_) => {
                        handle(Err(ApiError::ParsingFailure));
                    }
                }
            }
            Err(e) => {
                handle(Err(ApiError::UnknownPlant {
                    plant: name
                }));
            },
        }
    });
}