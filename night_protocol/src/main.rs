#[macro_use]
extern crate rocket;
use rand::Rng;
use rocket::response::content::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

mod blockchain;
use blockchain::blockchain::*;
use blockchain::master_chain::*;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: 