use std::time::Duration;
use zookeeper::{Acl, CreateMode, Stat, WatchedEvent, Watcher, ZkResult, ZooKeeper};

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, event: WatchedEvent) {
        fn_info!("{:?}", event)
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

    pub fn set_data(&self, path: &str, data: Vec<u8>) -> ZkResult<Stat> {
        self.zk.set_data(path, data, None)
    }
}
