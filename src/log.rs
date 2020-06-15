use loglib;
use std::sync::{Arc, Mutex, mpsc::{SyncSender, sync_channel}};
use std::str::FromStr;

pub use loglib::{Level, error, warn, info, debug, trace};

static LOG_LVL_FLAG: &str = "-lvl=";
static LOG_DST_FLAG: &str = "-logdst=";

static NEWLINE: [u8; 1] = ['\n' as u8];

pub fn init_stdout_logging(lvl: Level) {
    set_logger(vec![Arc::new(Mutex::new(std::io::stdout()))], lvl);
}

pub fn init_logging(lvl: Level, dst: String) -> std::io::Result<()> {
    let f = std::fs::File::create(dst)?;
    set_logger(vec![Arc::new(Mutex::new(f))], lvl);
    Ok(())
}

pub fn init_logging_from_args() {
    let mut lvl: Option<loglib::Level> = Some(loglib::Level::Debug);
    let mut w: Option<Arc<Mutex<dyn std::io::Write + Send>>> = None;
    for arg in std::env::args() {
        if arg.starts_with(LOG_LVL_FLAG) {
            let lvl_str = &arg[LOG_LVL_FLAG.len()..];
            if let Ok(l) = loglib::Level::from_str(&lvl_str) {
                lvl = Some(l);
            }

        } else if arg.starts_with(LOG_DST_FLAG) {
            let dst_str = &arg[LOG_LVL_FLAG.len()..];
            if let Ok(f) = std::fs::File::open(dst_str) {
                w = Some(Arc::new(Mutex::new(f)));
            }
        }
    }
    if w.is_none() {
        w = Some(Arc::new(Mutex::new(std::io::stdout())));
    }
    set_logger(vec![w.unwrap()], lvl.unwrap());
}

fn set_logger(w: Vec<Arc<Mutex<dyn std::io::Write + Send>>>, lvl: loglib::Level) {
    let logger = EmojiLogger::new(w, lvl);
    loglib::set_boxed_logger(Box::new(logger)).map(|()| loglib::set_max_level(loglib::LevelFilter::Info)).unwrap();
}


struct EmojiLogger {
    sender: SyncSender<String>,
    level: loglib::Level,
    w: Vec<Arc<Mutex<dyn std::io::Write + Send>>>,
}

impl EmojiLogger {
    fn new(w: Vec<Arc<Mutex<dyn std::io::Write + Send>>>, lvl: loglib::Level) -> Self {
        let (s, r) = sync_channel::<String>(256);
        let w_cloned = w.clone();
        std::thread::Builder::new().name("emojilog_t".to_owned()).spawn(move|| {
            for m in r {
                for wrl in &w_cloned {
                    let mut wr = wrl.lock().unwrap();
                    wr.write_all(m.as_bytes()).expect("write to log stream");
                    wr.write_all(&NEWLINE).expect("write to log stream");
                    wr.flush().unwrap();
                }
            }
        }).expect("spawning emojibomb logger thread");
        Self {
            sender: s,
            level: lvl,
            w,
        }
    }
}

impl loglib::Log for EmojiLogger {
    fn enabled(&self, metadata: &loglib::Metadata) -> bool {
        metadata.level() <= self.level
    }
    fn log(&self, record: &loglib::Record) {
        if self.enabled(record.metadata()) {
            self.sender.send(format!("[{}] {}", record.level(), record.args())).expect("send log record to log stream");
        }
    }
    fn flush(&self) {
        for wr in &self.w {
            wr.lock().unwrap().flush().unwrap();
        }
    }
}

