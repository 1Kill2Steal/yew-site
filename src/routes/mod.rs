use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Homepage,
    #[at("/about-me")]
    AboutMe,
    #[at("/gallery")]
    Gallery,

    #[at("/blog")]
    Blog,
    #[at("/blog/test")]
    BlogTest,

    #[not_found]
    #[at("/404")]
    NotFound,
}
