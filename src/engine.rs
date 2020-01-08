use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{ffi::OsStr, process::Stdio, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    time::timeout,
};

pub struct Engine {
    pub token: String,
    command: Command,
    child: Option<Child>,
}

impl Engine {
    pub fn new<I, S>(program: &str, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let secret: String = thread_rng().sample_iter(&Alphanumeric).take(8).collect();
        let token = format!("token:{}", secret);
        let mut command = Command::new(program);
        command
            .arg("--enable-rpc=true")
            .arg(format!("--rpc-secret={}", secret))
            .args(args)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::piped());
        Engine {
            token,
            command,
            child: None,
        }
    }
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.command.arg(arg);
        self
    }
    pub async fn start(&mut self) {
        let mut child = self.command.spawn().unwrap();
        let output = child.stdout.take().unwrap();
        let reader = BufReader::new(output);
        let mut lines = reader.lines();
        let mut result = false;
        timeout(Duration::from_secs(10), async {
            while let Some(line) = lines.next_line().await.unwrap() {
                if line.contains("[ERROR]") {
                    panic!();
                };
                if line.contains("IPv4 RPC: listening on TCP port 6800") {
                    result = true;
                    break;
                }
            }
        })
        .await
        .unwrap();
        if result == false {
            panic!();
        }
        self.child = Some(child);
    }
    pub fn stop(&mut self) {
        if let Some(child) = &mut self.child {
            child.kill().unwrap();
        }
    }
}
