mod get_torrent;
mod add_torrent;
mod torrent_set;
mod torrent_action;

pub use self::get_torrent::GetTorrent;
pub use self::add_torrent::AddTorrent;
pub use self::torrent_set::TorrentSet;
pub use self::torrent_action::{ActionType, ActionTarget, TorrentAction};

use serde_json::{self, Map, Value};
use serde::Serialize;

pub trait RequestArguments {
    fn arguments(&self) -> Value;
}

impl<T> RequestArguments for T where T: Serialize {
    fn arguments(&self) -> Value {
        // FIXME: fix the use of unwrap
        serde_json::to_value(self).unwrap()
    }
}

/// A trait representing a request sent to the daemon.
pub trait Request: RequestArguments {
    type Response;

    fn method_name(&self) -> &'static str;

    fn to_value(&self) -> Value {
        Value::Object({
            let mut obj = Map::new();
            obj.insert("method".to_string(), Value::String(self.method_name().to_string()));
            obj.insert("arguments".to_string(), self.arguments());
            obj.into()
        })
    }
}
