use aws_sdk_sts::output::AssumeRoleOutput;
use std::error::Error;
use aws_sdk_sts::Client as stsClient;
use aws_types::SdkConfig as SdkConfig;
use aws_types::region::Region;
use aws_sdk_iam::{Credentials as iamCredentials};
use std::borrow::Borrow;
use aws_config::meta::region::RegionProviderChain;
use crate::config;


pub async fn assume_role(account_number: &String, role_name: &String) -> Result<AssumeRoleOutput,Box<dyn Error + Send + Sync>>{
    let arn: String = format!("arn:aws:iam::{}:role/{}", account_number, role_name);
    let config = aws_config::load_from_env().await;
    let sts_client: stsClient = stsClient::new(&config);
    let assumed_role = sts_client.assume_role().role_arn(&arn).role_session_name(&format!("{}-{}",account_number, role_name)).send().await?;
    Ok(assumed_role)
}

pub async fn get_config(account_number: String, role_name: &String, region: &'static str) -> Result<config::Run,Box<dyn Error + Send + Sync>>{
    let assume_role = assume_role(&account_number, &role_name).await?;
    let credentials = assume_role.credentials.as_ref().unwrap();
    let assumed_credential = iamCredentials::from_keys(
        credentials.access_key_id.as_ref().unwrap(),
        credentials.secret_access_key.as_ref().unwrap(),
        credentials.session_token.borrow().clone(),);
    let sregion = &region[..];
    let region_provider = RegionProviderChain::first_try(Region::new(sregion));
    let config = aws_config::from_env().credentials_provider(assumed_credential).region(region_provider).load().await;
    return Ok(config::Run{config: config, account: account_number, region: region.to_owned()});
}

pub async fn get_caller(config: &SdkConfig) -> Result<String,Box<dyn Error + Send + Sync>>{
    let sts_client: stsClient = stsClient::new(config);
    let caller_identity = sts_client.get_caller_identity().send().await?; 
    println!("Caller identity: {:?}", caller_identity);
    Ok(caller_identity.account.unwrap_or_default())
}
