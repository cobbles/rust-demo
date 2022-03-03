use k8s_openapi::api::core::v1::Node;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};

pub async fn node_count() -> Result<(), Box<dyn std::error::Error>> {
    // Infer the runtime environment and try to create a Kubernetes Client
    print!("creating client...");
    let client = Client::try_default().await?;
    println!(" done");

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    print!("finding nodes...");
    println!(" done");
    let nodes: Api<Node> = Api::all(client);
    for n in nodes.list(&ListParams::default()).await? {
        println!("found node {}", n.name());
    }
    Ok(())
}
