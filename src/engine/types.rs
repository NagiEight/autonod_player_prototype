use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub id: u32,
    #[serde(flatten)]
    pub node: crate::engine::node::Node,
    pub position: Position,
    pub connections: Connections,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Deserialize)]
pub struct Connections {
    #[serde(default)]
    pub r#in: Vec<u32>,
    #[serde(default)]
    pub out: Vec<u32>,
}
