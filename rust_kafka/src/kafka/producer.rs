use anyhow::Result;
use kafka::producer::{Producer, Record, RequiredAcks};
use log::info;
use std::time::Duration;

use crate::kafka::config::KafkaConfig;

pub struct KafkaProducer {
    producer: Producer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(config: &KafkaConfig) -> Result<Self> {
        let producer = Producer::from_hosts(vec![config.broker.clone()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()?;

        Ok(Self {
            producer,
            topic: config.topic.clone(),
        })
    }

    pub fn send(&mut self, message: &str) -> Result<()> {
        self.producer.send(&Record {
            topic: &self.topic,
            partition: -1, // ✅ gunakan -1 untuk partition otomatis
            key: "",       // ✅ string kosong, karena `None` gak diterima
            value: message.as_bytes(),
        })?;

        info!("✅ sent: {}", message);
        Ok(())
    }
}
