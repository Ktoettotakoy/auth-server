// Utility to hash passwords for existing users
// Run this as: cargo run --bin zero_day

use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let passwords = vec![
        ("putpasswordhere", "admin@zeroday.com"),
    ];

    println!("Password hashes for existing users:");
    println!("=====================================");

    for (password, email) in passwords {
        let uid = uuid::Uuid::new_v4().to_string();
        match hash(password, DEFAULT_COST) {
            Ok(hashed) => {
                println!("INSERT users (id, email, password_hash, role) VALUES ('{}', '{}', '{}', 'admin');", uid, email, hashed);
                println!();
            }
            Err(e) => eprintln!("Error hashing password for {}: {}", email, e),
        }
    }

    println!("Copy the INSERT statements above and run them in your MySQL database.");
}
