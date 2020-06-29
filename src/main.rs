use std::{
    env::args,
    cmp::PartialEq,
    path::Path,
    fs::File,
    io::{self, BufRead, BufReader, Write, ErrorKind},
    string::String,
};


#[derive(PartialEq)]
struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: ' '
        };
    }   
}

impl PartialEq<TodoItem> for &TodoItem{
    fn eq(&self, other: &TodoItem) -> bool {
        self.name == other.name
    }
}


struct TodoList {
    list: Vec<TodoItem>
}


impl TodoList {
    fn new() -> TodoList{
        return TodoList {list: Vec::new()};
    }

    fn add_to_list(&mut self, name: String){
        let todo_item= TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print (&mut self){
        for (index, item) in self.list.iter().enumerate(){
            println!("{}[{}]-{}", index, item.completed, item.name);
        }
    }
    
    fn save_to_file(&self, filename: impl AsRef<Path>){
        let todo_list = &self;
        let mut buff = File::create(filename).expect("[ERR] creating new file!!!");
        for item in todo_list.list.iter(){
            if item != todo_list.list[0]{
                write!(buff,"\n{}{}",item.completed, item.name).expect("[ERR] writing to new file!!!");
            }else{
                write!(buff,"{}{}",item.completed, item.name).expect("[ERR] writing to new file!!!");

            }
        }
    }

    fn done(&mut self, index: usize){
        if self.list[index].completed == ' '{
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' ';
        }
    }

    fn del(&mut self, index: usize){
        self.list.remove(index);
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>>{
    let f = File::open(&filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("[ERR] Creating file {:?}!!!", error);
            })
        } else {
            panic!("[ERR] Opening file {:?}!!!", error);
        }
    });
    BufReader::new(f).lines().collect()
}

fn todo_read(filename: impl AsRef<Path>) -> TodoList{
        let lines = lines_from_file(filename).expect("[ERR] loading lines!!!");

        let mut todo_list = TodoList::new();

        for line in lines{
            let todo_item = TodoItem::new(line);
            todo_list.list.push(todo_item);
        }
        todo_list
}


enum Command {
    Read,
    Add(String),
    Del(usize),
    Done(usize),
}


fn main() {
    let args: Vec<String> = args().collect();
    
    let path = Path::new("./todo.txt");
    let mut todo_list = todo_read(path);

    let command = match args[1].as_str(){
        "read" => Command::Read,
        "add" => Command::Add(args[2].clone()),
        "del" => Command::Del(args[2].parse().unwrap()),
        "done" => Command::Done(args[2].parse().unwrap()),
        _ => panic!("[ERR] Usage: todolist read\nadd <task>\ndel <index>\ndone <index>!!!!")
    };

    match command{
        Command::Read => todo_list.print(),
        
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.save_to_file(path);
            todo_list.print();
        },
        
        Command::Done(index) => {//TODO iscitat vektor iz fajla,oznacit item, spasit vektor u fajl
            todo_list.done(index);
            todo_list.save_to_file(path);
            todo_list.print(); 
       },

        Command::Del(index) => {//TODO iscitat vektor iz fajla,oznacit item, spasit vektor u fajl
            todo_list.del(index);
            todo_list.save_to_file(path);
            todo_list.print();  
       },
    }
}

