use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError, QueueProperties};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;
impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let one_second = time::Duration::from_millis(1000);
        let _now = time::Instant::now();
        thread::sleep(one_second);

        println!("In sefriano’s Computer [2206818966]. Message received: {:?}", message);
        Ok(())
    }
}

fn main() {
    let listener =
        CrosstownBus::new_queue_listener("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    _ = listener.listen(
            "user_created".to_owned(),
            UserCreatedHandler{},
            QueueProperties {
                auto_delete: false,
                durable: false,
                use_dead_letter: true,
            });

    loop {
    }
}