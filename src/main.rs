pub mod puzzle;
use ncurses::*;
use puzzle::Puzzle;

fn main() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    let mut puzzle = Puzzle::new();

    let winner = play_game(&mut puzzle);

    clear();

    if winner {
        addstr("Congratulations! You solved the puzzle!\n").unwrap();
    } else {
        addstr("Thanks for playing!\n").unwrap();
    }

    addstr(&format!("\nNumber of moves: {}\n", puzzle.get_moves())).unwrap();

    addstr("\nPress any key to exit.\n").unwrap();
    getch();

    endwin();
}

fn play_game(puzzle: &mut Puzzle) -> bool {
    let mut victory = false;
    puzzle.shuffle();
    loop {
        addstr("Use the arrow keys or H/J/K/L to move the tiles, or Q to quit.\n").unwrap();
        addstr(&format!("{}\n", puzzle.to_string())).unwrap();
        addstr(&format!("Moves: {}\n", puzzle.get_moves())).unwrap();
        let ch = getch();
        if ch == 'q' as i32 {
            break;
        }
        puzzle.handle_input(ch);
        if puzzle.is_solved() {
            victory = true;
            break;
        }
        clear();

        refresh();
    }

    victory
}
