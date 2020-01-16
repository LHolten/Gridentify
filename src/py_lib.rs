mod client;
mod connection;
mod database;
mod grid;
mod local;
mod random;

use crate::client::ClientGridentify;
use crate::database::Score;
use crate::grid::{Action, Gridentify};
use crate::local::LocalGridentify;
use pyo3::prelude::*;

#[pyclass(name = Gridentify)]
#[text_signature = "(cls, seed)"]
#[derive(Copy, Clone)]
struct PyLocalGridentify {
    rust: LocalGridentify<u64>,
}

#[pyclass(name = GridentifyClient)]
#[text_signature = "(cls, host, nickname)"]
struct PyClientGridentify {
    rust: ClientGridentify,
}

#[pymethods]
impl PyLocalGridentify {
    #[new]
    fn new(obj: &PyRawObject, seed: u64) {
        obj.init({
            Self {
                rust: LocalGridentify::new(seed),
            }
        });
    }

    #[text_signature = "($self)"]
    fn copy(&self) -> PyResult<Self> {
        Ok(*self)
    }

    #[text_signature = "($self)"]
    fn valid_moves(&self) -> PyResult<Vec<Action>> {
        Ok(self.rust.state.valid_moves())
    }

    #[text_signature = "($self, action)"]
    fn make_move(&mut self, action: Action) -> PyResult<()> {
        Ok(self.rust.make_move(action))
    }

    #[text_signature = "($self)"]
    fn show_board(&self) -> PyResult<()> {
        Ok(self.rust.state.show_board())
    }

    #[getter]
    fn board(&self) -> PyResult<Vec<u32>> {
        Ok(self.rust.state.board.to_vec())
    }

    #[getter]
    fn score(&self) -> PyResult<u32> {
        Ok(self.rust.state.score)
    }

    #[text_signature = "($self)"]
    fn get_neighbours_of(&self) -> PyResult<Vec<Vec<usize>>> {
        Ok(self.rust.state.get_neighbours().to_vec())
    }
}

#[pymethods]
impl PyClientGridentify {
    #[new]
    fn new(obj: &PyRawObject, host: &str, nickname: &str) {
        obj.init({
            Self {
                rust: ClientGridentify::new(host, nickname),
            }
        });
    }

    #[text_signature = "($self)"]
    fn valid_moves(&self) -> PyResult<Vec<Action>> {
        Ok(self.rust.state.valid_moves())
    }

    #[text_signature = "($self, action)"]
    fn make_move(&mut self, action: Action) -> PyResult<()> {
        Ok(self.rust.make_move(action))
    }

    #[text_signature = "($self)"]
    fn show_board(&self) -> PyResult<()> {
        Ok(self.rust.state.show_board())
    }

    #[getter]
    fn board(&self) -> PyResult<Vec<u32>> {
        Ok(self.rust.state.board.to_vec())
    }

    #[getter]
    fn score(&self) -> PyResult<u32> {
        Ok(self.rust.state.score)
    }

    #[text_signature = "($self)"]
    fn get_neighbours_of(&self) -> PyResult<Vec<Vec<usize>>> {
        Ok(self.rust.state.get_neighbours().to_vec())
    }
}

#[pymodule]
pub fn gridentify(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "show_move")]
    #[text_signature = "(action)"]
    fn show_move(_py: Python, action: Action) -> PyResult<()> {
        Ok(grid::show_move(action))
    }

    #[pyfn(m, "server_scores")]
    #[text_signature = "(action)"]
    fn server_scores(_py: Python, host: &str) -> PyResult<Vec<Score>> {
        Ok(client::get_scores(host))
    }

    m.add_class::<PyLocalGridentify>()?;
    m.add_class::<PyClientGridentify>()?;
    Ok(())
}
