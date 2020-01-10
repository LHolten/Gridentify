mod grid;

use crate::grid::{Action, Gridentify};
use pyo3::prelude::*;

#[pyclass]
#[derive(Copy, Clone)]
struct PyGridentify {
    rust: Gridentify,
}

#[pymethods]
impl PyGridentify {
    #[new]
    fn new(obj: &PyRawObject, seed: u64) {
        obj.init(PyGridentify {
            rust: Gridentify::new(seed),
        })
    }

    fn valid_moves(&self) -> PyResult<Vec<Action>> {
        Ok(self.rust.valid_moves())
    }

    fn make_move(&mut self, action: Action) -> PyResult<()> {
        Ok(self.rust.make_move(action))
    }

    fn show_board(&self) -> PyResult<()> {
        Ok(self.rust.show_board())
    }

    fn copy(&self) -> PyResult<PyGridentify> {
        Ok(*self)
    }

    #[getter]
    fn board(&self) -> PyResult<Vec<u32>> {
        Ok(self.rust.board.to_vec())
    }

    #[getter]
    fn score(&self) -> PyResult<u64> {
        Ok(self.rust.score)
    }

    fn get_neighbours_of(&self) -> PyResult<Vec<Vec<usize>>> {
        Ok(self.rust.get_neighbours_of().to_vec())
    }
}

#[pymodule]
pub fn gridentify(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "show_move")]
    fn show_move(_py: Python, action: Action) -> PyResult<()> {
        Ok(grid::show_move(action))
    }

    m.add_class::<PyGridentify>()?;
    Ok(())
}
