#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Number{
    number : i8,
}

#[get("/num")]
fn getRequest()-> Html<String>{
    let h1=format!("<form action='/num' method='post'>
    <input type='text' placeholder='enter number' name='number' id='number'>
    <input type='Submit'>
    </form>");
    Html(h1)
}

#[post("/num", data = "<number>")]
fn recieveRequest(number : Form<Number>) -> Html<String> {
   let h1=format!("<h1 style='color: blue;'>hello guys {}</h1>", number.number + 2);
    Html(h1)
}


fn main() {
    rocket::ignite().mount("/", routes![getRequest, recieveRequest]).launch();
}