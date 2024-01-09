use std::{num::ParseIntError, ops::RangeInclusive, path::PathBuf, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Action {
    Save,
    Delete,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModifyMap {
    root_path: PathBuf,
    default_action: Action,
    exception_regions: Vec<MapRegion>,
}

impl ModifyMap {
    pub fn new(
        root_path: PathBuf,
        default_action: Action,
        exception_regions: Vec<MapRegion>,
    ) -> Self {
        Self {
            root_path,
            default_action,
            exception_regions,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SavePaths {
    root_path: PathBuf,
    player_path: PathBuf,
    iso_region_path: PathBuf,
}

impl SavePaths {
    pub fn new(root_path: PathBuf) -> Option<SavePaths> {
        let mut dir_name = root_path.file_name()?.to_os_string();
        dir_name.push("_player");
        let player_path = root_path.with_file_name(&dir_name);
        let iso_region_path = root_path.join("isoregiondata");

        Some(SavePaths {
            root_path,
            player_path,
            iso_region_path,
        })
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MapRegion {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl MapRegion {
    pub fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Self {
        Self { x_range, y_range }
    }

    fn contains(&self, map_coord: &MapCoord) -> bool {
        self.x_range.contains(&map_coord.x) && self.y_range.contains(&map_coord.y)
    }

    fn containing_chunk_region(&self) -> MapChunkRegion {
        MapChunkRegion::new(
            coords_to_chunks(&self.x_range),
            coords_to_chunks(&self.y_range),
        )
    }
}

fn coords_to_chunks(coords: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    let chunk_start = coords.start().div_euclid(300);
    let chunk_end = coords.end().div_euclid(300) + 1;
    RangeInclusive::new(chunk_start, chunk_end)
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapChunkRegion {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl MapChunkRegion {
    pub fn new(x_range: RangeInclusive<i32>, y_range: RangeInclusive<i32>) -> Self {
        Self { x_range, y_range }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct MapCoord {
    x: i32,
    y: i32,
}

impl MapCoord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromStr for MapCoord {
    type Err = MapCoordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MapCoordParseError::*;

        if !(s.starts_with("chunkdata_") || s.starts_with("zpop_")) {
            return Err(UnknownPrefix);
        }

        let mut splits = s.split('_');
        splits.next(); // pop the prefix
        let x: i32 = splits.next().ok_or(NotEnoughSegments)?.parse()?;
        let y: i32 = splits.next().ok_or(NotEnoughSegments)?.parse()?;

        if splits.next().is_some() {
            return Err(TooManySegments);
        }

        Ok(MapCoord::new(x, y))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MapCoordParseError {
    NonNumberSegment,
    NotEnoughSegments,
    UnknownPrefix,
    TooManySegments,
}

impl From<ParseIntError> for MapCoordParseError {
    fn from(_value: ParseIntError) -> Self {
        MapCoordParseError::NonNumberSegment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::{test_case, test_matrix};

    #[test_case(-1, 9)]
    #[test_case(0, 9)]
    #[test_case(-1, 10)]
    #[test_case(11, 20)]
    #[test_case(10, 21)]
    #[test_case(11, 21)]
    fn map_regions_outside_either_is_excluded(x: i32, y: i32) {
        let range = MapRegion::new(0..=10, 10..=20);
        assert!(!range.contains(&MapCoord::new(x, y)));
    }

    #[test_matrix(
        0..=2,
        10..=12
    )]
    fn map_regions_inside_both_coords_are_included(x: i32, y: i32) {
        let range = MapRegion::new(0..=2, 10..=12);
        assert!(range.contains(&MapCoord::new(x, y)));
    }

    #[test_case(21..=23, 6596..=6866)]
    #[test_case(17..=19, 5286..=5568)]
    fn map_coords_to_chunks(chunks: RangeInclusive<i32>, coords: RangeInclusive<i32>) {
        assert_eq!(chunks, coords_to_chunks(&coords));
    }

    #[test]
    fn map_coord_region_to_chunk_region() {
        let map_coords = MapRegion::new(6596..=6866, 5286..=5568);
        let expected_map_chunks = MapChunkRegion::new(21..=23, 17..=19);
        assert_eq!(expected_map_chunks, map_coords.containing_chunk_region());
    }

    mod filename_parsing {
        use super::*;

        use self::test_case;
        use MapCoordParseError::*;

        #[test_case("chunkdata_110_474")]
        #[test_case("zpop_110_474")]
        fn parse_filenames_to_coordinates(filename: &str) -> Result<(), MapCoordParseError> {
            let coord: MapCoord = filename.parse()?;
            assert_eq!(MapCoord::new(110, 474), coord);
            Ok(())
        }

        #[test_case("badprefix_10_10", UnknownPrefix)]
        #[test_case("zpop_nonnum_10", NonNumberSegment)]
        #[test_case("chunkdata_10", NotEnoughSegments)]
        #[test_case("zpop_777_666_final", TooManySegments)]
        fn invalid_filenames_dont_parse(filename: &str, expected_parse_err: MapCoordParseError) {
            let parse_result = filename.parse::<MapCoord>();
            assert_eq!(Err(expected_parse_err), parse_result);
        }
    }
}
