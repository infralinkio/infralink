use strum::{Display, EnumIter, EnumString};

#[derive(EnumString, EnumIter, PartialEq, Eq, Display)]
#[strum(ascii_case_insensitive)]
pub enum NodeType {
    Master,
    Worker,
    LoadBalancer,
}
