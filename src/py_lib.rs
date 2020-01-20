mod action;
mod client;
mod connection;
mod high_score;
mod local;
mod random;
mod state;

use crate::action::Action;
use crate::client::Client;
use crate::high_score::HighScore;
use crate::local::Local;
use pyo3::prelude::*;

#[pyclass(name = Gridentify)]
#[text_signature = "(cls, seed)"]
#[derive(Copy, Clone)]
struct PyLocalGridentify {
    rust: Local<u64>,
}

#[pyclass(name = GridentifyClient)]
#[text_signature = "(cls, host, nickname)"]
struct PyClientGridentify {
    rust: Client,
}

#[pymethods]
impl PyLocalGridentify {
    #[new]
    fn new(obj: &PyRawObject, seed: u64) {
        obj.init({
            Self {
                rust: Local::new(seed),
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
                rust: Client::new(host, nickname),
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
        Ok(action::show_action(action))
    }

    #[pyfn(m, "server_scores")]
    #[text_signature = "(action)"]
    fn server_scores(_py: Python, host: &str) -> PyResult<Vec<HighScore>> {
        Ok(client::get_scores(host))
    }

    m.add_class::<PyLocalGridentify>()?;
    m.add_class::<PyClientGridentify>()?;
    Ok(())
}
