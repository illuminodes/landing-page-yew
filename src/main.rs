use landing_page_yew::{AppLayout, HomePage, TranslationProvider};
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <TranslationProvider>
            <AppLayout>
                <HomePage />
            </AppLayout>
        </TranslationProvider>
    }
}
