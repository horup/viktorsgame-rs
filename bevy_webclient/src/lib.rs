use std::{marker::PhantomData, time::Duration};

use serde::{Serialize, de::DeserializeOwned};
pub trait Message : Send + Sync + Clone + Serialize + DeserializeOwned + 'static {}
impl<T> Message for T where T : Send + Sync + Clone + Serialize + DeserializeOwned + 'static {}

pub struct BevyWebclientPlugin<T> {
    phantom:PhantomData<T>
}

#[test]
fn test() {
    loop {
        let (mut sender, receiver) = ewebsock::connect("wss://socketsbay.com/wss/v2/1/demo/").unwrap();
        loop {
            //sender.send(ewebsock::WsMessage::Text("Hello!".into()));
            while let Some(event) = receiver.try_recv() {
                println!("Received {:?}", event);
            }

            std::thread::sleep(Duration::from_millis(100));
        }

    }
  
}