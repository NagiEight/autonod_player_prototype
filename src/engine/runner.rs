use crate::engine::node::Node;
use crate::engine::types::Workflow;
use crate::engine::utils::check_process_running;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub fn run_workflow_from_file(path: &str) {
    let file = File::open(path).expect("Failed to open workflow file");
    let reader = BufReader::new(file);

    // Step 1: Read raw JSON for inspection
    let raw_json: Value = serde_json::from_reader(reader).expect("Invalid JSON format");
    println!("Raw JSON loaded:\n{:#}", raw_json);

    // Step 2: Deserialize into Workflow structs
    let workflow: Vec<Workflow> = serde_json::from_value(raw_json).expect("Deserialization failed");

    // Step 3: Run the workflow
    run_workflow(workflow);
}

pub fn run_workflow(workflow: Vec<Workflow>) {
    use std::collections::HashMap;

    let node_map: HashMap<u32, &Workflow> = workflow.iter().map(|n| (n.id, n)).collect();
    let mut current_id = 1;

    loop {
        let node = match node_map.get(&current_id) {
            Some(n) => n,
            None => {
                println!("Node {} not found", current_id);
                break;
            }
        };

        match &node.node {
            Node::Start { .. } => println!("→ Node {}: Start", node.id),
            Node::CheckTask { data } => println!(
                "→ Node {}: CheckTask (process: '{}')",
                node.id,
                data.process.as_deref().unwrap_or("<none>")
            ),
            Node::LaunchApp { data } => println!(
                "→ Node {}: LaunchApp (path: '{}', args: '{}')",
                node.id,
                data.path.as_deref().unwrap_or("<none>"),
                data.args.as_deref().unwrap_or("")
            ),
            Node::End { .. } => println!("→ Node {}: End", node.id),
            Node::Unknown => println!("→ Node {}: Unknown", node.id),
        }

        match &node.node {
            Node::Start { .. } => {
                current_id = node.connections.out.get(0).copied().unwrap_or_else(|| {
                    //println!("No outgoing connection from Start node");
                    0
                });
            }
            Node::CheckTask { data } => {
                let proc_name = data.process.as_deref().unwrap_or("");
                //println!("Checking if process '{}' is running...", proc_name);
                let is_running = check_process_running(proc_name);
                current_id = if is_running {
                    node.connections.out.get(1).copied().unwrap_or(0)
                } else {
                    node.connections.out.get(0).copied().unwrap_or(0)
                };
            }

            Node::LaunchApp { data } => {
                let path = data.path.as_deref().unwrap_or("<missing>");
                let args = data.args.as_deref().unwrap_or("");
                //println!("Launching '{}' with args: '{}'", path, args);
                current_id = node.connections.out.get(0).copied().unwrap_or(0);
            }

            Node::End { .. } => {
                //println!("Workflow ended at node {}", node.id);
                break;
            }
            Node::Unknown => {
                //println!("Unknown node type at node {}", node.id);
                break;
            }
        }

        if current_id == 0 {
            println!("Terminating due to missing connection");
            break;
        }
    }
}
