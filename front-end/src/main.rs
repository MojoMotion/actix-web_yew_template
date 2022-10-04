#![warn(clippy::all, rust_2018_idioms)]

use yew::prelude::*;
use yew_router::prelude::*;

mod header;
mod listing_control;
mod categories_listing;
mod categories_slider;
mod last_articles;
mod footer;


#[derive(Clone, Routable, PartialEq)]
enum Route {
//    #[at("/login")]
//    Login,
    #[at("/")]
    Main,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
//        Route::Login => html! { <Login/> },
        Route::Main => html! { <App/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}


struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message)-> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        //let link = ctx.link();
        html!(
            <body>
                <header::Header/>
                <listing_control::ListingControl/>
                <categories_listing::CategoriesListing/>
                <categories_slider::CategoriesSlider/>
                <last_articles::LastArticles/>
                <footer::Footer/>
             </body>
        )
    }
}

fn main() {
    yew::start_app::<App>();
}
