use array_init;

use pyo3::prelude::*;

type Board = [u32; 25];

type Action = Vec<usize>;

#[pyclass]
#[derive(Copy, Clone)]
struct Gridentify {
    seed: u64,
    #[pyo3(get)]
    score: u64,
    board: Board,
}

#[pymethods]
impl Gridentify {
    #[new]
    fn new(obj: &PyRawObject, mut seed: u64) {
        obj.init({
            Gridentify {
                seed,
                score: 0,
                board: Gridentify::new_board(&mut seed),
            }
        })
    }

    fn valid_moves(&self) -> PyResult<Vec<Action>> {
        let neighbours_of = self.get_neighbours_of_rust();
        let mut moves = Vec::new();

        for i in 0..25 {
            Gridentify::discover_for(&mut moves, &neighbours_of, &vec![i])
        }

        Ok(moves)
    }

    fn make_move(&mut self, action: Action) -> PyResult<()> {
        self.board[*action.last().unwrap()] *= action.len() as u32;
        for &tile in action[..action.len() - 1].iter() {
            self.board[tile] = Gridentify::new_num(&mut self.seed);
        }

        self.score += self.board[*action.last().unwrap()] as u64;
        Ok(())
    }

    fn show_board(&self) -> PyResult<()> {
        for i in 0..5 {
            println!("{:?}", &self.board[i * 5..i * 5 + 5]);
        }
        Ok(())
    }

    fn copy(&self) -> PyResult<Gridentify> {
        Ok(*self)
    }

    #[getter]
    fn board(&self) -> PyResult<Vec<u32>> {
        Ok(self.board.to_vec())
    }

    #[link_name = "get_neighbours_of"]
    fn get_neighbours_of(&self) -> PyResult<Vec<Vec<usize>>> {
        Ok(self.get_neighbours_of_rust().to_vec())
    }
}

impl Gridentify {
    fn new_board(seed: &mut u64) -> Board {
        array_init::array_init(|_| Gridentify::new_num(seed))
    }

    fn new_num(seed: &mut u64) -> u32 {
        let e = (16807 * *seed) % 1924421567;
        *seed = if e > 0 { e } else { e + 3229763266 };
        ((e % 3) + 1) as u32
    }

    fn get_neighbours_of_rust(&self) -> [Vec<usize>; 25] {
        array_init::array_init(|i| {
            let value = self.board[i];
            let mut neighbours = Vec::new();
            let x = i % 5;
            let y = i / 5;
            if x < 4 && self.board[i + 1] == value {
                neighbours.push(i + 1)
            }
            if y < 4 && self.board[i + 5] == value {
                neighbours.push(i + 5)
            }
            if x > 0 && self.board[i - 1] == value {
                neighbours.push(i - 1)
            }
            if y > 0 && self.board[i - 5] == value {
                neighbours.push(i - 5)
            }
            neighbours
        })
    }

    fn discover_for(moves: &mut Vec<Action>, neighbours_of: &[Vec<usize>; 25], action: &Action) {
        for neighbour in neighbours_of[*action.last().unwrap()].iter() {
            if !action.contains(neighbour) {
                let mut branch = action.clone();

                branch.push(*neighbour);

                Gridentify::discover_for(moves, neighbours_of, &branch);

                moves.push(branch);
            }
        }
    }
}

#[pymodule]
pub fn gridentify(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "show_move")]
    fn show_move(_py: Python, action: Action) -> PyResult<()> {
        let mut board = [0; 25];
        for (order, tile) in action.iter().enumerate() {
            board[*tile] = order + 1;
        }
        for i in 0..5 {
            println!("{:?}", &board[i * 5..i * 5 + 5]);
        }
        Ok(())
    }

    m.add_class::<Gridentify>()?;
    Ok(())
}
