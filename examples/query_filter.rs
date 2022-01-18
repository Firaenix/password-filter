use dialoguer::Input;


fn main() {
    let password: String =  Input::new()
            .with_prompt("Input password")
            .default("password123".into())
            .show_default(true)
            .allow_empty(true)
            .interact()
            .unwrap_or("password123".into());

    let result = password_filter::is_compromised_password(&password);

    println!("Is {} compromised? {}", password, result)
}