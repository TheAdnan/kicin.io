#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {

    "Home page"
}

#[get("/about")]
fn about() -> String {

    "This is an about page".to_string()

}

fn main() {
    rocket::ignite().mount("/", routes![index, about]).launch();
}
