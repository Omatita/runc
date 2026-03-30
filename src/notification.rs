use zbus::{connection, interface};
use std::{collections::HashMap, sync::{Arc, Mutex, atomic::AtomicU32}};
use zbus::zvariant::OwnedValue;


#[derive(Debug, Clone)]
pub struct NotificationEntry{
    pub id: u32,
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub icon: String,
    pub actions: Vec<String>,
    pub timeout: i32,

    pub hints: HashMap<String, OwnedValue>,
}

pub struct NotificationServer{
    last_id: AtomicU32,
    notifications: Arc<Mutex<NotificationEntry>>,
}


#[interface(name = "org.freedesktop.Notifications")]
impl NotificationServer{

    fn get_server_information(&self) -> (String, String, String, String) {
        (
            "runc".to_string(),
            "matita".to_string(),
            "0.1.0".to_string(),
            "1.2".to_string(),
        )
    }

    fn get_capabilities(&self) -> Vec<String> {
        vec!["body".to_string(), "persistence".to_string()]
    }

    fn notify(
        &self,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: HashMap<String, OwnedValue>,
        expire_timeout: i32,
    ) -> u32 {

        let notify = NotificationEntry {id:replaces_id, app_name:app_name, title:summary, body:body, icon:app_icon, actions:actions, timeout:expire_timeout, hints: hints};
        println!("Notifica da {}: {} - {}", notify.app_name, notify.title, notify.body);
        
        notify.id
    }

    fn close_notification(&self, _id: u32) {}
}
