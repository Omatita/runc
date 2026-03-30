use notification::NotificationServer;
use zbus::{connection, interface};
use std::{error::Error, future::pending};

mod notification;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let notify = NotificationServer;

    let _conn = connection::Builder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", notify)?
        .build()
        .await?;
    
    println!("Waiting");
    pending::<()>().await;

    Ok(())
}

