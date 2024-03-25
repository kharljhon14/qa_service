use serde::Serialize;
use std::str::FromStr;
use warp::Filter;

#[derive(Debug, Serialize)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(QuestionId(s.to_string()))
    }
}

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

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of the question".to_string(),
        Some(vec!["Faq".to_string()]),
    );

    Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
    // Create a path Filter
    let get_items = warp::get()
        .and(warp::path("questions").and(warp::path::end()))
        .and_then(get_questions);

    let routes = get_items;

    // Start Server adn pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
