use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {

    let mut place1_1= String::from(' ');
    let mut place1_2= String::from(' ');
    let mut place1_3= String::from(' ');
    let mut place2_1= String::from(' ');
    let mut place2_2= String::from(' ');
    let mut place2_3= String::from(' ');
    let mut place3_1= String::from(' ');
    let mut place3_2= String::from(' ');
    let mut place3_3= String::from(' ');
    
    let mut field_places = vec!["1","2","3","4","5","6","7","8","9"];

    let allowed_characters = ["x", "y"];

    // let mut taken_field_places = HashMap::from([
    //     ("1_1", 0),
    //     ("1_2", 0),
    //     ("1_3", 0),
    //     ("2_1", 0),
    //     ("2_2", 0),
    //     ("2_3", 0),
    //     ("3_1", 0),
    //     ("3_2", 0),
    //     ("3_3", 0),
    // ]);

    let mut taken_field_places: HashMap<String, String> = HashMap::new();

    let mut place_choice = String::new();
    let mut character = String::new();

    loop{
    
        println!("Where do you want to place first>");
        io::stdin() 
            .read_line(&mut place_choice)
            .expect("Failed to read line");
        println!("{}", place_choice);
        println!("What character? Allowed characters in this game {:?}", allowed_characters);
        
        io::stdin()
            .read_line(&mut character)
            .expect("Failed to read line");


            
        // let place = read_char();
        if place_choice.trim() == "1"{
            place_choice = String::from("1_1");
            println!("{}", place_choice);
        }
  
        taken_field_places.insert(place_choice, character);
        println!("{:?}", taken_field_places);
            
       // todo Hashmap paramether into the function so it can render the map


        render(place1_1, place1_2, place1_3, place2_1, place2_2, place2_3, place3_1, place3_2, place3_3);

    
        break;
    }

    
}
// todo: this must be a tic-tac-toe game in terminal

fn render(place1_1:String, place1_2:String, place1_3:String, place2_1:String,
     place2_2:String, place2_3:String, place3_1:String, place3_2:String, place3_3:String){

        
    println!("\n              | {} | {} | {} |
              | {} | {} | {} | 
              | {} | {} | {} |\n", place1_1, place1_2, place1_3, place2_1, place2_2, place2_3, place3_1, place3_2, place3_3);
}

// fn read_char() -> char {
//     let mut input = String::new();

//     // Flush stdout so the prompt appears before input
//     io::stdout()
//     .flush().unwrap(); // flush() immediate output with no lane buffering \n // unwrap is like .read_line().expect

//     io::stdin()
//     .read_line(&mut input).expect("Failed to read line");

//     input.trim()
//     .chars().next().expect("No character entered")
// }
