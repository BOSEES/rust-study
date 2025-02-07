use serde::{Serialize, Deserialize};

// JSON 구조체
#[derive(Serialize, Deserialize, Debug)]
pub struct UserJson {
    pub name: String,
    pub age: u8,
    pub history: Vec<String>, // 변환 히스토리 추가
}

// YAML 구조체
#[derive(Serialize, Deserialize, Debug)]
pub struct UserYaml {
    pub name: String,
    pub age: u8,
    pub history: Vec<String>,
}

// XML 구조체
#[derive(Serialize, Deserialize, Debug)]
pub struct UserXml {
    pub name: String,
    pub age: u8,
    pub history: Vec<String>,
}

// TOML 구조체 (최종 반환)
#[derive(Serialize, Deserialize, Debug)]
pub struct UserToml {
    pub name: String,
    pub age: u8,
    pub history: Vec<String>,
}