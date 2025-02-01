use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use std::time::{Duration, Instant};
use rand::Rng;

#[derive(Debug)]
struct NeuralSpaceDistortion {
    device_id: String,
    signal_data: Arc<Mutex<Vec<f64>>>,
    data_sender: mpsc::Sender<f64>,
    distortion_factor: Arc<Mutex<f64>>,
}

impl NeuralSpaceDistortion {
    fn new(device_id: &str) -> Self {
        let (tx, _) = mpsc::channel(100);
        NeuralSpaceDistortion {
            device_id: device_id.to_string(),
            signal_data: Arc::new(Mutex::new(vec![])),
            data_sender: tx,
            distortion_factor: Arc::new(Mutex::new(1.0)),
        }
    }

    async fn initialize(&self) -> Result<(), String> {
        println!("Initializing Neural Space Distortion for device {}", self.device_id);
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn generate_distortion_signals(&self, duration: Duration) -> Result<(), String> {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        while start.elapsed() < duration {
            let signal = rng.gen_range(0.0..1.0);
            self.data_sender.send(signal).await.unwrap();
            tokio::time::sleep(Duration::from_millis(150)).await;
        }
        Ok(())
    }

    fn evaluate_distortion(&self) -> f64 {
        let data = self.signal_data.lock().unwrap();
        let total: f64 = data.iter().sum();
        total / data.len() as f64
    }

    async fn apply_space_distortion(&self) -> Result<(), String> {
        let distortion_level = self.evaluate_distortion();
        let mut factor = self.distortion_factor.lock().unwrap();

        if distortion_level > 0.8 {
            println!("Severe neural space distortion detected for device {}: {}", self.device_id, distortion_level);
            *factor *= 1.5;  // Increase distortion factor
            tokio::time::sleep(Duration::from_secs(2)).await;
        } else {
            println!("Minimal distortion detected for device {}: {}", self.device_id, distortion_level);
        }

        Ok(())
    }

    fn reset_distortion_data(&self) {
        let mut data = self.signal_data.lock().unwrap();
        data.clear();
    }
}

async fn execute_neural_space_distortion(device: &NeuralSpaceDistortion) -> Result<(), String> {
    device.initialize().await?;
    device.generate_distortion_signals(Duration::from_secs(10)).await?;
    device.apply_space_distortion().await?;
    device.reset_distortion_data();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let device = NeuralSpaceDistortion::new("NSD123");
    let device_task = task::spawn(execute_neural_space_distortion(&device));

    device_task.await??;
    Ok(())
}
