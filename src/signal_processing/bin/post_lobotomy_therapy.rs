use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task;
use rand::Rng;

#[derive(Debug)]
struct PostLobotomyTherapy {
    patient_id: String,
    therapy_data: Arc<Mutex<Vec<f64>>>,
    therapy_sender: mpsc::Sender<f64>,
    therapy_effectiveness: Arc<Mutex<bool>>,
}

impl PostLobotomyTherapy {
    fn new(patient_id: &str) -> Self {
        let (tx, _) = mpsc::channel(100);
        PostLobotomyTherapy {
            patient_id: patient_id.to_string(),
            therapy_data: Arc::new(Mutex::new(vec![])),
            therapy_sender: tx,
            therapy_effectiveness: Arc::new(Mutex::new(false)),
        }
    }

    async fn begin_therapy(&self) -> Result<(), String> {
        println!("Beginning post-lobotomy therapy for patient {}", self.patient_id);
        tokio::time::sleep(Duration::from_secs(1)).await;
        Ok(())
    }

    async fn monitor_symptoms(&self, duration: Duration) -> Result<(), String> {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        while start.elapsed() < duration {
            let symptom_intensity = rng.gen_range(0.0..1.0);
            self.therapy_sender.send(symptom_intensity).await.unwrap();
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        Ok(())
    }

    fn evaluate_symptoms(&self) -> f64 {
        let data = self.therapy_data.lock().unwrap();
        let total: f64 = data.iter().sum();
        total / data.len() as f64
    }

    async fn apply_therapy(&self) -> Result<(), String> {
        let severity = self.evaluate_symptoms();
        if severity > 0.6 {
            println!("High severity detected for patient {}. Applying corrective measures.", self.patient_id);
            tokio::time::sleep(Duration::from_secs(2)).await;
            *self.therapy_effectiveness.lock().unwrap() = true;
        } else {
            println!("Symptom severity for patient {} is low. Therapy completed successfully.", self.patient_id);
        }
        Ok(())
    }

    fn reset_therapy_data(&self) {
        let mut data = self.therapy_data.lock().unwrap();
        data.clear();
    }
}

async fn execute_post_lobotomy_therapy(device: &PostLobotomyTherapy) -> Result<(), String> {
    device.begin_therapy().await?;
    device.monitor_symptoms(Duration::from_secs(6)).await?;
    device.apply_therapy().await?;
    device.reset_therapy_data();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let therapy_device = PostLobotomyTherapy::new("P9876");
    let therapy_task = task::spawn(execute_post_lobotomy_therapy(&therapy_device));

    therapy_task.await??;
    Ok(())
}
