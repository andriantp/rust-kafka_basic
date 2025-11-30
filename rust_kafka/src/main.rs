mod kafka;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use kafka::{config::KafkaConfig, consumer::KafkaConsumer, producer::KafkaProducer};
use log::info;
use rand::Rng;
use chrono::Utc;
use std::{thread, time::Duration};

#[derive(Parser)]
#[command(name = "rust_kafka", version, about = "Simple Kafka Producer and Consumer in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run Kafka producer
    Producer,
    /// Run Kafka consumer
    Consumer,
}

fn main() -> Result<()> {
    println!("📂 Current dir: {:?}", std::env::current_dir());
    
    // load .env file
    if dotenvy::dotenv().is_err() {
        panic!("⚠️ .env file not found or unreadable");
    }
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init(); // Initialize logger

    let cli = Cli::parse();
    let config = KafkaConfig::new();

    match cli.command {
        Commands::Producer => run_producer_mode(config)?,
        Commands::Consumer => run_consumer_mode(config)?,
    }

    Ok(())
}

fn run_producer_mode(config: KafkaConfig) -> Result<()> {
    let mut producer = KafkaProducer::new(&config)?;
    info!("🚀 Starting producer stream... Press Ctrl+C to stop");

    for i in 0.. {
        // Generate dummy JSON (sensor data)
        let temp = rand::thread_rng().gen_range(20.0..30.0);
        let humidity = rand::thread_rng().gen_range(50.0..80.0);
        let timestamp = Utc::now().to_rfc3339();

        let msg = format!(
            r#"{{"sensor":"A1","index":{},"temp":{:.2},"humidity":{:.2},"time":"{}"}}"#,
            i, temp, humidity, timestamp
        );

        producer.send(&msg)?;
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}

fn run_consumer_mode(config: KafkaConfig) -> Result<()> {
    let mut consumer = KafkaConsumer::new(&config)?;
    consumer.poll_messages()
}
