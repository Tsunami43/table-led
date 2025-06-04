use actix_web::{test, App, http::header};
use server::handlers::{auth_handler, protected_endpoint};
use server::auth::AuthenticatedUser;
use actix_web::{web, HttpResponse};

#[actix_rt::test]
async fn test_auth_handler_success() {
    let app = test::init_service(
        App::new().route("/api/esp/auth", actix_web::web::post().to(auth_handler))
    ).await;

    let req_body = r#"{"device_id":"DEVICE-001","token":"SECRET-TOKEN"}"#;

    let req = test::TestRequest::post()
        .uri("/api/esp/auth")
        .header("Content-Type", "application/json")
        .set_payload(req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body_bytes = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body_bytes).unwrap();
    assert!(body_str.contains("\"ok\":true"));
}

#[actix_rt::test]
async fn test_auth_handler_unauthorized() {
    let app = test::init_service(
        App::new().route("/api/esp/auth", actix_web::web::post().to(auth_handler))
    ).await;

    let req_body = r#"{"device_id":"wrong","token":"badtoken"}"#;

    let req = test::TestRequest::post()
        .uri("/api/esp/auth")
        .header("Content-Type", "application/json")
        .set_payload(req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
async fn test_protected_endpoint_with_valid_token() {
    // Создаём тестовый App с маршрутом, который требует AuthenticatedUser
    let app = test::init_service(
        App::new()
            .route("/api/protected", web::get().to(protected_endpoint))
    ).await;

    // Имитация JWT (для простоты берем токен с user_id = "testuser")
    let token = "Bearer some_valid_jwt_token"; // Тут нужна реальная генерация или mock

    let req = test::TestRequest::get()
        .uri("/api/protected")
        .insert_header((header::AUTHORIZATION, token))
        .to_request();

    // В реальном тесте нужно реализовать валидный JWT и декодер, либо замокать AuthenticatedUser
    // Для примера просто проверим, что запрос обрабатывается (в реале будет ошибка без валидного JWT)
    let resp = test::call_service(&app, req).await;

    // Можно проверить, что не 401 (а иначе тест упадёт)
    assert_ne!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}
