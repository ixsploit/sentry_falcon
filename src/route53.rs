use aws_sdk_route53::Client;
use std::error::Error;
use aws_types::SdkConfig;
use futures::future::*;
use tokio::join;
use crate::sts;
use crate::config;

pub async fn get_domains(run: config::Run) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>{
    println!("-------LIST---------");
    let client = Client::new(&run.config);
    let zones = client.list_hosted_zones().send().await?;
    let zones = zones.hosted_zones.unwrap_or_default();
    let ret: Vec<String> = zones.into_iter().map(|x| x.name.unwrap_or_default()).collect();
    println!("End of list");
    Ok(ret)
}


pub async fn get_subdomains(run: config::Run) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>{
    println!("-------LIST---------");
    let client = Client::new(&run.config);
    let zones = client.list_hosted_zones().send().await?;
    let zones = zones.hosted_zones.unwrap_or_default();
    let record_sets: Vec<_> = zones.into_iter().map(|zone| {client.list_resource_record_sets().hosted_zone_id(zone.id.unwrap_or_default()).max_items(2000).send()}).collect();
    
    let record_sets = join_all(record_sets).await;
    for record in record_sets {
       println!("{:?}", record);
       let record = record.unwrap();
       println!("{:?}", record.is_truncated);
       for record in record.resource_record_sets {
           for record in record {
               println!("{:?}",record);
               println!("{}", record.name.unwrap_or_default());
           }
       }
   }
    let ret: Vec<String> = vec!["".to_string()]; 
    println!("End of list");
    Ok(ret)
}
