#[deny(missing_docs)]
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::Notify;
use tonic::transport::Server;

use lib_hdl::commhandler::CommHandler;
use lib_hdl::msgserver::MsgServer;
use lib_hdl::regserver::RegServer;
use lib_hdl::ui::UI;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "HDL Peer-to-Peers Chat",
    about = "A simple peer-to-peers chat program"
)]
struct Args {
    #[structopt(short = "p", long = "--port", default_value = "2480")]
    port: u32,
}

async fn setup_interrupts(notify: Arc<Notify>) -> Result<()> {
    let mut signals = signal(SignalKind::interrupt())?;
    tokio::spawn(async move {
        signals.recv().await;
        notify.notify_one();
    });
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();
    let addr: SocketAddr = format!("0.0.0.0:{}", args.port).parse()?;
    let commhandler = CommHandler::new(args.port).await?;
    let reg_server = RegServer::new(commhandler.clone()).await?;
    let msg_server = MsgServer::new().await?;

    let notify = Arc::new(Notify::new());
    let notify_copy = notify.clone();

    // setup the interrupt handles
    if let Err(err) = setup_interrupts(notify_copy).await {
        eprintln!("Could not setup interrupt handlers: {}", err);
        std::process::exit(1);
    }

    let mut handles = Vec::new();

    // start the UI
    let notify_copy = notify.clone();
    handles.push(tokio::spawn(async move {
        let ui = UI::new(notify_copy, commhandler.clone())
            .await
            .expect("Could not create UI");
        ui.run().await.expect("Could not start UI");
        Ok::<(), anyhow::Error>(())
    }));

    // start the server
    println!("Starting the server on {}", &addr.to_string());
    // handles.push(tokio::spawn(async move {
    Server::builder()
        .add_service(reg_server)
        .add_service(msg_server)
        .serve_with_shutdown(addr, notify.notified())
        .await
        .map_err(anyhow::Error::msg)?;
    // Ok::<(), anyhow::Error>(())
    // }));

    Ok(())
}
