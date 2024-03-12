use std::io;

fn main() {
    let mut board = [[' '; 3]; 3]; 

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut gracz = true;
    let mut tura = 0;
 
    loop {
    
        tura += 1;
        
        if tura == 10 {
            print(&board);
            println!("REMIS");
            break;
        }
        
        print(&board);
        let cords: (usize, usize) = loop {
            let kordy: (usize, usize) = play(gracz);

            if kordy.0 > 2 || kordy.1 > 2 {
                println!("Błędne dane");
            } else if board[kordy.0][kordy.1] != ' ' {
                println!("Miejsce jest zajęte");
            } else {
                // println!("Koordynaty to {:?}", kordy);
                break kordy;
            }
        };
        
        println!("Koordynaty: {:?}", cords);
        if gracz == true {
            board[cords.0][cords.1] = 'X';
            
            let wynik: bool = check(&board, cords, true);
            
            if wynik == true {
                print(&board);
                println!("X WYGRYWA!");
                break;
            }
    
            gracz = false;
        }
        else {
            board[cords.0][cords.1] = 'O';
            
            let wynik: bool = check(&board, cords, false);
            
            if wynik == true {
                print(&board);
                println!("O WYGRYWA!");
                break;
            }
            
            gracz = true;
        }
        
    }

}

fn print(board: &[[char; 3]; 3]) {
    println!("Stan gry:");
    println!("{:?}", board[0]);
    println!("{:?}", board[1]);
    println!("{:?}", board[2]);
}

fn play(player: bool) -> (usize, usize) {
    let mut user_input = String::new();
 
    if player == true {
        println!("Gra krzyżyk");
    } else {
        println!("Gra kółko");
    }
    
    println!("Podaj koordynaty:");
    println!("x:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let x: usize = input.trim().parse().expect("");

    println!("Wprowadzona liczba: {}", x);
    
    println!("y:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let y: usize = input.trim().parse().expect("");

    println!("Wprowadzona liczba: {}", y);
 
    (x, y)
}

fn check(board: &[[char; 3]; 3], cords: (usize, usize), gracz: bool) -> bool {

    let x = cords.0;
    let y = cords.1;
    
    if gracz == true {
        
        if board[x][0] == 'X' && board[x][1] == 'X'&& board[x][2] == 'X' {
            true
        }
        else if board[0][y] == 'X' && board[1][y] == 'X'&& board[2][y] == 'X'
        {
            true
        }
        else if board[0][0] == 'X' && board[1][1] == 'X'&& board[2][2] == 'X' {
        
            true
        }
        else if board[2][0] == 'X' && board[1][1] == 'X'&& board[0][2] == 'X' {
        
            true
        }
        
        else {
            false
        }
    }
    
    else {
    
        if board[x][0] == 'O' && board[x][1] == 'O' && board[x][2] == 'O' {
            true
        }
        else if board[0][y] == 'O' && board[1][y] == 'O' && board[2][y] == 'O'
        {
            true
        }
        else if board[0][0] == 'O' && board[1][1] == 'O' && board[2][2] == 'O' {
        
            true
        }
        else if board[2][0] == 'O' && board[1][1] == 'O'&& board[0][2] == 'O' {
        
            true
        }
        
        else {
            false
        }
    }
}
