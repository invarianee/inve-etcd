use inve_etcd::{Client, ClientConfig, Endpoint, KeyValueOp, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = ClientConfig::new([
        Endpoint::from("http://127.0.0.1:12379")
            .tls(
                "etcd-1",
                "./helper/certs/ca.pem",
                "./helper/certs/etcd-1.pem",
                "./helper/certs/etcd-1-key.pem",
            )
            .await?,
        Endpoint::from("http://127.0.0.1:22379")
            .tls(
                "etcd-2",
                "./helper/certs/ca.pem",
                "./helper/certs/etcd-2.pem",
                "./helper/certs/etcd-2-key.pem",
            )
            .await?,
        Endpoint::from("http://127.0.0.1:32379")
            .tls(
                "etcd-3",
                "./helper/certs/ca.pem",
                "./helper/certs/etcd-3.pem",
                "./helper/certs/etcd-3-key.pem",
            )
            .await?,
    ]);

    config.user_auth("invar".to_string(), "invar".to_string());

    let client = Client::connect(config).await?;

    client.put(("foo", "bar")).await.expect("put kv");
    let resp = client.get("foo").await.expect("get kv");

    assert_eq!(resp.kvs.len(), 1);
    assert_eq!(resp.kvs[0].key_str(), "foo");
    assert_eq!(resp.kvs[0].value_str(), "bar");

    Ok(())
}
