use flume::Sender;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct MessageSender {
    sender:Sender<Msg>
}
impl MessageSender {
    /// Unsafe because the user of the API should not construct their own MessageSender
    pub unsafe fn new(sender:Sender<Msg>) -> Self {
        Self { sender }
    }
    pub fn send_message(&self,msg:Msg) -> Result<()> {
        self.sender.send(msg)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct UpdateContext {
    pub id:String,
    pub collison_data: Option<CollisionData>,
    pub msg_sender: MessageSender
}

#[derive(Debug, Clone)]
pub struct CollisionData {
    pub other_shape_id:String,
}

#[derive(Debug, Clone)]
pub enum Msg {
    DeleteShape(String),
    AddShape
}

pub trait Update {
    fn update(&mut self,ctx:UpdateContext);
}
