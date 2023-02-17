use yew::{html, function_component, Html};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{"Hello World!"}</h1>
        </div>
    }
}