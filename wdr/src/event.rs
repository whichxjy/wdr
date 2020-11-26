use crossbeam::channel::Receiver;
use std::collections::HashMap;
use wdrlib::info::{NodeInfo, ProcessInfo};
use wdrlib::zk::CreateMode;
use wdrlib::zk_node_info_path;

use crate::manager::ZK_CLIENT;
use crate::setting::get_wdr_node_name;

pub fn listen_event(process_info_receiver: Receiver<ProcessInfo>) {
    // process name => process info
    let mut process_info_map: HashMap<String, ProcessInfo> = HashMap::new();

    loop {
        let process_info = process_info_receiver.recv().unwrap();
        process_info_map.insert(process_info.name.to_owned(), process_info);

        // Convert process info map to process info list.
        let process_info_list = process_info_map
            .iter()
            .map(|(_, pi)| pi.to_owned())
            .collect::<Vec<ProcessInfo>>();

        // Write node info to zk.
        let node_info = NodeInfo { process_info_list };
        let data = serde_json::to_string(&node_info).unwrap();

        match ZK_CLIENT.set_data(
            &zk_node_info_path!(get_wdr_node_name()),
            data.as_bytes().to_vec(),
            CreateMode::Ephemeral,
        ) {
            Err(err) => fn_error!("Fail to write node info: {}", err),
            _ => fn_debug!("Write node info: {}", data),
        }
    }
}
