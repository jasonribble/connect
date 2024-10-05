use std::env;

mod commander;
mod db;
mod errors;
mod models;
mod utils;

use clap::Parser;
use commander::{Cli, Commands};
use db::{Connection, ContactRepo};
use models::{Contact, ContactBuilder};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let data_repo = Connection::new(pool);

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Create(value) => {
            let contact = Contact::new(
                value.first_name.as_deref().unwrap_or(""),
                value.last_name.as_deref().unwrap_or(""),
                value.email.as_deref().unwrap_or(""),
                value.phone_number.as_deref().unwrap_or(""),
            );

            let contact = contact.unwrap();

            let id = data_repo.create_contact(contact).await?;

            println!("Successfully saved contact {id}");
        }
        Commands::Edit(value) => {
            let contact = ContactBuilder::new(
                value.id,
                value.first_name.clone(),
                value.last_name.clone(),
                value.display_name.clone(),
                value.email.clone(),
                value.phone_number.clone(),
            )
            .unwrap();

            let _ = data_repo.update_contact(contact).await;
        }
        Commands::Show => {
            let contacts = data_repo.get_all_contacts().await?;

            println!("{contacts:?}");
        }
        Commands::Get(value) => {
            let id = value.id;

            let contact = data_repo.get_contact_by_id(id).await?;

            println!("{contact:?}");
        }
        Commands::Delete(value) => {
            let id = value.id;

            let contact_id = data_repo.delete_contact_by_id(id).await?;

            println!("Successfully deleted contact {contact_id}")
        }
    }

    Ok(())
}
