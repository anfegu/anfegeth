#[cfg(test)]
use super::*;

#[test]
fn test_validate_handshake_success() {
    let network_id = "1".to_string();
    let our_network_id = "1".to_string();

    let result = validate_handshake(&network_id, &our_network_id);
    assert!(result.is_ok());
}

#[test]
fn test_validate_handshake_error() {
    let network_id = "2".to_string();
    let our_network_id = "1".to_string();

    let result = validate_handshake(&network_id, &our_network_id);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::Other);
}

#[test]
fn test_get_stream_success() {
    let url = "http://127.0.0.1:8545";
    
    let result = get_stream(&url);
    assert!(result.is_ok());
}

#[test]
fn test_get_stream_error() {
    let url = "http://127.0.0.1:93333";
    
    let result = get_stream(&url);
    assert!(result.is_err());

}

#[test]
fn test_read_data_net_success() {
    let file = "network.json";

    let result = fs::read_to_string(file);
    assert!(result.is_ok());
}

#[test]
fn test_read_data_net_error() {
    let file = "network2.json";

    let result = fs::read_to_string(file);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
}

#[test]
fn test_parse_json_success() {
    let file = "network.json";
    let data = fs::read_to_string(file).unwrap();
    let result:Vec<Network> = serde_json::from_str(&data).unwrap();
    assert!(result.len() > 0);
}

#[test]
fn test_parse_json_empty() {
    let file = "test_empty.json";
    let data = fs::read_to_string(file).unwrap();
    let result:Vec<Network> = serde_json::from_str(&data).unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_env_success() {
    dotenv().ok();
    let url = std::env::var("URL");
    assert!(url.is_ok());
}

#[test]
fn test_env_err() {
    dotenv().ok();
    let url = std::env::var("URL2");
    assert!(url.is_err());
}