#[cfg(test)]
mod tests {
    extern crate wasm_bindgen_test;
    use password_filter::is_known_compromised_password;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();


    #[wasm_bindgen_test]
    fn test_common_passwords() {
        assert!(is_known_compromised_password("123456"), "123456");
        assert!(is_known_compromised_password("123456789"), "123456789");
        assert!(is_known_compromised_password("qwerty"), "qwerty");
        assert!(is_known_compromised_password("password"), "password");
        assert!(is_known_compromised_password("1111111"), "1111111");
        assert!(is_known_compromised_password("12345678"), "12345678");
        assert!(is_known_compromised_password("abc123"), "abc123");
        assert!(is_known_compromised_password("1234567"), "1234567");
        assert!(is_known_compromised_password("password1"), "password1");
        assert!(is_known_compromised_password("Iloveyou"), "Iloveyou");
        assert_eq!(is_known_compromised_password("21e8"), false, "21e8");
    }
}