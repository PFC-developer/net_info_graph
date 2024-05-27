use std::collections::HashMap;

use dotenv::dotenv;

use crate::out::NodeInfoOut;
use crate::types::{NetInfo, NetResult};

mod types;
mod error;
mod out;

fn main() {
    dotenv().ok(); // this fails if .env isn't present. It is safe to be ignored
    env_logger::init();
    if let Err(ref err) = run() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));
    }
}

pub fn run() -> anyhow::Result<()> {
    let mut nodes: HashMap<String, NodeInfoOut> = HashMap::new();
    let in_file = std::fs::read_to_string("./sheets.csv")?;
    let mut first = true;
    for line in in_file.split("\n") {
        if first {
            first = false
        } else {
            if !line.trim().is_empty() {
                let fields = line.split(",").collect::<Vec<&str>>();
                if fields.len() != 5 {
                    log::error!("Bad Line {}", line);
                } else {
                    parse_gist(&mut nodes, fields[2], fields[4])?;
                }
            }
        }

    }
   let in_nodes = nodes.iter().filter(|x| x.1.seen_cnt > 0).count();
    eprintln!("{}/{} nodes found", in_nodes, nodes.len());
    println!("digraph nodes {{");
    for k in nodes {

        let node_out = k.1;
        for n in node_out.outbound_peers {
            println!("\t\"{}\" -> \"{}\";", node_out.node_id,n);
        }
      //  println!("{} - {} {}/{}/{}", k.0, node_out.seen_cnt, node_out.in_peers.len(), node_out.outbound_peers.len(), node_out.nonoutbound_peers.len())
    }
    println!("}}");
    Ok(())
}

pub fn parse_gist(nodes: &mut HashMap<String, NodeInfoOut>, id: &str, url: &str) -> anyhow::Result<()> {
   log::info!("URL: {}", url);
    let response = reqwest::blocking::get(url)?;

    let json: NetResult<NetInfo> = response.json()?;
    // let json: NetResult<NetInfo> = serde_json::from_str(&data)?;
    let net_info = json.result;


    let n_peers = net_info.n_peers.parse::<u64>()?;
    nodes.entry(id.to_string()).and_modify(|x| {
        x.seen_cnt += 1;
        x.n_peers = n_peers
    }).or_insert(NodeInfoOut {
        n_peers,
        node_id: id.to_string(),
        seen_cnt: 1,
        in_peers: Default::default(),
        outbound_peers: Default::default(),
        nonoutbound_peers: Default::default(),
    });
    for node in net_info.peers {
        nodes.entry(node.node_info.id.clone()).and_modify(|x| {
            if node.is_outbound {
                x.outbound_peers.insert(id.to_string());
            } else {
                x.nonoutbound_peers.insert(id.to_string());
            }
        }).or_insert({
            let mut node_new = NodeInfoOut {
                n_peers: 0,
                node_id: node.node_info.id.to_string(),
                seen_cnt: 0,
                in_peers: Default::default(),
                outbound_peers: Default::default(),
                nonoutbound_peers: Default::default(),
            };
            if node.is_outbound {
                node_new.outbound_peers.insert(id.to_string());
            } else {
                node_new.nonoutbound_peers.insert(id.to_string());
            }
            node_new
        });
        nodes.entry(id.to_string()).and_modify(|x| { x.in_peers.insert(node.node_info.id); });
    }
    Ok(())
}