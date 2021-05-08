use crate::promise;

#[test]
fn can_be_constructed_from_string() {
    let error = promise::Error::from("promise error 1234");
    assert_eq!(error.to_string(), String::from("promise error 1234"));

    let error = promise::Error::from(format!("promise error {}", 5678));
    assert_eq!(error.to_string(), String::from("promise error 5678"));

}
