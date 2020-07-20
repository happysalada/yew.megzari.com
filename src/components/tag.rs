use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    name: String,
    link: String,
}
#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub tag: Tag,
}

impl Component for Tag {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let Tag { name, link } = props.tag;
        Tag { name, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
        <a href=self.link.as_str() target="_blank">
          <span
            class="inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium leading-5 text-blue-500 border-2 border-blue-500 mx-1 my-1 hover:bg-blue-500 hover:text-black"
          >
            { &self.name }
          </span>
        </a>
        }
    }
}
