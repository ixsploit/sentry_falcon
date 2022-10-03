use aws_sdk_s3::Client;
use std::error::Error;
use aws_types::SdkConfig;
use tokio::join;
use crate::sts;
use crate::config;

pub async fn main() -> Result<(), aws_sdk_s3::Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();

    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    println!();
    Ok(())
}
pub async fn list(run: config::Run) -> Result<Vec<String>, Box<dyn Error + Send + Sync>>{
    println!("-------LIST---------");
    let client = Client::new(&run.config);
    let buckets = client.list_buckets().send().await?;
    let buckets = buckets.buckets.unwrap_or_default();
    let ret: Vec<String> = buckets.into_iter().map(|x| x.name.unwrap_or_default()).collect();
    for r in ret.iter() {
        println!("{:50} {:<5}", r, run.account);
    }
    println!("End of list");
    Ok(ret)
}


