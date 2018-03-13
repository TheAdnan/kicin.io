#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {

    "Home page"
}

#[get("/about")]
fn about() -> &'static str {

    "This is an about page"

}

fn main() {
    rocket::ignite().mount("/", routes![index, about])ll.launch();
}
