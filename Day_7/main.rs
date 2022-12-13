use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;

// For the tree structure, I use Rc and RefCell structures
// Rc allows us to have multiple pointers on an object
// RefCell allows us to have multiple mutable pointers

// Struct that defines a file
struct Fichier {
    name: String,
    taille: u32, 
    depth: u32,
    parent: Option<Rc<RefCell<Dossier>>>,
}

// Struct that defines a folder
struct Dossier {
    name: String,
    taille: u32,
    depth: u32,
    parent: Option<Rc<RefCell<Dossier>>>,
    dossiers: Vec<Rc<RefCell<Dossier>>>,
    fichiers: Vec<Rc<RefCell<Fichier>>>,
}

impl Fichier {
    pub fn new(name: String, taille: u32) -> Fichier {
        return Fichier {
          name: name,
          taille: taille,
          depth: 0,
          parent: None,
        };
    } 
    
    pub fn print(&self) -> String {
        return String::from("\t").repeat((self.depth).try_into().unwrap()) + 
        &String::from("- ") + &self.name + &String::from(" (file size=") + &self.taille.to_string() + &String::from(")\n");
      
    }
}

impl Dossier {
    pub fn new(name: String) -> Dossier {
      return Dossier {
        name: name,
        taille: 0,
        depth: 0,
        dossiers: vec![],
        fichiers: vec![],
        parent: None,
      };
    }
  
    pub fn add_file(&mut self, new_node: Rc<RefCell<Fichier>>) {
        new_node.borrow_mut().depth = self.depth + 1;
        self.taille += new_node.borrow().taille;
        self.fichiers.push(new_node);
    }

    pub fn add_folder(&mut self, new_node: Rc<RefCell<Dossier>>, parent: Rc<RefCell<Dossier>>) {
        new_node.borrow_mut().depth = self.depth + 1;
        new_node.borrow_mut().parent = Some(parent);
        self.dossiers.push(new_node);
      }
  
    pub fn print(&self) -> String {
        return String::from("\t").repeat((self.depth).try_into().unwrap()) + 
        &String::from("- ") + &self.name + &String::from(" (dir size=") + &self.taille.to_string() + &String::from(")\n") +
           &self
            .dossiers
            .iter()
            .map(|tn| tn.borrow().print())
            .collect::<Vec<String>>()
            .join("") + 
            &self
            .fichiers
            .iter()
            .map(|tn| tn.borrow().print())
            .collect::<Vec<String>>()
            .join("");
      
    }

    pub fn find(&self, name: String) -> Result<Rc<RefCell<Dossier>>,()> {
        for c in self
            .dossiers
            .iter()
            .filter(|tn| tn.borrow().name == name){
                //println!("{}",c.borrow().print());
                return Ok(Rc::clone(c));
            }
        Err(())
    }

    pub fn get_parent(&self) -> Result<Rc<RefCell<Dossier>>,()>{
        match &self.parent {
            Some(p) => {Ok(Rc::clone(p))}
            None => {Err(())}
        }
    }
}

fn update_size(dossier: Rc<RefCell<Dossier>>) -> u32 {

    let mut to_add = 0;
    if dossier.borrow().dossiers.len() != 0{
        for d in &dossier.borrow_mut().dossiers {
            to_add += update_size(Rc::clone(d));
        }
    }
    dossier.borrow_mut().taille += to_add;

    // for file in &dossier.borrow().fichiers {
    //     dossier.borrow_mut().taille += file.borrow().taille;
    // }
    dossier.borrow().taille
}

fn find_folders_less_than(dossier: Rc<RefCell<Dossier>>, limit: u32) -> u32{
    let mut to_add = 0;
    if dossier.borrow().dossiers.len() != 0{
        for d in &dossier.borrow_mut().dossiers {
            to_add += find_folders_less_than(Rc::clone(d), limit);
        }
    }

    if dossier.borrow().taille < limit {
        println!("{} {}",dossier.borrow().name, dossier.borrow().taille);
        return to_add + dossier.borrow().taille;
    }
    to_add
}

fn find_folders_greater(dossier: Rc<RefCell<Dossier>>, limit: u32) -> u32{
    let mut to_add = 0;
    if dossier.borrow().dossiers.len() != 0{
        for d in &dossier.borrow_mut().dossiers {
            to_add += find_folders_greater(Rc::clone(d), limit);
        }
    }

    if dossier.borrow().taille > limit {
        println!("{} {}",dossier.borrow().name, dossier.borrow().taille);
        return to_add + dossier.borrow().taille;
    }
    to_add
}

fn step1() {
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    
    let root = Rc::new(RefCell::new(Dossier::new("/".to_string())));
    let mut current = Rc::clone(&root);

    // Build the file system based on input.txt
    for line in buf_reader.lines() {
        let line = line.unwrap();

        
        if line.starts_with("$"){
            match &line[2..4]{
                "cd" => { // change the current directory ("current" variable)

                    if &line[5..6] == "/"{
                        current = Rc::clone(&root);
                    }else if &line[5..] == ".."{
                        let current_clone = Rc::clone(&current);
                        current = current_clone.borrow().get_parent().unwrap();
                    }else{
                        let current_clone = Rc::clone(&current);
                        current = current_clone.borrow().find(line[5..].to_string()).unwrap();
                    }
                },
                "ls" => {
                    //println!("LS !");
                },
                _ => {
                    println!("Unknown cmd {line}");
                }
            }

        // If it's not a command, then it's a file or a folder
        // Let's add it to our file system
        }else{ 
            let arr = line.split(" ").collect::<Vec<&str>>();
            if arr[0] == "dir"{
                current.borrow_mut().add_folder(Rc::new(RefCell::new(
                    Dossier::new(arr[1].to_string())
                )), Rc::clone(&current));
            }else {
                current.borrow_mut().add_file(Rc::new(RefCell::new(
                    Fichier::new(arr[1].to_string(), arr[0].parse::<u32>().unwrap())
                )));
            }
        }
       
    }
    println!("{}", root.borrow().print());

    // Once the filee system (fs) is complete, we can compute the size of each folder
    // by browsing from leaves to root
    update_size(Rc::clone(&root));

    println!("{}", root.borrow().print());

    // Now our fs is up to date, we just need to browse and find the folder smaller than 100000
    println!("{}",find_folders_less_than(Rc::clone(&root), 100000));
}   

fn step2() {
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    
    let root = Rc::new(RefCell::new(Dossier::new("/".to_string())));
    let mut current = Rc::clone(&root);

    // Build the file system based on input.txt
    for line in buf_reader.lines() {
        let line = line.unwrap();

        
        if line.starts_with("$"){
            match &line[2..4]{
                "cd" => { // change the current directory ("current" variable)

                    if &line[5..6] == "/"{
                        current = Rc::clone(&root);
                    }else if &line[5..] == ".."{
                        let current_clone = Rc::clone(&current);
                        current = current_clone.borrow().get_parent().unwrap();
                    }else{
                        let current_clone = Rc::clone(&current);
                        current = current_clone.borrow().find(line[5..].to_string()).unwrap();
                    }
                },
                "ls" => {
                    //println!("LS !");
                },
                _ => {
                    println!("Unknown cmd {line}");
                }
            }

        // If it's not a command, then it's a file or a folder
        // Let's add it to our file system
        }else{ 
            let arr = line.split(" ").collect::<Vec<&str>>();
            if arr[0] == "dir"{
                current.borrow_mut().add_folder(Rc::new(RefCell::new(
                    Dossier::new(arr[1].to_string())
                )), Rc::clone(&current));
            }else {
                current.borrow_mut().add_file(Rc::new(RefCell::new(
                    Fichier::new(arr[1].to_string(), arr[0].parse::<u32>().unwrap())
                )));
            }
        }
       
    }
    println!("{}", root.borrow().print());

    // Once the filee system (fs) is complete, we can compute the size of each folder
    // by browsing from leaves to root
    update_size(Rc::clone(&root));

    println!("{}", root.borrow().print());

    println!("Used space: {} free space: {} need to remove {}", root.borrow().taille,70000000 - root.borrow().taille, 30000000 - (70000000 - root.borrow().taille));

    // Now our fs is up to date, we just need to browse and find the folder smaller than 100000
    let limit =  30000000 - (70000000 - root.borrow().taille);
    println!("{}",find_folders_greater(Rc::clone(&root), limit));
}

fn main(){
    //step1();
    step2();
}