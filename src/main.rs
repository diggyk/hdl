use anyhow::{Context, Error, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Notify;
use tonic::transport::Server;

use lib_hdl::regserver::RegServer;
use lib_hdl::ui::UI;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;
    let reg_server = RegServer::new().await?;

    let notify = Arc::new(Notify::new());
    let notify_copy = notify.clone();

    // setup the interrupt handles
    let mut signals = signal(SignalKind::interrupt())?;
    tokio::spawn(async move {
        signals.recv().await;
        println!("We got a signal");
        notify_copy.notify_one();
    });

    let mut handles = Vec::new();
    handles.push(tokio::spawn(async {
        let ui = UI::new().await.expect("Could not create UI");
        ui.run().await.expect("Could not start UI");
    }));

    println!("Server start");
    Server::builder()
        .add_service(reg_server)
        .serve_with_shutdown(addr, notify.notified())
        .await?;
    println!("Server stops");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    for handle in &handles {
        handle.abort();
    }
    println!("UI aborts");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    Ok(())
}
