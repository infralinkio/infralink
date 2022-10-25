use colored::Colorize;
use std::str::FromStr;
use strum::EnumIter;

#[derive(EnumIter, PartialEq, Eq, Debug)]
#[strum(ascii_case_insensitive)]
pub enum NodeType {
    Master,
    Worker,
    LoadBalancer,
}

impl ToString for NodeType {
    fn to_string(&self) -> String {
        match self {
            Self::Master => format!(
                "Master {}",
                "- To manage and coordinate worker nodes.".bright_black()
            ),
            Self::Worker => format!(
                "Worker {}",
                "- To execute a containerized-process in your cluster.".bright_black()
            ),
            Self::LoadBalancer => format!(
                "Load Balancer {}",
                "- To manage and distribute routing between different nodes.".bright_black()
            ),
        }
    }
}

impl FromStr for NodeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut node_type: Self = Self::Master;
        if s.starts_with("Worker") {
            node_type = Self::Worker
        } else if s.starts_with("Load Balancer") {
            node_type = Self::LoadBalancer
        }
        Ok(node_type)
    }
}
