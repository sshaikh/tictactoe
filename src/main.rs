use std::collections::HashMap;
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

    println!("turn\tint\tdraws\txwins\towins");
    let states = play_games(vec![initial_board], 'X', 1);
    println!(
        "total\t\t{}\t{}\t{}",
        states.get(&TicTacToeState::Draw).unwrap().len(),
        states.get(&TicTacToeState::XWin).unwrap().len(),
        states.get(&TicTacToeState::OWin).unwrap().len()
    );
}

fn print_states(turn: u8, states: &HashMap<TicTacToeState, Vec<State>>) {
    println!(
        "{}\t{}\t{}\t{}\t{}",
        turn,
        states.get(&TicTacToeState::Intermediate).unwrap().len(),
        states.get(&TicTacToeState::Draw).unwrap().len(),
        states.get(&TicTacToeState::XWin).unwrap().len(),
        states.get(&TicTacToeState::OWin).unwrap().len()
    );
}

fn play_games(states: Vec<State>, player: char, turn: u8) -> HashMap<TicTacToeState, Vec<State>> {
    let next_turns = states
        .into_iter()
        .flat_map(|state| generate_next_turns(state, player))
        .collect::<Vec<State>>();
    let mut classified = classify(next_turns);

    //print status here
    print_states(turn, &classified);

    let intermediate = classified.remove(&TicTacToeState::Intermediate).unwrap();
    if !intermediate.is_empty() {
        let mut result = play_games(intermediate, get_next_player(player), turn + 1);
        classified
            .get_mut(&TicTacToeState::Draw)
            .unwrap()
            .append(result.get_mut(&TicTacToeState::Draw).unwrap());
        classified
            .get_mut(&TicTacToeState::XWin)
            .unwrap()
            .append(result.get_mut(&TicTacToeState::XWin).unwrap());
        classified
            .get_mut(&TicTacToeState::OWin)
            .unwrap()
            .append(result.get_mut(&TicTacToeState::OWin).unwrap());
    }
    classified
}

fn get_next_player(player: char) -> char {
    if player == 'X' {
        return 'O';
    }
    'X'
}

fn classify(states: Vec<State>) -> HashMap<TicTacToeState, Vec<State>> {
    let mut result = HashMap::new();
    result.insert(TicTacToeState::Intermediate, Vec::new());
    result.insert(TicTacToeState::Draw, Vec::new());
    result.insert(TicTacToeState::XWin, Vec::new());
    result.insert(TicTacToeState::OWin, Vec::new());
    for state in states {
        result.get_mut(&get_end_state(&state)).unwrap().push(state);
    }
    result
}

fn get_ord_from_coords(row: usize, col: usize) -> u8 {
    u8::try_from(col + (3 * row) + 1).unwrap()
}

fn generate_next_turns(state: State, player: char) -> Vec<State> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TicTacToeState {
    Intermediate,
    Draw,
    XWin,
    OWin,
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
    (state[0][0] == player && state[1][1] == player && state[2][2] == player)
        || (state[2][0] == player && state[1][1] == player && state[0][2] == player)
}

fn get_end_state(state: &State) -> TicTacToeState {
    if check_win_for(&state.board, 'X') {
        return TicTacToeState::XWin;
    }
    if check_win_for(&state.board, 'O') {
        return TicTacToeState::OWin;
    }
    if state.path.len() == 9 {
        return TicTacToeState::Draw;
    }
    TicTacToeState::Intermediate
}
