use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
         // let onclick = {
         // let counter = counter.clone();
         //    move |_| {
         //        let value = *counter + 1;
         //        counter.set(value);
         //    }
         // };

    html! {
            <div class="container is-fluid">
                <div class="title is-1">
                    <h1 id={ "projectName" }>
                        <span class={ "blackName" }>{ "t" }</span>
                        <span class={ "redName" }>{ "rust" }</span>
                        <span class={ "blackName" }>{ "y gardener" }</span>
                    </h1>
                </div>
                <div class="columns">
                    <div class="column is-two-thirds">
                        <div class="subtitle is-2">{"Beschreibung:"}</div>
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
                                <div class="tile is-child box">
                                    <img src="plant1.png" alt="a plant" />
                                </div>
                            </div>
                        </div>
                        <div class="tile is-ancestor">
                            <div class="tile is-parent">
                                <div class="tile is-child box">
                                    <p class="title">{ "submit your plant" }</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <footer class="footer">
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