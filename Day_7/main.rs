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

fn step1() {
    let file_path = "input.txt";

    let file = File::open(file_path)
        .expect("file not found!");

    let  buf_reader = BufReader::new(file);
    
    let root = Rc::new(RefCell::new(Dossier::new("/".to_string())));
    let mut current = Rc::clone(&root);

    for line in buf_reader.lines() {
        let line = line.unwrap();

        println!("{line}");
        if line.starts_with("$"){
            match &line[2..4]{
                "cd" => {

                    if &line[5..6] == "/"{
                        current = Rc::clone(&root);
                    }else if &line[5..7] == ".."{
                        let current_clone = Rc::clone(&current);
                        current = current_clone.borrow().get_parent().unwrap();
                    }else{
                        let current_clone = Rc::clone(&current);
                        current = current_clone.borrow().find(line[5..].to_string()).unwrap();
                    }
                },
                "ls" => {
                    println!("LS !");
                },
                _ => {
                    println!("Unknown cmd {line}");
                }
            }
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
}

fn init_tree()  {
    let root = Rc::new(RefCell::new(Dossier::new("/".to_string())));
    let current = Rc::clone(&root);

    current.borrow_mut().add_folder(Rc::new(RefCell::new(Dossier::new("b".to_string()))), Rc::clone(&current));
    current.borrow_mut().add_folder(Rc::new(RefCell::new(Dossier::new("c".to_string()))), Rc::clone(&current));
    current.borrow_mut().add_file(Rc::new(RefCell::new(Fichier::new("b".to_string(), 1))));


    let sub = current.borrow().find("b".to_string()).unwrap();
    sub.borrow_mut().add_folder(Rc::new(RefCell::new(Dossier::new("d".to_string()))), Rc::clone(&sub));
    sub.borrow_mut().add_file(Rc::new(RefCell::new(Fichier::new("b".to_string(), 85))));
    sub.borrow_mut().add_file(Rc::new(RefCell::new(Fichier::new("i".to_string(), 1))));

    let sub = sub.borrow().find("d".to_string()).unwrap();


    println!("{}", root.borrow().print());

    println!("{}", sub.borrow().get_parent().unwrap().borrow().print());

}

    

// fn step2(){
//     let file_path = "input.txt";

//     let file = File::open(file_path)
//         .expect("file not found!");

//     let  buf_reader = BufReader::new(file);

//     for line in buf_reader.lines() {
//         let line = line.unwrap();

//     }
// }

fn main(){
    // init_tree();
    step1();
    // step2();
}