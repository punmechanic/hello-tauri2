use hyper::Client as HyperClient;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Container {
  #[serde(alias = "Id")]
  pub id: String,
  #[serde(alias = "ImageID")]
  pub image_id: String,
}

#[derive(Debug)]
pub enum DockerError {}

impl From<std::io::Error> for DockerError {
  fn from(_err: std::io::Error) -> DockerError {
    unimplemented!();
  }
}

impl From<hyper::Error> for DockerError {
  fn from(_err: hyper::Error) -> DockerError {
    unimplemented!();
  }
}

impl From<serde_json::Error> for DockerError {
  fn from(_err: serde_json::Error) -> DockerError {
    unimplemented!();
  }
}

struct DockerProtocol {}

type UnixClient = HyperClient<UnixConnector>;
async fn json<T>(client: &UnixClient, url: hyper::Uri) -> Result<T, DockerError>
where
  T: DeserializeOwned,
{
  let mut response = client.get(url).await?;
  // This is probably bad as it loads entire body into memory at once
  let b = hyper::body::to_bytes(response.body_mut()).await?;
  let data: T = serde_json::from_slice(&b)?;
  Ok(data)
}

impl DockerProtocol {
  fn build_uri(&self, path: &str) -> hyper::Uri {
    Uri::new("/var/run/docker.sock", path).into()
  }

  async fn list_containers(&self, client: &UnixClient) -> Result<Vec<Container>, DockerError> {
    let url = self.build_uri("/containers/json?all=true");
    let data: Vec<Container> = json(client, url).await?;
    Ok(data)
  }
}

pub struct Client {
  client: HyperClient<UnixConnector>,
  protocol: DockerProtocol,
}

impl<'a> Client {
  pub fn new() -> Self {
    Client {
      client: HyperClient::unix(),
      protocol: DockerProtocol {},
    }
  }

  pub async fn list_containers(&self) -> Result<Vec<Container>, DockerError> {
    self.protocol.list_containers(&self.client).await
  }
}
