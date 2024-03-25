use warp::Filter;

#[tokio::main]
async fn main() {
    // Create a path Filter
    let hello = warp::get().map(|| format!("Hello, World"));

    // Start Server adn pass the route filter to it
    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;
}
