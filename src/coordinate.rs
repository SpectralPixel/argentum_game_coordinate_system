pub use coordinate_trait::CoordinateTrait;
pub use signed_coordinate::SignedCoordinate;
pub use unsigned_coordinate::UnsignedCoordinate;

mod coordinate_trait;
mod signed_coordinate;
mod unsigned_coordinate;

/// `Coordinate`'s field type.
///
/// i32: From −2,147,483,648 to 2,147,483,647.
///
/// I don't believe a larger size is necessary, as the RAM usage per instance
/// would double. Heck, even this is already overkill.
pub type CoordinateType = i32;

/// 3D Coordinate in absolute space.
pub type Coordinate = SignedCoordinate<CoordinateType>;
