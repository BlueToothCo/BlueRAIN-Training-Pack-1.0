use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use rand::Rng;
use tokio::sync::mpsc;
use tokio::task;

#[derive(Debug)]
struct BrainwaveModule {
    device_id: String,
    signal_data: Arc<Mutex<Vec<f64>>>,
    data_sender: mpsc::Sender<f64>,
}

impl BrainwaveModule {
    fn new(device_id: &str) -> Self {
        let (tx, _) = mpsc::channel(100);
        BrainwaveModule {
            device_id: device_id.to_string(),
            signal_data: Arc::new(Mutex::new(vec![])),
            data_sender: tx,
        }
    }

    async fn initialize(&self) -> Result<(), String> {
        println!("Initializing device {}", self.device_id);
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn collect_data(&self, duration: Duration) -> Result<(), String> {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        while start.elapsed() < duration {
            let signal = rng.gen_range(0.0..1.0);
            self.data_sender.send(signal).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok(())
    }

    fn analyze_data(&self) -> f64 {
        let data = self.signal_data.lock().unwrap();
        let sum: f64 = data.iter().sum();
        sum / data.len() as f64
    }

    fn reset_data(&self) {
        let mut data = self.signal_data.lock().unwrap();
        data.clear();
    }
}

async fn process_device_data(device: &BrainwaveModule) -> Result<(), String> {
    device.initialize().await?;
    device.collect_data(Duration::from_secs(5)).await?;
    let avg_signal = device.analyze_data();
    println!("Average brainwave signal for {}: {}", device.device_id, avg_signal);
    device.reset_data();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let device = BrainwaveModule::new("D987");
    let device_handler = task::spawn(process_device_data(&device));

    device_handler.await??;
    Ok(())
}
