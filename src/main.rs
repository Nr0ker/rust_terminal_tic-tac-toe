use std::io;
use std::io::Write;

fn main() {

    // let place1_1: char =  'x';
    let mut place1_2: char = ' ';
    let mut place1_3: char = ' ';
    let mut place2_1: char = ' ';
    let mut place2_2: char = ' ';
    let mut place2_3: char = ' ';
    let mut place3_1: char = ' ';
    let mut place3_2: char = ' ';
    let mut place3_3: char = ' ';
    
    let mut field_places = vec!["1","2","3","4","5","6","7","8","9"];

    let mut place_choice = String::new();

    loop{
    
        println!("Where do you want to place first>");
        io::stdin() 
            .read_line(&mut place_choice)
            .expect("Failed to read line");
        println!("What character?>");
        let place1_1 = read_char();
        
        for i in field_places{
            if place_choice == i{
                let mut field_places = field_places.remove(i);
            }
        }



        render(place1_1, place1_2, place1_3, place2_1, place2_2, place2_3, place3_1, place3_2, place3_3);

    
        break;
    }

    
}
// todo: this must be a tic-tac-toe game in terminal

fn render(place1_1:char, place1_2:char, place1_3:char, place2_1:char,
     place2_2:char, place2_3:char, place3_1:char, place3_2:char, place3_3:char){

        
    println!("\n              | {} | {} | {} |
              | {} | {} | {} | 
              | {} | {} | {} |\n", place1_1, place1_2, place1_3, place2_1, place2_2, place2_3, place3_1, place3_2, place3_3);
}

fn read_char() -> char {
    let mut input = String::new();

    // Flush stdout so the prompt appears before input
    io::stdout()
    .flush().unwrap(); // flush() immediate output with no lane buffering \n // unwrap is like .read_line().expect

    io::stdin()
    .read_line(&mut input).expect("Failed to read line");

    input.trim()
    .chars().next().expect("No character entered")
}
