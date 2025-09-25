use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

//2D array - 3x3
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

// Initializes the game board with empty spaces
fn initialize_board() -> Board {
    [[' '; BOARD_SIZE]; BOARD_SIZE]
}

// Prints the current state of the board in a formatted way
fn print_board(board: &Board) {
    println!("  0 1 2");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", i);
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

// Prompts the current player for their move and validates input
fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!(
            "Player {}: Enter your move as 'row col' (e.g., 1 2):",
            current_player
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();
        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            } else {
                println!("That cell is already taken or out of bounds. Try again.");
            }
        } else {
            println!("Invalid input format. Please enter two numbers separated by a space.");
        }
    }
}

// Checks if the current player has won the game
fn check_winner(current_player: char, board: &Board) -> bool {
    // Check rows
    for row in 0..BOARD_SIZE {
        if board[row][0] == current_player
            && board[row][1] == current_player
            && board[row][2] == current_player
        {
            return true;
        }
    }
    // Check columns
    for col in 0..BOARD_SIZE {
        if board[0][col] == current_player
            && board[1][col] == current_player
            && board[2][col] == current_player
        {
            return true;
        }
    }
    // Check diagonals
    if (board[0][0] == current_player
        && board[1][1] == current_player
        && board[2][2] == current_player)
        || (board[0][2] == current_player
            && board[1][1] == current_player
            && board[2][0] == current_player)
    {
        return true;
    }
    false
}

// Checks if the game is a draw (no empty cells left)
fn check_draw(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }
    true
}

// Main game loop for playing Tic-Tac-Toe
fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("\nCurrent Board:");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        if check_winner(current_player, &board) {
            println!("\nFinal Board:");
            print_board(&board);
            println!("\nPlayer {} wins! Congratulations!", current_player);
            break;
        }

        if check_draw(&board) {
            println!("\nFinal Board:");
            print_board(&board);
            println!("\nThe game is a draw!");
            break;
        }

        // Switch player
        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }
}

// Entry point of the program
fn main() {
    println!("Welcome to the Tic-Tac-Toe Game!");
    println!("Player X and Player O take turns to mark the board.");
    println!("To make a move, enter the row and column number separated by a space (e.g., 1 2).\n");
    play_game();
}
