use serde_json::{Map, Value};
use super::{Request, RequestArguments};

pub enum ActionType {
    Start,
    Stop,
    Verify,
    Reannounce
}

pub enum ActionTarget {
    Single { id: u64 },
    All,
    List { ids: Vec<u64>, hashes: Vec<String> },
    RecentlyActive
}

pub struct TorrentAction {
    typ: ActionType,
    target: ActionTarget
}

impl TorrentAction {
    pub fn new(typ: ActionType, target: ActionTarget) -> TorrentAction {
        TorrentAction {
            typ: typ,
            target: target
        }
    }
}

impl Request for TorrentAction {
    type Response = ::responses::TorrentAction;

    fn method_name(&self) -> &'static str {
        match self.typ {
            ActionType::Start => "torrent-start",
            ActionType::Stop => "torrent-stop",
            ActionType::Verify => "torrent-verify",
            ActionType::Reannounce => "torrent-reannounce"
        }
    }
}

impl RequestArguments for TorrentAction {
    fn arguments(&self) -> Value {
        let mut args = Map::new();

        match self.target {
            ActionTarget::All => (),
            ActionTarget::List { ref ids, ref hashes } => {
                let mut vIds = ids.iter().map(|id| Value::Number((*id).into()));
                let mut vHashes = hashes.iter().map(|hash| Value::String(hash.clone()));
                let list = vIds.chain(vHashes).collect();

                args.insert("id".to_string(), Value::Array(list));
            },
            ActionTarget::RecentlyActive => {
                args.insert("id".to_string(), Value::String("recently-active".to_string()));
            },
            ActionTarget::Single { id } => {
                args.insert("id".to_string(), Value::Number(id.into()));
            }
        }

        Value::Object(args)
    }
}
