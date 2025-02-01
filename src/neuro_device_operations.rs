use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use rand::Rng;
use tokio::time::sleep;
use async_trait::async_trait;

#[derive(Debug)]
struct NeuroDevice {
    id: String,
    connection_status: bool,
    brainwave_data: Arc<Mutex<Vec<f32>>>,
}

impl NeuroDevice {
    fn new(id: &str) -> Self {
        NeuroDevice {
            id: id.to_string(),
            connection_status: false,
            brainwave_data: Arc::new(Mutex::new(vec![])),
        }
    }

    async fn establish_connection(&mut self) -> Result<(), String> {
        if self.connection_status {
            return Err("Connection already established".to_string());
        }

        sleep(Duration::from_secs(1)).await;
        self.connection_status = true;
        println!("Device {} connected", self.id);
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        if !self.connection_status {
            return Err("Device not connected".to_string());
        }

        sleep(Duration::from_secs(1)).await;
        self.connection_status = false;
        println!("Device {} disconnected", self.id);
        Ok(())
    }

    async fn collect_brainwave_data(&self, duration: Duration) -> Result<(), String> {
        let start = Instant::now();
        let mut data_lock = self.brainwave_data.lock().unwrap();

        while start.elapsed() < duration {
            let simulated_data = rand::thread_rng().gen_range(0.0..1.0);
            data_lock.push(simulated_data);
            sleep(Duration::from_millis(500)).await;
        }
        println!("Data collection completed for device {}", self.id);
        Ok(())
    }

    fn analyze_data(&self) -> f32 {
        let data_lock = self.brainwave_data.lock().unwrap();
        let total: f32 = data_lock.iter().sum();
        total / data_lock.len() as f32
    }

    fn reset_data(&self) {
        let mut data_lock = self.brainwave_data.lock().unwrap();
        data_lock.clear();
        println!("Data reset for device {}", self.id);
    }
}

#[async_trait]
trait DeviceOperations {
    async fn start(&mut self) -> Result<(), String>;
    async fn stop(&mut self) -> Result<(), String>;
}

#[async_trait]
impl DeviceOperations for NeuroDevice {
    async fn start(&mut self) -> Result<(), String> {
        self.establish_connection().await?;
        self.collect_brainwave_data(Duration::from_secs(10)).await?;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), String> {
        self.reset_data();
        self.disconnect().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut device = NeuroDevice::new("A123");
    device.start().await?;

    let avg_signal = device.analyze_data();
    println!("Average brainwave signal: {}", avg_signal);

    device.stop().await?;
    Ok(())
}
