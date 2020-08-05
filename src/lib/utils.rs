use crate::lib::{
    types::Result,
    errors::AppError,
    constants::PREFIX_MAX_LENGTH,
};

pub fn maybe_strip_hex_prefix(hex: &str) -> Result<String> {
    let lowercase_hex_prefix = "0x";
    let uppercase_hex_prefix = "0X";
    match hex.starts_with(lowercase_hex_prefix) || hex.starts_with(uppercase_hex_prefix) {
        true => Ok(hex.trim_start_matches(lowercase_hex_prefix).trim_start_matches(uppercase_hex_prefix).to_string()),
        false => Ok(hex.to_string()),
    }
}

pub fn maybe_pad_hex(hex: &str) -> String {
    match hex.chars().collect::<Vec<char>>().len() % 2 {
        0 => hex.to_string(),
        _ => format!("0{}", hex),
    }
}

pub fn validate_hex(hex: &str) -> Result<String> {
    match hex::decode(hex) {
        Ok(_) => Ok(hex.to_string()),
        Err(_) => Err(AppError::Custom("Could not decode hex - please check your input!".to_string()))
    }
}

pub fn validate_prefix_hex_length(hex: &str) -> Result<String> {
    if hex.chars().collect::<Vec<char>>().len() <= PREFIX_MAX_LENGTH {
        Ok(hex.to_string())
    } else {
        Err(AppError::Custom(format!("Your hex prefix should be <= {}!", PREFIX_MAX_LENGTH)))
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_strip_hex_prefix() {
        let prefixed_hex = "0xc0ffee";
        let expected_result = "c0ffee";
        let result = maybe_strip_hex_prefix(&prefixed_hex).unwrap();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_not_strip_non_existing_hex_prefix() {
        let prefixed_hex = "c0ffee";
        let result = maybe_strip_hex_prefix(&prefixed_hex).unwrap();
        assert_eq!(result, prefixed_hex);
    }

    #[test]
    fn should_maybe_pad_hex_if_odd_in_length() {
        let odd_length_hex = "decaf";
        let expected_result = "0decaf";
        let result = maybe_pad_hex(odd_length_hex);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_not_pad_hex_if_length_is_even() {
        let odd_length_hex = "c0ffee";
        let result = maybe_pad_hex(odd_length_hex);
        assert_eq!(result, odd_length_hex);
    }

    #[test]
    fn should_validate_valid_hex() {
        let valid_hex = "c0ffee";
        let result = validate_hex(&valid_hex).unwrap();
        assert_eq!(result, valid_hex);
    }

    #[test]
    fn should_error_when_validating_invalid_hex() {
        let invalid_hex = "coffee";
        let expected_error = "Could not decode hex - please check your input!";
        match validate_hex(&invalid_hex) {
            Err(AppError::Custom(err)) => assert_eq!(err, expected_error),
            Err(e) => panic!("Wrong error recieved: {}", e),
            Ok(_) => panic!("Should not have succeeded!"),
        }
    }

    #[test]
    fn should_validate_prefix_length() {
        let good_length_prefix = "c0ffee";
        assert!(good_length_prefix.chars().collect::<Vec<char>>().len() <= PREFIX_MAX_LENGTH);
        let result = validate_prefix_hex_length(&good_length_prefix);
        assert!(result.is_ok());
    }

    #[test]
    fn should_fail_to_validate_bad_length_prefix() {
        let good_length_prefix = "c0ffeec0ffeec0ffeec0ffeec0ffeec0ffee";
        let expected_error = format!("Your hex prefix should be <= {}!", PREFIX_MAX_LENGTH);
        assert!(good_length_prefix.chars().collect::<Vec<char>>().len() > PREFIX_MAX_LENGTH);
        match validate_prefix_hex_length(&good_length_prefix) {
            Err(AppError::Custom(err)) => assert_eq!(err, expected_error),
            Err(err) => panic!("Wrong error: {}", err),
            Ok(_) => panic!("Should not have succeeded!"),
        }
    }
}
