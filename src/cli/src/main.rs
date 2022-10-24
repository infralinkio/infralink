pub mod commands;
pub mod parser;
pub mod setup;

use commands::Process;

#[tokio::main]
async fn main() {
    setup::setup();

    let arg = parser::Argument::from_cli().await;

    let process = Process::from_args(arg).await;

    process.route().await;
}
