use std::sync::Arc;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};

pub struct ProgressService {
    sender: broadcast::Sender<ProgressEvent>,
}

impl ProgressService {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ProgressEvent> {
        self.sender.subscribe()
    }

    pub fn emit(&self, event: ProgressEvent) {
        let _ = self.sender.send(event);
    }

    pub fn emit_progress(&self, task_id: &str, stage: ProgressStage, progress: u8, message: &str) {
        let event = ProgressEvent {
            task_id: task_id.to_string(),
            stage,
            progress,
            message: message.to_string(),
        };
        self.emit(event);
    }

    pub fn emit_completed(&self, task_id: &str, message: &str) {
        self.emit_progress(task_id, ProgressStage::Completed, 100, message);
    }

    pub fn emit_failed(&self, task_id: &str, error: &str) {
        self.emit_progress(task_id, ProgressStage::Failed, 0, error);
    }
}

impl Default for ProgressService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub task_id: String,
    pub stage: ProgressStage,
    pub progress: u8,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ProgressStage {
    Parsing,
    Analyzing,
    Generating,
    Exporting,
    Completed,
    Failed,
}

impl ProgressStage {
    pub fn as_str(&self) -> &str {
        match self {
            ProgressStage::Parsing => "parsing",
            ProgressStage::Analyzing => "analyzing",
            ProgressStage::Generating => "generating",
            ProgressStage::Exporting => "exporting",
            ProgressStage::Completed => "completed",
            ProgressStage::Failed => "failed",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ProgressStage::Parsing => "正在解析文档",
            ProgressStage::Analyzing => "正在分析内容",
            ProgressStage::Generating => "正在生成PPT",
            ProgressStage::Exporting => "正在导出",
            ProgressStage::Completed => "已完成",
            ProgressStage::Failed => "失败",
        }
    }
}

pub struct ProgressTracker {
    task_id: String,
    service: Arc<ProgressService>,
    current_stage: ProgressStage,
    current_progress: u8,
}

impl ProgressTracker {
    pub fn new(task_id: String, service: Arc<ProgressService>) -> Self {
        Self {
            task_id,
            service,
            current_stage: ProgressStage::Parsing,
            current_progress: 0,
        }
    }

    pub fn set_stage(&mut self, stage: ProgressStage) {
        self.current_stage = stage;
        self.current_progress = 0;
        self.emit("开始处理");
    }

    pub fn set_progress(&mut self, progress: u8, message: &str) {
        self.current_progress = progress.min(100);
        self.emit(message);
    }

    pub fn complete(&mut self, message: &str) {
        self.current_stage = ProgressStage::Completed;
        self.current_progress = 100;
        self.emit(message);
    }

    pub fn fail(&mut self, error: &str) {
        self.current_stage = ProgressStage::Failed;
        self.emit(error);
    }

    fn emit(&self, message: &str) {
        self.service.emit_progress(
            &self.task_id,
            self.current_stage.clone(),
            self.current_progress,
            message,
        );
    }
}
