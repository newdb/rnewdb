use std::env;
extern crate rnewdb;

fn main() {
    if let Some(ref opt) = env::args().nth(1) {
        if opt == "-c" {
            let mut client = rnewdb::LocalClient;
            println!("starting rnewdb shell");
            client.shell_loop();
        } else {
            println!("invalid option {}, only support `-c`", opt);
        }
    } else {
        rnewdb::run_server();
    }
}
