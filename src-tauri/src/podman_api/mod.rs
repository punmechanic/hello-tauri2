#[derive(Debug)]
pub enum PodmanError {}

#[derive(Debug)]
pub struct Container;

pub struct Client;

impl <'a> Client {
  pub fn new() -> Self {
    Client {}
  }

  pub fn list_containers(&self) -> Result<Vec<Container>, PodmanError> {
    Ok(vec![])
  }
}
