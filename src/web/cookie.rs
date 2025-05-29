use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::{Duration, SystemTime},
};

use crate::utils::priority_collections::priority_vec::PriorityVec;

type Cookie = String;

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
    next_time: TimeOrForever, // 最近的将有 Cookie 被清除的时间
    max_cookies_size: usize,  // 设定 PriorityVec 的最大容量，防止爆内存
    timed_cookies: PriorityVec<SystemTime, Cookie>,
    _forever_cookies: HashSet<Cookie>,
    sessions: HashMap<Cookie, SessionContent>,
}

impl CookiesAndSessions {
    fn with_max_capcity(size: usize) -> CookiesAndSessions {
        CookiesAndSessions {
            next_time: TimeOrForever::Forever,
            max_cookies_size: size,
            timed_cookies: PriorityVec::new(),
            _forever_cookies: HashSet::new(),
            sessions: HashMap::new(),
        }
    }
    fn insert(&mut self, c: Cookie, ss: SessionContent) -> bool {
        if self.timed_cookies.len_val() < self.max_cookies_size {
            if let TimeOrForever::SystemTime(time_end) = ss.time_end {
                match self.next_time {
                    TimeOrForever::Forever => self.next_time = TimeOrForever::SystemTime(time_end),
                    TimeOrForever::SystemTime(st) => {
                        if st > time_end {
                            self.next_time = TimeOrForever::SystemTime(time_end);
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
        self.next_time = next_time;
    }
}

struct AutoManage {
    cs: Arc<Mutex<CookiesAndSessions>>,
}

impl AutoManage {
    fn new() -> AutoManage {
        let cookie_session = CookiesAndSessions::with_max_capcity(100);
        AutoManage {
            cs: Arc::new(Mutex::new(cookie_session)),
        }
    }

    fn load_local(&mut self) {
        // 未实现
        todo!()
    }
    
    fn set(&mut self, c: Cookie, live_time: Duration) {
        let time_now = SystemTime::now();
        // 记住之后改check_add
        let time_end = time_now + live_time;
        let cs = Arc::clone(&self.cs);
        let mut cs = cs.lock().unwrap();
        cs.insert(
            c,
            SessionContent {
                _time_start: time_now,
                time_end: TimeOrForever::SystemTime(time_end),
            },
        );
    }
    
    fn check_and_clean(&mut self, duration: Duration) {
        let cs = Arc::clone(&self.cs);
        thread::spawn(move || {
            loop {
                let cs = Arc::clone(&cs);
                let mut cs = cs.lock().unwrap();
                if let TimeOrForever::SystemTime(st) = cs.next_time {
                    let time_now = SystemTime::now();
                    if time_now >= st {
                        cs.remove_top();
                    }
                }
                /*
                #[cfg(debug_assertions)]
                {
                    if cs.next_time == TimeOrForever::Forever {
                        println!("Will Forever");
                        break;
                    }
                } */
                // 退出机制，当有线程修改最大cookie容量为0
                if cs.max_cookies_size == 0{
                    break;
                }
                drop(cs);
                sleep(duration);
            }
        });
    }

    fn stop(&mut self){
        let mut cs = self.cs.lock().unwrap();
        cs.max_cookies_size = 0;
    }
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn load_test(&mut self) {
        let cs = Arc::clone(&self.cs);
        let mut cs = cs.lock().unwrap();
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
    #[allow(dead_code)]
    #[cfg(debug_assertions)]
    fn show(&mut self) {
        println!("Waiting For lock!");
        let cs = Arc::clone(&self.cs);
        let cs = cs.lock().unwrap();
        println!(
            "len: idx {} val {}",
            cs.timed_cookies.len_idx(),
            cs.timed_cookies.len_val()
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut auto_manage = AutoManage::new();
        auto_manage.load_test();
        auto_manage.check_and_clean(Duration::from_secs(1));
        for _ in 0..5 {
            auto_manage.show();
            sleep(Duration::from_secs(1));
        }
    }
}
