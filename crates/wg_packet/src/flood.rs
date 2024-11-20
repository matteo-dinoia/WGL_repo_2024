use wg_network::{NodeId, SourceRoutingHeader};

pub enum NodeType {
    Client,
    Drone,
    Server,
}

pub struct Query {
    pub flood_id: u64,
    pub initiator_id: NodeId,
    pub ttl: u8,
    pub path_trace: Vec<(NodeId, NodeType)>,
}

pub struct QueryResult {
    pub flood_id: u64,
    pub source_routing_header: SourceRoutingHeader,
    pub path_trace: Vec<(NodeId, NodeType)>,
}
