extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket;
extern crate anyhow;
extern crate dirs_next;
extern crate lazy_static;

use std::{cmp::max, collections::VecDeque, time::{SystemTime, UNIX_EPOCH}};

use lazy_static::lazy_static;
use rand::Rng;
use rocket::{http::Status, serde::json::Json, tokio::sync::Mutex};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};

async fn start() {
    const GROUPS: u32 = 7;
    let mut problem_list = PROBLEM_LIST.lock().await;
    problem_list.clear();

    let mut group_status = GROUP_STATUS.lock().await;
    group_status.clear();
    group_status.push(GroupStatus {
        id: 0,
        progress: 0_u16,
        end: None
    });
    for col in 1..=GROUPS {
        group_status.push(GroupStatus {
            id: col as u16,
            progress: 0_u16,
            end: None
        });
    }
}

lazy_static! {
    static ref PROBLEM_LIST: Mutex<Vec<ProblemListItem>> = Mutex::new(Vec::new());
    static ref GROUP_STATUS: Mutex<Vec<GroupStatus>> = Mutex::new(Vec::new());
    static ref JUDGE_TASKS: Mutex<[VecDeque<u32>; 8]> = Mutex::new([
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new()
    ]);
}

fn get_problems_from_config() -> anyhow::Result<Vec<Problem>> {
    let mut problems = Vec::new();
    let config_dir = dirs_next::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let problems_path = config_dir.join("wordgame").join("problems.json");
    if !config_dir.join("wordgame").exists() {
        std::fs::create_dir_all(config_dir.join("wordgame"))?;
    }
    if !problems_path.exists() {
        std::fs::write(&problems_path, "[]")?;
    }
    if let Ok(data) = std::fs::read_to_string(problems_path)
        && let Ok(parsed_problems) = serde_json::from_str::<Vec<Problem>>(&data)
    {
        problems = parsed_problems;
    }
    Ok(problems)
}

fn delete_problem_from_config(id: u32) -> anyhow::Result<()> {
    let config_dir = dirs_next::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let problems_path = config_dir.join("wordgame").join("problems.json");
    let mut problems = get_problems_from_config()?;
    problems.retain(|p| p.id != id);
    let serialized = serde_json::to_string_pretty(&problems)?;
    std::fs::write(problems_path, serialized)?;
    Ok(())
}

fn upload_problem_to_config(problem: Problem) -> anyhow::Result<()> {
    let config_dir = dirs_next::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let problems_path = config_dir.join("wordgame").join("problems.json");
    let mut problems = get_problems_from_config()?;
    let mut max_id = 0;
    for problem in problems.clone().into_iter() {
        max_id = max(max_id, problem.id);
    }
    let mut problem = problem;
    problem.id = max_id + 1;
    problems.push(problem);
    let serialized = serde_json::to_string_pretty(&problems)?;
    std::fs::write(problems_path, serialized)?;
    Ok(())
}

#[launch]
async fn rocket() -> _ {
    start().await;
    rocket::build()
        .mount(
            "/api",
            routes![
                start_game,
                get_problem_list,
                get_all_problems,
                upload_problem,
                delete_problem,
                get_problem_from_id,
                get_group_status,
                finish,
                reveal,
                judge,
                get_judge_task
            ],
        )
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
    pub score: u32,
}

/**
 * Unrevealed -> (reveal)
 * Pending -> (submit)
 * Judging -> (judge)
 * Solved / Failed
 */
#[derive(Serialize, Deserialize, Clone, PartialEq)]
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
    pub index: u32, 
    pub status: ProblemStatus,
    pub group: u32,
    pub score: u32,
    pub problem: Option<Problem>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct GroupStatus {
    pub id: u16,
    pub progress: u16,
    pub end: Option<u64>
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
async fn get_all_problems() -> Json<Vec<Problem>> {
    Json(get_problems_from_config().unwrap_or_default())
}

#[get("/problem/<id>")]
async fn get_problem_from_id(id: u32) -> Json<Option<Problem>> {
    let problems = get_problems_from_config().unwrap_or_default();
    for problem in problems {
        if problem.id == id {
            return Json(Some(problem));
        }
    }
    Json(None)
}

#[post("/problem/upload", data = "<data>")]
async fn upload_problem(data: Json<Problem>) -> Status {
    match upload_problem_to_config(data.0) {
        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict,
    }
}

#[delete("/problem/<id>")]
async fn delete_problem(id: u32) -> Status {
    match delete_problem_from_config(id) {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotFound,
    }
}

#[get("/group/status")]
async fn get_group_status() -> Json<Vec<GroupStatus>> {
    Json(GROUP_STATUS.lock().await.clone())
}

#[get("/judge/<group>")]
async fn get_judge_task(group: u32) -> Json<Option<Problem>> {
    let judge_tasks = &mut JUDGE_TASKS.lock().await;
    let problem_list = &PROBLEM_LIST.lock().await;
    if judge_tasks[group as usize].is_empty() {
        return Json(None);
    }
    while !judge_tasks[group as usize].is_empty() && judge_tasks[group as usize].front().is_none() {
        judge_tasks[group as usize].pop_front();
    }
    Json(problem_list[*judge_tasks[group as usize].front().unwrap() as usize].problem.clone())
}

#[post("/judge/<group>/<result>")]
async fn judge(group: u32, result: &str) -> Status {
    let judge_tasks = &mut JUDGE_TASKS.lock().await;
    let problem_list = &mut PROBLEM_LIST.lock().await;
    if judge_tasks.is_empty() {
        return Status::Conflict;
    }
    
    match result {
        "incorrect" => {
            problem_list[*judge_tasks[group as usize].front().unwrap() as usize].status = ProblemStatus::Failed;
            judge_tasks[group as usize].pop_front();
        },
        "correct" => {
            problem_list[*judge_tasks[group as usize].front().unwrap() as usize].status = ProblemStatus::Solved;
            judge_tasks[group as usize].pop_front();
        },
        _ => {
            return Status::Conflict;
        }
    }
    Status::Ok
}

#[get("/group/<id>/finish")]
async fn finish(id: u32) -> Status {
    let group_list = &mut GROUP_STATUS.lock().await;
    let problem_list = &mut PROBLEM_LIST.lock().await;
    let judge_tasks = &mut JUDGE_TASKS.lock().await;

    let mut answering_indices: Vec<usize> = Vec::new();
    let length = problem_list.len();
    for i in 0..length {
        let p: ProblemListItem = problem_list[i].clone();
        if p.group == id && p.status == ProblemStatus::Answering {
            answering_indices.push(i);
        }
    }

    for i in 0..group_list.len() {
        let group = &mut group_list[i];
        if group.id == id as u16 {
            if group.end.is_none() {
                let current = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                group.end = Some(current + 900);
            }
            for &idx in &answering_indices {
                let end = group.end.unwrap();
                let current = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

                if end as i64 - current as i64 <= 0 {
                    problem_list[idx].status = ProblemStatus::Failed;
                    continue;
                }
                
                problem_list[idx].status = ProblemStatus::Judging;
                judge_tasks[id as usize].push_back(idx as u32);
            }

            if answering_indices.len() == 1 || group.progress == 0 {
                group.progress += 1;
            }

            break;
        }
    }
    Status::Ok
}

#[get("/group/<id>/reveal")]
async fn reveal(id: u32) -> Status {
    let problem_list = &mut PROBLEM_LIST.lock().await;
    let all_problems = get_problems_from_config().unwrap_or_default();

    if all_problems.is_empty() {
        return Status::Conflict;
    }

    let total_length = all_problems.len();
    let pick_problem = rand::rng().random_range(0..total_length);

    let index = problem_list.len() as u32;
    problem_list.push(
        ProblemListItem { 
            index, 
            status: ProblemStatus::Answering, 
            group: id, 
            score: all_problems[pick_problem].score, 
            problem: Some(all_problems[pick_problem].clone())
        }
    );

    Status::Ok
}