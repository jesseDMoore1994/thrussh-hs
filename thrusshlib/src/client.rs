extern crate thrussh;
extern crate thrussh_keys;
extern crate futures;
extern crate tokio;
extern crate env_logger;

use anyhow::Result;
use std::sync::Arc;
use thrussh::*;
use thrussh_keys::*;
use std::io::Write;

#[repr(C)]
struct Client {}

impl client::Handler for Client {
    type Error = thrussh::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), Self::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), Self::Error>>;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
        futures::future::ready(Ok((self, b)))
    }
    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, session)))
    }
    fn check_server_key(self, _server_public_key: &key::PublicKey) -> Self::FutureBool {
        self.finished_bool(true)
    }
}

#[repr(C)]
pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    pub async fn connect(
        addr: impl std::net::ToSocketAddrs,
        user: impl Into<String>,
        passwd: &str,
    ) -> Result<Self> {
        let config = client::Config::default();
        let config = Arc::new(config);
        let sh = Client {};
        let mut session = client::connect(config, addr, sh).await?;
        let auth_res = session.authenticate_password(user, passwd).await;
        let _auth_res = auth_res?;
        Ok(Self { session })
    }

    pub async fn call(&mut self, command: &str) -> Result<CommandResult> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;
        let mut output = Vec::new();
        let mut code = None;
        while let Some(msg) = channel.wait().await {
            match msg {
                thrussh::ChannelMsg::Data { ref data } => {
                    output.write_all(data).unwrap();
                }
                thrussh::ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                }
                _ => {}
            }
        }
        Ok(CommandResult { output, code })
    }

    pub async fn close(&mut self) -> Result<()> {
        println!("made it here in close");
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

pub struct CommandResult {
    output: Vec<u8>,
    code: Option<u32>,
}

impl CommandResult {
    pub fn output(&self) -> String {
        String::from_utf8_lossy(&self.output).into()
    }

    pub fn success(&self) -> bool {
        self.code == Some(0)
    }
}
