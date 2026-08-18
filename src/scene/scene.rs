use crate::scene::{
    math::matrix::RowMat,
    renderer::{ camera::Camera, mesh::Mesh },
    scene::NodeData::Empty,
};

use slotmap::{ DefaultKey, SlotMap };

pub type NodeId = DefaultKey;

// Types of nodes
pub enum NodeData {
    Empty,
    Mesh(Mesh),
    Camera(Camera),
}

pub struct NodeProperties {
    position: RowMat<3>,
    rotation: RowMat<4>,
    scale: RowMat<3>,
}
impl Default for NodeProperties {
    fn default() -> Self {
        Self {
            position: RowMat::new(),
            rotation: RowMat::from_data([[1.0, 0.0, 0.0, 0.0]]),
            scale: RowMat::from_data([[1.0, 1.0, 1.0]]),
        }
    }
}

pub struct Node {
    pub id: NodeId,
    pub parent_id: NodeId, // Equal to id if root
    pub props: NodeProperties,
    pub data: NodeData,
    pub children_ids: Vec<NodeId>,
}

impl Node {
    fn is_root(&self) -> bool {
        self.id == self.parent_id
    }

    pub fn translate(&mut self, by: RowMat<3>) {
        self.props.position += by;
    }

    pub fn scale(&mut self, by: RowMat<3>) {
        self.props.scale.apply_to(&by, |a, b| a * b);
    }

    pub fn set_scale(&mut self, scale: RowMat<3>) {
        self.props.scale.clone_from(&scale);
    }

    pub fn rotate(&mut self, axis: RowMat<3>, by: f32) {
        self.props.rotation.rotate_mut(&axis, by);
    }
}

pub struct SceneGraph {
    nodes: SlotMap<NodeId, Node>,
    root: NodeId,
}

impl SceneGraph {
    pub fn new() -> Self {
        let mut nodes = SlotMap::<NodeId, Node>::new();

        let root = nodes.insert_with_key(|id| Node {
            id,
            parent_id: id,
            props: NodeProperties::default(),
            data: Empty,
            children_ids: Vec::new(),
        });

        Self { nodes, root }
    }

    fn create_node_from_data(key: NodeId, parent: NodeId, data: NodeData) -> Node {
        Node {
            id: key,
            parent_id: parent,
            props: NodeProperties::default(),
            data,
            children_ids: Vec::new(),
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn insert(&mut self, data: NodeData) -> NodeId {
        self.nodes.insert_with_key(|key| SceneGraph::create_node_from_data(key, self.root, data))
    }

    pub fn insert_under(&mut self, data: NodeData, under_id: NodeId) -> Option<NodeId> {
        if !self.nodes.contains_key(under_id) {
            return None;
        }

        let child_id = self.nodes.insert_with_key(|key|
            SceneGraph::create_node_from_data(key, under_id, data)
        );

        self.nodes.get_mut(under_id)?.children_ids.push(child_id);

        Some(child_id)
    }

    pub fn delete(&mut self, id: NodeId) -> Result<(), String> {
        if !self.nodes.contains_key(id) {
            return Err(String::from("No such node with specified id"));
        }

        let node = self.nodes.get(id).expect("Unable to get node from scene");
        if node.is_root() {
            return Err(String::from("Cannot remove root from scene tree"));
        }
        let parent_id = node.parent_id;

        if !node.children_ids.is_empty() {
            return Err(String::from("Cannot remove interstitial node"));
        }

        self.nodes.remove(id);

        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.children_ids.retain(|&c| c != id);
        }

        Ok(())
    }
}

pub struct Scene {
    pub graph: SceneGraph,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            graph: SceneGraph::new(),
        }
    }
}
