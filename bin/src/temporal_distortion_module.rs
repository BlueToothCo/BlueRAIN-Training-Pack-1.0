use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use std::time::{Duration, Instant};
use rand::Rng;

#[derive(Debug)]
struct TemporalDistortionModule {
    device_id: String,
    temporal_data: Arc<Mutex<Vec<f64>>>,
    data_sender: mpsc::Sender<f64>,
    timewarp_factor: Arc<Mutex<f64>>,
}

impl TemporalDistortionModule {
    fn new(device_id: &str) -> Self {
        let (tx, _) = mpsc::channel(100);
        TemporalDistortionModule {
            device_id: device_id.to_string(),
            temporal_data: Arc::new(Mutex::new(vec![])),
            data_sender: tx,
            timewarp_factor: Arc::new(Mutex::new(1.0)),
        }
    }

    async fn initialize(&self) -> Result<(), String> {
        println!("Initializing Temporal Distortion Module for device {}", self.device_id);
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn generate_timewarp_signals(&self, duration: Duration) -> Result<(), String> {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        while start.elapsed() < duration {
            let signal = rng.gen_range(0.0..1.0);
            self.data_sender.send(signal).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok(())
    }

    fn evaluate_timewarp(&self) -> f64 {
        let data = self.temporal_data.lock().unwrap();
        let total: f64 = data.iter().sum();
        total / data.len() as f64
    }

    async fn apply_temporal_distortion(&self) -> Result<(), String> {
        let distortion_level = self.evaluate_timewarp();
        let mut factor = self.timewarp_factor.lock().unwrap();

        if distortion_level > 0.75 {
            println!("High temporal distortion detected for device {}: {}", self.device_id, distortion_level);
            *factor *= 2.0;  // Double the timewarp factor
            tokio::time::sleep(Duration::from_secs(3)).await;
        } else {
            println!("Minimal temporal distortion for device {}: {}", self.device_id, distortion_level);
        }

        Ok(())
    }

    fn reset_temporal_data(&self) {
        let mut data = self.temporal_data.lock().unwrap();
        data.clear();
    }
}

async fn execute_temporal_distortion(device: &TemporalDistortionModule) -> Result<(), String> {
    device.initialize().await?;
    device.generate_timewarp_signals(Duration::from_secs(12)).await?;
    device.apply_temporal_distortion().await?;
    device.reset_temporal_data();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let device = TemporalDistortionModule::new("TDM987");
    let device_task = task::spawn(execute_temporal_distortion(&device));

    device_task.await??;
    Ok(())
}
