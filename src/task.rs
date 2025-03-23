use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task{
    id:usize,
    name:String,
    done:bool,
}

impl Task {
   pub fn new(id:usize, name:String)-> Self{
    Task{
        id:id,
        name:name,
        done: false
    }
   } 

   pub fn mark_done(&mut self){
    self.done = true;
   }

   pub fn edit(&mut self, name:String){
    self.name = name;
   }
}

pub struct TaskManager{
    pub tasks: Vec<Task>,
    mark:usize,
}

impl TaskManager {
    pub fn new()->Self{
        TaskManager{
            tasks:Vec::new(),
            mark:0,
        }
    }

    pub fn add(&mut self, name:String){
        
        self.mark+=1;
        self.tasks.push(Task::new(self.mark,name));
    }

    fn is_found(&self, id :usize) -> bool{
        
        self.tasks.iter().any(|x| x.id == id)
    }

    fn get(&mut self, id:usize)-> Option<&mut Task> {
        for i in &mut self.tasks {
            if i.id == id {
                return  Some(i);
            }
        }
        None
    }

    fn get_id(&self, id:usize)->Option<usize>{
        self.tasks.iter().position(|task| task.id == id)
    }
    pub fn done(&mut self, id:usize){
        if let Some(id) = self.get_id(id) {
            self.tasks[id].mark_done();
        }else{
            println!("Indexed out of range: {id}");
        }
    }

    pub fn edit(&mut self, id:usize, name:String){
        if let Some(task) = self.get(id){
            task.edit(name);
        }else{
            println!("Indexed out of range:{id}");
        }
    }

    pub fn remove_task(&mut self, id:usize){
        if self.is_found(id){
            self.tasks.retain( |task| task.id != id);
        }
    }

    pub fn list(&self){
        for task in &self.tasks{
            println!("{:?}", task);
        }
    }

    pub fn save(&self, file_name:String){
        let file = File::create(file_name).expect("Cannot save the changes");
        let data = serde_json::to_string(&self.tasks).expect("Cannot convert the data");

        writeln!(&file, "{}",data).expect("Cannot write the data");
    }

    pub fn load(&mut self, file_name:String){
     let mut file = match File::open(&file_name){
        Ok(f) => f,
        Err(_t) =>{
            println!("Cannot open file {file_name}");
            return;
        }
     };

     let mut data = String::new();
     if let Err(_t) = file.read_to_string(&mut data) {
        println!("Cannot read the file");
        return;
     }
     
     let tasks:Vec<Task> = match serde_json::from_str(&data){
        Ok(t) => t,
        Err(_e) =>{
            println!("Cannot load the data");
            return;
        }
     };
     if let Some(weight) = tasks.last() {
         self.mark = weight.id;
     }
     self.tasks = tasks;
    }
}