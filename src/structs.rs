//Structs used to create custom data types

struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

struct Person{
    first_name: String,
    last_name: String,
}

impl Person {
    //Method to construct Person
    fn new(first: &str, last: &str) -> Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Method to print full name. &self refers to Person
    fn full_name(&self) -> String{
        //format is similar to println but it doesn't print
        format!("{} {}",self.first_name,self.last_name)
    }

    //Method to change last name
    fn set_last_name(&mut self, last: &str){
        self.last_name=last.to_string();
    }

    //Create a person tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

//Another way to define structs
//struct Color(u8,u8,u8);

pub fn run(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };

    //let mut c = Color (255,0,0);
    //println!("{:?}", c.0, c.1, c.2 );


    c.red = 150;

    println!("{:?}", (c.red,c.green,c.blue) );

    let mut p = Person::new("Kevin","Benni");
    println!("{} {}",p.first_name, p.last_name );
    println!("{}", p.full_name());

    p.set_last_name("Thysserry");

    println!("{}",p.full_name());

    println!("{:?}", p.to_tuple());
}
