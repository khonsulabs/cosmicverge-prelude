use bonsaidb::cli::CommandLine;
use cosmic_prelude_server::cli::ServerCli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    ServerCli.run().await
}
