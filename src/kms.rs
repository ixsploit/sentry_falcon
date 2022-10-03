use aws_sdk_kms::Client;
use std::error::Error;
use aws_types::SdkConfig;
use tokio::join;
use crate::sts;
use futures::future::*;
use crate::config;

pub async fn list(run: config::Run) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>{
    println!("-------LIST---------");
    let client = Client::new(&run.config);
    let keys = client.list_keys().send().await?;
    let keys = keys.keys.unwrap_or_default(); 
    let key_ids: Vec<String> = keys.into_iter().map(|x| x.key_id.unwrap_or_default()).collect();
    let keys: Vec<_> = key_ids
        .into_iter()
        .map ( |key_id| {
            let c = client.to_owned();
            tokio::spawn(async move {c.clone().describe_key().key_id(key_id).send().await})
        })
        .collect();
    let keys = join_all(keys).await;
    let keys: Vec<_> = keys 
        .into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.ok())
        .collect();
    for key in keys.into_iter(){
        if key.key_metadata.is_some() {
            let key_metadata = key.key_metadata.unwrap();
            println!("KEY:,{:?},{:?},{:?},{:?},{:?},{:?},{:?}", key_metadata.arn.unwrap_or_default(), key_metadata.key_id.unwrap_or_default(), key_metadata.description.unwrap_or_default(), key_metadata.encryption_algorithms.unwrap(), key_metadata.key_spec.unwrap(), run.account, run.region);
        }
    }



    println!("End of list");
    Ok(vec!["".to_string()])
}


