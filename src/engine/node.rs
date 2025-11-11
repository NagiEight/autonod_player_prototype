use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Node {
    Start {
        #[serde(default)]
        data: Option<NodeData>,
    },
    CheckTask {
        data: NodeData,
    },
    LaunchApp {
        data: NodeData,
    },
    End {
        #[serde(default)]
        data: Option<NodeData>,
    },
    #[serde(other)]
    Unknown,
}
#[derive(Debug, Deserialize)]
pub struct NodeData {
    #[serde(default)]
    pub process: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub args: Option<String>,
}
