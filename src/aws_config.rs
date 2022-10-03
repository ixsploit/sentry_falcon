use aws_sdk_config::Client;
use aws_sdk_config::model::ResourceType;
use std::error::Error;
use aws_types::SdkConfig;
use crate::config;
use tokio::join;
use crate::sts;

pub async fn show_resources(run: config::Run) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let account = sts::get_caller(&run.config);
    let client = Client::new(&run.config);
    for value in ResourceType::values() {
        let parsed = ResourceType::from(*value);

        let resp = client
            .list_discovered_resources()
            .resource_type(parsed)
            .send()
            .await?;
        

        let resources = resp.resource_identifiers().unwrap_or_default();

        if !resources.is_empty() {
            println!();
            println!("Resources of type {}", value);
        }

        //for resource in resources {
        //    println!(
        //        "  Resource ID: {}",
        //        resource.resource_id().as_deref().unwrap_or_default()
        //    );
        //}
    }

    println!();

    Ok(vec!["Implementation missiong".to_string()])
}
