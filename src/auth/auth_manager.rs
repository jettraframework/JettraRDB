use std::collections::HashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const SUPER_USER: &str = "super-user";
pub const DEFAULT_SUPER_PASSWORD: &str = "superUserZ";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub role: String,
    pub databases: Vec<String>,
    pub requires_password_change: bool,
}

pub struct AuthManager {
    user_passwords: RwLock<HashMap<String, String>>,
    user_metadata: RwLock<HashMap<String, UserInfo>>,
    active_tokens: RwLock<HashMap<String, String>>, // token -> username
}

impl AuthManager {
    pub fn new() -> Self {
        let mut passwords = HashMap::new();
        let mut metadata = HashMap::new();

        passwords.insert("admin".to_string(), "admin".to_string());
        metadata.insert(
            "admin".to_string(),
            UserInfo {
                username: "admin".to_string(),
                role: "ADMIN".to_string(),
                databases: vec!["*".to_string()],
                requires_password_change: false,
            },
        );

        passwords.insert(SUPER_USER.to_string(), DEFAULT_SUPER_PASSWORD.to_string());
        metadata.insert(
            SUPER_USER.to_string(),
            UserInfo {
                username: SUPER_USER.to_string(),
                role: "SUPERUSER".to_string(),
                databases: vec!["*".to_string()],
                requires_password_change: true,
            },
        );

        Self {
            user_passwords: RwLock::new(passwords),
            user_metadata: RwLock::new(metadata),
            active_tokens: RwLock::new(HashMap::new()),
        }
    }

    pub fn login(&self, username: &str, password: &str) -> Result<(String, bool), &'static str> {
        let r_pass = self.user_passwords.read();
        if let Some(stored) = r_pass.get(username) {
            if stored == password {
                let token = Uuid::new_v4().to_string();
                let mut tokens = self.active_tokens.write();
                tokens.insert(token.clone(), username.to_string());

                let meta = self.user_metadata.read();
                let must_change = meta
                    .get(username)
                    .map(|u| u.requires_password_change)
                    .unwrap_or(false);

                return Ok((token, must_change));
            }
        }
        Err("Invalid credentials")
    }

    pub fn validate_token(&self, token: &str) -> bool {
        let tokens = self.active_tokens.read();
        tokens.contains_key(token)
    }

    pub fn get_user_from_token(&self, token: &str) -> Option<String> {
        let tokens = self.active_tokens.read();
        tokens.get(token).cloned()
    }

    pub fn change_password(
        &self,
        username: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), &'static str> {
        let mut passwords = self.user_passwords.write();
        if let Some(stored) = passwords.get_mut(username) {
            if stored == old_password {
                *stored = new_password.to_string();
                let mut meta = self.user_metadata.write();
                if let Some(u) = meta.get_mut(username) {
                    u.requires_password_change = false;
                }
                return Ok(());
            }
        }
        Err("Invalid old password or user not found")
    }

    pub fn list_users(&self) -> Vec<UserInfo> {
        let meta = self.user_metadata.read();
        meta.values().cloned().collect()
    }

    pub fn add_user(&self, username: &str, password: &str, role: &str, databases: Vec<String>) {
        let mut passwords = self.user_passwords.write();
        passwords.insert(username.to_string(), password.to_string());

        let mut meta = self.user_metadata.write();
        meta.insert(
            username.to_string(),
            UserInfo {
                username: username.to_string(),
                role: role.to_string(),
                databases,
                requires_password_change: false,
            },
        );
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

