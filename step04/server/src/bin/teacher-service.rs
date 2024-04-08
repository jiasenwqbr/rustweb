use std::io;

#[path = "../db_access.rs"]
mod db_access;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    Ok(())
}
