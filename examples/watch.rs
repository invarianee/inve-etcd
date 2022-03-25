use inve_etcd::{Client, ClientConfig, KeyRange, KeyValueOp, Result, WatchInbound, WatchOp};

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = ClientConfig::new(["http://127.0.0.1:2379".into()]);
    config.user_auth("invar".to_string(), "invar".to_string());

    let client = Client::connect(config).await?;

    let (mut stream, cancel) = client
        .watch(KeyRange::prefix("foo"))
        .await
        .expect("watch by prefix");

    tokio::spawn(async move {
        client.put(("foo1", "1")).await.expect("put kv");
        client.put(("bar", "2")).await.expect("put kv");
        client.put(("foo2", "3")).await.expect("put kv");
        client.put(("bar", "4")).await.expect("put kv");
        client.put(("foo2", "5")).await.expect("put kv");
        client.delete("foo1").await.expect("delete kv");
        client.delete("bar").await.expect("delete kv");

        cancel.cancel().await.expect("cancel watch");
    });

    loop {
        match stream.inbound().await {
            WatchInbound::Ready(resp) => {
                println!("receive event: {:?}", resp);
            }
            WatchInbound::Interrupted(e) => {
                eprintln!("encounter error: {:?}", e);
            }
            WatchInbound::Closed => {
                println!("watch stream closed");
                break;
            }
        }
    }

    Ok(())
}
