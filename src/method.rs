use hyper::body::Bytes;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Clone)]
pub struct Method {
    bin: PathBuf,
}

impl From<PathBuf> for Method {
    fn from(value: PathBuf) -> Self {
        Self { bin: value }
    }
}

impl From<String> for Method {
    fn from(value: String) -> Self {
        Self { bin: value.into() }
    }
}

impl Method {
    pub async fn execute(&self, args: Vec<String>, stdin: Option<Bytes>) {
        println!("Executing [{:?}]...", self.bin);
        println!("[{:?}] stdout:", self.bin);

        let child = Command::new(&self.bin)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        match child {
            Ok(mut child) => {
                if let Some(bytes) = stdin {
                    if let Some(mut child_stdin) = child.stdin.take() {
                        if let Err(err) = child_stdin.write_all(bytes.as_ref()).await {
                            eprintln!("Exec [{:?}] failed: STDIN IO Error: {err:?}", self.bin);
                        }
                    }
                }

                match child.wait().await {
                    Ok(status) => println!(
                        "[{:?}] exited with exit code: {}",
                        self.bin,
                        status
                            .code()
                            .map(|i| i.to_string())
                            .unwrap_or("None".into())
                    ),
                    Err(err) => eprintln!("Error: {err}"),
                }
            }
            Err(err) => {
                eprintln!("Exec [{:?}] failed: IO Error: {err:?}", self.bin);
            }
        }
    }
}
