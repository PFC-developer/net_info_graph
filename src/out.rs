use std::collections::HashSet;

pub struct NodeInfoOut {
    pub n_peers: u64,
    pub node_id: String,
    pub seen_cnt: u64,
    pub in_peers: HashSet<String>,
    pub outbound_peers: HashSet<String>,
    pub nonoutbound_peers: HashSet<String>,
}
