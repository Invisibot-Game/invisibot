use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum GameMessage {
    ClientHello(String),
    ClientGoodbye(String),
}
