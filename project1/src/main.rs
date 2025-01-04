#[macro_use] extern crate nickel;
use nickel::Nickel;
fn main() {
   let mut server = Nickel::new();

   server.utilize( router! {
    get "/other" => |_req,_res| {
        "other path"
    }
    get "**" => |_req,_res| {
        "Hello world"
    }
   });


   server.listen("127.0.0.1:6767").unwrap();


}
