use std::error::Error;
use std::fs::File;
use std::sync::Arc;
use axum::{
    routing::{get},
    extract::Query,
    http::StatusCode,
    Json, Router,
};
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

struct AppState {
    users: Vec<User>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    //tracing_subscriber::fmt::init();
    let users = read_csv("/usr/local/bin/data/dummy_data.csv").unwrap();

    let shared_data = Arc::new(AppState { users });
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/user", get(get_user))
        .with_state(shared_data);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn read_csv(path: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut users = Vec::new();

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: User = result?;
        //println!("{:?}", record);
        users.push(record);
    }
    Ok(users)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_user(q_params: Query<GetUser>, State(state): State<Arc<AppState>>) -> Response {
    let user = state.users.iter().find(|&x| x.username == q_params.username);
    match user {
        Some(user) => (StatusCode::OK, Json(user.clone())).into_response(),

        _ => StatusCode::NOT_FOUND.into_response()
    }
}


#[derive(Deserialize)]
struct GetUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    #[serde(rename = "Username")]
    username: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Phone Number")]
    phone: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Email")]
    email: String,
}