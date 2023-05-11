use log::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once, use_toggle};

use yew_playground_model::Plant;

use crate::components::Footer;
use crate::components::PlantView;

mod components;
mod api;

#[function_component(App)]
fn app() -> Html {
    let selected_plant = use_state_eq(|| Option::<Plant>::None);
    let async_plant_list = {
        let selected_plant = Clone::clone(&selected_plant);
        use_async(async move {
            let result = api::get_plants().await;
            if let Ok(plants) = &result {
                if selected_plant.is_none() && !plants.is_empty() {
                    selected_plant.set(Some(Clone::clone(&plants[plants.len() - 1])));
                }
            }
            result
        })
    };

    {
        let async_plant_list = Clone::clone(&async_plant_list);
        use_effect_once(move || {
            async_plant_list.run();
            || {}
        });
    }

    let plant_changed_action = {
        let async_plant_list = Clone::clone(&async_plant_list);
        let selected_plant = Clone::clone(&selected_plant);
        move |event: Event| {
            if let Some(plant_list) = &async_plant_list.data {
                let html_select = event.target_dyn_into::<web_sys::HtmlSelectElement>().unwrap();
                let plant = Clone::clone(&plant_list[html_select.selected_index() as usize]);
                let name = Clone::clone(&plant.name);
                selected_plant.set(Some(plant));
                info!("Selected plant: {}", name);
            }
        }
    };

    let show_create_plant_modal = use_toggle("modal", "modal is-active");
    let toggle_create_plant_modal = {
        let show_create_plant_modal = Clone::clone(&show_create_plant_modal);
        move |_: MouseEvent| {
            show_create_plant_modal.toggle();
        }
    };

    let create_plant_modal_name = use_state(|| String::new());
    let create_plant_modal_species = use_state(|| String::new());
    let create_plant_modal_name_changed = {
        let name = Clone::clone(&create_plant_modal_name);
        move |event: KeyboardEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            info!("Created name: {}", input.value());
            name.set(input.value());
        }
    };

    html! {
        <div class="container hero is-fluid is-fullheight">
            <div class={*show_create_plant_modal}>
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{"Create new plant"}</p>
                    </header>
                    <section class="modal-card-body">
                        <div class="field">
                            <label class="label">{"Name:"}</label>
                            <div class="control">
                                <input onkeypress={create_plant_modal_name_changed} class="input" type="text" placeholder={"e.g Silver Birch"}/>
                            </div>
                        </div>
                        <div class="field">
                            <label class="label">{"Species:"}</label>
                            <div class="control">
                                <input class="input" type="text" placeholder={"e.g betula pendula"}/>
                            </div>
                        </div>
                    </section>
                    <footer class="modal-card-foot">
                        <button class="button is-success" onclick={Clone::clone(&toggle_create_plant_modal)}>{"Create"}</button>
                        <button class="button" onclick={Clone::clone(&toggle_create_plant_modal)}>{"Cancel"}</button>
                    </footer>
                </div>
            </div>
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
                                        if let Some(plant_list) = &async_plant_list.data {
                                            plant_list.iter().cloned().map(|plant| {
                                                html!{<option value={Clone::clone(&plant.name)}>{plant.name}</option>}
                                            }).collect::<Html>()
                                        }
                                        else {
                                            html!{<option>{ "empty" }</option>}
                                        }
                                    }
                                </select>
                            </div>
                            <button class="button is-medium" onclick={toggle_create_plant_modal}>
                                    <i class="fa-solid fa-plus"></i>
                            </button>
                            <button class="button is-medium subtitle">
                                <i class="fa-solid fa-arrow-up-from-bracket"></i>
                            </button>
                        </article>
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

