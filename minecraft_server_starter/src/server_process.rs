use tokio::io::{BufWriter, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::process::{ChildStdout, Child};
use tokio::io::Lines;
use tokio::sync::mpsc::{UnboundedSender, self};
use tokio::sync::watch::{self, Receiver};

pub struct ServerProcess{
    stdout_rx: Receiver<String>,
    stdin_tx: UnboundedSender<String>
}

impl ServerProcess {
    pub async fn new(mut child: Child) -> Self{

        let std = child.stdout.take().expect("no std");
        let mut result: Lines<BufReader<ChildStdout>> = BufReader::new(std).lines();
        let (stdout_tx, stdout_rx) = watch::channel(String::new());

        tokio::spawn(async move {
            while let Some(line) = result.next_line().await.unwrap() {
                println!("TAKS: {}", line.clone());

                stdout_tx.send(line).expect("Couldn't send stdout");
            };
        });
 
        let (stdin_tx, mut stdin_rx) = mpsc::unbounded_channel::<String>();
        let mut stdin = BufWriter::new(child.stdin.unwrap());

        tokio::spawn(async move{
            loop {
                match stdin_rx.recv().await {
                    Some(val) => {
                        stdin.write(val.as_bytes()).await.expect("Couldn't write string");
                        stdin.write_all(b"\n").await.expect("Couldn't write newline char");
                        stdin.flush().await.expect("Couldn't flush stdin");

                        println!("WRITEN!")
                    },
                    None => {
                        break;
              },
                };
            };
        });

        Self{
            stdout_rx,
            stdin_tx
        }
    }

    pub fn write_to_stdin(&mut self, message: String){
        self.stdin_tx.send(message).expect("Couldn't pass message to sending task");
    }

    pub async fn read_from_stdout(&mut self) -> String{
        if self.stdout_rx.changed().await.is_ok(){
            return self.stdout_rx.borrow().clone()
        }
        panic!("Couldn't read from stdout")
    }

    pub fn last_stdout(&mut self) -> String{
        self.stdout_rx.borrow().clone()
    }
}