use axum::{http, response::IntoResponse, routing::get, Router};
use bytes::Bytes;
use futures::Stream;
use rand::Rng;
use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct SseResponse {
    rx: ReceiverStream<String>,
}

impl SseResponse {
    fn new(rx: ReceiverStream<String>) -> Self {
        Self { rx }
    }
}

impl Stream for SseResponse {
    type Item = Result<Bytes, Infallible>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.rx)
            .poll_next(cx)
            .map(|opt| opt.map(|s| Ok(Bytes::from(s))))
    }
}

impl IntoResponse for SseResponse {
    fn into_response(self) -> axum::response::Response {
        let stream = self;
        let body = axum::body::Body::from_stream(stream);

        http::Response::builder()
            .header("Content-Type", "text/event-stream")
            .header("Cache-Control", "no-cache")
            .header("Connection", "keep-alive")
            .body(body)
            .unwrap()
    }
}

const GREETINGS: &[&str] = &[
    "Hello there! 👋",
    "Greetings from Goose! 🪿",
    "Hey! How's it going? 😊",
    "Welcome back! 🎉",
    "Good to see you! ✨",
    "Howdy! 🤠",
    "Hi friend! 💙",
    "What's up? 🚀",
    "Salutations! 🎩",
    "Yo! 🔥",
];

fn get_random_greeting() -> String {
    let mut rng = rand::rng();
    let index = rng.random_range(0..GREETINGS.len());
    GREETINGS[index].to_string()
}

#[utoipa::path(
    get,
    path = "/greeting",
    responses(
        (status = 200, description = "SSE stream of greeting messages", content_type = "text/event-stream"),
        (status = 401, description = "Unauthorized - invalid secret key")
    )
)]
pub async fn greeting_stream() -> SseResponse {
    let (tx, rx) = mpsc::channel(100);
    let stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            let greeting = get_random_greeting();
            let message = format!("data: {}\n\n", greeting);

            if tx.send(message).await.is_err() {
                tracing::info!("Greeting stream client disconnected");
                break;
            }
        }
    });

    SseResponse::new(stream)
}

pub fn routes() -> Router {
    Router::new().route("/greeting", get(greeting_stream))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_greeting() {
        let greeting = get_random_greeting();
        assert!(GREETINGS.contains(&greeting.as_str()));
    }
}
