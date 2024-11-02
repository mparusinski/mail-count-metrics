extern crate imap;
extern crate native_tls;

use postgres::{Client,NoTls};
use postgres::error::Error;
use std::time::SystemTime;
use num_traits::cast::ToPrimitive;

fn fetch_inbox() -> imap::error::Result<Option<u32>> {
    let domain = "127.0.0.1";
    let port = 1143;
    // Using proton mail bridge which is why invalid certs are required
    let tls = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let client = imap::connect_starttls((domain, port), domain, &tls).unwrap();

    let username = std::env::var("MAIL_USERNAME").unwrap();
    let password = std::env::var("MAIL_PASSWORD").unwrap();

    let mut imap_session = client
        .login(username, password)
        .map_err(|e| e.0)?;
    
    let mailbox = imap_session.examine("INBOX")?;
    let exists = mailbox.exists;

    // be nice to the server and log out
    let _ = imap_session.logout(); // Ignore error here because Proton is weird

    Ok(Some(exists))
}

fn write_db(value: u32) -> Result<(), Error> {
    let db_host = std::env::var("DB_HOST").unwrap();
    let db_name = std::env::var("DB_NAME").unwrap();
    let db_user = std::env::var("DB_USER").unwrap();
    let db_pass = std::env::var("DB_PASS").unwrap();

    let conn_str = std::format!(
        "host={db_host} dbname={db_name} user={db_user} password={db_pass}");
    let mut client = Client::connect(&conn_str, NoTls)?;
    let now = SystemTime::now();
    let record_value = value.to_i32().unwrap_or(0);
    client.execute(
        "INSERT INTO mail_counter (record_date, counter) VALUES ($1, $2)",
        &[&now, &record_value],
    )?; 
    Ok(())
}

fn main() {
    match fetch_inbox() {
        Ok(v) => {
            match v {
                None => println!("Error: no value obtained"),
                Some(v) => {
                    println!("{v} emails unread");
                    match write_db(v) {
                        Ok(()) => println!("Written to DB successfully"),
                        Err(e) => panic!("Problem writing to DB: {e:?}"),
                    }
                }
            }
        },
        Err(e) => panic!("Problem with connection: {e:?}"),
    }
}
