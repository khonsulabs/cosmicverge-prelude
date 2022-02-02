use actionable::{Permissions, Statement};
use bonsaidb::{
    cli::CommandLine,
    core::{
        async_trait::async_trait,
        connection::StorageConnection,
        permissions::bonsai::{BonsaiAction, ServerAction},
    },
    local::config::Builder,
    server::{DefaultPermissions, ServerConfiguration},
    AnyServerConnection,
};
use clap::Subcommand;
use cosmic_prelude_shared::{api::Prelude, db::PreludeSchema};

use crate::server::PreludeServer;

#[async_trait]
impl CommandLine for ServerCli {
    type Backend = PreludeServer;
    type Subcommand = Cli;

    async fn configuration(&mut self) -> anyhow::Result<ServerConfiguration> {
        Ok(ServerConfiguration::new("cosmic-prelude.bonsaidb")
            .default_permissions(DefaultPermissions::Permissions(Permissions::from(
                Statement::for_any().allowing(&BonsaiAction::Server(ServerAction::Connect)),
            )))
            .authenticated_permissions(DefaultPermissions::Permissions(Permissions::from(
                Statement::for_any().allowing(&Prelude::CreateCharacter),
            )))
            .with_schema::<PreludeSchema>()?)
    }

    async fn execute(
        &mut self,
        command: Self::Subcommand,
        connection: AnyServerConnection<PreludeServer>,
    ) -> anyhow::Result<()> {
        let database = connection.database::<PreludeSchema>("prelude").await?;
        match command {}
    }
}

#[derive(Subcommand, Debug)]
pub enum Cli {}

pub struct ServerCli;
