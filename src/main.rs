use yew::prelude::*;
use gloo_timers;
use gloo_timers::callback::Timeout;

#[function_component(App)]
fn app() -> Html {
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
            <div class="container hero is-fluid is-fullheight">

                <div class="title is-1">
                    <h1 id={ "projectName" }>
                        <span class={ "blackName" }>{ "T" }</span>
                        <span class={ "redName" }>{ "RUST" }</span>
                        <span class={ "blackName" }>{ "Y GARDENER" }</span>
                    </h1>
                </div>
                <div class="columns">
                    <div class="column is-two-thirds">
                        <div class="subtitle is-3">{"Beschreibung:"}</div>
                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title">{"Wasserstand"}</p>
                                </article>
                            </div>
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title">{"Helligkeit"}</p>
                                </article>
                            </div>
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title">{"Gießen"}</p>
                                    <p class="subtitle is-4">{ "Drücke auf das Bild nach dem Gießen:" }</p>
                                    <img onclick={watering_action} src={*watering_icon} title="watering can" width="100" height="100"/>
                                </article>
                            </div>
                        </div>
                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title">{"History"}</p>
                                </article>
                            </div>
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title">{"optional"}</p>
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
                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <article class="tile is-child box">
                                    <p class="title">{ "submit your plant" }</p>
                                </article>
                            </div>
                        </div>
                    </div>
                </div>

                <footer class="footer mt-auto">
                    <div class="content has-text-centered">
                        <p>{"footer"}</p>
                    </div>
                </footer>
            </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}