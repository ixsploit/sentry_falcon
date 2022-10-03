use serde::Serialize;
use aws_types::SdkConfig;
use std::error::Error;
use std::io::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub struct Config {
    pub root_id: String,
    pub cross_account_role_name: String,
    pub accounts: Option<Vec<String>>,
    pub default_region: &'static str   
}

#[derive(Debug, Clone)]
pub struct Run {
    pub config: SdkConfig,
    pub account: String,
    pub region: String
}

pub const REGIONS: &'static [&'static str] = &[
    "eu-north-1",
    "ap-south-1",
    "eu-west-3",
    "eu-west-2",
    "eu-west-1",
    "ap-northeast-3",
    "ap-northeast-2",
    "ap-northeast-1",
    "sa-east-1",
    "ca-central-1",
    "ap-southeast-1",
    "ap-southeast-2",
    "eu-central-1",
    "us-east-1",
    "us-east-2",
    "us-west-1",
    "us-west-2",
];

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let home_dir = dirs::home_dir().unwrap();
        let file = std::fs::File::open(format!("{}/.sfrc", home_dir.display()))?;
        let yaml_content: serde_yaml::Value = serde_yaml::from_reader(file)?;

        let root_id = yaml_content
            .get("root_id")
            .expect("No root id defined")
            .as_u64()
            .unwrap()
            .to_string();
        let cross_account_role_name = yaml_content 
            .get("cross_account_role_name")
            .expect("No CrossAccountRoleName")
            .as_str()
            .unwrap();
        let regions = yaml_content.get("regions").unwrap().as_sequence().unwrap().to_owned();
        let accounts = yaml_content.get("accounts").unwrap().as_sequence().unwrap();
        let accounts_string = accounts
            .iter()
            .map(|x| x.as_u64().unwrap().to_string())
            .collect();
        let config = Config {
            root_id: String::from(root_id),
            cross_account_role_name: String::from(cross_account_role_name),
            accounts: Some(accounts_string),
            default_region: "eu-west-1",
        };
        Ok(config)
    }
}

pub fn init() -> std::io::Result<()> {
    let home_dir = dirs::home_dir().unwrap();
    let mut file = std::fs::File::create(format!("{}/.sfrc", home_dir.display()))?;
    let root_id = String::from("1234567890");
    let cross_account_role_name = String::from("OrganizationAccountAccessRole");
    let config = Config {
        root_id: root_id,
        cross_account_role_name: cross_account_role_name,
        accounts: None,
        default_region: "eu-west-1"
    };
    file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes())?;
    Ok(())
}
