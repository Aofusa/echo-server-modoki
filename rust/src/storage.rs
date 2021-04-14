use actix::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Msg {
    Set(String),
    Get,
}

impl Message for Msg {
    type Result = String;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MsgActor {
    pub msg: String,
}

impl Actor for MsgActor {
    type Context = Context<Self>;
}

impl Handler<Msg> for MsgActor {
    type Result = String;

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) -> Self::Result {
        match msg {
            Msg::Set(x) => {
                self.msg = x;
                self.msg.clone()
            },
            Msg::Get => self.msg.clone(),
        }
    }
}

#[cfg(test)]
mod tests_storage {
    use actix::prelude::*;
    use crate::storage::*;

    #[actix::test]
    async fn test_actor() {
        let storage = MsgActor { msg: "initialize msg".to_string() }.start();

        let msg = storage.send(Msg::Get).await.unwrap();
        assert_eq!(msg, "initialize msg".to_string());

        let msg = storage.send(Msg::Set("echo msg".to_string())).await.unwrap();
        assert_eq!(msg, "echo msg".to_string());

        let msg = storage.send(Msg::Get).await.unwrap();
        assert_eq!(msg, "echo msg".to_string());
    }
}

