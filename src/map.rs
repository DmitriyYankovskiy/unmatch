use std::collections::HashSet;

use raph::Graph;

#[derive(Debug, Clone)]
struct NodeState {
    area: HashSet<u32>,
}

type EdgeState = ();

pub struct Map {
    graph: Graph<NodeState, EdgeState>,
}