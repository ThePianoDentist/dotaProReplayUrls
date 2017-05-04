#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate postgres;

#[cfg(test)] mod tests;

use std::sync::Mutex;
use rocket::{Rocket, State};
use postgres::{Connection, TlsMode};
use rocket_contrib::{JSON, Value};

#[derive(Serialize, Deserialize)]
struct matchIDs{
    contents: Vec<u64>
}

#[derive(Serialize, Deserialize)]
struct replayURLs{
    contents: Vec<String>
}


#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/matches", format="application/json", data= "<jMatchIDs>")]
fn matches(jMatchIDs: JSON<matchIDs>) -> JSON<Value>{
    let matchIDs: Vec<u64> = jMatchIDs.0.contents;
    let mut output: Vec<String> = Vec::new();
    for matchID in matchIDs.iter(){
        let replaySalt: String = "ha".to_string();
        let replayCluster: String = "bo".to_string();
        let replayURL = format!("http://replay{}.valve.net/570/{}_{}.dem.bz2", replayCluster, matchID.to_string(), replaySalt);
        output.push(replayURL);
    }
    return JSON(json!(output))
    //"list of replay urls"
}

fn rocket() -> Rocket {
    let conn = Connection::connect("postgres://jdog@localhost:5432/replayurls", TlsMode::None);
        // Have Rocket manage the database pool.
    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/", routes![hello, matches])
}

fn main() {
    rocket().launch();
}
