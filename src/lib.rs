use derive_more::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(
	Add,
	AddAssign,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	Sub,
	SubAssign,
	Debug,
	Ord,
	PartialOrd,
	Mul,
	MulAssign,
)]
pub struct Coordinate(pub i64, pub i64);

#[derive(
	Add,
	AddAssign,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	Sub,
	SubAssign,
	Debug,
	Ord,
	PartialOrd,
	Mul,
	MulAssign,
)]
pub struct Coordinate3D(pub i64, pub i64, pub i64);

#[derive(
	Add,
	AddAssign,
	Clone,
	Copy,
	PartialEq,
	Eq,
	Hash,
	Sub,
	SubAssign,
	Debug,
	Ord,
	PartialOrd,
	Mul,
	MulAssign,
)]
pub struct Coordinate4D(pub i64, pub i64, pub i64, pub i64);
