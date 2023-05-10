mod components;
mod api;

use yew::prelude::*;
use gloo_timers;
use gloo_timers::callback::Timeout;
use gloo_file::*;
use gloo_net::http::Request;
use log::info;
use yew::platform::spawn_local;
use yew_router::Routable;
use yew_playground_model::Plant;

use crate::components::Footer;
use crate::components::PlantView;

#[function_component(App)]
fn app() -> Html {
    let plant_list = use_state(|| Vec::<Plant>::new());
    let selected_plant = use_state(|| Option::<Plant>::None);

    {
        let plant_list = Clone::clone(&plant_list);
        let selected_plant = Clone::clone(&selected_plant);
        use_effect(move || {
            let plant_list = Clone::clone(&plant_list);
            let selected_plant = Clone::clone(&selected_plant);
            if plant_list.is_empty() { // TODO: remove check a soon we known what we do
                wasm_bindgen_futures::spawn_local(async move {
                    let plants_endpoint = "/api/plants";
                    let fetch_plants = Request::get(&plants_endpoint).send().await;

                    match fetch_plants {
                        Ok(response) => {
                            let json: Result<Vec<Plant>, _> = response.json().await;

                            match json {
                                Ok(data) => {
                                    selected_plant.set(Some(Clone::clone(&data[data.len() - 1])));
                                    plant_list.set(data);
                                }
                                Err(e) => {
                                    plant_list.set(Vec::new());
                                },
                            }
                        }
                        Err(e) => {
                            plant_list.set(Vec::new())
                        },
                    }
                });
            }
        });
    }

    let plant_changed_action = {
        let plant_list = Clone::clone(&plant_list);
        let selected_plant = Clone::clone(&selected_plant);
        move |event: Event| {
            let html_select = event.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
            let plant = Clone::clone(&(*plant_list)[html_select.selected_index() as usize]);
            let name = Clone::clone(&plant.name);
            selected_plant.set(Some(plant));
            info!("Selected plant: {}", name);
        }
    };



    html! {
            <div class="container hero is-fluid is-fullheight">
                <div class="title is-1">
                    <h1 id={ "projectName" }>
                        <span class={ "blackName" }>{ "T" }</span>
                        <span class={ "redName" }>{ "RUST" }</span>
                        <span class={ "blackName" }>{ "Y GARDENER" }</span>
                    </h1>
                    <div class="tile is-ancestor">
                        <div class="tile is-parent">
                            <article class="tile p-2 is-child box">
                                <div class="select is-medium">
                                    <select onchange={plant_changed_action}>
                                        {
                                            for plant_list.iter().cloned().map(|plant| {
                                                html_nested!{<option value={Clone::clone(&plant.name)}>{plant.name}</option>}
                                            })
                                        }
                                    </select>
                                </div>
                            </article>
                            <div class="box p-2 subtitle">
                                { "uploading files" }
                            </div>
                        </div>
                    </div>
                </div>
                {
                    if let Some(plant) = selected_plant.as_ref() {
                        html_nested!{<div><PlantView plant={Clone::clone(plant)}></PlantView></div>}
                    }
                    else {
                        html_nested!{<div></div>}
                    }
                }


                <Footer></Footer>
            </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("trusty gardener");
    yew::Renderer::<App>::new().render();
}
