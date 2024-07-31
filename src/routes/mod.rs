use rocket::Route;
use crate::controllers::{auth, authors, books};

pub fn auth_routes() -> Vec<Route> {
    routes![
        auth::sign_in,
        auth::sign_up,
        auth::me
    ]
}

pub fn author_routes() -> Vec<Route> {
    routes![
        authors::index,
        authors::create,
        authors::show,
        authors::update,
        authors::delete,
        authors::get_books
    ]
}

pub fn book_routes() -> Vec<Route> {
    routes![
        books::index,
        books::create,
        books::show,
        books::update,
        books::delete
    ]
}
