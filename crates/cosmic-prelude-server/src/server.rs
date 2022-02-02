use actionable::{Permissions, ResourceName};
use bonsaidb::{
    core::{
        async_trait::async_trait,
        connection::{SensitiveString, StorageConnection},
        document::CollectionDocument,
        permissions::Dispatcher,
        schema::SerializedCollection,
    },
    server::{
        Backend, BackendError, ConnectedClient, CustomApiDispatcher, CustomServer, ServerDatabase,
    },
};
use cosmic_prelude_shared::{
    api::{
        Api, CreateCharacterHandler, Error, Prelude, RegisterHandler, Request, RequestDispatcher,
        Response,
    },
    db::{character::Character, PreludeSchema},
};

#[derive(Debug, Dispatcher)]
#[dispatcher(input = Request)]
pub struct PreludeServer {
    server: CustomServer<Self>,
    client: ConnectedClient<Self>,
}

impl RequestDispatcher for PreludeServer {
    type Output = Response;
    type Error = BackendError<Error>;
}

impl PreludeServer {
    const DATABASE_NAME: &'static str = "prelude";

    pub async fn database(&self) -> Result<ServerDatabase<Self>, bonsaidb::core::Error> {
        self.server
            .database::<PreludeSchema>(Self::DATABASE_NAME)
            .await
    }
}

#[async_trait]
impl Backend for PreludeServer {
    type CustomApi = Api;
    type ClientData = Player;
    type CustomApiDispatcher = Self;

    async fn initialize(server: &CustomServer<Self>) {
        // TODO this should be real error handling.
        server
            .create_database::<PreludeSchema>(Self::DATABASE_NAME, true)
            .await
            .unwrap();
    }
}

#[derive(Debug)]
pub struct Player {
    character: CollectionDocument<Character>,
}

impl CustomApiDispatcher<Self> for PreludeServer {
    fn new(server: &CustomServer<Self>, client: &ConnectedClient<Self>) -> Self {
        Self {
            server: server.clone(),
            client: client.clone(),
        }
    }
}

#[async_trait]
impl RegisterHandler for PreludeServer {
    async fn handle(
        &self,
        _permissions: &Permissions,
        username: String,
        password: SensitiveString,
    ) -> Result<Response, BackendError<Error>> {
        let user_id = self.server.create_user(&username).await?;
        self.server.set_user_password(user_id, password).await?;
        self.server
            .authenticate_client_as(user_id, &self.client)
            .await?;

        Ok(Response::Ok)
    }
}

#[async_trait]
impl CreateCharacterHandler for PreludeServer {
    type Action = Prelude;

    async fn resource_name<'a>(
        &'a self,
        _character_name: &'a String,
    ) -> Result<ResourceName<'a>, BackendError<Error>> {
        Ok(ResourceName::named("prelude"))
    }

    fn action() -> Self::Action {
        Prelude::CreateCharacter
    }

    async fn handle_protected(
        &self,
        _permissions: &Permissions,
        character_name: String,
    ) -> Result<Response, BackendError<Error>> {
        let db = self.database().await?;
        let user_id = self
            .client
            .user_id()
            .await
            .expect("must be authed to get here");
        let character = Character {
            user_id,
            name: character_name,
        }
        .push_into(&db)
        .await
        .map_err(|err| {
            if matches!(err.error, bonsaidb::core::Error::UniqueKeyViolation { .. }) {
                BackendError::Backend(Error::CharacterNameTaken)
            } else {
                BackendError::from(err)
            }
        })?;

        self.client.set_client_data(Player { character }).await;

        Ok(Response::Ok)
    }
}
