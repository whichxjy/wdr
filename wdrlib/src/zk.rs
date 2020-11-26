use std::time::Duration;
pub use zookeeper::CreateMode;
use zookeeper::{Acl, Stat, WatchedEvent, Watcher, ZkResult, ZooKeeper};

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

    fn create(&self, path: &str, mode: CreateMode) -> ZkResult<String> {
        self.zk
            .create(path, vec![], Acl::open_unsafe().clone(), mode)
    }

    fn exists(&self, path: &str) -> bool {
        match self.zk.exists(path, false) {
            Ok(stat) => stat.is_some(),
            _ => false,
        }
    }

    pub fn ensure(&self, path: &str, mode: CreateMode) -> ZkResult<()> {
        fn ensure_recur(
            zk_client: &ZkClient,
            path: &str,
            mode: Option<CreateMode>,
        ) -> ZkResult<()> {
            if path.is_empty() || zk_client.exists(path) {
                return Ok(());
            }

            let last_index = path.rfind('/').unwrap_or(0);
            let parent_path = &path[..last_index];

            ensure_recur(zk_client, parent_path, None)?;

            match mode {
                Some(mode) => zk_client.create(path, mode).map(|_| ()),
                None => zk_client.create(path, CreateMode::Persistent).map(|_| ()),
            }
        }

        ensure_recur(self, path, Some(mode))
    }

    pub fn delete(&self, path: &str) -> ZkResult<()> {
        self.zk.delete(path, None)
    }

    pub fn get_data(&self, path: &str) -> ZkResult<Vec<u8>> {
        self.zk.get_data(path, false).map(|(data, _)| data)
    }

    pub fn set_data(&self, path: &str, data: Vec<u8>, mode: CreateMode) -> ZkResult<Stat> {
        self.ensure(path, mode)
            .and(self.zk.set_data(path, data, None))
    }

    pub fn get_children(&self, path: &str) -> ZkResult<Vec<String>> {
        self.zk.get_children(path, false)
    }
}
