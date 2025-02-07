use axum::{routing::post, Router, extract::Json};
use tokio::net::TcpListener;
use reqwest;
use std::net::SocketAddr;
use serde_yaml;
use simple_test::models::{UserJson, UserYaml};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/a", post(process_json));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("A 서버 실행 중: {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

// JSON 데이터를 받고 YAML로 변환한 후 B 서버로 전달하는 핸들러
async fn process_json(Json(mut payload): Json<UserJson>) -> String {
    println!("A 서버에서 받은 JSON 데이터: {:?}", payload);
    payload.history.push("JSON → YAML".to_string());

    let user_yaml = UserYaml {
        name: payload.name,
        age: payload.age,
        history: payload.history,
    };
    let yaml_string = serde_yaml::to_string(&user_yaml).unwrap();

    // B 서버로 전송
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:3001/b")
        .body(yaml_string)
        .send()
        .await
        .unwrap();

    res.text().await.unwrap()
}