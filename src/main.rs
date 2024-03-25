use serde::Serialize;
use std::str::FromStr;
use warp::{http::StatusCode, reject::Reject, Filter, Rejection, Reply};

#[derive(Debug, Serialize)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(QuestionId(s.to_string()))
    }
}

#[derive(Debug)]
struct InvalidId;

impl Reject for InvalidId {}

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

async fn get_questions() -> Result<impl Reply, Rejection> {
    let question = Question::new(
        QuestionId::from_str("0").expect("No id provided"),
        "First Question".to_string(),
        "Content of the question".to_string(),
        Some(vec!["Faq".to_string()]),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(_invalid_id) = r.find::<QuestionId>() {
        Ok(warp::reply::with_status(
            "No valid Id presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    // Create a path Filter
    let get_items = warp::get()
        .and(warp::path("questions").and(warp::path::end()))
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items;

    // Start Server adn pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
