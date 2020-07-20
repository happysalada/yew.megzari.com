use crate::components::tag::Tag;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewtil::fetch::{Fetch, FetchAction, FetchRequest, FetchState, Json, MethodBody};
use yewtil::future::LinkFuture;

/// About page
pub struct About {
    projects: Fetch<Request, Vec<Project>>,
}
#[derive(Default, Debug, Clone)]
pub struct Request;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Client {
    name: String,
    url: String,
    caption: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    title: String,
    description: String,
    client: Client,
    tags: Vec<Tag>,
}

impl FetchRequest for Request {
    type RequestBody = ();
    type ResponseBody = Vec<Project>;
    type Format = Json;

    fn url(&self) -> String {
        "/projects.json".to_string()
    }

    fn method(&self) -> MethodBody<Self::RequestBody> {
        MethodBody::Get
    }

    fn headers(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn use_cors(&self) -> bool {
        true
    }
}
pub enum Msg {
    SetProjectsFetchState(FetchAction<Vec<Project>>),
}

impl Component for About {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let projects: Fetch<Request, Vec<Project>> = Default::default();
        link.send_future(projects.fetch(Msg::SetProjectsFetchState));
        About { projects }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetProjectsFetchState(fetch_state) => {
                self.projects.apply(fetch_state);
            }
        }
        true
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
        <div class="content-grid">
          <div
            class="content w-full text-xl leading-normal text-white"
            style="font-family: Georgia, serif;"
          >
            <div style="height: 80vh;">
              <div class="font-sans">
                <h1
                  class="font-sans break-normal text-white pt-6 pb-2 text-3xl
                  md:text-4xl"
                >
                  { "About me" }
                </h1>
              </div>
              <p class="py-6">
                { "I'm a developper who thinks of programming as a form of Art!" }
                <br />
                <br />
                { "I'm quite interested in the biotech space. contact me if there is
                something you want to make." }
                <br />
                <br />
                { "raphael at megzari dot com" }
                <br />
                <br />
                { "Here is my portfolio" }
              </p>
            </div>
            {
              match self.projects.as_ref().state() {
                  FetchState::Fetching(_) => html! {"Fetching"},
                  FetchState::Fetched(data) => data.iter().map(render_project).collect(),
                  FetchState::Failed(_, err) => html! {&err},
                  FetchState::NotFetching(_) => html! {"Not Fetching"}
              }
            }
            // {#each projects as project}
            //   <Project {project} />
            // {/each}
          </div>
        </div>
                }
    }
}

fn render_project(project: &Project) -> Html {
    let Project {
        client,
        description,
        title,
        tags,
    } = project;
    html! {
    <div
      class="flex flex-col justify-center text-center my-auto mx-auto"
      style="height: 80vh;"
    >
      <h2
        class="text-4xl tracking-tight leading-10 font-extrabold text-white mt-3 md:mt-10"
      >
        { title }
      </h2>
      <p class="mt-3 md:mt-10 text-base md:text-xl">
        { "For: " }
        <a
          href=client.url.as_str()
          class="leading-6 text-blue-500 hover:text-blue-300"
        >
          { client.name.as_str() }
        </a>
        { " - " } { client.caption.as_str() }
      </p>

      <p class="mt-3 md:mt-10 text-base md:text-xl">
        { description }
      </p>
      <p class="mt-3 md:mt-10 text-sm font-normal text-gray-500">
        { "Tags: " }
        { for tags.iter().map(|tag| {
          html!{  <Tag tag=tag /> }
        } )}
      </p>
    </div>
            }
}
