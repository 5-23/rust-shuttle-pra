use axum::{routing::get, Router, response::Html, Json};

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
struct Counter {
    val: isize
}
static mut COUNTER: Counter = Counter{val: 0};


async fn hello_world() -> Html<&'static str> {
    Html("<h1>어썸링님 그는 신이야!</h1>")
}


async fn counting() -> Json<Counter> {
    unsafe{ COUNTER.val += 1 }
    Json(unsafe { COUNTER })
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/cnt", get(counting));

    Ok(router.into())
}
