# Serialization and HTTP Requests Test
**Rust + Axum**을 사용하여 데이터를 여러 형식(JSON → YAML → XML → TOML)으로 변환하는 다단계 테스트 서버

## **1. 프로젝트 구조**
- **A 서버**: 클라이언트로부터 JSON 데이터를 받아 **YAML**로 변환 후 B 서버로 전송
- **B 서버**: YAML 데이터를 받아 **XML**로 변환 후 C 서버로 전송
- **C 서버**: XML 데이터를 받아 **TOML**로 변환 후 최종적으로 **JSON**으로 클라이언트에게 반환

---

## **2. 실행 방법**
### **1️⃣ 프로젝트 빌드**
```sh
// 터미널 3개 띄우고서 각각  

//터미널 1
$ cargo run --bin a_server

//터미널 2
$ cargo run --bin b_server

//터미널 3
$ cargo run --bin c_server
```

## **3. 요청 방법**
```
curl -X POST "http://127.0.0.1:3000/a" \
     -H "Content-Type: application/json" \
     -d '{"name":"bosees","age":32,"history":[]}'
```

```
//정상 응답,
{"age":31,"history":["JSON → YAML","YAML → XML","XML → TOML"],"name":"bosees"}
```
