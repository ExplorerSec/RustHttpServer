pub mod cookie;
pub mod easy_auth;

use cookie::{Cookie, AutoSessionManage, auto_session_manage};
use easy_auth::{new_auth, ArcAuth};
use std::time::Duration;

pub struct Auth {
    auth:ArcAuth,
    auto_sessions: AutoSessionManage,
}

impl Auth {
    pub fn init() -> Auth {
        let auth =new_auth("data/account.ini");
        let auto_sessions: AutoSessionManage = auto_session_manage(Duration::from_secs(1));
        Auth {
            auth,
            auto_sessions,
        }
    }

    pub fn judge_login(&self,c:&Cookie) -> bool{
        self.auto_sessions.judge(c)
    }

    pub fn login(&mut self, str: &str) -> Option<Cookie> {
        #[cfg(debug_assertions)]
        {
            println!("Login router/login");
            println!("---> text:{}", str);
        }
    
        if let Some((usr, pwd)) = str.rsplit_once('~') {
            let auth_inner = self.auth.lock().unwrap();
            if auth_inner
                .auth_accounts(usr.to_string(), pwd.trim_matches('\0').to_string())
                .is_ok(){
                    return Some(self.auto_sessions.set_quick(Duration::from_secs(3600)));
                }
        }
        None
    }

}


