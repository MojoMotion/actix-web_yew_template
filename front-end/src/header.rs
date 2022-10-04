#![warn(clippy::all, rust_2018_idioms)]

use yew::prelude::*;


pub struct Header {
}

impl Component for Header {
    type Message  = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <header>
              <a href="http://www.wiki-bio.com/">
                <img id="logo" src="img/logo.png" alt="Wiki Bio" />
              </a>
              <nav id="headnav">
                <a class="nav_field" href="https://www.wiki-bio.com/blog/archives/listing-category/encyclopedie-des-plantes">{"LES PLANTES"}</a>
                <a class="nav_field" href="http://www.wiki-bio.com/blog/archives/listings/les-complements-alimentaires-de-a-a-z">{"LES COMPLÉMENTS"}</a>
                <a class="nav_field" href="http://www.wiki-bio.com/blog/archives/listings/les-maladies-de-a-a-z">{"LES MALADIES"}</a>
              </nav>
              <nav id="nav_social">
                <a href="http://www.wiki-bio.com/feed" target="_blank">
                  <img alt="Rss" src="http://www.wiki-bio.com/wp-content/themes/wikibio/images/rss.png" />
                </a>
                <a href="https://www.facebook.com/wikibio" target="_blank">
                  <img alt="Facebook" src="http://www.wiki-bio.com/wp-content/themes/wikibio/images/facebook.png" />
                </a>
              </nav>
            </header>
        )
    }
}
