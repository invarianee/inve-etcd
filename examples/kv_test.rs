use inve_etcd::{Client, ClientConfig, KeyValueOp, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = ClientConfig::new(["http://127.0.0.1:2379".into()]);

    config.user_auth("invar".to_string(), "invar".to_string());

    let client = Client::connect(config).await?;

    client.put(("/invar/foo/a", "bara")).await.expect("put kv");
    client.put(("/invar/foo/b", "barb")).await.expect("put kv");
    client.put(("/invar/foo/c", "barc")).await.expect("put kv");

    let kvs0 = client.get_by_prefix("/invar/foo").await;
    match kvs0 {
        Ok(kv) => {
            println!("kv.kvs.len() = {}", kv.kvs.len());
            if kv.kvs.len() > 0 {
                for i in 0..kv.kvs.len() {
                    println!(
                        "get by prefix {:#?}: {:#?}",
                        kv.kvs[i].key_str(),
                        kv.kvs[i].value_str()
                    );
                }
            }
        }
        Err(_) => {}
    };

    let kvs = client.get("/invar/foo/a").await.expect("get kv");
    println!(
        "get {:#?}: {:#?}",
        kvs.kvs[0].key_str(),
        kvs.kvs[0].value_str()
    );

    let _kvs1 = client.delete_by_prefix("/invar/foo").await.expect("get kv");

    let kvs2 = client.get_by_prefix("/invar/foo").await;
    match kvs2 {
        Ok(kv) => {
            if kv.kvs.len() > 0 {
                for i in 0..kv.kvs.len() {
                    println!(
                        "get by prefix {:#?}: {:#?}",
                        kv.kvs[i].key_str(),
                        kv.kvs[i].value_str()
                    );
                }
            }
        }
        Err(_) => {}
    };

    client.put(("/invar/foo1/a", "bara")).await.expect("put kv");
    client.put(("/invar/foo1/b", "bara")).await.expect("put kv");
    client.put(("/invar/foo1/c", "bara")).await.expect("put kv");

    client.delete("/invar/foo1").await.expect("get kv");

    client.delete("/invar/foo1/a").await.expect("get kv");
    client.delete("/invar/foo1/b").await.expect("get kv");
    client.delete("/invar/foo1/c").await.expect("get kv");

    Ok(())
}
