use aws_sdk_iam::Client as iamClient;

pub async fn get_account_alias(sdkconfig: &aws_types::SdkConfig) -> Result<String, aws_sdk_iam::Error> {
    let client = iamClient::new(sdkconfig);
    let resp = client.list_account_aliases().send().await?;
    println!("{:?}", resp);
    Ok("ok".to_string())
}

