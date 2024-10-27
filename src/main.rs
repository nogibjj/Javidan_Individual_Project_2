use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Insert {
        /// The URL of the record
        url: String,
        /// The ID of the record
        id: i32,
        /// The Federation of the record
        federation: String,
        /// The Form_Fed of the record
        form_fed: String,
        /// The TransferDate of the record
        transfer_date: String,
    },
    Read {
        /// The ID of the record to read
        id: i32,
    },
    Update {
        /// The ID of the record to update
        id: i32,
        /// The new URL for the record
        new_url: String,
        /// The new Federation for the record
        new_federation: String,
        /// The new Form_Fed for the record
        new_form_fed: String,
        /// The new TransferDate for the record
        new_transfer_date: String,
    },
    Delete {
        /// The ID of the record to delete
        id: i32,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Establish database connection
    let conn = Connection::open("data/sqlite_db.sqlite")?;

    // rust_main::all()

    match args.command {
        Commands::Insert {
            url,
            id,
            federation,
            form_fed,
            transfer_date,
        } => {
            rust_main::insert_record(&conn, &url, id, &federation, &form_fed, &transfer_date)?;
            println!("Inserted record with ID: {}", id);
        }
        Commands::Read { id } => {
            match rust_main::read_record_by_id(&conn, id)? {
                Some(record) => {
                    println!("Record found: {:?}", record);
                }
                None => {
                    println!("No record found with ID: {}", id);
                }
            }
        }
        Commands::Update {
            id,
            new_url,
            new_federation,
            new_form_fed,
            new_transfer_date,
        } => {
            rust_main::update_record(&conn, id, &new_url, &new_federation, &new_form_fed, &new_transfer_date)?;
            println!("Updated record with ID: {}", id);
        }
        Commands::Delete { id } => {
            rust_main::delete_record(&conn, id)?;
            println!("Deleted record with ID: {}", id);
        }
    }

    Ok(()) 
}
