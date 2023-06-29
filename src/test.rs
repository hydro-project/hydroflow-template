use hydroflow::util::wait_for_process_output;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test() {
    let mut server = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args("--role server --addr 127.0.0.100:2048".split(' '))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut server_output = server.stdout.take().unwrap();

    let mut client = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args("--role client --server-addr 127.0.0.100:2048".split(' '))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut client_input = client.stdin.take().unwrap();
    let mut client_output = client.stdout.take().unwrap();

    let mut server_output_so_far = String::new();
    let mut client_output_so_far = String::new();

    wait_for_process_output(
        &mut server_output_so_far,
        &mut server_output,
        "Server live!\n",
    );

    wait_for_process_output(
        &mut client_output_so_far,
        &mut client_output,
        "Client live!\n",
    );

    client_input.write_all(b"Hello\n").unwrap();

    wait_for_process_output(
        &mut client_output_so_far,
        &mut client_output,
        r#"Got Echo \{ payload: "Hello", ts: .* \} from 127.0.0.100:2048"#,
    );

    server.kill().unwrap();
    client.kill().unwrap();

    server.wait().unwrap();
    client.wait().unwrap();
}
