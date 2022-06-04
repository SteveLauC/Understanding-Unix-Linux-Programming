use chrono::{DateTime, Local};
use nix::unistd::{fork, gethostname, ForkResult};
use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry, File, ReadDir};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Status {
    pub server_started: u64,
    pub server_requests: usize,
}

impl Status {
    pub fn setup() -> Self {
        Self {
            server_started: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            server_requests: 0,
        }
    }
}

pub fn process_request(header: String, client: &TcpStream, status: &Status) {
    if let ForkResult::Child = unsafe { fork().unwrap() } {
        // parse header
        let mut header: Vec<String> = header
            .split_whitespace()
            .map(|str| str.to_string())
            .collect();
        assert_eq!(header.len(), 3);

        header[1].remove(0); // remove the first `/`

        if header[0] != "GET" {
            not_implemented(client);
        } else if is_built_in(&header[1]) {
            built_in(client, &status);
        } else if not_exist(&header[1]) {
            do_404(client);
        } else if is_dir(&header[1]) {
            do_ls(&header[1], client);
        } else {
            do_cat(&header[1], client);
        }
    }
}
fn http_reply(
    stream: &mut BufWriter<&TcpStream>,
    code: u32,
    msg: &str,
    r#type: &str,
    content: Option<&str>,
) {
    stream
        .write(format!("HTTP/1.1 {} {}\r\n", code, msg).as_bytes())
        .unwrap();
    stream
        .write(format!("Content-type: {}\r\n\r\n", r#type).as_bytes())
        .unwrap();
    if let Some(con) = content {
        stream.write(format!("{}\r\n", con).as_bytes()).unwrap();
    }
}

fn not_implemented(stream: &TcpStream) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);

    // header
    http_reply(
        &mut buffered_writer,
        501,
        "Not Implemented",
        "text/plain",
        Some("That command is not implemented"),
    );
}

fn do_404(stream: &TcpStream) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);

    http_reply(
        &mut buffered_writer,
        404,
        "Not Found",
        "text/plain",
        Some("The item you seek is not here"),
    );
}

fn is_dir(file_name: &str) -> bool {
    Path::new(file_name).is_dir()
}

fn not_exist(file_name: &str) -> bool {
    eprintln!("debug: file_name: {}", file_name);
    !Path::new(file_name).exists()
}

fn file_type(file_name: &str) -> Option<&OsStr> {
    Path::new(file_name).extension()
}

fn do_ls(dir_name: &str, stream: &TcpStream) {
    let entries: ReadDir = read_dir(dir_name).unwrap();
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    http_reply(&mut buffered_writer, 200, "OK", "text/plain", None);

    for entry in entries {
        let entry: DirEntry = entry.unwrap();
        buffered_writer
            .write(entry.file_name().into_string().unwrap().as_bytes())
            .unwrap();
    }
    buffered_writer.write(b"\r\n").unwrap();
}

fn do_cat(file_name: &str, stream: &TcpStream) {
    let mut content_type: &str = "text/plain";
    if let Some(ty) = file_type(file_name) {
        let ty: &str = ty.to_str().unwrap();
        match ty {
            "html" => content_type = "text/html",
            "gif" => content_type = "image/gif",
            "jpg" => content_type = "image/jpg",
            "jpeg" => content_type = "image/jpeg",
            _ => (),
        }
    }

    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    let mut file_reader: BufReader<File> = BufReader::new(File::open(file_name).unwrap());
    let mut buf: String = String::with_capacity(300);
    file_reader.read_to_string(&mut buf).unwrap();

    http_reply(&mut buffered_writer, 200, "OK", content_type, None);
    buffered_writer.write(buf.as_bytes()).unwrap();

    buffered_writer.write(b"\r\n").unwrap();
}

fn is_built_in(arg: &str) -> bool {
    "status" == arg
}

fn show_time(seconds: u64) -> String {
    let date: DateTime<Local> = DateTime::from(UNIX_EPOCH + Duration::from_secs(seconds));
    date.format("%b %e %H:%M:%S %Y").to_string()
}
fn built_in(stream: &TcpStream, status: &Status) {
    let mut buffered_writer: BufWriter<&TcpStream> = BufWriter::new(stream);
    http_reply(&mut buffered_writer, 200, "OK", "text/plain", None);

    buffered_writer
        .write(format!("Server started: {}\n", show_time(status.server_started)).as_bytes())
        .unwrap();
    buffered_writer
        .write(format!("Server requests: {}\n", status.server_requests).as_bytes())
        .unwrap();
    buffered_writer.write(b"\r\n").unwrap();
}

pub fn hostname() -> String {
    let mut host_name: [u8; 64] = [0; 64];
    gethostname(&mut host_name)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
}
