use std::time::Duration;
use zookeeper::{Acl, CreateMode, Stat, WatchedEvent, Watcher, ZkResult, ZooKeeper};

use crate::config::ZK_CONNECT_STRING;

lazy_static! {
    pub static ref ZK_CLIENT: ZkClient =
        ZkClient::new(&ZK_CONNECT_STRING).expect("Fail to connect to zk");
}

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, event: WatchedEvent) {
        wdr_info!("{:?}", event)
    }
}

pub struct ZkClient {
    zk: ZooKeeper,
}

impl ZkClient {
    pub fn new(connect_string: &str) -> ZkResult<Self> {
        match ZooKeeper::connect(connect_string, Duration::from_secs(15), LoggingWatcher) {
            Ok(zk) => Ok(ZkClient { zk }),
            Err(err) => Err(err),
        }
    }

    pub fn create(&self, path: &str, mode: CreateMode) -> ZkResult<String> {
        self.zk
            .create(path, vec![], Acl::open_unsafe().clone(), mode)
    }

    pub fn exists(&self, path: &str) -> bool {
        match self.zk.exists(path, false) {
            Ok(stat) => stat.is_some(),
            _ => false,
        }
    }

    pub fn get_data(&self, path: &str) -> ZkResult<Vec<u8>> {
        match self.zk.get_data(path, false) {
            Ok((data, _)) => Ok(data),
            Err(err) => Err(err),
        }
    }

    #[allow(unused)]
    pub fn set_data(&self, path: &str, data: Vec<u8>) -> ZkResult<Stat> {
        self.zk.set_data(path, data, None)
    }
}
