use k8s_openapi::api::core::v1::Node;
use kube::{
    api::{Api, ListParams, ResourceExt},
    Client,
};

use crate::pod::PodMetrics;

pub async fn node_count() -> Result<(), Box<dyn std::error::Error>> {
    // Infer the runtime environment and try to create a Kubernetes Client
    print!("creating client...");
    let client = Client::try_default().await?;
    println!(" done");

    // Read pods in the configured namespace into the typed interface from k8s-openapi

    print!("finding nodes...");
    println!(" done");
    let nodes = node_names(client).await?;
    for n in nodes {
        println!("{}", n);
    }
    Ok(())
}

pub async fn node_names(client: Client) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = vec![];
    let nodes: Api<Node> = Api::all(client);
    for n in nodes.list(&ListParams::default()).await? {
        result.push(n.name());
    }
    return Ok(result);
}

pub async fn pods(client: Client) -> Result<(), Box<dyn std::error::Error>> {
    let pods = Api::<PodMetrics>::namespaced(
        client.clone(),
    "home-server"
    );
    for p in pods.list(&ListParams::default()).await? {
        println!("{:?} CPU: {:?} ", &p.name(), &p.containers[0].usage.cpu);
        println!("{:?} Memory: {:?}", &p.name(), &p.containers[0].usage.memory);
    }
    return Ok(());
}

