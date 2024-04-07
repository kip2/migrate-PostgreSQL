use clap::Parser;
use std::error::Error;

use crate::{
    db::{create_migration_table, get_executable_query_count, migrate, roolback},
    file::create_migration_file,
};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short = 'c', long = "create", help = "Create migrate files")]
    create: bool,

    #[arg(
        short = 'i',
        long = "init",
        help = "Create migrate table if it doesn't exist."
    )]
    init: bool,

    #[arg(
        short = 'r',
        long = "rollback",
        help = "Rollback database",
        default_value = "0"
    )]
    rollback: u64,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.create {
        create_migration_file().expect("Failed migration files");
    } else if args.init {
        create_migration_table().await;
    } else if args.rollback > 0 {
        let count = get_executable_query_count(args.rollback).await;

        if count == 0 {
            return Err("No targets available for rollback".into());
        }

        println!("Available execute rollback count: {}", count);

        if count > args.rollback {
            println!(
                "Limiting rollback to {} due to user requests",
                args.rollback
            );
            roolback(args.rollback)
                .await
                .expect("Failed rollback migrations");
        } else {
            println!("Executing {} rollbacks as requested", count);
            roolback(count).await.expect("Failed rollback migrations");
        }
    } else {
        migrate().await.expect("Failed migration");
    }

    Ok(())
}
