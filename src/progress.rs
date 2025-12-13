use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMessage {
    pub r#type: String,
    pub validator: String,
    pub completed: usize,
    pub total: usize,
    pub status: String,
}

pub struct ProgressTracker {
    sender: broadcast::Sender<ProgressMessage>,
    total_validators: usize,
    completed_validators: usize,
}

impl ProgressTracker {
    pub fn new(total_validators: usize) -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            sender,
            total_validators,
            completed_validators: 0,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ProgressMessage> {
        self.sender.subscribe()
    }

    pub fn update(&mut self, validator_name: String, status: String) {
        self.completed_validators += 1;
        let message = ProgressMessage {
            r#type: "progress".to_string(),
            validator: validator_name,
            completed: self.completed_validators,
            total: self.total_validators,
            status,
        };
        let _ = self.sender.send(message);
    }

    #[allow(dead_code)]
    pub fn mark_complete(&self, report_json: String) {
        let message = ProgressMessage {
            r#type: "complete".to_string(),
            validator: String::new(),
            completed: self.total_validators,
            total: self.total_validators,
            status: report_json,
        };
        let _ = self.sender.send(message);
    }
}
