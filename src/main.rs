mod engine;

fn main() {
    engine::runner::run_workflow_from_file("workflows/example.json");
}
