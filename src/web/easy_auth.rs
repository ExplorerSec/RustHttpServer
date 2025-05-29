use crate::utils::base64;
use base64::Base64Codec;

use std::{
    collections::HashMap,
    fs,
    path::Path,
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
    /*
    // 测试用账号密码
    fn init_test_accounts_map(&mut self) {
        let s_usr = String::from("usr_01");
        let s_pwd = String::from("pwd_01");
        self.user_password.insert(s_usr, s_pwd);
    }*/
    // 加载本地账号密码文件
    fn load_local_account_map<P: AsRef<Path>>(&mut self, path: P) -> usize {
        if let Ok(content) = fs::read_to_string(path) {
            let cc = Base64Codec::web_default();
            let lines = content.split('\n');
            for line in lines {
                if let Some((usr, pwd)) = line.trim().split_once('~') {
                    self.user_password
                        .insert(cc.decode_str(usr), cc.decode_str(pwd));
                }
            }
        }
        #[cfg(debug_assertions)]
        {
            println!("Debug info: Accounts");
            for (k, v) in &self.user_password {
                println!("usr:{:?},pwd:{:?}", k, v);
            }
        }
        self.user_password.len()
    }
    // 保存账号密码表到本地文件，未测试
    fn _save_account_map<P: AsRef<Path>>(&mut self, path: P) -> bool {
        let mut s = String::new();
        let cc = Base64Codec::web_default();
        for (k, v) in &self.user_password {
            s.push_str(&cc.encode_str(k));
            s.push('~');
            s.push_str(&cc.encode_str(v));
            s.push('\n');
        }
        fs::write(path, s).is_ok()
    }
    // 账号登录验证
    pub fn auth_accounts(&self, usr: String, pwd: String) -> Result<(), AuthError> {
        #[cfg(debug_assertions)]{
            println!("Login auth_accounts:");
            println!("---> user:{}",usr);
            println!("---> pwd: {}",pwd);
        }
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
    // 修改密码，未测试
    fn _edit_account_password(&mut self, usr: String, pwd: String) -> bool {
        if let Some(val) = self.user_password.get_mut(&usr) {
            *val = pwd;
            true
        } else {
            false
        }
    }
}

pub type Auth = Arc<Mutex<AuthInner>>;

pub fn new_auth<P: AsRef<Path>>(path: P) -> Auth {
    let mut auth_inner = AuthInner::new();
    //auth_inner.init_test_accounts_map();
    auth_inner.load_local_account_map(path);

    Arc::new(Mutex::new(auth_inner))
}
