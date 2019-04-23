#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate rand;

mod memory;

use memory::utils::*;
use memory::*;
use rocket::Data;
use rocket_contrib::json::Json;
use std::io::Read;
// use serde_json::json::{Json, JsonValue};

#[derive(Debug, Deserialize)]
struct RequestJson {
    pages: usize,
    requests: String,
}

#[derive(Debug, Serialize)]
struct OutputJson {
    // steps: usize,
    hits: u64,
    misses: u64,
    output_stage: String,
    body: String,
}

#[post(
    "/fifo",
    format = "text/plain; charset=UTF-8",
    data = "<plain_request>"
)]
fn fifo(plain_request: Data) -> Json<OutputJson> {
    let mut string_request = String::new();
    plain_request
        .open()
        .read_to_string(&mut string_request)
        .unwrap();
    println!("string request: {}", string_request);
    let request: RequestJson = serde_json::from_str(&string_request).unwrap();

    let mut mem = Memory::new(request.pages);
    let refs = split_references(request.requests);

    mem.simulate_fifo(&refs);

    let output = OutputJson {
        hits: mem.hits(),
        misses: mem.misses(),
        output_stage: mem.last_state(),
        body: mem.outcome(),
    };

    Json(output)
}

#[post("/lru", format = "text/plain; charset=UTF-8", data = "<plain_request>")]
fn lru(plain_request: Data) -> Json<OutputJson> {
    let mut string_request = String::new();
    plain_request
        .open()
        .read_to_string(&mut string_request)
        .unwrap();
    let request: RequestJson = serde_json::from_str(&string_request).unwrap();
    let mut mem = Memory::new(request.pages);
    let refs = split_references(request.requests);
    mem.simulate_lru(&refs);

    let output = OutputJson {
        hits: mem.hits(),
        misses: mem.misses(),
        output_stage: mem.last_state(),
        body: mem.outcome(),
    };

    Json(output)
}

#[post(
    "/alru",
    format = "text/plain; charset=UTF-8",
    data = "<plain_request>"
)]
fn alru(plain_request: Data) -> Json<OutputJson> {
    let mut string_request = String::new();
    plain_request
        .open()
        .read_to_string(&mut string_request)
        .unwrap();
    let request: RequestJson = serde_json::from_str(&string_request).unwrap();
    let mut mem = Memory::new(request.pages);
    let refs = split_references(request.requests);
    mem.simulate_alru(&refs);

    let output = OutputJson {
        hits: mem.hits(),
        misses: mem.misses(),
        output_stage: mem.last_state(),
        body: mem.outcome(),
    };

    Json(output)
}

#[post("/opt", format = "text/plain; charset=UTF-8", data = "<plain_request>")]
fn opt(plain_request: Data) -> Json<OutputJson> {
    let mut string_request = String::new();
    plain_request
        .open()
        .read_to_string(&mut string_request)
        .unwrap();
    let request: RequestJson = serde_json::from_str(&string_request).unwrap();
    let mut mem = Memory::new(request.pages);
    let refs = split_references(request.requests);
    mem.simulate_opt(&refs);

    let output = OutputJson {
        hits: mem.hits(),
        misses: mem.misses(),
        output_stage: mem.last_state(),
        body: mem.outcome(),
    };

    Json(output)
}

#[post(
    "/rand",
    format = "text/plain; charset=UTF-8",
    data = "<plain_request>"
)]
fn rand(plain_request: Data) -> Json<OutputJson> {
    let mut string_request = String::new();
    plain_request
        .open()
        .read_to_string(&mut string_request)
        .unwrap();
    let request: RequestJson = serde_json::from_str(&string_request).unwrap();
    let mut mem = Memory::new(request.pages);
    let refs = split_references(request.requests);
    mem.simulate_rand(&refs);

    let output = OutputJson {
        hits: mem.hits(),
        misses: mem.misses(),
        output_stage: mem.last_state(),
        body: mem.outcome(),
    };

    Json(output)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![fifo, alru, lru, opt, rand])
        .launch();
}
