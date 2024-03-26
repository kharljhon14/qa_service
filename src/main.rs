use serde::{Deserialize, Serialize};
use warp::{
    filters::cors::CorsForbidden,
    http::{Method, StatusCode},
    Filter, Rejection, Reply,
};

use std::collections::HashMap;

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }

    // fn add_question(mut self, question: Question) -> Self {
    //     self.questions.insert(question.id.clone(), question);

    //     self
    // }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    println!("{:?}", params);
    let res = store.questions.values().cloned().collect::<Vec<Question>>();

    Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }

    // else if let Some(InvalidId) = r.find() {
    //     Ok(warp::reply::with_status(
    //         "No valid Id presented".to_string(),
    //         StatusCode::UNPROCESSABLE_ENTITY,
    //     ))
    // }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    // Create a path Filter
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

    // Start Server adn pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
