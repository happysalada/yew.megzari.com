use yew_router::prelude::*;
use yew_router::switch::Permissive;

pub mod about;
pub mod blog;
pub mod home;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/blog"]
    Blog,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/"]
    Home,
}
