use std::{collections::HashMap, fmt::Debug, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{stream::SplitSink, SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{json, value::RawValue};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::{app_state::AppState, diagnostics};

type Tx = Arc<Mutex<SplitSink<WebSocket, Message>>>;

#[derive(Clone, Debug)]
pub(crate) struct PubSubState {
    topics: Arc<RwLock<HashMap<String, Topic>>>,
}
impl PubSubState {
    pub(crate) fn new() -> Self {
        Self {
            topics: Arc::new(RwLock::new(HashMap::default())),
        }
    }
}

#[derive(Debug)]
struct Topic {
    topic: String,
    subscribers: HashMap<Uuid, Subscriber>,
}

impl Topic {
    pub fn new(topic: String) -> Self {
        Self {
            topic,
            subscribers: HashMap::default(),
        }
    }
}

#[derive(Debug)]
struct Subscriber {
    uuid: Uuid,
    tx: Tx,
}

impl Subscriber {
    fn new(uuid: Uuid, tx: Arc<Mutex<SplitSink<WebSocket, Message>>>) -> Self {
        Self { uuid, tx }
    }
}

#[derive(Deserialize, Debug)]
struct Packet<'a> {
    op: &'a str,
    data: &'a RawValue,
}

#[derive(Deserialize, Debug)]
struct Subscribe<'a> {
    topic: &'a str,
}

#[derive(Deserialize, Debug)]
struct Publish<'a> {
    topic: &'a str,
    message: &'a str,
}

#[derive(Deserialize, Debug)]
struct Cancel<'a> {
    topic: &'a str,
}

async fn on_error<E>(tx: &'_ Tx, error: E)
where
    E: Debug,
{
    let _ = tx
        .lock()
        .await
        .send(Message::Text(format!("error : {error:?}",)))
        .await;
}

// async fn pubsub_handler(mut websocket: WebSocket, state: PubSubState) {
//     let (tx, mut rx) = websocket.split();
//     while let Some(Ok(message)) = rx.next().await {
//         match message {
//             _ => tracing::debug!("message")
//         }
//     }
//     tracing::debug!("???");
// }

async fn pubsub_handler(websocket: WebSocket, state: PubSubState) {
    let (tx, mut rx) = websocket.split();
    let tx = Arc::new(Mutex::new(tx));
    let uuid = Uuid::new_v4();
    while let Some(Ok(message)) = rx.next().await {
        match message {
            Message::Text(text) => {
                tracing::debug!(text);
                let packet = match serde_json::from_str::<Packet>(&text) {
                    Ok(v) => v,
                    Err(error) => {
                        on_error(&tx, error).await;
                        break;
                    }
                };
                match packet.op {
                    "subscribe" => {
                        let subscribe = match serde_json::from_str::<Subscribe>(packet.data.get()) {
                            Ok(v) => v,
                            Err(error) => {
                                on_error(&tx, error).await;
                                break;
                            }
                        };
                        {
                            let mut topics = state.topics.write().await;
                            let topic = topics
                                .entry(subscribe.topic.to_owned())
                                .or_insert_with(|| Topic::new(subscribe.topic.to_owned()));
                            let _ = topic
                                .subscribers
                                .entry(uuid.clone())
                                .or_insert_with(|| Subscriber::new(uuid.clone(), tx.clone()));
                        }
                    }
                    "publish" => {
                        let publish = match serde_json::from_str::<Publish>(packet.data.get()) {
                            Ok(v) => v,
                            Err(error) => {
                                on_error(&tx, error).await;
                                break;
                            }
                        };
                        let topics = state.topics.read().await;
                        match topics.get(publish.topic) {
                            Some(topic) => {
                                for (_k, v) in topic.subscribers.iter() {
                                    v.tx.lock()
                                        .await
                                        .send(Message::Text(
                                            json!({ "topic": publish.topic.to_owned(),
                                                    "message": publish.message.to_owned() })
                                            .to_string(),
                                        ))
                                        .await
                                        .unwrap();
                                }
                            }
                            _ => {}
                        }
                    }
                    "cancel" => {
                        let cancel = match serde_json::from_str::<Cancel>(packet.data.get()) {
                            Ok(v) => v,
                            Err(error) => {
                                on_error(&tx, error).await;
                                break;
                            }
                        };
                        let mut topics = state.topics.write().await;
                        match topics.get_mut(cancel.topic) {
                            Some(topic) => {
                                topic.subscribers.remove(&uuid);
                                if topic.subscribers.is_empty() {
                                    topics.remove(cancel.topic);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        on_error(
                            &tx,
                            diagnostics::Error::Message("Unknown message".to_owned()),
                        )
                        .await;
                        break;
                    }
                }
            }
            Message::Binary(_bin) => {}
            Message::Close(_close) => {
                break;
            }
            Message::Ping(_ping) => {
                let _ = tx.lock().await.send(Message::Pong("Pong".into())).await;
                ()
            }
            Message::Pong(_pong) => {
                let _ = tx.lock().await.send(Message::Pong("ping".into())).await;
                ()
            }
        }
    }
    {
        let mut topics = state.topics.write().await;
        for (_, topic) in topics.iter_mut() {
            match topic.subscribers.get(&uuid) {
                Some(_) => {
                    topic.subscribers.remove(&uuid);
                }
                _ => {}
            }
        }
        topics.retain(|_, v| !v.subscribers.is_empty())
    }
}

#[cfg(feature = "enable_websocket_pubsub_sample")]
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    let s = state.pubsub.clone();
    ws.on_upgrade(move |socket| pubsub_handler(socket, s))
}

#[cfg(feature = "enable_websocket_pubsub_sample")]
pub(crate) fn router() -> Router<AppState> {
    Router::new().route("/", get(ws_handler))
}
