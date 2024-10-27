use csv::ReaderBuilder;
use reqwest::blocking::get;
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Function to read records from the database
pub fn read_records(conn: &Connection) -> Result<Vec<(String, i32, String, String, String)>> {
    let mut stmt =
        conn.prepare("SELECT url, ID, Federation, Form_Fed, TransferDate FROM chess_transfers")?;
    let records = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    })?;

    records.collect()
}

// Function to update a record in the database
pub fn update_record(
    conn: &Connection,
    id: i32,
    new_url: &str,
    new_federation: &str,
    new_form: &str,
    new_transfer_date: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE chess_transfers SET url = ?, Federation = ?, Form_Fed = ?, TransferDate = ? WHERE ID = ?",
        params![new_url, new_federation, new_form, new_transfer_date, id],
    )?;
    Ok(())
}

// Function to delete a record from the database
pub fn delete_record(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM chess_transfers WHERE ID = ?", params![id])?;
    println!("Row successfully deleted - {}", id);
    Ok(())
}

pub fn load_csv_to_db(csv_file_path: &str, db_file_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(csv_file_path)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Connect to the SQLite database
    let conn = Connection::open(db_file_path)?;

    conn.execute("DELETE FROM chess_transfers", []);
    // Prepare the SQL statement
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chess_transfers (
            url TEXT NOT NULL,
            ID INTEGER ,
            Federation VARCHAR(10) ,
            Form_Fed VARCHAR(10),
            TransferDate VARCHAR(30)
        )",
        [],
    )?;

    // Insert each record from the CSV into the database
    for result in rdr.records() {
        let record = result?;

        // Check if the record has the expected number of fields
        if record.len() < 5 {
            eprintln!("Skipping record, not enough fields: {:?}", record);
            continue; // Skip this record if it doesn't have enough fields
        }

        // Extract fields assuming the CSV follows the specified order
        let url = &record[0]; // assuming url is in the first column
        let id: i32 = match record[1].parse() {
            Ok(parsed_id) => parsed_id, // Successfully parsed, use the value
            Err(err) => {
                eprintln!(
                    "Failed to parse ID from record: {:?}, error: {}",
                    record, err
                );
                continue; // Skip this record on error
            }
        };
        let federation = &record[2]; // assuming Federation is in the third column
        let form_fed = &record[3]; // assuming Form is in the fourth column
        let transfer_date = &record[4]; // assuming TransferDate is in the fifth column

        // Insert the record into the database
        conn.execute(
            "INSERT INTO chess_transfers (url, ID, Federation, Form_Fed, TransferDate) VALUES (?, ?, ?, ?, ?)",
            params![url, id, federation, form_fed, transfer_date],
        )?;
    }

    Ok(())
}

// Function to read a record by ID
pub fn read_record_by_id(
    conn: &Connection,
    id: i32,
) -> Result<Option<(String, i32, String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT url, ID, Federation, Form_Fed, TransferDate FROM chess_transfers WHERE ID = ?",
    )?;

    // Use query_row and handle the result with pattern matching
    match stmt.query_row(params![id], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    }) {
        Ok(record) => Ok(Some(record)), // Return Some(record) if found
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None), // Return None if no record found
        Err(err) => Err(err.into()),    // Propagate other errors
    }
}

pub fn insert_record(
    conn: &Connection,
    url: &str,
    id: i32,
    federation: &str,
    form_fed: &str,
    transfer_date: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO chess_transfers (url, ID, Federation, Form_Fed, TransferDate) VALUES (?, ?, ?, ?, ?)",
        params![url, id, federation, form_fed, transfer_date],
    )?;
    Ok(())
}

pub fn connect_db(database_conn: &str) -> Result<Connection> {
    // Establishing a connection to the database
    let conn = Connection::open(database_conn)?;
    Ok(conn)
}

pub fn extract_csv(
    url: &str,
    file_path: &str,
    directory: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Check if the directory exists, if not, create it
    if !Path::new(directory).exists() {
        create_dir_all(directory)?;
    }

    // Send a GET request to the provided URL
    let response = get(url)?;
    let mut file = File::create(format!("{}/{}", directory, file_path))?;
    file.write_all(&response.bytes()?)?;

    Ok(file_path.to_string())
}

pub fn all() -> Result<()> {
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/chess-transfers/transfers.csv";
    let file_path = "chess_transfers.csv";
    let directory = "data";

    let result = extract_csv(url, file_path, directory);

    let csv_file_path = "data/chess_transfers.csv"; // Update with your CSV file path
    let db_file_path = "data/sqlite_db.sqlite";

    // Load CSV data into the database
    let result2 = load_csv_to_db(csv_file_path, db_file_path);

    // Connect to the database to perform read, update, and delete operations
    let conn = Connection::open(db_file_path)?;

    // Read all records
    let records = read_records(&conn)?;
    for record in &records {
        println!("Record: {:?}", record);
    }

    // Read a record by ID (example ID)

    // Insert a new record
    insert_record(
        &conn,
        "https://example.com",
        9999,
        "USA",
        "New_Form",
        "2023-01-01",
    )?;
    println!("Inserted new record.");

    // Read all records again to confirm insertion
    // let updated_records = read_records(&conn)?;
    // for record in &updated_records {
    //     println!("Updated Record: {:?}", record);
    // }

    // Update a record (example ID, update values accordingly)
    update_record(
        &conn,
        100919,
        "new_url",
        "new_federation",
        "new_form",
        "new_date",
    )?;

    if let Some(record) = read_record_by_id(&conn, 100919)? {
        println!("Record with ID 1: {:?}", record);
    } else {
        println!("No record found with ID 1.");
    }
    // Delete a record (example ID)
    delete_record(&conn, 100919)?;
    if let Some(record) = read_record_by_id(&conn, 100919)? {
        println!("Record with ID 1: {:?}", record);
    } else {
        println!("No record found with ID 1.");
    }
    Ok(())
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE chess_transfers (
                url TEXT NOT NULL,
                ID INTEGER,
                Federation VARCHAR(10),
                Form_Fed VARCHAR(10),
                TransferDate VARCHAR(30)
            )",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_insert_and_read_record() {
        let conn = setup_db();
        let _ = insert_record(&conn, "http://example.com", 1, "USA", "A", "2024-01-01").unwrap();
        let record = read_record_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(record.0, "http://example.com");
        assert_eq!(record.1, 1);
        assert_eq!(record.2, "USA");
        assert_eq!(record.3, "A");
        assert_eq!(record.4, "2024-01-01");
    }

    #[test]
    fn test_update_record() {
        let conn = setup_db();
        let _ = insert_record(&conn, "http://example.com", 1, "USA", "A", "2024-01-01").unwrap();
        update_record(&conn, 1, "http://new-url.com", "CAN", "B", "2024-02-01").unwrap();

        let record = read_record_by_id(&conn, 1).unwrap().unwrap();
        assert_eq!(record.0, "http://new-url.com");
        assert_eq!(record.2, "CAN");
        assert_eq!(record.3, "B");
        assert_eq!(record.4, "2024-02-01");
    }

    #[test]
    fn test_delete_record() {
        let conn = setup_db();
        let _ = insert_record(&conn, "http://example.com", 1, "USA", "A", "2024-01-01").unwrap();
        delete_record(&conn, 1).unwrap();

        let record = read_record_by_id(&conn, 1).unwrap();
        assert!(record.is_none());
    }

    #[test]
    fn test_read_records() {
        let conn = setup_db();
        let _ = insert_record(&conn, "http://example.com", 1, "USA", "A", "2024-01-01").unwrap();
        let _ = insert_record(&conn, "http://example2.com", 2, "CAN", "B", "2024-02-01").unwrap();

        let records = read_records(&conn).unwrap();
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_load_csv_to_db() {
        let conn = setup_db();

        // Write a temporary CSV file for testing
        let csv_data = "url,ID,Federation,Form_Fed,TransferDate\n\
                        http://example.com,1,FED,FORM1,2024-01-01\n\
                        http://example.com,2,FED,FORM2,2024-02-01\n";

        let csv_path = "data/test_data.csv";
        std::fs::write(csv_path, csv_data).expect("Unable to write test CSV file");

        // Call the function to load CSV into DB
        let result = load_csv_to_db(csv_path, "data/test.db");
        assert!(result.is_ok());

        // Clean up the test CSV file
        std::fs::remove_file(csv_path).expect("Unable to delete test CSV file");
    }
}
