# Lockbox - Secure Password Manager CLI

A command-line password manager built with Rust that uses military-grade AES-256-GCM encryption and Argon2 key derivation to keep your passwords safe.

## Instructions for Build and Use

Steps to build and/or run the software:

1. Install Rust toolchain from https://rustup.rs/
2. Clone this repository and navigate to the project directory
3. Run `cargo build --release` to compile the optimized binary
4. The executable will be in `target/release/lockbox` (or `lockbox.exe` on Windows)

Instructions for using the software:

1. **Add a password**: `cargo run -- add -s <service> -u <username> -p <password>`
   - Example: `cargo run -- add -s github -u myuser -p secret123`
   - You'll be prompted for your master password (invisible typing)

2. **List all passwords**: `cargo run -- list`
   - Add `-v` flag to show passwords in plain text: `cargo run -- list -v`

3. **Show specific password**: `cargo run -- show <service>`
   - Example: `cargo run -- show github`

4. **Remove a password**: `cargo run -- remove <service>`
   - Add `--force` flag to skip confirmation: `cargo run -- remove github --force`

5. **Generate random password**: `cargo run -- generate -l <length>`
   - Example: `cargo run -- generate -l 16`

**Important Notes:**
- Your master password is required for all operations
- Two files are created: `passwords.enc` (encrypted data) and `lockbox.salt` (public salt)
- **Never delete `lockbox.salt`** - you cannot decrypt your passwords without it
- If you forget your master password, your data is permanently inaccessible (by design)

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Rust 1.70 or higher (with Cargo package manager)
* Dependencies (automatically installed via Cargo):
  - `clap = { version = "4.5", features = ["derive"] }` - Command-line argument parsing
  - `serde = { version = "1.0", features = ["derive"] }` - Serialization framework
  - `serde_json = "1.0"` - JSON serialization
  - `aes-gcm = "0.10"` - AES-256-GCM encryption
  - `argon2 = "0.5"` - Key derivation function
  - `rand = "0.8"` - Random number generation
  - `rpassword = "7.3"` - Secure password input

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Rust Book](https://doc.rust-lang.org/book/) - Official Rust programming guide
* [Clap Documentation](https://docs.rs/clap/latest/clap/) - CLI argument parsing library
* [AES-GCM Encryption](https://docs.rs/aes-gcm/latest/aes_gcm/) - Authenticated encryption
* [Argon2 Documentation](https://docs.rs/argon2/latest/argon2/) - Password hashing
* [Serde Documentation](https://serde.rs/) - Serialization/deserialization framework

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Implement password strength checker with visual feedback
* [ ] Add ability to change master password (re-encrypt all data)
* [ ] Export/import functionality (encrypted backup)
* [ ] Search passwords by username or service with fuzzy matching
* [ ] Add clipboard integration to copy passwords without displaying
* [ ] Implement password expiration reminders
* [ ] Add encrypted notes field for each password entry
* [ ] Multi-vault support (separate encrypted files for different contexts)
* [ ] Two-factor authentication integration (TOTP generator)
* [ ] Auto-lock feature after inactivity timeout