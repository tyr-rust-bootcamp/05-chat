use axum::response::IntoResponse;

pub(crate) async fn send_message_handler() -> impl IntoResponse {
    "send message"
}

pub(crate) async fn list_message_handler() -> impl IntoResponse {
    "list message"
}
