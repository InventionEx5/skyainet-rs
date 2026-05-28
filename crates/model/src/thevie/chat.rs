// crates/secure-transport/src/chat.rs
// =====================================================
// AI Chat Manager v2.0 — 100% Compatible avec le HTML
// Gère : Tâches, Conversations, Épinglage, Renommage, Suppression, IA
// =====================================================

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub content: String,
    pub is_user: bool,
    pub timestamp: u64,
    pub ai_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: u64,
    pub title: String,
    pub messages: Vec<Message>,
    pub created_at: u64,
    pub pinned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub date: Option<String>,
    pub planning: Option<String>,
    pub completed: bool,
}

pub struct AIChatManager {
    pub conversations: Vec<Conversation>,
    pub tasks: Vec<Task>,
    pub current_conversation_id: Option<u64>,
    pub current_ai: String,
}

impl AIChatManager {
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
            tasks: Vec::new(),
            current_conversation_id: None,
            current_ai: "Thevie".to_string(),
        }
    }

    // =====================================================
    // CONVERSATIONS
    // =====================================================
    pub fn create_conversation(&mut self, title: Option<String>) -> u64 {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let title = title.unwrap_or_else(|| {
            format!("Conversation {}", chrono::Utc::now().format("%d/%m %H:%M"))
        });

        let conversation = Conversation {
            id,
            title,
            messages: Vec::new(),
            created_at: id,
            pinned: false,
        };

        self.conversations.insert(0, conversation);
        self.current_conversation_id = Some(id);

        info!("[Chat] Conversation créée : {}", id);
        id
    }

    pub fn get_conversations(&self) -> &Vec<Conversation> {
        &self.conversations
    }

    pub fn rename_conversation(&mut self, id: u64, new_title: &str) -> bool {
        if let Some(conv) = self.conversations.iter_mut().find(|c| c.id == id) {
            conv.title = new_title.to_string();
            info!("[Chat] Conversation renommée : {}", id);
            return true;
        }
        false
    }

    pub fn delete_conversation(&mut self, id: u64) -> bool {
        let len_before = self.conversations.len();
        self.conversations.retain(|c| c.id != id);
        
        if self.current_conversation_id == Some(id) {
            self.current_conversation_id = None;
        }
        
        info!("[Chat] Conversation supprimée : {}", id);
        self.conversations.len() < len_before
    }

    pub fn toggle_pin_conversation(&mut self, id: u64) -> bool {
        if let Some(conv) = self.conversations.iter_mut().find(|c| c.id == id) {
            // On décoche les autres si on épingle celle-ci
            if !conv.pinned {
                for c in &mut self.conversations {
                    c.pinned = false;
                }
            }
            conv.pinned = !conv.pinned;
            return true;
        }
        false
    }

    // =====================================================
    // TÂCHES
    // =====================================================
    pub fn add_task(&mut self, title: &str, date: Option<String>, planning: Option<String>) -> u64 {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let task = Task {
            id,
            title: title.to_string(),
            date,
            planning,
            completed: false,
        };

        self.tasks.push(task);
        info!("[Chat] Tâche ajoutée : {}", title);
        id
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn rename_task(&mut self, id: u64, new_title: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.title = new_title.to_string();
            return true;
        }
        false
    }

    pub fn delete_task(&mut self, id: u64) -> bool {
        let len_before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < len_before
    }

    // =====================================================
    // IA & CHAT
    // =====================================================
    pub fn switch_ai(&mut self, ai_name: &str) {
        self.current_ai = ai_name.to_string();
        info!("[Chat] IA changée : {}", ai_name);
    }

    pub fn get_current_ai(&self) -> &str {
        &self.current_ai
    }

    pub fn set_current_conversation(&mut self, id: u64) {
        self.current_conversation_id = Some(id);
    }
}