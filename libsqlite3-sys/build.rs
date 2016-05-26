extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/sqlite3.c")
        .include("src")
        .compile("libsqlite3.a");
}
