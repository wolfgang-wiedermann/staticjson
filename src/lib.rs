pub mod filehandler;
pub mod cmdlinehandler;
pub mod model;
pub mod parser;
pub mod validator;
pub mod util;

pub mod generator {
//  pub mod swift;
  pub mod jsoninc;
  pub mod jaxrs;
  pub mod java_client;
  pub mod jquery;
  pub mod knockout;
  pub mod dotnet;
//  pub mod rust;
  pub mod htmldoc; // Generator for html documentation
}

mod test {
  mod filehandler;
  mod cmdlinehandler;
  mod parser;
  mod validator;
  mod types;
  mod attributes;
  mod util;
}

#[test]
fn it_works() {
}
