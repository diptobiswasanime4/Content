use axum::{handler::get, routing::get, response::Html, Router, Json};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(home_handler))
                           .route("/get_todos", get(get_todos_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub struct Todo {
    id: String,
    name: String,
    completed: bool
}

async fn home_handler() -> Json<Todo> {
    Json(Todo {
        id: String::from("1"),
        name: String::from("Write Python"),
        completed: false,
    })
}

async fn get_todos_handler() -> Html<&'static str> {
    Html("<h1>Get Todos.</h1>")
}
