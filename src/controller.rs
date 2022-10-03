use aws_sdk_sts::output::AssumeRoleOutput;
use std::error::Error;
use std::future::Future;
use aws_sdk_sts::Client as stsClient;
use crate::config;
use aws_types::SdkConfig;
use futures::future::*;
use crate::sts;

pub enum Runon {
    Account,
    Region,
}

pub async fn assume_role(account_number: String, role_name: String) -> Result<AssumeRoleOutput,Box<dyn Error + Send + Sync>>{
    println!("Assume role {} {}", account_number, role_name);
    let arn: String = format!("arn:aws:iam::{}:role/{}", account_number, role_name);
    let config = aws_config::load_from_env().await;
    let sts_client: stsClient = stsClient::new(&config);
    let assumed_role = sts_client.assume_role().role_arn(&arn).role_session_name(&format!("{}-{}",account_number, role_name)).send().await?;
    Ok(assumed_role)
}
async fn get_config_account(config: config::Config) -> Result<Vec<config::Run>,Box<dyn Error>> {
    let accounts = config.accounts.unwrap();
    let rolename = &config.cross_account_role_name;
    println!("{:?} {:?}", accounts, rolename);
    let sdk_configs: Vec<_> =  accounts
            .into_iter()
            .map( |account| { println!("iterate configs");
            let rn = rolename.to_owned();
            tokio::spawn(async move {sts::get_config(account.clone(),&rn,"eu-west-1").await})
            })
            .collect();
    let sdk_configs = join_all(sdk_configs).await;
    println!("{:?}", sdk_configs);
    let sdk_configs: Vec<_> = sdk_configs
        .into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.ok())
        .collect();
    Ok(sdk_configs)
}
async fn get_config_region(config: config::Config) -> Result<Vec<config::Run>,Box<dyn Error>> {
    let accounts = config.accounts.unwrap();
    let rolename = &config.cross_account_role_name;
    let regions = config::REGIONS;
    println!("{:?} {:?} {:?}", accounts, rolename, regions);
    let sdk_configs: Vec<_> = accounts
            .into_iter()
            .flat_map( |account| { println!("iterate configs");
                regions 
                .iter()
                .map(move |reg| {
                let rn = rolename.to_owned();
                let acc = account.to_owned();
                println!("{:?} {:?} {:?}", &acc, &rn, &reg);
                tokio::spawn(async move {sts::get_config(acc.clone(),&rn,&reg).await})
                })
            })
            .collect();  
    let sdk_configs = join_all(sdk_configs).await;
    println!("{:?}", sdk_configs);
    let sdk_configs: Vec<_> = sdk_configs
        .into_iter()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.ok())
        .collect();
    Ok(sdk_configs)
}

pub async fn run_function<F: 'static, Fut: 'static>(runs: Vec<config::Run>, f: F) -> Result<(),Box<dyn Error>>
where
  F: Fn(config::Run) -> Fut + std::marker::Send + Copy+ std::marker::Sync,
  Fut: Future<Output = Result<Vec<String>, Box<(dyn Error + Sync + std::marker::Send)>>> + std::marker::Send,
{
    let f_futures: Vec<_> = runs 
        .into_iter()
        .map(|run| {
            tokio::spawn(async move{f(run).await})
        })
        .collect();
    let ret: Vec<_> = join_all(f_futures).await;
    println!("run_function done");
    Ok(())
}
pub async fn run_account<F: 'static, Fut: 'static>(config: config::Config, f: F) -> Result<(),Box<dyn Error>>
where
  F: Fn(config::Run) -> Fut + std::marker::Send + Copy+ std::marker::Sync,
  Fut: Future<Output = Result<Vec<String>, Box<(dyn Error + Sync + std::marker::Send)>>> + std::marker::Send,
{ 
    println!("async account");
    let runs = get_config_account(config).await?;
    run_function(runs, f).await?;
    println!("async account - OK");
    Ok(())        
}
pub async fn run_region<F: 'static, Fut: 'static>(config: config::Config, f: F) -> Result<(),Box<dyn Error>>
where
  F: Fn(config::Run) -> Fut + std::marker::Send + Copy+ std::marker::Sync,
  Fut: Future<Output = Result<Vec<String>, Box<(dyn Error + Sync + std::marker::Send)>>> + std::marker::Send,
{ 
    println!("async region");
    let runs = get_config_region(config).await?;
    run_function(runs, f).await?;
    println!("async region - OK");
    Ok(())        
}

