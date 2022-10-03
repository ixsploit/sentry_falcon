use aws_sdk_rds::Client;
use std::error::Error;
use crate::config;

pub async fn list(run: config::Run) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>{
    println!("-------LIST---------{} ---- {}", run.account, run.region);
    let client = Client::new(&run.config);
    let instances = client.describe_db_instances().send().await?;
    let mut marker = instances.marker;
    let mut instances_vec = instances.db_instances.unwrap_or_default(); 
    while marker.is_some() {
        let instances = client
            .describe_db_instances()
            .marker(marker.unwrap())
            .send().await?;
        marker = instances.marker;
        let mut instances = instances.db_instances.unwrap_or_default();
        instances_vec.append(&mut instances); 
    }
    for instance in instances_vec.into_iter(){
        println!("{:50} {:<5} {:<10}", instance.db_instance_identifier.unwrap_or_default(), run.account, run.region);
    }
    println!("End of list {} {}", run.account, run.region);
    Ok(vec!["".to_string()])
}


