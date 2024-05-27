use serde::Deserialize;

#[derive(Deserialize)]
pub struct NetResult<T> {
    pub result: T,
}

#[derive(Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub listen_addr: String,
    pub moniker: String,

}
#[derive(Deserialize)]
pub struct Peer {
    pub node_info: NodeInfo,
    pub is_outbound: bool
}

#[derive(Deserialize)]
pub struct NetInfo {
    pub listening: bool,
    pub n_peers: String,
    pub peers: Vec<Peer>,
}

