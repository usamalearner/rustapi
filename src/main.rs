#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Html;
#[derive(FromForm)]
struct Number{
    number:i32,
}

#[get("/num")]
fn get_request()->Html<String>{
    let h1 = format!("
    <div style='font-family: Helvetica, Arail; text-align: center;'>
    <h1 style='color:white; padding: 10px; background-color:#c50073;'>Making Rust Server</h1>
    <h1 style='color:#181718; padding: 10px; background-color:#c337e6;'>BY Syed Usama Ali</h1>
    <p style='font-size: 20px;'>Enter a Number, It will reply you the number add with 2.</p>
    <form style='margin: 100px 0px; margin-bottom: 80px;' action='/num' method='post'>
        <input style='padding: 5px 10px; border-radius: 3px;  height: 40px;
        width: 500px; font-size: 24px; border: 1px solid #c337e6;' type='number' placeholder='Enter number'
            name='number' id='number'>
        <input style='font-size: 24px; padding:5px 12px; 
        border: none; height: 40px;
        border-radius: 3px;  color: white; background-color: #da1175

        ;' type='Submit'>
    </form>
</div>
    ");
    Html(h1)
}

#[post("/num", data = "<number>")]
fn receive_request(number : Form<Number>) -> Html<String> {
    let h1=format!("<div style='font-family: Helvetica, Arail; text-align: center;'>
 <h1 style='color:white; padding: 10px; background-color:#c50073;'>Making Rust Server</h1>
 <h1 style='color:#181718; padding: 10px; background-color:#c337e6;'>BY Syed Usama Ali</h1>
 <p style='font-size: 20px;'>The number you entered is : {} and the result is : {}</p>
</div>", number.number, number.number+2);
 Html(h1)

}

#[get("/")]
fn home()->Html<String> {
    let html = format!(" 
    <body style=' background-color: rgb(211, 95, 118) ;'>
      <div style='text-align: center; background-color: rgb(211, 95, 118) ; '>
 
    
    <H1 style='text-align: center; color: beige;font-size: 4rem; font-weight: bolder; color: white;  text-shadow: 2px 2px 4px #000000;'>Rust Server</H1>
    <a href='/num' style='padding: 10px 20px; text-align: center;
    font-size: 24px; color: white; background-color: #a30404; text-decoration: none; border-radius: 5px;'>Lets
        Start</a>
    <h1 style='text-align: center; color: rgb(8, 8, 8);font-size: 4rem; font-weight: bolder; color: rgb(10, 10, 10);  text-shadow: 2px 2px 4px #000000;'>By Syed Usama Ali</h1>

</div></body>");
Html(html)
}
fn main(){
    rocket::ignite().mount("/",routes![home,get_request,receive_request]).launch();
}
