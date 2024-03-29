# Understanding-Unix-Linux-Programming
Source code of Understanding Unix/Linux Programming. The book provides example code in C, I would like to replicate it in Rust.

### Environment
```shell
$ uname -a
Linux pop-os 5.17.5-76051705-generic #202204271406~1651504840~22.04~63e51bd SMP PREEMPT Mon May 2 15: x86_64 x86_64 x86_64 GNU/Linux
$ ldd --version
ldd (Ubuntu GLIBC 2.35-0ubuntu3) 2.35
$ gcc --version
gcc (Ubuntu 11.2.0-19ubuntu1) 11.2.0
$ rustc --version
rustc 1.61.0 (fe5b13d68 2022-05-18)
```

> For the version info of external crate used, you can find it in `Cargo.toml` in each 
corresponding project folder.

### Note
Few rust implementations are buggy or may not work at all. Any help would be much
appreciated.

| project| state | reason or bug behaviour |
| -------|-------|--------|
| [Ch7 03.ticker_demo](https://github.com/SteveLauC/Understanding-Unix-Linux-Programming/blob/main/Ch7/03.ticker_demo/ticker_demo.rs)| Unimplemented| `getitimer` and `setitimer` are missing in crate `libc`|
| [Ch7 05.bounce](https://github.com/SteveLauC/Understanding-Unix-Linux-Programming/tree/main/Ch7/05.bounce)|Unimplemented|`getitimer` and `setitimer` are missing in crate `libc`|
| [Ch12 01.socklib socklib.rs](https://github.com/SteveLauC/Understanding-Unix-Linux-Programming/blob/main/Ch12/01.socklib/socklib-rs/src/lib.rs)|Unimplemented| socklib.c is just some encapsulations of POSIX socket APIs, there is no need to do this in Rust cuz the API provided by std is pretty good|
| [Ch14 04.twordcount twordcount4-rs](https://github.com/SteveLauC/Understanding-Unix-Linux-Programming/blob/main/Ch14/04.twordcount/twordcount4-rs/src/main.rs)|Unimplemented|I don't know how to use condition variable in Rust|
| [Ch14 07.tanimate tanimate-rs](https://github.com/SteveLauC/Understanding-Unix-Linux-Programming/blob/main/Ch14/07.tanimate/tanimate-rs/src/main.rs)|does not work|IDK:(|


Enjoy:)
