use std::cmp;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
struct State {
    board: Vec<Vec<char>>,
    path: Vec<u8>,
}

fn main() {

    let initial_board = State {
        board: vec![vec![' '; 3]; 3],
        path: Vec::new(),
    };
    
    let first_turn = generate_next_turns(&initial_board, 'X');

    println!("{:?}", first_turn);

    

}

fn get_ord_from_coords(row: usize, col: usize) -> u8 {
    u8::try_from(col + (3*row) + 1).unwrap()
}

fn generate_next_turns(state: &State, player: char) -> Vec<State> {

    let mut res = Vec::new();
    for (rownum, row) in state.board.iter().enumerate() {
        for (colnum, &c) in row.iter().enumerate() {
            if c == ' ' {
                let mut next = state.clone();
                next.board[rownum][colnum] = player;
                next.path.push(get_ord_from_coords(rownum, colnum));
                res.push(next);
            }
        }
    }

    res

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TicTacToeState{
    Intermediate,
    Won(char),
    Draw
}

fn check_win_for(state: &Vec<Vec<char>>, player: char) -> bool {
    // check rows
    for row in state {
        if row.iter().all(|&c| c == player) {
            return true;
        }
    }
    //check columns
    for colnum in 0..3 {
        if state.iter().all(|row| row[colnum] == player) {
            return true;
        }
    }
    //check forward diagonals
    (state[0][0] == player && state[1][1] == player && state[2][2] == player) ||
    (state[2][0] == player && state[1][1] == player && state[0][2] == player) 
}

fn get_end_state(state: &State) -> TicTacToeState {
    if check_win_for(&state.board, 'X')  {return TicTacToeState::Won('X');}
    if check_win_for(&state.board, 'O')  {return TicTacToeState::Won('O');}
    if state.path.len() == 9 {return TicTacToeState::Draw;}
    TicTacToeState::Intermediate
}

fn is_end_state(state: &State) -> bool{
    get_end_state(state) != TicTacToeState::Intermediate
}
