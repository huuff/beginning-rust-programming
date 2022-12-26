use sqlite;
use sqlite::State;
use sqlite::Connection;
use std::io;

pub fn add_record(conn: &Connection) -> io::Result<()> {
    let mut title = String::new();
    let mut findings = String::new();
    let mut details = String::new();
    let mut justification = String::new();

    println!("Title");
    io::stdin().read_line(&mut title)?;
    println!("Finding text");
    io::stdin().read_line(&mut findings)?;
    println!("Details of the finding");
    io::stdin().read_line(&mut details)?;
    println!("Justificaton");
    io::stdin().read_line(&mut justification)?;

    let command_string = fortmat!(r#"
        INSERT INTO FINDINGS (title, finding, details, justification)
        VALUES ("{}", "{}", "{}", "{}")
    "#, title.trim(), finding.trim(), details.trim(), justification.trim());
    let _statement = conn.execute(&command_string).unwrap();

    Ok(())
}

pub fn list_records(conn: &Connection) {
    let mut statement = conn
        .prepare("SELECT * FROM findings")
        .unwrap();

    while let State::Row = statement.next().unwrap() {
        println!("-------------------");
        println!("Title = {}", statement.read::<String>(1).unwrap());
        println!("Finding = {}", statement.read::<String>(2).unwrap());
        println!("Details = {}", statement.read::<String>(3).unwrap());
        println!("Justification = {}", statement.read::<String>(4).unwrap());
    }
}
