use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{self, Debug};
use std::fs::File;
use std::io::{BufWriter, Write};

use serde::Serialize;

type Board = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct State {
    board: Board,
    path: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
struct GameData {
    game: u32,
    synonyms: Vec<u32>,
}

#[derive(Debug, Clone, Serialize)]
struct GameTypeData {
    games: HashMap<u32, GameData>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TicTacToeState {
    Intermediate,
    Draw,
    XWin,
    OWin,
}

impl fmt::Display for TicTacToeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn collapse_game(game: &[u8]) -> u32 {
    let len: u32 = game.len() as u32 - 1;
    game.iter().enumerate().fold(0, |acc, (pos, &e)| {
        acc + (e as u32 * 10u32.pow(len - pos as u32))
    })
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

    //    println!("{:?}", states.get(&TicTacToeState::Draw).unwrap().get(1000).unwrap());

    let reduced: HashMap<_, _> = states
        .into_iter()
        .map(|(k, v)| (k, reduce_states(v)))
        .collect();
    println!(
        "reduced\t\t{}\t{}\t{}",
        reduced.get(&TicTacToeState::Draw).unwrap().len(),
        reduced.get(&TicTacToeState::XWin).unwrap().len(),
        reduced.get(&TicTacToeState::OWin).unwrap().len()
    );

    for (key, value) in reduced.iter() {
        let mut transformed: HashMap<u32, GameData> = HashMap::new();
        for games in value {
            println!(
                "doing games for {} of length {}",
                &key.to_string(),
                games.len()
            );
            let first_game = collapse_game(&games[0]);
            let collapsed_games = games.iter().map(|game| collapse_game(game)).collect();
            let game_data = GameData {
                game: first_game,
                synonyms: collapsed_games,
            };
            transformed.insert(first_game, game_data);
        }

        let game_type_data = GameTypeData { games: transformed };

        let file = File::create("result_".to_owned() + &key.to_string()).unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &game_type_data).unwrap();
        writer.flush().unwrap();
    }
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

fn check_win_for(state: &Board, player: char) -> bool {
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
    //check diagonals
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

fn get_next_player(player: char) -> char {
    if player == 'X' {
        return 'O';
    }
    'X'
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

fn get_ord(c: char) -> u32 {
    match c {
        'X' => 1,
        'O' => 2,
        ' ' => 3,
        unknown => panic!("No ord for {}", unknown),
    }
}

fn get_hash(board: &Board) -> u32 {
    let mut result: u32 = 0;
    for row in board {
        for col in row {
            result = 10 * result + get_ord(*col);
        }
    }
    result
}

fn rotate_board(board: &mut Board) {
    //clone first row
    let first_row: Vec<char> = board[0].clone();
    //rotate anti clockwise
    board[0][0] = board[0][2];
    board[0][1] = board[1][2];
    board[0][2] = board[2][2];

    board[1][2] = board[2][1];
    board[2][2] = board[2][0];

    board[2][1] = board[1][0];
    board[2][0] = first_row[0];
    board[1][0] = first_row[1];
}

fn reflect_board(
    board: &Board,
    x1: usize,
    y1: usize,
    x1n: usize,
    y1n: usize,
    x2: usize,
    y2: usize,
    x2n: usize,
    y2n: usize,
    x3: usize,
    y3: usize,
    x3n: usize,
    y3n: usize,
) -> Board {
    let mut reflect = board.to_owned();
    reflect[x1][y1] = board[x1n][y1n];
    reflect[x2][y2] = board[x2n][y2n];
    reflect[x3][y3] = board[x3n][y3n];

    reflect[x1n][y1n] = board[x1][y1];
    reflect[x2n][y2n] = board[x2][y2];
    reflect[x3n][y3n] = board[x3][y3];

    reflect
}

fn reflect_board_x(board: &Board) -> Board {
    reflect_board(board, 0, 0, 2, 0, 0, 1, 2, 1, 0, 2, 2, 2)
}

fn reflect_board_y(board: &Board) -> Board {
    reflect_board(board, 0, 0, 0, 2, 1, 0, 1, 2, 2, 0, 2, 2)
}

fn reflect_board_forward_diag(board: &Board) -> Board {
    reflect_board(board, 0, 0, 2, 2, 0, 1, 1, 2, 1, 0, 2, 1)
}

fn reflect_board_backward_diag(board: &Board) -> Board {
    reflect_board(board, 0, 2, 2, 0, 0, 1, 1, 0, 1, 2, 2, 1)
}

fn get_smallest_hash(board: &Board) -> (u32, bool) {
    let mut hashes = vec![get_hash(board)];
    let mut clone: Board = board.to_vec();
    // get rotations
    for _i in 0..3 {
        rotate_board(&mut clone);
        hashes.push(get_hash(&clone));
    }
    // get reflections
    hashes.push(get_hash(&reflect_board_x(board)));
    hashes.push(get_hash(&reflect_board_y(board)));
    hashes.push(get_hash(&reflect_board_forward_diag(board)));
    hashes.push(get_hash(&reflect_board_backward_diag(board)));

    let min_hash = *hashes.iter().min().unwrap();
    (min_hash, min_hash == hashes[0])
}

fn reduce_states(states: Vec<State>) -> Vec<Vec<Vec<u8>>> {
    let mut working = HashMap::new();

    for state in states {
        let (hash, _) = get_smallest_hash(&state.board);
        let paths = working.entry(hash).or_insert_with(Vec::new);
        paths.push(state.path);
    }

    let mut result = Vec::new();
    let mut its = working.keys().cloned().collect::<Vec<_>>();
    its.sort_unstable();

    for k in its {
        let mut paths = working.remove(&k).unwrap();
        paths.sort();
        result.push(paths);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{collapse_game, get_hash, get_smallest_hash, reduce_states, rotate_board, State};
    fn test_state() -> State {
        let board = vec![
            vec!['X', 'X', 'O'],
            vec!['O', 'X', 'X'],
            vec!['X', 'O', 'O'],
        ];
        let path = vec![1, 3, 5, 4, 7, 9, 2, 8, 6];
        State { board, path }
    }

    #[test]
    fn test_hash() {
        let state = test_state();
        let hash = get_hash(&state.board);

        assert_eq!(hash, 112211122);
    }

    #[test]
    fn test_rotate() {
        let mut state = test_state();
        rotate_board(&mut state.board);
        let hash = get_hash(&state.board);

        assert_eq!(hash, 212112121);
    }

    #[test]
    fn test_get_smallest_hash() {
        let mut state = test_state();
        let (hash, is_smallest) = get_smallest_hash(&state.board);
        assert_eq!(hash, 112211122);
        assert!(is_smallest);

        rotate_board(&mut state.board);
        let (hash, is_smallest) = get_smallest_hash(&state.board);

        assert_eq!(hash, 112211122);
        assert!(!is_smallest);
    }

    #[test]
    fn test_reduction() {
        let state = test_state();
        let mut state2 = test_state();
        rotate_board(&mut state2.board);
        let states = vec![state2, state];

        let reduced = reduce_states(states);

        let paths = &reduced[0];
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_collapse_game() {
        let input = vec![1, 2, 3, 4];
        let result = collapse_game(&input);

        assert_eq!(result, 1234);
    }
}
