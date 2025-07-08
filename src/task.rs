#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub deps: Vec<String>,
    pub commands: Vec<String>,
}
