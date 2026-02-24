# Lockbox - Secure Password Manager CLI

A command-line password manager built with Rust that uses military-grade AES-256-GCM encryption and Argon2 key derivation to keep your passwords safe.

## Instructions for Build and Use

### Building the Software

Steps to build the software:

1. Install Rust toolchain from https://rustup.rs/
2. Clone this repository and navigate to the project directory
3. Build the optimized binary:
   ```bash
   cargo build --release
   ```

The executable will be in:
   - **Windows**: `target\release\lockbox.exe`
   - **Linux/Mac**: `target/release/lockbox`

### Running the Program

You have three options to run the program:

#### Option 1: Development Mode (with Cargo)

Use this during development or if you haven't built the release binary:

```bash
cargo run -- add -s github -u myuser -p secret123
cargo run -- list -v
cargo run -- show github
```

#### Option 2: Run Compiled Binary Directly

After building with `cargo build --release`, execute the binary directly:

**Windows (PowerShell):**
```powershell
.\target\release\lockbox.exe add -s github -u myuser -p secret123
.\target\release\lockbox.exe list -v
.\target\release\lockbox.exe show github
```

**Linux/Mac:**
```bash
./target/release/lockbox add -s github -u myuser -p secret123
./target/release/lockbox list -v
./target/release/lockbox show github
```

#### Option 3: Install Globally (Recommended)

Install the binary to your system PATH:

```bash
cargo install --path .
```

Now you can use `lockbox` from anywhere:

```bash
lockbox add -s github -u myuser -p secret123
lockbox list -v
lockbox show github
```

The binary will be installed to:
- **Windows**: `C:\Users\YourUser\.cargo\bin\lockbox.exe`
- **Linux/Mac**: `~/.cargo/bin/lockbox`

### Commands Reference

1. **Add a password**: `lockbox add -s <service> -u <username> -p <password>`
   - Example: `lockbox add -s github -u myuser -p secret123`
   - Username is optional: `lockbox add -s wifi -p mypassword`
   - You'll be prompted for your master password (invisible typing)

2. **List all passwords**: `lockbox list`
   - Add `-v` or `--verbose` flag to show passwords in plain text: `lockbox list -v`

3. **Show specific password**: `lockbox show <service>`
   - Example: `lockbox show github`

4. **Remove a password**: `lockbox remove <service>`
   - Add `--force` flag to skip confirmation: `lockbox remove github --force`

5. **Generate random password**: `lockbox generate <length>`
   - Example: `lockbox generate 16`
   - Default length is 16 characters

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