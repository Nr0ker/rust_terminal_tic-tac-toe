use std::io;
use std::collections::HashMap;

fn main() {



    let mut field_places: HashMap<String, String> = HashMap::from([
        ("1".to_string(), " ".to_string()),
        ("2".to_string(), " ".to_string()),
        ("3".to_string(), " ".to_string()),
        ("4".to_string(), " ".to_string()),
        ("5".to_string(), " ".to_string()),
        ("6".to_string(), " ".to_string()),
        ("7".to_string(), " ".to_string()),
        ("8".to_string(), " ".to_string()),
        ("9".to_string(), " ".to_string()),
    ]);
    // let result: String = numbers.iter().map(|n| n.to_string()).collect();
    // places on a field


    // a combination of vield places when a player wins


    let mut loop_count: i8 = 0;

    // count for a loop to tell how many rounds there is and for some conditions

    loop{


        let mut place_choice = String::new();
        let mut character = String::new();

        loop_count += 1;

        println!("Round {}", loop_count);

        if loop_count == 1{

            println!("Welcome to the tic-tac-toe in terminal! 
            \n The count of the field goes like this> | 1 | 2 | 3 |
                                        | 4 | 5 | 6 |
                                        | 7 | 8 | 9 |")

        }
        if loop_count == 10{
            println!("The round has ended, lets see who is the winner>");

            count_the_winner(&field_places);

            break;
        }

        loop {
            println!("Where do you want to place>\n");

            place_choice.clear();
            io::stdin()
                .read_line(&mut place_choice)
                .expect("Failed to read line");
            let place_choice = place_choice.trim();

            // Parse & validate
            match place_choice.parse::<i8>() {
                Ok(n) =>{
                    if n > 9 || n <= 0 {
                    println!("Please enter the value in the range of numbers from 1 to 9");
                    continue;
                    }           
                }
                _ => {
                    println!("Please enter the value in the range of numbers from 1 to 9");
                    continue;
                }
            };

            // Check if occupied
            // let occupied = field_places.iter().any(|(place, _)| place == place_choice);
           let mut occupied = false;
            for (place, value) in &field_places {
                if place == place_choice && value != " " {
                    println!("This place is already occupied, please enter a valid place");
                    occupied = true;
                    break; // stop checking further
                }
            }

            if occupied {

                continue; // now this continues the outer loop  
            }
          

            break;
            
        }
    




        loop{

            println!("What character? Allowed characters in this game are - x - or - o -");

            io::stdin()
                .read_line(&mut character)
                .expect("Failed to read line");
            character = character.trim().to_string();
            if character == "x"{
                break;
            }
            else if character == "o"{
                break;
            }
            else{
                println!("Please enter the valid character");
                character.clear();
            }
            

        }
        

        field_places.insert(place_choice, character);
            
        field_render(&field_places);
        
    }

    
}


fn field_render(map: &HashMap<String, String>){
    println!("\n    | {} | {} | {} |
    | {} | {} | {} | 
    | {} | {} | {} |\n",
                    map["1"], map["2"], map["3"],
    map["4"], map["5"], map["6"], 
    map["7"], map["8"], map["9"]);

}

fn count_the_winner(map: &HashMap<String, String>){
    let win_combination: [&str; 8] = ["123", "456", "789", "147", "258", "369", "159", "357"];

    let mut amount_x: i8 = 0;
    let mut amount_o: i8 = 0;

    let mut x_placed: Vec<&String> = Vec::new();
    let mut o_placed: Vec<&String> = Vec::new();

    for (place, value) in map{
        if value == "x"{
            amount_x += 1;
            x_placed.push(place);
        }
        else{
            amount_o += 1;
            o_placed.push(place);
        }

    }
    println!("{:?}x {:?}o", x_placed, o_placed);
}

// TODO: make a win and a lose condition 
// TODO: make normal validation and saving for a field_choice
