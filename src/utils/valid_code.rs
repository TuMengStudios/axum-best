use rand::random;

/// Generates a numeric verification code of the specified length
///
/// # Arguments
/// * `len` - Code length
///
/// # Returns
/// * `String` - Generated verification code string
pub fn gen_valid_code(len: usize) -> String {
    let mut code = String::with_capacity(len);

    for i in 0..len {
        // Generate a random digit (0-9)
        let mut digit = (random::<u8>() % 10).to_string();

        // If the first digit is 0, regenerate to avoid starting with 0
        if i == 0 && digit.contains("0") {
            digit = (random::<u8>() % 10).to_string()
        }

        // Append the digit to the verification code string
        code.push_str(&digit);
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gen_valid_code() {
        let valid_code = gen_valid_code(4);
        assert_eq!(valid_code.len(), 4);
        println!("{valid_code:?}");
    }
}
