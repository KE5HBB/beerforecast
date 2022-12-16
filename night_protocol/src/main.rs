#[macro_use]
extern crate rocket;
use rand::Rng;
use rocket::response::content::Json;
use rocket::State;
use serde::{Deserialize, Seriali