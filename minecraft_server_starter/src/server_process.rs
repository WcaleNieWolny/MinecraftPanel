use std::process::exit;

use regex::Regex;
use tokio::io::{BufWriter, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::process::{ChildStdout, Child};
use tokio::io::Lines;
use tokio::sync::mpsc::{UnboundedSender, self};
use tokio::sync::watch::{self, Receiver};

pub struct ServerProcess{
    pub stdout_rx: Receiver<String>,
    stdin_tx: UnboundedSender<String>,
    process: Child
}

impl ServerProcess {
    pub async fn new(mut child: Child) -> Self{

        let std = child.stdout.take().expect("no std");

        let mut result: Lines<BufReader<ChildStdout>> = BufReader::new(std).lines();
        let (stdout_tx, stdout_rx) = watch::channel(String::new());

        tokio::spawn(async move {
            while let Some(line) = result.next_line().await.unwrap() {
                println!("{}", line.clone());

                stdout_tx.send(line).expect("Couldn't send stdout");
            };
        });
 
        let (stdin_tx, mut stdin_rx) = mpsc::unbounded_channel::<String>();
        let mut stdin = BufWriter::new(child.stdin.take().unwrap());

        tokio::spawn(async move{
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
        });

        Self{
            stdout_rx,
            stdin_tx,
            process: child
        }
    }

    fn check_process_running(process: &mut Child){
        match process.try_wait() {
            Ok(code) => {
                if code.is_some() {
                    eprint!("Child process is dead ({})", code.unwrap());
                    exit(-1)
                }
            }
            Err(_) => {}
        }
    }

    fn send_termination_signal(&mut self){
        if cfg!(unix){
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(self.process.id().unwrap() as i32),
                nix::sys::signal::Signal::SIGTERM)
                .expect("Couldn't send UNIX SIGTEM");
            println!("SEND UNIX TERMINATION SIGNAL!")
        }else{
            self.write_to_stdin("stop".to_string());
        }
    }

    pub async fn await_shutdown(&mut self){
        Self::send_termination_signal(self);
        let sleep = tokio::time::sleep(std::time::Duration::from_secs(15));

        tokio::select! {
            biased;
            exit_code = async {
                self.process.wait().await
            } => {
                println!("EXIT CODE: {}", exit_code.unwrap())
            },

            _ = async {
                sleep.await
            } => {
                println!("We couldn't get exit code of a child process! Server will proceed to kill it!")
            }
        };
    }

    pub fn write_to_stdin(&mut self, message: String){
        Self::check_process_running(&mut self.process);
        self.stdin_tx.send(message).expect("Couldn't pass message to sending task");
    }

    pub async fn read_from_stdout(&mut self) ->anyhow::Result<String>{
        Self::check_process_running(&mut self.process);

        if self.stdout_rx.changed().await.is_ok(){
            return Ok(self.stdout_rx.borrow().clone())
        };

        Err(anyhow::Error::msg("Couldn't read stdout?"))
    }

    pub fn last_stdout(&self) -> String{
        self.stdout_rx.borrow().clone()
    }

    pub async fn list_players(&mut self) -> Vec<String>{
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