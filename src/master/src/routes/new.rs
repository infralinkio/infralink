use crate::models::new::{NodeTypes, Request};
use crate::state::State;
use axum::{Extension, Json};
use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use futures_util::TryStreamExt;
use std::collections::HashMap;

pub async fn new(request: Json<Request>, state: Extension<State>) -> &'static str {
    let docker = state.docker.clone();

    return match &request.node_type {
        NodeTypes::Worker => {
            // pull the image from docker hub and run it
            docker
                .create_image(
                    Some(CreateImageOptions {
                        from_image: "varunpotti/fleet-docs",
                        ..Default::default()
                    }),
                    None,
                    None,
                )
                .try_collect::<Vec<_>>()
                .await
                .unwrap();

            let empty = HashMap::<(), ()>::new();
            let mut exposed_ports = HashMap::new();
            let exposed_port = "4173/tcp";
            exposed_ports.insert(exposed_port, empty);

            let config = Config {
                image: Some("varunpotti/fleet-docs"),
                tty: Some(true),
                attach_stdin: Some(true),
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                open_stdin: Some(true),
                exposed_ports: Some(exposed_ports),
                ..Default::default()
            };

            let options = Some(CreateContainerOptions {
                name: "fleet-docs".to_string(),
            });

            let container = docker.create_container(options, config).await.unwrap();
            // forward the port to the host

            docker
                .start_container::<String>(&container.id, None)
                .await
                .unwrap();

            // somethings not being imported
            "worker"
        }
        NodeTypes::LoadBalancer => "load balancer",
    };
}
