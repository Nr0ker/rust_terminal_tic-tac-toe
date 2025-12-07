use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {

    let allowed_characters = ["x", "o"];

    let mut taken_field_places: HashMap<String, String> = HashMap::from([
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

    // let mut taken_field_places: HashMap<String, String> = HashMap::new()



    let mut loop_count: i8 = 0;

    loop{
        let mut place_choice = String::new();
        let mut character = String::new();

        loop_count += 1;

        if loop_count == 1{

            println!("Welcome to the tic-tac-toe in terminal! 
            \n The count of the field goes like this> | 1 | 2 | 3 |
                                        | 4 | 5 | 6 |
                                        | 7 | 8 | 9 |")

        }

        println!("Where do you want to place first>");
        io::stdin() 
            .read_line(&mut place_choice)
            .expect("Failed to read line");
        place_choice = place_choice.trim().to_string();


        println!("What character? Allowed characters in this game {:?}", allowed_characters);
        io::stdin()
            .read_line(&mut character)
            .expect("Failed to read line");
        character = character.trim().to_string();
        

        taken_field_places.insert(place_choice, character);
            
        render2(&taken_field_places);
        
    }

    
}


fn render2(map: &HashMap<String, String>){
    // let output = String::new();

    // let mut k = String::from("1");

    // for (place, value) in map.iter() {

    //     if map.contains_key(&k){
            
    //         // println!( println!("\n              | {} | {} | {} |
    //         //   | {} | {} | {} | 
    //         //   | {} | {} | {} |\n");)
    //     }
    //     k = (map.len() + 1).to_string();

    // }


     println!("\n    | {} | {} | {} |
    | {} | {} | {} | 
    | {} | {} | {} |\n",
             map["1"], map["2"], map["3"], map["4"], map["5"], map["6"], map["7"], map["8"], map["9"]);

}

// TODO: make a win and a lose condition 
