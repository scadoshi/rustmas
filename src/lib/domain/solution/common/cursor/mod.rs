//! A position on a grid that moves around it without leaving it.

pub mod cell_on_grid;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("expected a cell on the grid")]
pub struct CellOutOfBounds;
