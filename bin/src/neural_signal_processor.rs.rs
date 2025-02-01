use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use std::time::Duration;
use futures::stream::StreamExt;
use rand::Rng;

#[derive(Debug)]
struct NeuralInterface {
    id: String,
    signal_channel: mpsc::Sender<f32>,
    signal_receiver: Arc<Mutex<mpsc::Receiver<f32>>>,
}

impl NeuralInterface {
    fn new(id: &str) -> Self {
        let (tx, rx) = mpsc::channel(100);
        NeuralInterface {
            id: id.to_string(),
            signal_channel: tx,
            signal_receiver: Arc::new(Mutex::new(rx)),
        }
    }

    async fn connect(&self) -> Result<(), String> {
        // Simulate connection establishment
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Device {} connected", self.id);
        Ok(())
    }

    async fn disconnect(&self) -> Result<(), String> {
        // Simulate disconnection
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Device {} disconnected", self.id);
        Ok(())
    }

    async fn collect_signals(&self) -> Result<(), String> {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let signal_value = rng.gen_range(0.0..1.0);
            self.signal_channel.send(signal_value).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok(())
    }

    async fn process_signals(&self) -> Result<f32, String> {
        let mut total_signal = 0.0;
        let receiver_lock = self.signal_receiver.clone();
        let receiver = receiver_lock.lock().unwrap();

        let mut signal_stream = receiver.clone().into_stream();
        while let Some(signal) = signal_stream.next().await {
            total_signal += signal;
        }

        let average_signal = total_signal / 100.0;
        Ok(average_signal)
    }
}

async fn handle_neuro_device_operations(device: &NeuralInterface) -> Result<(), String> {
    device.connect().await?;
    device.collect_signals().await?;
    let avg_signal = device.process_signals().await?;
    println!("Processed average signal: {}", avg_signal);
    device.disconnect().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let device = NeuralInterface::new("N123");
    let device_handler = task::spawn(handle_neuro_device_operations(&device));

    device_handler.await??;
    Ok(())
}
