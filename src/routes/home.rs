use axum::response::Html;

pub async fn home() -> Html<& 'static str> {
    Html("hello world")
}
