use gloo_timers::callback::Timeout;
use yew::{Html, html};
use yew::prelude::*;
use yew_playground_model::Plant;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub plant: Plant
}

#[function_component(PlantView)]
pub fn plant_view(props: &Props) -> Html {
    let name = Clone::clone(&props.plant.name);
    let watering_icon = use_state(|| "wateringcan.png" );

    let watering_action = {
        let watering_icon = Clone::clone(&watering_icon);
        move |_: MouseEvent| {
            let value = "wateringcan3.gif";
            watering_icon.set(value);

            {
                let watering_icon = Clone::clone(&watering_icon.clone());
                Timeout::new(3000, move || watering_icon.set("wateringcan.png"))
                    .forget();
            }
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
                            <img onclick={watering_action} src={*watering_icon} alt="watering" title="watering can" width="100" height="100"/>
                        </article>
                    </div>
                </div>
                <div class="tile is-ancestor">
                    <div class="tile is-parent">
                        <article class="tile is-child box">
                            <p class="title is-4">{"History"}</p>
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