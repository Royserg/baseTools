use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            // Hero section
            <div class="w-100 bg-neutral-500">
                <div class="container mx-auto">
                    <div class="flex flex-col items-center justify-center h-44">
                        <h1 class="text-5xl font-bold">{"Fullstack Rust"}</h1>
                        <div class="border w-1/3 p-4 flex items-center justify-center">{"Logo"}</div>
                    </div>
                </div>
            </div>

            // Content
            <div class="rounded-b-xl bg-slate-800 p-4 w-100 grow">

                <div class="flex w-80 mx-auto">
                    <Link<Route> to={Route::TimeChamber} classes="border-10 p-4 bg-red-800">
                    { "Time Chamber" }
                    </Link<Route>>
                </div>

            </div>
        </>
    }
}
