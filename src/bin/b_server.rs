use axum::{routing::post, Router, extract::Request};
use tokio::net::TcpListener;
use serde_yaml;
use serde_xml_rs::to_string as to_xml;
use std::net::SocketAddr;
use reqwest;
use axum::body::{Bytes, to_bytes};
use simple_test::models::{UserYaml, UserXml};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/b", post(convert_to_xml));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("B 서버 실행 중: {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

// YAML 데이터를 받고 XML로 변환한 후 C 서버로 전달하는 핸들러
async fn convert_to_xml(req: Request) -> String {
    let body = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let yaml_data = String::from_utf8(body.to_vec()).unwrap();
    println!("B 서버에서 받은 YAML 데이터:\n{}", yaml_data);

    let mut user_yaml: UserYaml = serde_yaml::from_str(&yaml_data).unwrap();
    user_yaml.history.push("YAML → XML".to_string());

    let user_xml = UserXml {
        name: user_yaml.name,
        age: user_yaml.age,
        history: user_yaml.history,
    };
    let xml_string = to_xml(&user_xml).unwrap();  // ✅ XML 변환

    // C 서버로 전송
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:3002/c")
        .body(xml_string)
        .send()
        .await
        .unwrap();

    res.text().await.unwrap()
}