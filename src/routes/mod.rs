use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Homepage,
    #[at("/details")]
    Details,
    #[at("/gallery")]
    Gallery,
    #[not_found]
    #[at("/404")]
    NotFound,
}
