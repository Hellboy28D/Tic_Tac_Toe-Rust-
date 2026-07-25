use std::io;

fn print_board(board: &[char; 9]) {
    println!();
    println!(" {} | {} | {} ", board[0], board[1], board[2]);
    println!("-----------");
    println!(" {} | {} | {} ", board[3], board[4], board[5]);
    println!("-----------");
    println!(" {} | {} | {} ", board[6], board[7], board[8]);
    println!();
}

fn check_winner(board: &[char; 9]) -> Option<char> {
    let winning_positions = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combo in winning_positions.iter() {
        let [a, b, c] = *combo;
        if board[a] == board[b]
            && board[b] == board[c]
            && board[a] != ' '
        {
            return Some(board[a]);
        }
    }

    None
}

fn is_draw(board: &[char; 9]) -> bool {
    board.iter().all(|&c| c != ' ')
}

fn main() {
    let mut board = [' '; 9];
    let mut current_player = 'X';

    println!("========================");
    println!("     TIC TAC TOE");
    println!("========================");
    println!("Positions:");
    println!(" 1 | 2 | 3 ");
    println!("-----------");
    println!(" 4 | 5 | 6 ");
    println!("-----------");
    println!(" 7 | 8 | 9 ");
    println!();

    loop {
        print_board(&board);

        println!("Player {}'s turn.", current_player);
        println!("Enter position (1-9):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let position: usize = match input.trim().parse() {
            Ok(num) if (1..=9).contains(&num) => num - 1,
            _ => {
                println!("Invalid input! Enter a number from 1 to 9.\n");
                continue;
            }
        };

        if board[position] != ' ' {
            println!("Cell already occupied!\n");
            continue;
        }

        board[position] = current_player;

        if let Some(winner) = check_winner(&board) {
            print_board(&board);
            println!("🎉 Player {} wins!", winner);
            break;
        }

        if is_draw(&board) {
            print_board(&board);
            println!("🤝 It's a draw!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}