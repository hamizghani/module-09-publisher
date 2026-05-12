use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
}

fn main() {
    let mut publisher =
        CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();

    _ = publisher.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "HamizGhani-Amir".to_owned(),
        },
    );
    _ = publisher.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "HamizGhani-Budi".to_owned(),
        },
    );
    _ = publisher.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "HamizGhani-Cica".to_owned(),
        },
    );
    _ = publisher.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "HamizGhani-Dira".to_owned(),
        },
    );
    _ = publisher.publish_event(
        "user_created".to_owned(),
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "HamizGhani-Emir".to_owned(),
        },
    );
}
