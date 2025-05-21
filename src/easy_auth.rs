//mod base64;
//use base64::Base64Codec;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub enum AuthError {
    AccountNotFound,
    PasswordError,
}

pub struct AuthInner {
    user_password: HashMap<String, String>,
}

impl AuthInner {
    // 创建账号表 账号-口令密码
    pub fn new() -> AuthInner {
        AuthInner {
            user_password: HashMap::new(),
        }
    }
    // 测试用账号密码
    fn init_test_accounts_map(&mut self) {
        let s_usr = String::from("usr_01");
        let s_pwd = String::from("pwd_01");
        self.user_password.insert(s_usr, s_pwd);
    }
    // 账号登录验证
    pub fn auth_accounts(&self, usr: String, pwd: String) -> Result<(), AuthError> {
        if let Some(password) = self.user_password.get(&usr) {
            if pwd == *password {
                Ok(())
            } else {
                Err(AuthError::PasswordError)
            }
        } else {
            Err(AuthError::AccountNotFound)
        }
    }
}

pub type Auth = Arc<Mutex<AuthInner>>;

pub fn new_auth() -> Auth {
    let mut auth_inner = AuthInner::new();
    auth_inner.init_test_accounts_map();

    Arc::new(Mutex::new(auth_inner))
}
