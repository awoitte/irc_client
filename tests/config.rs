extern crate irc_client;
use irc_client::config::*;

use std::io::*;

#[test]
fn it_should_provide_a_default_config(){
    let config = default_config();
    assert_eq!(config.name, "guest");
}

#[test]
fn it_should_parse_json_string(){
    let config = get_config_from_string("{\"name\": \"test\"}").expect("parsing json failed");
    assert_eq!(config.name, "test");
}
