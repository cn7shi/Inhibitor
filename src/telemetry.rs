use std::io;
use tokio::sync::broadcast;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use axum::{
    routing::{get, post},
    Router,
    response::sse::{Event as SseEvent, Sse},
};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use std::convert::Infallible;
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct ChannelWriter {
    pub sender: broadcast::Sender<String>,
}

impl io::Write for ChannelWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Ok(s) = std::str::from_utf8(buf) {
            
            let _ = self.sender.send(s.to_string());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for ChannelWriter {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        self.clone()
    }
}

pub fn setup_tracing() -> broadcast::Sender<String> {
    let (tx, _) = broadcast::channel(1024);
    let channel_writer = ChannelWriter { sender: tx.clone() };

    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_writer(std::io::stdout.and(channel_writer))
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    tx
}

pub async fn start_server(tx: broadcast::Sender<String>) {
    let app = Router::new()
        .route("/api/logs", get(sse_handler))
        .route("/api/config", get(get_config_handler).post(update_config_handler))
        .route("/proxy/v1/chat/completions", post(crate::gateway::proxy_handler))
        .layer(CorsLayer::permissive())
        .with_state(tx);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Web server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn sse_handler(
    axum::extract::State(tx): axum::extract::State<broadcast::Sender<String>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<SseEvent, Infallible>>> {
    let rx = tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|res| {
            match res {
                Ok(msg) => Some(Ok(SseEvent::default().data(msg))),
                Err(_) => None,
            }
        });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}

// 获取当前配置
async fn get_config_handler() -> axum::response::Result<axum::Json<crate::config::Config>, (axum::http::StatusCode, String)> {
    match crate::config::Config::load() {
        Ok(cfg) => Ok(axum::Json(cfg)),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

// 更新配置并保存到文件
async fn update_config_handler(
    axum::extract::Json(payload): axum::extract::Json<crate::config::Config>,
) -> axum::response::Result<&'static str, (axum::http::StatusCode, String)> {
    match payload.save() {
        Ok(_) => {
            tracing::info!("配置已通过 API 热更新");
            Ok("Config updated successfully")
        }
        Err(e) => {
            tracing::error!("保存配置失败: {}", e);
            Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))
        }
    }
}

