use std::io;

fn main() {
    let mut board = [0 ; 9];
    let lines : [(usize,usize);8] = [(0,1),(3,1),(6,1),(0,3),(1,3),(2,3),(0,4),(2,2)];

    'game_loop : loop {
        println!("-----");
        for (index, value) in board.iter().enumerate() {
            match value {
                0 => {
                    print!("{} ",index)
                }
                1 => {
                    print!("{} ",'X')
                }
                2 => {
                    print!("{} ",'O')
                }
                _ => {
                    panic!("Unimplemented")
                }
            }
            if (index+1) % 3 == 0 {
                println!();
            }
        }
        println!("-----");




        for (start,step) in lines {
            if board[start] !=0 && board[start] == board[start+step] && board[start] == board[start+step+step] {
                if board[start] == 1 {
                    println!("User won!");
                } else {
                    println!("Computer won!");
                }
                break 'game_loop;
            }
        }
        let mut matches = 0;
        for value in board {
            if value != 0 {
                matches += 1;
            }
        }
        if matches >= 9 {
            println!("Game over! Draw");
            break 'game_loop;
        }

        println!("Please enter a number: ");

        // Create a mutable String to store the input
        let mut input = String::new();

        // Read input from the user
        io::stdin()
            .read_line(&mut input) // Read input and store it in `input`
            .expect("Failed to read line");

        // Trim the input and parse it to an integer
        let number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                return; // Exit the program if parsing fails
            }
        };
        if number > 8 {
            println!("Invalid range");
            continue;
        }

        if board[number] != 0 {
            println!("Position taken");
        }

        board[number] = 1;

        let mut computer_played = false;
        'computer_play : for (start,step) in lines {
            let mut matches = 0;
            for index in [start,start+step,start+step+step] {
                if board[index] == 1 {
                    matches = matches + 1;
                } else if board[index] == 2 {
                    matches = matches - 1;
                }
            }

            if matches >= 2 {
                for index in [start,start+step,start+step+step] {
                    if board[index] != 1 {
                        board[index] = 2;
                        computer_played = true;
                        break 'computer_play;
                    }
                }
            }
        }       

        if !computer_played {
            for (index, value) in board.iter().enumerate() {
                if *value != 1 {
                    board[index] = 2;
                    break;
                }
            }
        }



        println!("You entered: {}", number);
        
        


    
    }
  
}
