use std::cmp;

type State = Vec<Vec<char>>;

const _WIDTH: usize = 3;
const _HEIGHT: usize = 3;

fn main() {


    let _initial_board = vec![vec![' '; _WIDTH]; _HEIGHT];
    
    let first_turn = generate_next_turns(_initial_board, 'X');

    println!("{:?}", first_turn);

    

}

fn generate_next_turns(state: State, player: char) -> Vec<State> {

    let mut res = Vec::new();
    for (rownum, row) in state.iter().enumerate() {
    for (colnum, &c) in row.iter().enumerate() {
        if c == ' ' {
            let mut next = state.clone();
            next[rownum][colnum] = player;
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

fn check_win_for(state: State, player: char) -> bool {
    // check rows
    for row in &state {
        if row.iter().all(|&c| c == player) {
            return true;
        }
    }
    //check columns
    for colnum in 0.._WIDTH {
        if state.iter().all(|row| row[colnum] == player) {
            return true;
        }
    }
    //check forward diagonals
    for colnum in 0..(_WIDTH - _HEIGHT) {
        for rownum in 0..(_HEIGHT - _WIDTH) {
            let mut result = true;
            for offset in 0..cmp::min(_WIDTH, _HEIGHT) {
                if state[rownum+offset][colnum+offset] != player {
                    result = false;
                    break;
                }
            }
            if result {return true}
        }
    }
    //check backward diagonals
    for colnum in _WIDTH-1.._HEIGHT {
        for rownum in 0..(_HEIGHT - _WIDTH) {
            let mut result = true;
            for offset in 0..cmp::min(_WIDTH, _HEIGHT) {
                if state[rownum + offset][colnum - offset] != player {
                    result = false;
                    break;
                }
            }
            if result {return true}
        }
    }
    false
}

fn get_end_state(state: State) -> TicTacToeState {
    check_win_for(state, 'X');
    TicTacToeState::Draw
}

fn is_end_state(state: State) -> bool{
    get_end_state(state) != TicTacToeState::Intermediate
}
