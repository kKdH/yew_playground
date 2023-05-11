use gloo_timers::callback::Timeout;
use log::{error, info};
use yew::{Html, html};
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};

use yew_playground_model::Plant;
use crate::api;

const WATERING_CAN_STATIC: &'static str = "wateringcan.png";
const WATERING_CAN_ANIMATED: &'static str = "wateringcan3.gif";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub plant: Plant
}
#[derive(PartialEq)]
pub struct Counter {
    pub value: i64
}

#[function_component(PlantView)]
pub fn plant_view(props: &Props) -> Html {

    let name = Clone::clone(&props.plant.name);
    let watering_icon = use_state(|| WATERING_CAN_STATIC);

    let async_watering_history = {
        let name = Clone::clone(&props.plant.name);
        use_async(api::get_watering_history(name))
    };

    {
        let async_watering_history = Clone::clone(&async_watering_history);
        use_effect_once(move || {
            async_watering_history.run();
            || {}
        });
    }

    let watering_action = {
        let async_watering_history = Clone::clone(&async_watering_history);
        let watering_icon = Clone::clone(&watering_icon);
        let name = Clone::clone(&name);
        move |_: MouseEvent| {
            let name = Clone::clone(&name);
            let async_watering_history = Clone::clone(&async_watering_history);
            watering_icon.set(WATERING_CAN_ANIMATED);
            {
                let watering_icon = Clone::clone(&watering_icon.clone());
                Timeout::new(3000, move || {
                    watering_icon.set(WATERING_CAN_STATIC);
                }).forget();
            }
            api::do_watering(Clone::clone(&name), move |result| {
                match result {
                    Ok(_) => {
                        async_watering_history.run();
                        info!("Watered {}", name);
                    }
                    Err(_) => {
                        error!("Failed to water plant: {}", name);
                    }
                }
            });
        }
    };

    let clear_watering_history = {
        let name = Clone::clone(&name);
        let async_watering_history = Clone::clone(&async_watering_history);
        move |_: MouseEvent| {
            let name = Clone::clone(&name);
            let async_watering_history = Clone::clone(&async_watering_history);
            api::clear_watering_history(Clone::clone(&name), move |result| {
                match result {
                    Ok(_) => {
                        async_watering_history.run();
                        info!("Cleared history of plant: {}", name);
                    }
                    Err(_) => {
                        error!("Failed to delete history of plant: {}", name);
                    }
                }
            });
        }
    };

    html! {
        <div class="columns">
            <div class="column is-two-thirds">
                <div class="subtitle is-3">{"Name: "}{name}</div>
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        <article class="tile is-child box">
                            <p class="title is-4">{"Wasserstand"}</p>
                        </article>
                    </div>
                    <div class="tile is-parent">
                        <article class="tile is-child box">
                            <p class="title is-4">{"Helligkeit"}</p>
                        </article>
                    </div>
                    <div class="tile is-parent">
                        <article class="tile is-child box">
                            <p class="title is-4">{"Gießen"}</p>
                            <p class="subtitle is-6">{ "Drücke auf das Bild, wenn du die Pflanze gegossen hast, um die Daten zu speichern:" }</p>
                            {
                                if let Some(watering_history) = &async_watering_history.data {
                                    html! { <p>{ watering_history.history.len() }</p> }
                                }
                                else {
                                    html!(<p>{"<< error >>"}</p>)
                                }
                            }
                            <img onclick={watering_action} src={*watering_icon} alt="watering" title="watering can" width="100" height="100"/>
                        </article>
                    </div>
                </div>
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        <article class="tile is-child box">
                            <p class="title is-4">{"History"}</p>
                            <button onclick={clear_watering_history}>{ "Clear" }</button>
                        </article>
                    </div>
                    <div class="tile is-parent">
                        <article class="tile is-child box">
                            <p class="title is-4">{"optional"}</p>
                        </article>
                    </div>
                </div>
            </div>
            <div class="column">
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        <article class="tile is-child box">
                            <img src="plant1.png" alt="a plant" />
                        </article>
                    </div>
                </div>
            </div>
        </div>
    }
}
