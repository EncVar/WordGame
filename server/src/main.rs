extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket;
extern crate lazy_static;

use rocket::{http::Status, serde::json::Json, tokio::sync::Mutex};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

async fn start() {
    const GROUPS: u32 = 7;
    let problem_list = &mut PROBLEM_LIST.lock().await;
    problem_list.clear();
    for col in 1..=GROUPS {
        problem_list.push(
            ProblemListItem { 
                status: ProblemStatus::Unrevealed, 
                column: col, 
                score: 100, 
                problem: None, 
                end: None,
            }
        );
        problem_list.push(
            ProblemListItem { 
                status: ProblemStatus::Judging, 
                column: col, 
                score: 200, 
                problem: None, 
                end: None,
            }
        );
        problem_list.push(
            ProblemListItem { 
                status: ProblemStatus::Failed, 
                column: col, 
                score: 300, 
                problem: None, 
                end: None,
            }
        );
        problem_list.push(
            ProblemListItem { 
                status: ProblemStatus::Solved, 
                column: col, 
                score: 400, 
                problem: None, 
                end: None,
            }
        );
    }
}

lazy_static! {
    static ref PROBLEM_LIST: Mutex<Vec<ProblemListItem>> = Mutex::new(Vec::new());
}

#[launch]
async fn rocket() -> _ {
    start().await;
    rocket::build()
        .mount("/api", routes![
            start_game, 
            get_problem_list,
            get_all_problems
        ])
        .attach(
            CorsOptions {
                allowed_origins: AllowedOrigins::all(),
                allow_credentials: true,
                ..Default::default()
            }
            .to_cors()
            .expect("error while building CORS"),
        )
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8000,
            ..rocket::Config::default()
        })
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct Problem {
    pub id: u32,
    pub question: String,
    pub answer: String,
    pub limit: u32,
    pub score: u32,
}


/**
 * Unrevealed -> (reveal)
 * Pending -> (submit)
 * Judging -> (judge)
 * Solved / Failed
 */
#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde", rename_all = "lowercase")]
enum ProblemStatus {
    Unrevealed,
    Answering,
    Solved,
    Failed,
    Judging,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct ProblemListItem {
    pub status: ProblemStatus,
    pub column: u32,
    pub score: u32,
    pub problem: Option<Problem>,
    pub end: Option<u64>,
}

#[post("/start")]
async fn start_game() -> Status {
    start().await;
    Status::Ok
}

#[get("/problems")]
async fn get_problem_list() -> Json<Vec<ProblemListItem>> {
    let problem_list = PROBLEM_LIST.lock().await;
    Json(problem_list.clone())
}

#[get("/problems/all")]
async fn get_all_problems() -> Json<Vec<ProblemListItem>> {
    Json(vec![])
}