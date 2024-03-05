//! Grid inspection data used for showing where the grid lines are placed
//!
use crate::util::sys::GridTrackVec;

use super::GridTrack;

/// Inspection data for the grid layout
#[derive(Debug, Copy, Clone)]
pub struct GridInspect {
    /// Grid rows (f32::MAX when not set)
    rows: [f32; Self::MAX_LANES],
    /// Grid columns (f32::MAX when not set)
    columns: [f32; Self::MAX_LANES],
}

impl GridInspect {
    /// The maximum number of lanes inspected
    const MAX_LANES: usize = 10;

    /// Extract the grid lane sizes from the grid tracks
    pub(crate) fn new(rows: &GridTrackVec<GridTrack>, columns: &GridTrackVec<GridTrack>) -> Self {
        let mut row_sizes = [f32::MAX; Self::MAX_LANES];
        let mut column_sizes = [f32::MAX; Self::MAX_LANES];
        for (i, track) in rows.iter().take(Self::MAX_LANES).enumerate() {
            row_sizes[i] = track.base_size;
        }
        for (i, track) in columns.iter().take(Self::MAX_LANES).enumerate() {
            column_sizes[i] = track.base_size;
        }
        Self { rows: row_sizes, columns: column_sizes }
    }

    /// Get the row sizes
    pub fn rows(&self) -> impl Iterator<Item = f32> {
        self.rows.into_iter().filter(|v| *v != f32::MAX)
    }

    /// Get the column sizes
    pub fn columns(&self) -> impl Iterator<Item = f32> {
        self.columns.into_iter().filter(|v| *v != f32::MAX)
    }
}
