#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Number{
    number : i32,
}

#[get("/num")]
fn getRequest()-> Html<String>{
    let h1=format!("
    <div style='font-family: Helvetica, Arail; text-align: center;'>
        <h1 style='color:white; padding: 10px; background-color:#9e00c5;'>Making Rust Server</h1>
        <p style='font-size: 20px;'>Enter a Number, It will reply you the number add with 2.</p>
        <form style='margin: 100px 0px; margin-bottom: 80px;' action='/num' method='post'>
            <input style='padding: 5px 10px; border-radius: 3px;  height: 40px;
            width: 500px; font-size: 24px; border: 1px solid #9e00c5;' type='number' placeholder='Enter number'
                name='number' id='number'>
            <input style='font-size: 24px; padding:5px 12px; 
            border: none; height: 40px;
            border-radius: 3px;  color: white; background-color: #9e00c5

            ;' type='Submit'>
        </form>
        
    </div>
    ");
    Html(h1)
}

#[post("/num", data = "<number>")]
fn recieveRequest(number : Form<Number>) -> Html<String> {
   let h1=format!("
   <div style='font-family: Helvetica, Arail; text-align: center;'>
   <h1 style='color:white; padding: 10px; background-color:#9e00c5;'>Making Rust Server</h1>
   <p style='font-size: 24px; color:#770494; font-weight: bolder;'>
   The number you entered is {} and the result is :
   {}</p>
   
   </div>
    ", number.number, number.number+2);
    Html(h1)
}


#[get("/")]
fn home()-> Html<String>{
    let html=format!("
    <div
        style='position: relative; font-family: Helvetica, Arail; text-align: center; width: 100%; height:100vh; overflow: hidden;'>
        
        <div
            style='position: absolute; top:0; background-color: #9e00c5; width: inherit; height: inherit; opacity: 0.3;'>
        </div>
        <div style='position: absolute; top:40%; left: 40%;
        '>
            <h1
                style='margin-bottom:  10px; font-size: 4rem; font-weight: bolder; color: white;  text-shadow: 2px 2px 4px #000000;'>
                Rust Server</h1>
            <a href='/num' style='padding: 10px 20px; margin-top: 10px; 
            font-size: 24px; color: white; background-color: #9e00c5; text-decoration: none; border-radius: 5px;'>Get
                Started</a>
        </div>");
    Html(html)
}


fn main() {
    rocket::ignite().mount("/", routes![home, getRequest, recieveRequest]).launch();
}