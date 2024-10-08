use std::any::Any;
use std::path::PathBuf;

pub trait Event: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone)]
pub struct FileReadEvent {
    pub path: PathBuf,
    pub content: Option<String>,
    pub error: Option<String>,
}

impl Event for FileReadEvent {
    fn event_type(&self) -> &'static str {
        "file_read"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct NetworkEvent {
    pub url: String,
    pub status: u16,
    pub response: Option<String>,
    pub error: Option<String>,
}

impl Event for NetworkEvent {
    fn event_type(&self) -> &'static str {
        "network"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
