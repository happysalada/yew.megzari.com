use yew::prelude::*;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, route::Route};

use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::routes::{about::About, blog::Blog, home::Home, post::PostPage, AppRoute};

/// Root component
pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
          <Router<AppRoute, ()>
            render = Router::render(|switch: AppRoute | {
              html!{
                <div class="grid">
                  <Nav />
                  <main class="main h-full">
                  { match switch {
                      AppRoute::Home => html!{ <Home /> },
                      AppRoute::About => html!{ <About /> },
                      AppRoute::Blog => html!{ <Blog /> },
                      AppRoute::Post(slug) => html!{ <PostPage slug=slug /> },
                      AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                      AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                    }
                  }
                  </main>
                  <footer class="footer">
                    <Footer />
                  </footer>
                </div>
              }
            })
            redirect = Router::redirect(|route: Route<()>| {
              AppRoute::PageNotFound(Permissive(Some(route.route)))
            })
          />
        }
    }
}
