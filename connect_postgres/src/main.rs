extern crate postgres;
use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:password@127.0.0.1:5432/testdb",
        NoTls,
    )?;

    for row in client.query("SELECT id, name FROM users", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        println!("{} {}", id, name);
    }

    Ok(())
}
