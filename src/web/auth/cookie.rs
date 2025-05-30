use std::{
    collections::{HashMap, HashSet}, sync::{Arc, Mutex}, thread::{self, sleep}, time::{Duration, SystemTime}
};

use crate::utils::priority_collections::priority_vec::PriorityVec;

pub type Cookie = String;

#[derive(Clone, PartialEq)]
enum TimeOrForever {
    SystemTime(SystemTime), // 暂时有效，过期时间
    Forever,                // 永久有效
}

struct SessionContent {
    //cookie:String,
    //user:String,
    _time_start: SystemTime,
    time_end: TimeOrForever,
}

struct CookiesAndSessions {
    running_flag: bool,
    next_clean_time: TimeOrForever, // 最近的将有 Cookie 被清除的时间
    max_cookies_size: usize,        // 设定 PriorityVec 的最大容量，防止爆内存
    timed_cookies: PriorityVec<SystemTime, Cookie>,
    _forever_cookies: HashSet<Cookie>,
    sessions: HashMap<Cookie, SessionContent>,
}

impl CookiesAndSessions {
    fn with_max_capcity(size: usize) -> CookiesAndSessions {
        CookiesAndSessions {
            running_flag: false,
            next_clean_time: TimeOrForever::Forever,
            max_cookies_size: size,
            timed_cookies: PriorityVec::new(),
            _forever_cookies: HashSet::new(),
            sessions: HashMap::new(),
        }
    }
    pub fn default() -> CookiesAndSessions{
        CookiesAndSessions::with_max_capcity(100)
    }

    fn judge_cookie(&self,c:&Cookie) -> bool{
         self.sessions.contains_key(c)
    }

    fn insert(&mut self, c: Cookie, ss: SessionContent) -> bool {
        if self.timed_cookies.len_val() < self.max_cookies_size {
            if let TimeOrForever::SystemTime(time_end) = ss.time_end {
                match self.next_clean_time {
                    TimeOrForever::Forever => {
                        self.next_clean_time = TimeOrForever::SystemTime(time_end)
                    }
                    TimeOrForever::SystemTime(st) => {
                        if st > time_end {
                            self.next_clean_time = TimeOrForever::SystemTime(time_end);
                        }
                    }
                }
                self.timed_cookies.insert(time_end, c.clone());
            }

            self.sessions.insert(c, ss);

            true
        } else {
            false
        }
    }

    fn remove_top(&mut self) {
        if let Some(vec) = self.timed_cookies.pop() {
            for cookie in vec {
                self.sessions.remove(&cookie);
            }
        }
        let mut next_time = TimeOrForever::Forever;
        if let Some(vec) = self.timed_cookies.peek() {
            if let Some(st) = vec.first() {
                if let Some(session_content) = self.sessions.get(st) {
                    next_time = session_content.time_end.clone();
                }
            }
        }
        self.next_clean_time = next_time;
    }
   
    pub fn gen_cookie(&self) -> Option<Cookie>{
        let mut r = None;
        if let Ok(t) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
            let mut s = format!("{}",t.as_millis() % 100_000_000) as Cookie;
            while self.sessions.contains_key(&s){
                s.push('0');
            } 
            r = Some(s);
        }
        r
    }
}

pub struct AutoSessionManage {
    cs: Arc<Mutex<CookiesAndSessions>>,
}

impl AutoSessionManage {
    pub fn new() -> AutoSessionManage {
        let cookie_session = CookiesAndSessions::default();
        AutoSessionManage {
            cs: Arc::new(Mutex::new(cookie_session)),
        }
    }
    pub fn judge(&self, s:&Cookie) -> bool{
        let cs =self.cs.lock().unwrap();
        cs.judge_cookie(s)
    }
    //
    pub fn run(&mut self, duration: Duration) {
        let cs = Arc::clone(&self.cs);
        thread::spawn(move || {
            let mut tmp_cs = cs.lock().unwrap();
            tmp_cs.running_flag = true;
            drop(tmp_cs);
            loop {
                //let cs = Arc::clone(&cs);
                let mut cs = cs.lock().unwrap();
                if let TimeOrForever::SystemTime(st) = cs.next_clean_time {
                    let time_now = SystemTime::now();
                    if time_now >= st {
                        cs.remove_top();
                    }
                }
                // 退出机制
                if !cs.running_flag {
                    break;
                }
                drop(cs);
                sleep(duration);
            }
        });
    }

    pub fn _stop(&mut self) {
        let mut cs = self.cs.lock().unwrap();
        cs.running_flag = false;
    }

    fn _load_local(&mut self) {
        // 未实现
        todo!()
    }

    fn _set_cookie_with_duration(&mut self, c: Cookie, live_time: Duration) {
        let time_now = SystemTime::now();
        // 记住之后改check_add
        let time_end = time_now + live_time;
        //let cs = Arc::clone(&self.cs);
        let mut cs = self.cs.lock().unwrap();
        cs.insert(
            c,
            SessionContent {
                _time_start: time_now,
                time_end: TimeOrForever::SystemTime(time_end),
            },
        );
    }
    pub fn set_quick(&mut self, live_time: Duration) -> Cookie{
        let time_now = SystemTime::now();
        // 记住之后改check_add
        let time_end = time_now + live_time;
        //let cs = Arc::clone(&self.cs);
        let mut cs: std::sync::MutexGuard<'_, CookiesAndSessions> = self.cs.lock().unwrap();
        let c = cs.gen_cookie().unwrap();
        let cc = c.clone();
        cs.insert(
            c,
            SessionContent {
                _time_start: time_now,
                time_end: TimeOrForever::SystemTime(time_end),
            },
        );
        cc
    }

}

pub fn auto_session_manage(duration:Duration) -> AutoSessionManage{
    let mut auto = AutoSessionManage::new();
    auto.run(duration);
    auto
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
impl AutoSessionManage {
    fn show(&mut self) {
        let cs = Arc::clone(&self.cs);
        let cs = cs.lock().unwrap();
        println!(
            "len: idx {} val {}\nCookies:{:?}",
            cs.timed_cookies.len_idx(),
            cs.timed_cookies.len_val(),
            cs.sessions.keys(),
        );
    }
    fn load_test(&mut self) {
        // let cs = Arc::clone(&self.cs);
        let mut cs = self.cs.lock().unwrap();
        let time_now = SystemTime::now();

        cs.insert(
            String::from("Cookie_001"),
            SessionContent {
                _time_start: time_now,
                time_end: TimeOrForever::SystemTime(time_now + Duration::from_secs(2)),
            },
        );
        cs.insert(
            String::from("Cookie_002"),
            SessionContent {
                _time_start: time_now,
                time_end: TimeOrForever::SystemTime(time_now + Duration::from_secs(3)),
            },
        );
        cs.insert(
            String::from("Cookie_003"),
            SessionContent {
                _time_start: time_now,
                time_end: TimeOrForever::SystemTime(time_now + Duration::from_secs(2)),
            },
        );
        cs.insert(
            String::from("Cookie_004"),
            SessionContent {
                _time_start: time_now,
                time_end: TimeOrForever::SystemTime(time_now + Duration::from_secs(4)),
            },
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut auto_manage = AutoSessionManage::new();
        auto_manage.load_test();
        auto_manage.run(Duration::from_secs(1));
        for _ in 0..9 {
            auto_manage.show();
            sleep(Duration::from_secs_f32(0.5));
        }
    }

}
