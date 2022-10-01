use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

// Common components
use components::app_bar::AppBar;
// Pages
use pages::home::home::Home;
use pages::time_chamber::time_chamber::TimeChamber;

// Main entry point
fn main() {
    yew::start_app::<App>();
}

// Router
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/time-chamber")]
    TimeChamber,
}

// --- Router Switch ---
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::TimeChamber => html! { <TimeChamber />},
    }
}

// --- Main Component ---
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            // Main App wrapper
            <div class="rounded-xl text-white w-screen h-screen border-1 flex flex-col">
                <AppBar />
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}
