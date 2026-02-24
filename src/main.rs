use clap::Parser;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::fs;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key
};

use argon2::Argon2;

use rand::RngCore;


#[derive(Serialize, Deserialize)]
struct PasswordManager {
    passwords:Vec<Password>,
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading the input");

    input.trim().to_string()

}

fn get_master_password(prompt: &str) -> String {
    rpassword::prompt_password(prompt).expect("Error trying to read the password")
}

fn derive_key(password: &str, salt: &[u8])-> [u8; 32] {
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];

    argon2.hash_password_into(password.as_bytes(), salt, &mut key).expect("Error trying to derivate the key");

    key
}

fn encrypt_data(data: &[u8], key: &[u8;32]) -> Result<Vec<u8>, String> {

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, data).map_err(|e| format!("Error while encrypting: {}", e))?;

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

fn decrypt_data(encrypted: &[u8], key: &[u8;32]) -> Result<Vec<u8>, String> {
    if encrypted.len() < 12 {
        return Err("Invalid encrypted data".to_string());
    }

    let (nonce_bytes, ciphertext) = encrypted.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    cipher.decrypt(nonce, ciphertext).map_err(|_|"Master password incorrect or corrupted data".to_string())
}

fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

fn save_salt(salt: &[u8], filepath: &str) -> std::io::Result<()> {
    fs::write(filepath, salt)
}

fn load_salt(filepath: &str) -> std::io::Result<Vec<u8>> {
    fs::read(filepath)
}

impl PasswordManager {
    fn new() -> PasswordManager {
        PasswordManager { passwords: Vec::new() }
    }

    fn add(&mut self, password: Password)
    {
        self.passwords.push(password);
    }

    fn save(&self, filepath:&str, master_password: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;

        let salt_file = "lockbox.salt";
        let salt = if let Ok(existing_salt) = load_salt(salt_file) {
            println!("Using existing salt");
            existing_salt
        }else {
            println!("Generating new salt");
            let new_salt = generate_salt();
            save_salt(&new_salt, salt_file)?;
            new_salt.to_vec()
        };

        let key = derive_key(master_password, &salt);

        let encrypted = encrypt_data(json.as_bytes(), &key).map_err( |e| format!("Error while encrypting: {}", e))?;

        match fs::write(filepath, encrypted) {
            Ok(_) => {},
            Err(e) =>{
                return Err(format!("the file cannot be saved: {}",e).into());
            }
        }

        println!("✅ Passwords saved safetly");
        Ok(())
    }

    fn load(filepath: &str, master_password: &str) -> Result<PasswordManager, Box<dyn std::error::Error>> {

        let encrypted = fs::read(filepath)?;

        let salt = load_salt("lockbox.salt").map_err( |_| "There is nor salt file. Is it the first time?")?;

        let key = derive_key(master_password, &salt);

        let decrypted = decrypt_data(&encrypted, &key).map_err(|_| "❌ Master password incorrector data corrupted")?;

        let json = String::from_utf8(decrypted).map_err(|_| "Invalid data decrypted")?;

        let manager: PasswordManager = serde_json::from_str(&json)?;

        println!("✅ Passwords loaded correctly");
        Ok(manager)
    }

    fn remove(&mut self, service: &str, force: bool){
        let position = self.passwords.iter().position(|p| p.service == service);

        match position {
            Some(index) => {
                if force {
                    self.passwords.remove(index);
                    println!("✅ Password of '{}' removed", service);
                } else {
                    let confirmation = read_input(&format!("⚠️ Remove password of {}? (y/n):", service));

                    if confirmation.to_lowercase().starts_with('y') || confirmation.to_lowercase() == "yes" {
                        self.passwords.remove(index);
                        println!("✅ Password of '{}' removed", service);
                    } else {
                        println!("❌ Operación cancelled");
                    }
                }
            } None => {
                println!("❌ There is not password for: {}", service)
            }
        }
    }

    fn show(&self, service: &str){

        let found = self.passwords.iter().find(|p| p.service == service);

        match found {
            Some(pass) => {
                println!("\n🔑 Password found:");
                println!("  Service: {}", pass.service);
                println!("  User: {}", pass.username.as_ref().unwrap_or(&"(none)".to_string()));
                println!("  Password: {}", pass.password);
            }
            None => {
                println!("❌ There is not password for the service: {}", service);
            }
        }
    }

    fn list(&self, verbose:bool){
        println!("\n📋 List of passwords:\n");

        for pass in &self.passwords {
            println!("Service: {}", pass.service);
                println!("User: {}", pass.username.as_ref().unwrap_or(&"(ninguno)".to_string()));

                if verbose {
                    println!("Password: {}", pass.password);
                } else {
                    println!("Password (hidden) {}", "*".repeat(pass.password.len()));
                }
                println!("");
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Password {
    service:String,
    username:Option<String>,
    password:String
}
#[derive(Parser)]
enum Command {
    
    Add {
        
        #[clap(short, long, value_name = "SERVICE")]
        service: String,
        
        
        #[clap(short, long, value_name = "USER")]
        username: Option<String>,
        
        
        #[clap(short, long, value_name = "PASSWORD")]
        password: String,
    },
    
    
    Generate {
        
        #[clap(value_name = "LONGITUD", default_value = "16")]
        length: usize,  
    },
    
    
    List {
        
        #[clap(short, long)]
        verbose: bool,
    },
    
    Remove {
        
        #[clap(value_name = "SERVICE")]
        service: String,
        
        #[clap(short, long)]
        force: bool,
    },
    
    Show {
        #[clap(value_name = "SERVICE")]
        service: String,
    },
}


#[derive(Parser)]
#[clap(
    name = "lockbox", 
    version = "0.1.0",
    author = "Tu Nombre",
    about = "🔐 Un gestor de contraseñas seguro desde la terminal"
)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}


fn main() {

    let args = Args::parse();

    let filepath = "passwords.enc";

    let master_password = get_master_password("🔐 type your master password");

    let mut manager = match PasswordManager::load(filepath, &master_password) {
        Ok(m) => {
            println!("✅ Passwords loaded from: {}", filepath);
            m
        }
        Err(e) => {
            println!("⚠️ cannot load {}", e );
            println!("📝 Creating a new lockbox...");
            PasswordManager::new()
        }
    };

    match args.command {
        Command::Add { service, username, password } => {
            println!("📝 Adding password for: {}", service);
            println!("   Username: {}", username.as_ref().unwrap_or(&"(ninguno)".to_string()));
            println!("   Password: {}", "*".repeat(password.len()));

            manager.add(Password { service, username, password});

            if let Err(e) = manager.save(filepath, &master_password) {
                eprintln!("❌ Error al guardar: {}", e);
            }else {
                println!("✅ Password added!");
            }
        }
        
        Command::Generate { length } => {
            println!("🎲 Creating Password of {} chacarters...", length);
        }
        
        Command::List { verbose } => {
            manager.list(verbose);
        }
        
        Command::Remove { service, force } => {
            manager.remove(&service, force);

            if let Err(e) = manager.save(filepath, &master_password) {
                eprintln!("❌ Error al guardar: {}", e);
            }
        }
        
        Command::Show { service } => {
            manager.show(&service);
        }
    }
}
