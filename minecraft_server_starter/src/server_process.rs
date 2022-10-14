use std::env;
use std::path::PathBuf;
use std::process::{exit, Stdio};
use std::sync::Arc;

use regex::Regex;
use rocket::fairing::AdHoc;
use tokio::io::{BufWriter, AsyncWriteExt, BufReader, AsyncBufReadExt, AsyncReadExt};
use tokio::process::{ChildStdout, Child, Command};
use tokio::io::Lines;
use tokio::sync::mpsc::{UnboundedSender, self};
use tokio::sync::watch::{self, Receiver};

use crate::config::ServerConfig;

pub struct ServerProcess{
    pub stdout_rx: Receiver<String>,
    stdin_tx: UnboundedSender<String>,
    process_id: i32,
    future_exit_code: tokio::sync::watch::Receiver<i32>,
}


impl ServerProcess {
    async fn new(mut child: Child) -> Self{

        let std = child.stdout.take().expect("no std");

        let mut result: Lines<BufReader<ChildStdout>> = BufReader::new(std).lines();
        let (stdout_tx, stdout_rx) = watch::channel(String::new());

        tokio::spawn(async move {
            while let Some(line) = result.next_line().await.unwrap() {
                println!("{}", line.clone());

                stdout_tx.send(line).expect("Couldn't send stdout");
            };
        });

        let id = child.id().unwrap() as i32;
 
        let (stdin_tx, mut stdin_rx) = mpsc::unbounded_channel::<String>();
        let mut stdin = BufWriter::new(child.stdin.take().unwrap());

        let (exit_tx, exit_rx) = watch::channel::<i32>(i32::MIN);
        tokio::spawn(async move{
            tokio::select! {
                _ = async {
                    loop {
                        match stdin_rx.recv().await {
                            Some(val) => {
                                stdin.write(val.as_bytes()).await.expect("Couldn't write string");
                                stdin.write_all(b"\n").await.expect("Couldn't write newline char");
                                stdin.flush().await.expect("Couldn't flush stdin");
                            },
                            None => {
                                break;
                            },
                        };
                    };
                } => {
                    
                },
    
                result = async {
                    child.wait().await
                } => {
                    warn!("Hello, we have a ded server process!");
                    match result{
                        Ok(val) => {
                            let val = match val.code() {
                                Some(val) => val,
                                None => 0,
                            };
                            exit_tx.send(val).unwrap();
                        },
                        Err(_) => todo!(),
                    };
                }
            };
        }); 

        Self{
            stdout_rx,
            stdin_tx,
            process_id: id,
            future_exit_code: exit_rx
        }
    }

    fn check_process_running(&self){
        match self.future_exit_code.has_changed() {
            Ok(code) => {
                if code == true {
                    error!("Child process is dead ({})", code);
                    exit(-1)
                }
            },
            Err(_) => {
                error!("Child process is likely dead");
                error!("Last known exit code: {}", *self.future_exit_code.borrow());
                exit(-1)
            },
        }
    }

    fn send_termination_signal(&self){
        if cfg!(unix){
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(self.process_id),
                nix::sys::signal::Signal::SIGTERM)
                .expect("Couldn't send UNIX SIGTERM");
            println!("SEND UNIX TERMINATION SIGNAL!")
        }else{
            self.write_to_stdin("stop".to_string());
        }
    }

    fn kill_process(&self) {
        if cfg!(unix){
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(self.process_id),
                nix::sys::signal::Signal::SIGKILL)
                .expect("Couldn't send UNIX SIGKILL");
            println!("SEND UNIX TERMINATION SIGKILL!")
        }else{
            error!("You are likely using windows! This is not officially supported. We were not able to kill subprocess!")
        }
    }

    pub async fn await_shutdown(&self){
        Self::send_termination_signal(self);
        let sleep = tokio::time::sleep(std::time::Duration::from_secs(15));
        let channel = &self.future_exit_code;
        tokio::select! {
            biased;
            _ = async {
                while let Ok(_) = channel.has_changed() {

                }
            } => {
                println!("EXIT CODE: {}", channel.borrow().clone())
            },

            _ = async {
                sleep.await
            } => {
                println!("We couldn't get exit code of a child process! Server will proceed to kill it!");
                self.kill_process();
            }
        };
    }

    pub fn write_to_stdin(&self, message: String){
        Self::check_process_running(&self);
        self.stdin_tx.send(message).expect("Couldn't pass message to sending task");
    }

    pub async fn read_from_stdout(&self) ->anyhow::Result<String>{
        Self::check_process_running(&self);
        let mut channel = self.stdout_rx.clone();

        if channel.changed().await.is_ok(){
            return Ok(self.stdout_rx.borrow().clone())
        };

        Err(anyhow::Error::msg("Couldn't read stdout?"))
    }

    pub fn last_stdout(&self) -> String{
        self.stdout_rx.borrow().clone()
    }

    pub async fn list_players(&self) -> Vec<String>{
        let mut vec = Vec::new();
        let list_patern_empty = Regex::new(r"\[[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]{1,3})? INFO]: There are [0-9]+ of a max of [0-9]+ players online:").unwrap();
        let list_patern_some = Regex::new(r"\[[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]{1,3})? INFO]: There are [0-9]+ of a max of [0-9]+ players online: [a-zA-Z]+").unwrap(); // [a-zA-Z]+

        self.write_to_stdin("list".to_string());

        let list: Vec<String> = loop {
            let val = self.read_from_stdout().await.unwrap();
            let mut player_list = Vec::new();
            
            if list_patern_some.is_match(val.as_str()) {
                let list_string = list_patern_empty.replace(val.as_str(), "");
                let list_string = list_string.trim();

                let mut push_string = String::new();

                for c in list_string.chars() {
                    if c != ' ' && c != ','{
                        push_string.push(c)
                    }else if !push_string.is_empty() {
                        player_list.push(push_string);
                        push_string = String::new()
                    }
                }

                if !push_string.is_empty() {
                    player_list.push(push_string)
                }

                break player_list
            }

            if list_patern_empty.is_match(val.as_str()){
                println!("NO PLAYERS!");
                break Vec::new();
            }
        };

        vec.extend(list);

        vec
    }
}

async fn prepere_server_jar(servrer_config: &ServerConfig) -> anyhow::Result<PathBuf>{
    let mut path = env::current_dir().unwrap();
    path.push("run");

    tokio::fs::create_dir_all(path).await?;

    let mut path = env::current_dir().unwrap();
    path.push("run/srv.jar");

    println!("EXIST: {}", path.exists());

    let version = servrer_config.version.replace("paper-", "");
    let build_id = make_paper_build_id_request(&version).await?;

    download_paper_jar(&version, build_id, &path).await?;

    Ok(path)
}

async fn make_paper_build_id_request(version: &String) -> anyhow::Result<i64>{
    let url = format!("https://api.papermc.io/v2/projects/paper/versions/{}", version);
    let builds_response = reqwest::Client::new()
    .get(url).send().await?;

    let build_json = builds_response.json::<serde_json::Value>().await?;

    let builds = &build_json["builds"];
    let builds = builds.as_array().expect("Coudln't get builds when fetching paper builds");

    let latest_build = &builds[builds.len() - 1];
    let latest_build = latest_build.as_i64().expect("Couln't convert version to i64");

    println!("Fetched latest paper build as {}", latest_build);

    Ok(latest_build)
}

async fn download_paper_jar(version: &String, build_id: i64, path: &PathBuf) -> anyhow::Result<()>{
    let url = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/paper-{}-{}.jar", version, build_id, version, build_id);

    let builds_response = reqwest::Client::new()
    .get(url).send().await?;
    let builds_response = builds_response.bytes().await?;

    let mut data = builds_response.take(builds_response.len() as u64);

    //let a = builds_response.iter().collect();
    //https://stackoverflow.com/questions/44438059/how-to-convert-from-stdiobytes-to-u8

    let mut file = tokio::fs::File::create(path).await?;

    tokio::io::copy(&mut data, &mut file).await?;
    //tokio::io::copy(&mut builds_response, file).await?;

    Ok(())
}

pub async fn stage() -> AdHoc{
    AdHoc::on_ignite("Process Process", |rocket| async {

        let config = rocket.state::<ServerConfig>().unwrap();
        let server_jar_path = prepere_server_jar(config).await.expect("Couldn't prepare server jar!");

        println!("VER: {}", config.version);
    
        let server_jar_path_str = server_jar_path.to_str().expect("Couldn't format server jar path to string");
        let server_jar_pwd_path = server_jar_path.parent().expect("Coudln't find parrent path for server jar");
    
        let cmd = Command::new("java")
            .arg("-Dterminal.jline=false")
            .arg("-Dterminal.ansi=true")
            .arg("-jar")
            .arg(server_jar_path_str)
            .arg("nogui")
            .current_dir(server_jar_pwd_path)
            .stdout(Stdio::piped()) // Can do the same for stderr
            .stdin(Stdio::piped())
            .spawn()
            .expect("cannot spawn");
    
        let server_process = ServerProcess::new(cmd).await;
    
        let stdout_rx = server_process.stdout_rx.clone(); 
    
        let server_process = Arc::new(server_process);
    
        let server_process_clone = server_process.clone();

        tokio::spawn(async move {
            let mut lines = BufReader::new(tokio::io::stdin()).lines();
            'main: loop{
                while let Some(line) = match lines.next_line().await {
                    Ok(v) => v,
                    Err(_) => break 'main,
                } {
                    server_process_clone.write_to_stdin(line);
                }
            }
        });

        rocket
            .manage(server_process)
            .manage(stdout_rx)
    })
}