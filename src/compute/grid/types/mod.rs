//! Structs and enums that are used within the grid module
mod cell_occupancy;
mod coordinates;
#[cfg(feature = "inspect")]
mod grid_inspect;
mod grid_item;
mod grid_track;
mod grid_track_counts;

// Publish only locally in the grid module
pub(super) use cell_occupancy::{CellOccupancyMatrix, CellOccupancyState};
pub(crate) use coordinates::{GridCoordinate, GridLine, OriginZeroLine};
pub(super) use grid_item::GridItem;
pub(crate) use grid_track::GridTrack;
pub(super) use grid_track_counts::TrackCounts;

#[allow(unused_imports)]
pub(super) use grid_track::GridTrackKind;

#[cfg(feature = "inspect")]
pub use grid_inspect::GridInspect;

use crate::{util::sys::GridTrackVec, LayoutOutput};

/// Extension trait for adding grid inspection data to the layout output
pub trait GridInspectExt {
    /// Get the grid inspection data
    fn with_grid_inspect(self, rows: &GridTrackVec<GridTrack>, columns: &GridTrackVec<GridTrack>) -> Self;
}

impl GridInspectExt for LayoutOutput {
    /// Attach grid inspect data to the layout output, if the inspect feature is enabled
    #[allow(unused_variables, unused_mut)]
    fn with_grid_inspect(mut self, rows: &GridTrackVec<GridTrack>, columns: &GridTrackVec<GridTrack>) -> Self {
        #[cfg(feature = "inspect")]
        {
            self.inspect = Some(crate::LayoutInspect::Grid(GridInspect::new(rows, columns)));
        }
        self
    }
}

// pub(super) enum GridPosition {
//     Auto,
//     LineIndex(i16),
//     LineName(u16),
//     // GridAreaStart(u16),
//     // GridAreaEnd(u16),
// }

// pub(super) struct NamedArea {
//     name: u16,
//     row_start: u16,
//     row_end: u16,
//     column_start: u16,
//     column_end: u16,
// }

// pub(super) struct CssGrid {
//     pub available_space: Size<AvailableSpace>,
//     pub cell_occupancy_matrix: CellOccupancyMatrix,
//     pub items: Vec<GridItem>,
//     pub columns: GridAxisTracks,
//     pub rows: GridAxisTracks,
//     pub named_areas: Vec<NamedArea>,
// }
