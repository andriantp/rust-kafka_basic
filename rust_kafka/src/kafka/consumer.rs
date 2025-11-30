use anyhow::Result;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use log::info;
use std::time::Duration;

use crate::kafka::config::KafkaConfig;

pub struct KafkaConsumer {
    consumer: Consumer,
}

impl KafkaConsumer {
    pub fn new(config: &KafkaConfig) -> Result<Self> {
        let consumer = Consumer::from_hosts(vec![config.broker.clone()])
            .with_topic(config.topic.clone())
            .with_group(config.group_id.clone())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()?;

        Ok(Self { consumer })
    }

    pub fn poll_messages(&mut self) -> Result<()> {
        info!("📥 Consumer started, waiting for messages...");

        loop {
            let message_sets = self.consumer.poll()?;
            for ms in message_sets.iter() {
                for m in ms.messages() {
                    let payload = String::from_utf8_lossy(m.value);
                    println!("✅ received: {}", payload);
                }

                // Commit offset after processing
                self.consumer.consume_messageset(ms)?;
            }

            // Commit consumed offsets to Kafka
            self.consumer.commit_consumed()?;
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
