//! # Argentum - Coordinate System
//!
//! `argentum_game_coordinate_system` contains the coordinate system that is
//! used by Argentum and some utilities related to it.
//! 
//! Import the `prelude` to gain access to all the crate's types.
//! 
//! The correct type to use when interfacing with Argentum is `Coordinate`.
//! 
//! Feel free to make other similar types by creating type aliases for
//! `Coord<T>`.
//!
//! For more information about Argentum, see the `argentum_game` crate.

/// Contains `Coordinate` and similar structs.
pub mod coordinate;

/// Contains `Region` and similar structs.
pub mod region;

/// Import me!
pub mod prelude;
