use async_trait::async_trait;
use bpp_command_api::{CommandError, export_command, structs::Message, traits::{Command, CommandRegistrar, YouTubeSendable}, userservice::user_service_client::UserServiceClient};
use tonic::transport::Channel;

#[derive(Clone)]
pub struct AddCanCommand;

#[async_trait]
impl Command<dyn YouTubeSendable> for AddCanCommand {
    async fn execute(&self, message: Message, sendable: &mut (dyn YouTubeSendable + 'static), user_client: &mut UserServiceClient<Channel>) -> Result<(), CommandError> {
        sendable.send_message("Added a can!").await;
        return Ok(());
    }
}

export_command!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn CommandRegistrar) {
    registrar.register_command("!addcan", &["!addbear", "!addjohn"], Box::new(AddCanCommand));
}