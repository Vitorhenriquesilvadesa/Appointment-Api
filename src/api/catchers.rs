use rocket::{Catcher, Request, catch, catchers};

pub fn global_catchers() -> Vec<Catcher> {
    catchers![not_found]
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Route not found: {}", req.uri())
}
