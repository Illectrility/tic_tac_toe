use std::io;

fn main() {
    let mut field: [[char; 4]; 4] = [
        [' ', 'A', 'B', 'C'],
        ['1', '-', '-', '-'],
        ['2', '-', '-', '-'],
        ['3', '-', '-', '-'],
    ];
    game_loop(&mut field);
}

fn render(field: [[char; 4]; 4]) {
    print!("{}[2J", 27 as char); //clear terminal or sth idk, found it here: https://stackoverflow.com/questions/34837011/
    println!(
        "
╔════╗╔══╗╔═══╗     ╔════╗╔═══╗╔═══╗     ╔════╗╔═══╗╔═══╗
║╔╗╔╗║╚╣╠╝║╔═╗║     ║╔╗╔╗║║╔═╗║║╔═╗║     ║╔╗╔╗║║╔═╗║║╔══╝
╚╝║║╚╝ ║║ ║║ ╚╝     ╚╝║║╚╝║║ ║║║║ ╚╝     ╚╝║║╚╝║║ ║║║╚══╗
  ║║   ║║ ║║ ╔╗╔═══╗  ║║  ║╚═╝║║║ ╔╗╔═══╗  ║║  ║║ ║║║╔══╝
 ╔╝╚╗ ╔╣╠╗║╚═╝║╚═══╝ ╔╝╚╗ ║╔═╗║║╚═╝║╚═══╝ ╔╝╚╗ ║╚═╝║║╚══╗
 ╚══╝ ╚══╝╚═══╝      ╚══╝ ╚╝ ╚╝╚═══╝      ╚══╝ ╚═══╝╚═══╝
    "
    );
    for row in field {
        let mut row_render: String = String::new();
        for element in row {
            row_render = format!("{}│ {} │ ", row_render, element);
        }
        println!("╭───╮ ╭───╮ ╭───╮ ╭───╮");
        println!("{row_render}");
        println!("╰───╯ ╰───╯ ╰───╯ ╰───╯")
    }
}

fn user_input(player: char, field: &[[char; 4]; 4]) -> (usize, usize) {
    loop {
        let mut guess: String = String::new();
        println! {"Player {player}, please insert coordinates."}
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.to_uppercase();
        if guess.len() > 2 {
            let column_char: char = guess.chars().nth(0).unwrap();
            let column = field[0].iter().position(|&r| r == column_char).unwrap();
            // converting the char to u32 | stackoverflow.com/questions/43983414
            let row: usize = guess.chars().nth(1).unwrap() as usize - '0' as usize;
            return (column, row);
        }
    }
}

fn update_field(field: &mut [[char; 4]; 4], coordinates: (usize, usize), round: u8) -> &mut [[char; 4]; 4] {
    if round % 2 == 0 {field[coordinates.1][coordinates.0] = 'O';}
    else {field[coordinates.1][coordinates.0] = 'X';}
    field
}

fn check_field(field: &mut [[char; 4]; 4], coordinates: (usize, usize)) -> bool {
    if field[coordinates.1][coordinates.0] != '-' {return false;}
    else {
        return true;
    }
}

fn check_win(field: &[[char;4]; 4]) -> bool {
    for i in 1..4 {
        let row: [char; 4] = field[i];
        if (row.iter().filter(|&n| *n == 'X').count() == 3) | (row.iter().filter(|&n| *n == 'O').count() == 3) {
            return true;
        }
        if field[1][i] == field[2][i] && field[1][i] == field[3][i] && ['X', 'O'].contains(&field[1][i]) {
            return true;
        }
    }
    if field[1][1] == field[2][2] && field[2][2] == field[3][3] && field[1][1] != '-' {
        return true;
    }
    if field[1][3] == field[2][2] && field[2][2] == field[3][1] && field[1][3] != '-' {
        return true;
    }
    false
}

fn game_loop(mut field: &mut [[char; 4]; 4]) {
    println!("Please insert the coordinates like this: [COLUMN][ROW] i.e. B1 or C2");
    let mut round: u8 = 0;
    render(*field);
    loop {
        for i in 1..4 {
            if field[i].iter().filter(|&n| *n == 'O').count() > 0 {
                break;
            }
        }
        let mut player: char = 'A';
        if round%2 != 0 {
            player = 'B';
        }
        loop {
            let coordinates = user_input(player, field);
            if check_field(field, coordinates) {
                field = update_field(field, coordinates, round);
                break;
            } else {
                println!("Bruh");
            }
        }
        render(*field);
        if check_win(field) {
            println!("Congratulations! Player {} won!", player);
            break;
        }
        round += 1;
    }
}
