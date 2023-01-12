use std::process::Stdio;
use std::process::Command;
use std::io::Write;
fn main(){
    let mut call_command;
    call_command = Command::new("/home/kwin/workspace/top/TOP-relayer/main");
    call_command.stdin(Stdio::piped());
    // call_command.stdout(Stdio::piped());
    let password = String::from("pswd");
    let mut child = call_command
        .spawn()
        .expect(format!("Failed to launch").as_str());
    // std::thread::sleep(std::time::Duration::from_secs(5));
    let mut stdin = child
        .stdin
        .take()
        .expect("Failed to get password from stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(password.as_bytes())
            .expect("Failed to write to stdin");
    });
    // let output = child
    //     .wait_with_output()
    //     .expect("Failed to get result from stdout");
    // println!("{}", String::from_utf8_lossy(&output.stdout));
    std::thread::sleep(std::time::Duration::from_secs(5));
}

