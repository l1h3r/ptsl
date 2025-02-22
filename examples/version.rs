use ptsl_client::client::Client;
use ptsl_client::error::Result;
use ptsl_client::types::VersionType;
use ptsl_protos::bridge::CommandExt;

const APPNAME: &str = "PTSL_Example_Version";
const COMPANY: &str = "PTSL";

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<()> {
  let mut client: Client = Client::new().await?;

  client.register_connection(APPNAME.into(), COMPANY.into()).await?;

  println!("[version][client]: {:?}", client.version(VersionType::Client).await?);
  println!("[version][method]: {:?}", client.version(VersionType::ClientAPI).await?);
  println!("[version][server]: {:?}", client.version(VersionType::Server).await?);

  Ok(())
}
