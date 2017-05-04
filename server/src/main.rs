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

type DbConn = Mutex<Connection>;

#[derive(Serialize, Deserialize)]
struct matchIDs{
    contents: Vec<i64>
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
fn matches(jMatchIDs: JSON<matchIDs>, db_conn: State<DbConn>) -> JSON<Value>{
    let matchIDs: Vec<i64> = jMatchIDs.0.contents;
    print!("{:?}", matchIDs);
    let mut output: Vec<String> = Vec::new();
    for matchID in matchIDs.iter(){
    let lock = db_conn.lock()
        .expect("db connection lock");
    let results = lock
        .query("SELECT url from replayurls where matchid = $1",
                   &[&matchID]).unwrap();
        let replayURL = results.get(0).get(0);
        output.push(replayURL);
    }
    return JSON(json!(output))
    //"list of replay urls"
}

fn rocket() -> Rocket {
    let conn = Connection::connect("postgres://jdog@localhost:5432/replayurls", TlsMode::None).expect("wut");
    // Have Rocket manage the database pool.
    rocket::ignite().manage(Mutex::new(conn)).mount("/", routes![hello, matches])
}

fn main() {
    rocket().launch();
}
