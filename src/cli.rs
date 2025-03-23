use crate::task::TaskManager;
use clap::{Parser, Subcommand};
use std::io::{self,Write};

#[derive(Parser)]
struct Cli{
    #[command(subcommand)]
    actions:Actions,
}

#[derive(Subcommand)]
enum Actions {
    Add { name: String },
    Done { id: usize },
    Edit { id: usize, name: String },
    Remove { id: usize },
    List,
    Save { file_name: String },
    Load { file_name: String },
    Exit,
}

pub fn start(){
    
    let mut tasks = TaskManager::new();
    
    loop{

        println!("=> ");

        io::stdout().flush().unwrap();
        
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Cannot read command");

        let args = std::iter::once("dummy").chain(input.trim().splitn(2, ' '));

        let cli = match Cli::try_parse_from(args){
            Ok(cli) => cli,
            Err(e) =>{
                println!("Cannot read argument: {}", e);
                continue;
            }
        };

        match cli.actions{
            Actions::Add{name} => tasks.add(name),
            
            Actions::Done{id} => tasks.done(id),
            
            Actions::Edit{id, name}=> tasks.edit(id, name.to_string()),
            
            Actions::Remove{id}=> tasks.remove_task(id),
            
            Actions::List => tasks.list(),
            
            Actions::Save{file_name}=> tasks.save(file_name),
            
            Actions::Load{file_name}=> tasks.load(file_name),
            
            Actions::Exit=> {
                println!("Exiting...");
                break;
            }
        }
    }

}