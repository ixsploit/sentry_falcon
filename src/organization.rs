use aws_sdk_organizations::Client as orgClient;
use itertools::Itertools;
use std::error::Error;
use std::fmt;
use crate::config;
use crate::sts;

#[derive(Debug)]
struct OrgError(String);

impl Error for OrgError {}

impl fmt::Display for OrgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Org Error: {}", self.0)
    }
}

pub async fn get_accounts(config: config::Config)->Result<config::Config, Box<dyn Error + Send + Sync>> {
    let root_id = config.root_id.clone();
    let carn = config.cross_account_role_name.clone();
    let sdkconfig = sts::get_config(root_id, &carn,config.default_region).await?;
    let org_client: orgClient = orgClient::new(&sdkconfig.config);
    let accounts = org_client
        .list_accounts()
        .send()
        .await?;
    let mut nexttoken = accounts.next_token;
    let mut accountlist = accounts
        .accounts
        .unwrap_or_default();
    while nexttoken.is_some() {
        let accounts = org_client
            .list_accounts()
            .next_token(nexttoken.unwrap())
            .send()
            .await?;
        nexttoken = accounts.next_token;
        let accountlisttemp = accounts
            .accounts
            .unwrap_or_default();
        accountlist.extend(accountlisttemp);

    }
    let mut account: Vec<_> = accountlist
        .into_iter()
        .map(|x| {x.id.unwrap_or_default()})
        .collect();
    account.extend(config.accounts.unwrap_or(vec![]));
    let account: Vec<_> = account
        .into_iter()
        .unique()
        .collect();
    let config = config::Config {
        root_id: config.root_id,
        cross_account_role_name: config.cross_account_role_name,
        accounts: Some(account),
        default_region: "eu-west-1",
    };
    Ok(config)
}
pub async fn get_account_name(id: &String, config: &config::Config)->Result<String, Box<dyn Error + Send + Sync>> {
    let root_id = config.root_id.clone();
    let carn = config.cross_account_role_name.clone();
    let sdkconfig = sts::get_config(root_id,&carn,config.default_region).await?;
    let org_client: orgClient = orgClient::new(&sdkconfig.config);
    let accounts = org_client
        .list_accounts()
        .send()
        .await?
        .accounts
        .unwrap_or_default();
    let account: Vec<_> = accounts
        .into_iter()
        .filter(|x| {x.id.as_ref().unwrap().eq(id)})
        .map(|x| {x.name.unwrap_or_default()})
        .collect();
    let account = account.first();
    match account {
        None => Ok(String::from("No Account found")),
        Some(ref x) => Ok(x.to_string()),    
        }
}
