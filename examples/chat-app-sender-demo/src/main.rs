use anyhow::Result;

use kv::*;
wit_bindgen_rust::import!("../../wit/kv.wit");
wit_error_rs::impl_error!(kv::Error);

use mq::*;
wit_bindgen_rust::import!("../../wit/mq.wit");
wit_error_rs::impl_error!(mq::Error);

fn main() -> Result<()> {
    let kv_store = Kv::open("my-container")?;
    let msg_queue = Mq::open("wasi-cloud-queue")?;

    let mut msg_counter = 0;
    loop {
        println!("📧 write a message to send: ");
        let mut msg = "".to_string();
        std::io::stdin().read_line(&mut msg)?;
        msg_queue.send(msg.as_bytes())?;
        println!("📨 sent message");
        kv_store.set(&format!("message-{}", msg_counter), msg.as_bytes())?;
        println!("🔑 added message to kv store");
        msg_counter += 1;
    }
}
