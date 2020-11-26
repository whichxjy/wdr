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
        ZooKeeper::connect(connect_string, Duration::from_secs(15), LoggingWatcher)
            .map(|zk| ZkClient { zk })
    }

    pub fn create(&self, path: &str) -> ZkResult<String> {
        self.zk.create(
            path,
            vec![],
            Acl::open_unsafe().clone(),
            CreateMode::Persistent,
        )
    }

    pub fn exists(&self, path: &str) -> bool {
        match self.zk.exists(path, false) {
            Ok(stat) => stat.is_some(),
            _ => false,
        }
    }

    pub fn ensure(&self, path: &str) -> ZkResult<()> {
        if path.is_empty() || self.exists(path) {
            return Ok(());
        }

        let last_index = path.rfind('/').unwrap_or(0);
        let parent_path = &path[..last_index];

        self.ensure(parent_path)?;
        self.create(path).map(|_| ())
    }

    pub fn delete(&self, path: &str) -> ZkResult<()> {
        self.zk.delete(path, None)
    }

    pub fn get_data(&self, path: &str) -> ZkResult<Vec<u8>> {
        self.zk.get_data(path, false).map(|(data, _)| data)
    }

    pub fn set_data(&self, path: &str, data: Vec<u8>) -> ZkResult<Stat> {
        self.ensure(path).and(self.zk.set_data(path, data, None))
    }

    pub fn get_children(&self, path: &str) -> ZkResult<Vec<String>> {
        self.zk.get_children(path, false)
    }
}
