
use actix_web::{test, App};
use your_crate::handlers::get_games;  // замени на правильный путь
use your_crate::auth::AuthenticatedUser;
use actix_web::http::header;

#[actix_web::test]
async fn test_get_games_authorized() {
    // Создаём приложение с маршрутом /games и мидлварой авторизации
    let app = test::init_service(
        App::new()
            .route("/games", actix_web::web::get().to(get_games))
    ).await;

    // Формируем запрос с заголовком авторизации
    let req = test::TestRequest::get()
        .uri("/games")
        .insert_header((header::AUTHORIZATION, "Bearer jwt_token_example"))
        .to_request();

    // Выполняем запрос
    let resp = test::call_service(&app, req).await;

    // Проверяем, что статус 200 OK
    assert!(resp.status().is_success());

    // Если нужно, можно десериализовать тело и проверить содержимое
    let body = test::read_body(resp).await;
    let games_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Пример проверки, что пришёл список игр (массив)
    assert!(games_json.is_array());
}

