pub struct MultiplayerSession {
    session_type : GameSessionType,
}

pub enum GameSessionType {
    Local,
    LAN,
    Internet
}