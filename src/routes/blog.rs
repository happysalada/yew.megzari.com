use yew::prelude::*;

/// Blog page
pub struct Blog;

impl Component for Blog {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Blog {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                <header class="app-header">
                    <p>
                        <a
                            class="app-link"
                            href="https://github.com/jetli/create-yew-app"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            { "Create Yew App" }
                        </a>
                        { ", Set up a modern yew web app by running one command." }
                    </p>
                    <p>
                        { "Edit " } <code>{ "src/components/about.rs" }</code> { " and save to reload." }
                    </p>
                </header>
            </div>
        }
    }
}
