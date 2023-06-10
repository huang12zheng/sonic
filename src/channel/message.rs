// Code source from https://github.com/valeriansaliou/sonic/blob/.../src/channel/message.rs
use super::command::{
    ChannelCommandBase, ChannelCommandError, ChannelCommandIngest, ChannelCommandResponse,
    ChannelCommandSearch,
};
use std::str::SplitWhitespace;
pub struct ChannelMessage;

impl ChannelMessage {
    fn extract(message: &str) -> (String, SplitWhitespace) {
        // Extract command name and arguments
        let mut parts = message.split_whitespace();
        let command = parts.next().unwrap_or("").to_uppercase();

        debug!("will dispatch search command: {}", command);

        (command, parts)
    }

    pub fn on(message: &str) -> Result<Vec<ChannelCommandResponse>, ChannelCommandError> {
        macro_rules! gen_channel_message_mode_handle2 {
            ($message:ident, { $($external:expr => $internal:expr),+, }) => {{
                let (command, parts) = ChannelMessage::extract($message);

                match command.as_str() {
                    "" => Ok(vec![ChannelCommandResponse::Void]),
                    $(
                        $external => $internal(parts),
                    )+
                    "PING" => ChannelCommandBase::dispatch_ping(parts),
                    "QUIT" => ChannelCommandBase::dispatch_quit(parts),
                    _ => Ok(vec![ChannelCommandResponse::Err(
                        ChannelCommandError::UnknownCommand,
                    )]),
                }
            }};
        }

        gen_channel_message_mode_handle2!(message,{
            // ChannelMessageModeSearch
            "QUERY" => ChannelCommandSearch::dispatch_query,
            "SUGGEST" => ChannelCommandSearch::dispatch_suggest,
            "LIST" => ChannelCommandSearch::dispatch_list,
            // ChannelMessageModeIngest
            "PUSH" => ChannelCommandIngest::dispatch_push,
            "POP" => ChannelCommandIngest::dispatch_pop,
        })
    }
}
