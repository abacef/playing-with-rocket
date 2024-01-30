#[macro_use] extern crate rocket;

#[get("/nms")]
fn nms() -> &'static str {
    "Hello, NMS!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![nms])
}
