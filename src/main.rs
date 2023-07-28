use std::{path::PathBuf, fs};

use axum::{routing::get, Router, response::Html, Json, extract::Query};
use tower_http::services::ServeDir;
use serde_json::json;

#[derive(serde::Deserialize, Debug)]
struct User {
    id: String,
    pw: String,
    email: String,
}

#[derive(serde::Deserialize, Debug)]
struct UserLogin {
    id: String,
    pw: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
struct Counter {
    val: isize
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
struct Status {
    error: bool,
    code: usize
}


static mut COUNTER: Counter = Counter{val: 0};


async fn hello_world() -> Html<&'static str> {
    Html("<h1>어썸링님 그는 신이야!</h1>")
}


async fn signup(q: Query<User>) -> Json<Status>{
    let mut data: Vec<User> = serde_json::from_str(&fs::read_to_string("user.json").unwrap()).unwrap();
    if q.id.len() > 20 || q.pw.len() > 20 || q.email.len() > 20  {
        return Json(Status { error: true, code: 1 });
    } 
    for u in data.iter(){
        if u.id == q.id{
            return Json(Status { error: true, code: 2 });
        }
    }
    data.push(User {
        id: q.id.clone(),
        pw: q.pw.clone(),
        email: q.email.clone()
    });

    let mut res = String::new();

    res.push_str("[");
    for (idx, q) in data.iter().enumerate(){
        if idx == data.len()-1{
            res.push_str(&format!("{{\"id\": {:?}, \"pw\": {:?}, \"email\": {:?}}}", q.id, q.pw, q.email));
        }else{
            res.push_str(&format!("{{\"id\": {:?}, \"pw\": {:?}, \"email\": {:?}}},", q.id, q.pw, q.email));
        }
    }
    res.push_str("]");

    println!("{}", res);
    fs::write("user.json", res).unwrap();
    return Json(Status { error: false, code: 0 });

}


async fn login(q: Query<UserLogin>) -> Json<Status>{
    let mut data: Vec<User> = serde_json::from_str(&fs::read_to_string("user.json").unwrap()).unwrap();

    for u in data.iter(){
        if q.id == u.id{
            if q.pw != u.pw {
                return Json(Status{ error: true, code: 1 })
            }else{
                break;
            }
        }
    }
    Json(Status { error: false, code: 0 })
}


async fn counting() -> Json<Counter> {
    unsafe{ COUNTER.val += 1 }
    Json(unsafe { COUNTER })
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "assets")] sf: PathBuf
) -> shuttle_axum::ShuttleAxum {
    println!("{sf:?}");
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/cnt", get(counting))
        .route("/signup", get(signup))
        .route("/login", get(login))
        .nest_service("/static", ServeDir::new(sf))
        ;

    Ok(router.into())
}
