use std::{collections::HashSet, ops::{Deref, DerefMut}};

use raph::Graph;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct NodeState{
    area: u32,
}

#[derive(Clone)]
pub struct MovableObject<T: Clone> {
    pos: raph::Pos<NodeState, EdgeState>,
    object: T,
}

impl<T: Clone> MovableObject<T>{
    fn set_to(&mut self, idx: raph::Idx) {
        self.pos.set_to(idx);
    }

    fn get_idx(&self) -> raph::Idx {
        raph::Pos::get_me(&self.pos)
    }
}

impl<T: Clone> Deref for MovableObject<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T: Clone> DerefMut for MovableObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct EdgeState {
    area: u32,
}

pub struct Map {
    graph: Graph<NodeState, EdgeState>,
}