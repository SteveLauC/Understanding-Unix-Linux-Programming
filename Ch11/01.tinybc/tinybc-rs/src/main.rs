use nix::unistd::{close, dup2, fork, pipe, read, ForkResult};
use nix::sys::wait::wait;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::prelude::CommandExt;
use std::process::{exit, Command};
use std::str;

fn main() {
    let (to_dc_read_end, to_dc_write_end): (RawFd, RawFd) = pipe().unwrap();
    let (from_dc_read_end, from_dc_write_end): (RawFd, RawFd) = pipe().unwrap();

    match unsafe { fork() } {
        Err(_) => (),
        Ok(res) => match res {
            ForkResult::Child => {
                close(to_dc_write_end).unwrap();
                close(from_dc_read_end).unwrap();
                be_dc(to_dc_read_end, from_dc_write_end);
            }
            ForkResult::Parent { child: _ } => {
                close(to_dc_read_end).unwrap();
                close(from_dc_write_end).unwrap();
                be_bc(to_dc_write_end, from_dc_read_end);
                wait().unwrap();
            }
        },
    }
}

fn be_dc(to_dc_read_end: RawFd, from_dc_write_end: RawFd) {
    // redirect stdout to from_dc_write_end
    dup2(from_dc_write_end, 1).unwrap();
    close(from_dc_write_end).unwrap();

    // redirect stdin to to_dc_read_end
    dup2(to_dc_read_end, 0).unwrap();
    close(to_dc_read_end).unwrap();

    // execute `dc`
    Command::new("dc").arg("-").exec();
}

fn be_bc(to_dc_write_end: RawFd, from_dc_read_end: RawFd) {
    let mut message: String = String::with_capacity(10);
    let mut result: [u8; 1024] = [0; 1024];
    let mut fpout: File = unsafe { File::from_raw_fd(to_dc_write_end) };

    // read user input
    print!("tinyby: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut message).unwrap();
    if message.is_empty() {
        exit(0)
    };
    loop {
        let user_input: Vec<&str> = message.split_whitespace().collect();
        assert_eq!(user_input.len(), 3);
        fpout
            .write(
                format!(
                    "{}\n{}\n{}\np\n",
                    user_input[0], user_input[2], user_input[1]
                )
                .as_bytes(),
            )
            .unwrap();
        fpout.flush().unwrap();

        read(from_dc_read_end, &mut result).unwrap();

        // print the result to stdout
        println!(
            "{} {} {} = {}",
            user_input[0],
            user_input[1],
            user_input[2],
            str::from_utf8(&result).unwrap()
        );

        message.clear();
        print!("tinyby: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut message).unwrap();
        if message.is_empty() {
            exit(0)
        };
    }
}
