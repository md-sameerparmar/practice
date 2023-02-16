if username.len() < 3 || username.len() > 12{
        is_valid = false;
        println!("Username must be between 3 to 12 characters long {}", username.len());
    }