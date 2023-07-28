use axum::{routing::get, Router, response::Html};

async fn hello_world() -> Html<&'static str> {
    Html("<h1>어썸링님 그는 신이야!</h1>")
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
