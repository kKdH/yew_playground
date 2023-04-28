use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <h1><span class={ "blackName" }>{ "t" }</span>
                <span class={ "redName" }>{ "rust" }</span>
                <span class={ "blackName" }>{ "y gardener" }</span>
            </h1>
            <h4>{ "Counter:" }</h4>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}