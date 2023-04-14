use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_s3 as s3;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let resp = req.send().await?;
    println!("Current DynamoDB tables: {:?}", resp.table_names);
    let s3_client = s3::Client::new(&shared_config);
    show_buckets(s3_client).await;
    Ok(())
}

async fn show_buckets(client: s3::Client) -> Result<(), s3::Error> {
    let resp = client.list_buckets().send().await?;
    println!("Current S3 Buckets: {:?}", resp.buckets);
    Ok(())
}