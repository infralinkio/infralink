use bollard::Docker;

#[derive(Clone)]
pub struct State {
    pub docker: Docker,
}
