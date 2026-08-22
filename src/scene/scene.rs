use std::{ collections::VecDeque, ops::AddAssign };

use crate::scene::{
    math::matrix::{ Matrix, Quaternion, RowMat, SqMat, Transform },
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
    rotation: Quaternion,
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
impl NodeProperties {
    pub fn get_transform(&self) -> Transform {
        let mut r = Transform::scale(self.scale);
        r.extend_reverse_mut(Transform::rotation(self.rotation));
        r.extend_reverse_mut(Transform::translation(self.position));
        r
    }

    pub fn translate(&mut self, by: RowMat<3>) {
        self.position += by;
    }

    pub fn rotate(&mut self, by: f32, axis: &RowMat<3>) {
        self.rotation.rotate_mut(axis, by);
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

pub struct SceneTree {
    nodes: SlotMap<NodeId, Node>,
    root: NodeId,
}

impl SceneTree {
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
        self.insert_under(data, self.root).expect("root always exists")
    }

    pub fn insert_under(&mut self, data: NodeData, under_id: NodeId) -> Option<NodeId> {
        if !self.nodes.contains_key(under_id) {
            return None;
        }

        let child_id = self.nodes.insert_with_key(|key|
            SceneTree::create_node_from_data(key, under_id, data)
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

    pub fn get_mesh_transforms(&self) -> Vec<(&Mesh, Transform)> {
        let mut res = Vec::<(&Mesh, Transform)>::new();

        let mut stack = Vec::<(NodeId, Transform)>::new();
        stack.push((self.root, Transform::default()));

        while let Some((node_id, transform)) = stack.pop() {
            let Some(node_ptr) = self.get(node_id) else {
                continue;
            };

            let next_transform = node_ptr.props.get_transform().extend_forward(transform);

            if let NodeData::Mesh(mesh) = &node_ptr.data {
                res.push((mesh, next_transform));
            }

            for &child_id in &node_ptr.children_ids {
                stack.push((child_id, next_transform));
            }
        }

        return res;
    }

    pub fn get_world_transform_for_node(&self, node_id: NodeId) -> Option<Transform> {
        let node = self.get(node_id)?;
        let mut transform = node.props.get_transform();

        let mut n_t = node;
        while !n_t.is_root() {
            let Some(parent) = self.get(n_t.parent_id) else {
                break;
            };
            n_t = parent;
            transform.extend_forward_mut(n_t.props.get_transform());
        }

        return Some(transform);
    }
}

pub struct Scene {
    pub tree: SceneTree,
    pub active_camera: Option<NodeId>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            tree: SceneTree::new(),
            active_camera: None,
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.tree.get(id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.tree.get_mut(id)
    }

    pub fn insert(&mut self, data: NodeData) -> NodeId {
        if let NodeData::Camera(_) = data {
            let id = self.tree.insert(data);
            self.active_camera = Some(id);
            return id;
        }

        self.tree.insert(data)
    }

    pub fn insert_under(&mut self, data: NodeData, under_id: NodeId) -> Option<NodeId> {
        if let NodeData::Camera(_) = data {
            let id = self.tree.insert_under(data, under_id)?;
            self.active_camera = Some(id);
            return Some(id);
        }
        self.tree.insert_under(data, under_id)
    }

    pub fn delete(&mut self, id: NodeId) -> Result<(), String> {
        self.tree.delete(id)
    }

    pub fn get_active_camera(&self) -> Option<&Node> {
        self.tree.get(self.active_camera?)
    }

    pub fn get_active_camera_mut(&mut self) -> Option<&mut Node> {
        self.tree.get_mut(self.active_camera?)
    }
}
