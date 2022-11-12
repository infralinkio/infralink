use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeTypes {
    Worker,
    LoadBalancer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub node_type: NodeTypes,
    pub data: RequestBody,
}
