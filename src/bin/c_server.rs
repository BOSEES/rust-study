use axum::{routing::post, Router, extract::Request, response::Json as AxumJson};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use serde_xml_rs::from_str as from_xml;
use serde_json;
use axum::body::{Bytes, to_bytes};
use simple_test::models::{UserXml, UserToml};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/c", post(finalize_data));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));
    println!("C 서버 실행 중: {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

// XML 데이터를 받고 TOML로 변환한 후 최종적으로 JSON으로 응답하는 핸들러
async fn finalize_data(req: Request) -> AxumJson<serde_json::Value> {
    let body = to_bytes(req.into_body(), usize::MAX).await.unwrap();
    let xml_data = String::from_utf8(body.to_vec()).unwrap();
    println!("C 서버에서 받은 XML 데이터:\n{}", xml_data);

    let mut user_xml: UserXml = from_xml(&xml_data).unwrap();
    user_xml.history.push("XML → TOML".to_string());

    let user_toml = UserToml {
        name: user_xml.name,
        age: user_xml.age,
        history: user_xml.history,
    };

    // JSON 변환 후 반환
    let json_value = serde_json::json!({
        "name": user_toml.name,
        "age": user_toml.age,
        "history": user_toml.history
    });

    AxumJson(json_value)
}