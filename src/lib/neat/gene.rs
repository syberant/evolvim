pub type Id = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Sensor,
    Hidden,
    Output,
}

#[derive(Clone)]
pub struct NodeGene {
    pub node_type: NodeType,
    pub id: Id,
}

#[derive(Clone)]
pub struct ConnectionGene {
    pub from: Id,
    pub to: Id,
    pub weight: f64,

    pub enabled: bool,
    pub innovation_number: usize,
}

impl NodeGene {
}

impl ConnectionGene {
    pub fn disable_and_info(&mut self) -> (Id, Id) {
        self.enabled = false;

        return (self.from, self.to);
    }

    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }
}