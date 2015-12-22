use std::fmt::{Display, self};
use std::str::FromStr;
use std::default::Default;
use rustc_serialize::json;
use std::convert::{From, Into};
use std::mem;
use std::borrow::ToOwned;
// use threadpool::ScopedPool;

#[derive(Debug, RustcEncodable, RustcDecodable, Clone, PartialEq)]
pub enum FaceDirection	 {
	Top,
	Bottom,
	Left,
	Right,
	Back,
	Front,
	Middle,

}

impl FaceDirection {
	pub fn opposite(&self) -> FaceDirection {
		match *self {
			FaceDirection::Top => FaceDirection::Bottom, 
			FaceDirection::Bottom => FaceDirection::Top, 
			FaceDirection::Left => FaceDirection::Right, 
			FaceDirection::Right => FaceDirection::Left, 
			FaceDirection::Back => FaceDirection::Front, 
			FaceDirection::Front => FaceDirection::Back, 
			FaceDirection::Middle => FaceDirection::Middle,
		}
	}


	pub fn transform(&self, &CubeTransform(ref r_side, ref r_dir, _, twice): &CubeTransform) -> FaceDirection {
		self.transform_by(r_side, r_dir, twice)
	}
	pub fn transform_by(&self, r_side: &FaceDirection, r_dir: &RotationDirection, twice: bool) -> FaceDirection {
		let dir = self;
		let res = match *dir {
			FaceDirection::Top => match *r_side {
				FaceDirection::Middle => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Back,
					RotationDirection::Anticlockwise => FaceDirection::Front
				},
				FaceDirection::Top => FaceDirection::Top, 
				FaceDirection::Bottom => FaceDirection::Top, 
				FaceDirection::Left => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Front,
					RotationDirection::Anticlockwise => FaceDirection::Back
				}, 
				FaceDirection::Right => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Back,
					RotationDirection::Anticlockwise => FaceDirection::Front
				}, 
				FaceDirection::Back => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Left,
					RotationDirection::Anticlockwise => FaceDirection::Right
				}, 
				FaceDirection::Front => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Right,
					RotationDirection::Anticlockwise => FaceDirection::Left
				}, 
			},
			FaceDirection::Bottom => match *r_side {
				FaceDirection::Middle => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Front,
					RotationDirection::Anticlockwise => FaceDirection::Back
				},
				FaceDirection::Top => FaceDirection::Bottom, 
				FaceDirection::Bottom => FaceDirection::Bottom, 
				FaceDirection::Left => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Back,
					RotationDirection::Anticlockwise => FaceDirection::Front
				}, 
				FaceDirection::Right => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Front,
					RotationDirection::Anticlockwise => FaceDirection::Back
				}, 
				FaceDirection::Back => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Right,
					RotationDirection::Anticlockwise => FaceDirection::Left
				}, 
				FaceDirection::Front => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Left,
					RotationDirection::Anticlockwise => FaceDirection::Right
				}, 
			},
			FaceDirection::Left => match *r_side {
				FaceDirection::Middle => FaceDirection::Left,
				FaceDirection::Top => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Back,
					RotationDirection::Anticlockwise => FaceDirection::Front
				}, 
				FaceDirection::Bottom => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Front,
					RotationDirection::Anticlockwise => FaceDirection::Back
				}, 
				FaceDirection::Left => FaceDirection::Left, 
				FaceDirection::Right => FaceDirection::Left, 
				FaceDirection::Back => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Bottom,
					RotationDirection::Anticlockwise => FaceDirection::Top
				}, 
				FaceDirection::Front => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Top,
					RotationDirection::Anticlockwise => FaceDirection::Bottom
				}, 
			},
			FaceDirection::Right => match *r_side {
				FaceDirection::Middle => FaceDirection::Right,
				FaceDirection::Top => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Front,
					RotationDirection::Anticlockwise => FaceDirection::Back
				}, 
				FaceDirection::Bottom => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Back,
					RotationDirection::Anticlockwise => FaceDirection::Front
				}, 
				FaceDirection::Left => FaceDirection::Right, 
				FaceDirection::Right => FaceDirection::Right, 
				FaceDirection::Back => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Top,
					RotationDirection::Anticlockwise => FaceDirection::Bottom
				},  
				FaceDirection::Front => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Bottom,
					RotationDirection::Anticlockwise => FaceDirection::Top
				}, 
			},
			FaceDirection::Back => match *r_side {
				FaceDirection::Middle => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Bottom,
					RotationDirection::Anticlockwise => FaceDirection::Top
				},
				FaceDirection::Top => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Right,
					RotationDirection::Anticlockwise => FaceDirection::Left
				}, 
				FaceDirection::Bottom => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Left,
					RotationDirection::Anticlockwise => FaceDirection::Right
				}, 
				FaceDirection::Left => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Top,
					RotationDirection::Anticlockwise => FaceDirection::Bottom
				}, 
				FaceDirection::Right => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Bottom,
					RotationDirection::Anticlockwise => FaceDirection::Top
				}, 
				FaceDirection::Back => FaceDirection::Back, 
				FaceDirection::Front => FaceDirection::Back, 
			},
			FaceDirection::Front => match *r_side {
				FaceDirection::Middle => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Top,
					RotationDirection::Anticlockwise => FaceDirection::Bottom
				},
				FaceDirection::Top => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Left,
					RotationDirection::Anticlockwise => FaceDirection::Right
				}, 
				FaceDirection::Bottom => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Right,
					RotationDirection::Anticlockwise => FaceDirection::Left
				}, 
				FaceDirection::Left => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Bottom,
					RotationDirection::Anticlockwise => FaceDirection::Top
				}, 
				FaceDirection::Right => match *r_dir {
					RotationDirection::Clockwise => FaceDirection::Top,
					RotationDirection::Anticlockwise => FaceDirection::Bottom
				}, 
				FaceDirection::Back => FaceDirection::Front, 
				FaceDirection::Front => FaceDirection::Front, 
			},
			FaceDirection::Middle => unreachable!(), 
		};
		if twice { res.transform_by(r_side, r_dir, false) }
		else { res }
	}
}

#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
pub enum RotationDirection {
	Clockwise, Anticlockwise
}

#[derive(Debug, RustcEncodable, RustcDecodable, Clone, PartialEq)]
pub enum FaceColor {
	Red, Orange, Yellow, White, Green, Blue, 
}

impl FaceColor {
	fn opposite(&self) -> FaceColor {
		match *self {
			FaceColor::Red => FaceColor::Orange, 
			FaceColor::Orange => FaceColor::Red, 
			FaceColor::Yellow => FaceColor::White, 
			FaceColor::White => FaceColor::Yellow, 
			FaceColor::Green => FaceColor::Blue, 
			FaceColor::Blue => FaceColor::Green, 

		}
	}
}

pub type DeepTurn = bool;
pub type Twice = bool;

#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
pub struct CubeTransform(pub FaceDirection, pub RotationDirection, pub DeepTurn, pub Twice);

impl FromStr for CubeTransform {
	type Err = ();

	fn from_str(s: &str) -> Result<CubeTransform, ()> {
		let mut chars = s.chars();
		let side_c = match chars.next() {
			Some(c) => c, None => return Err(())
		};
		let mod_c = chars.next();
		Ok( CubeTransform( 
			match side_c {
				'r' => FaceDirection::Right,
				'l' => FaceDirection::Left,
				'u' => FaceDirection::Top,
				'd' => FaceDirection::Bottom,
				'b' => FaceDirection::Back,
				'f' => FaceDirection::Front,
				'R' => FaceDirection::Right, 
				'L' => FaceDirection::Left, 
				'U' => FaceDirection::Top, 
				'D' => FaceDirection::Bottom, 
				'B' => FaceDirection::Back, 
				'F' => FaceDirection::Front,
				'M' => FaceDirection::Middle,
				_ => return Err(())
			},
			match mod_c {
				Some('\'') => RotationDirection::Anticlockwise,
				Some('2') => RotationDirection::Clockwise,
				Some(_) => return Err(()),
				None => RotationDirection::Clockwise,
			},
			side_c.is_lowercase(),
			match mod_c {
				Some('2') => true,
				_ => false,
			}
		) )
	}
}

#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
pub struct Algorithm(Vec<CubeTransform>);

impl FromStr for Algorithm {
	type Err = ();

	fn from_str(s: &str) -> Result<Algorithm, ()> {
		Ok( Algorithm( s.split(' ').fold(Vec::new(), |mut acc, token| {
			match token.parse() {
				Ok(v) => acc.push(v),
				Err(()) => {},
			};
			acc
		} ) ) )
	}
}

#[derive(Debug, RustcEncodable, RustcDecodable, Clone, PartialEq)]
pub struct Face(pub FaceDirection, pub FaceColor);

impl Face{
	pub fn transform_by(&mut self, r_side: &FaceDirection, r_dir: &RotationDirection, twice: bool) {
		self.0 = self.0.transform_by(r_side, r_dir, twice);
	}
	pub fn transform(&mut self, ct: &CubeTransform) {
		self.0 = self.0.transform(ct);
	}
	pub fn transform_to(&self, ct: &CubeTransform) -> Face {
		Face(self.0.transform(ct), self.1.clone())
	}
	pub fn transform_by_to(&self, r_side: &FaceDirection, r_dir: &RotationDirection, twice: bool) -> Face {
		Face(self.0.transform_by(r_side, r_dir, twice), self.1.clone())
	}

	fn opposite(&self) -> Face {
		Face(self.0.opposite(), self.1.opposite())
	}
}

#[derive(Debug, RustcEncodable, RustcDecodable, Clone, PartialEq)]
pub enum SubCube {
	Edge(Face, Face),
	Corner(Face, Face, Face),
	Center(Face)
}

impl SubCube {
	pub fn transform(&mut self, &CubeTransform(ref side, ref dir, deep, twice): &CubeTransform) {
		match *self {
			SubCube::Center(ref mut f1) => match deep {
				true => f1.transform_by(side, dir, twice),
				false if side == &FaceDirection::Middle => f1.transform_by(side, dir, twice),
				false => {}
			},
			SubCube::Edge(ref mut f1, ref mut f2) => match deep {
				false if f1.0 == *side || f2.0 == *side => {
					f1.transform_by(side, dir, twice);
					f2.transform_by(side, dir, twice);
				},
				true if f1.0 != side.opposite() && f2.0 != side.opposite() => {
					f1.transform_by(side, dir, twice);
					f2.transform_by(side, dir, twice);
				},
				false if side == &FaceDirection::Middle && (f1.0 != FaceDirection::Right && f2.0 != FaceDirection::Right) && (f1.0 != FaceDirection::Left && f2.0 != FaceDirection::Left) => {
					f1.transform_by(side, dir, twice);
					f2.transform_by(side, dir, twice);
				}
				_ => {}
			},
			SubCube::Corner(ref mut f1, ref mut f2, ref mut f3) => if f1.0 == *side || f2.0 == *side || f3.0 == *side {
					f1.transform_by(side, dir, twice);
					f2.transform_by(side, dir, twice);
					f3.transform_by(side, dir, twice);
			},
		}
	}

	pub fn transform_to(&mut self, &CubeTransform(ref side, ref dir, deep, twice): &CubeTransform) -> SubCube {
		let res = match *self {
			SubCube::Center(ref f1) => match deep {
				true => SubCube::Center(f1.transform_by_to(side, dir, twice)),
				false if side == &FaceDirection::Middle => SubCube::Center(f1.transform_by_to(side, dir, twice)),
				false => self.clone()
			},
			SubCube::Edge(ref f1, ref f2) => match deep {
				false if f1.0 == *side || f2.0 == *side => {
					SubCube::Edge(
						f1.transform_by_to(side, dir, twice),
						f2.transform_by_to(side, dir, twice)
						)
				},
				true if f1.0 != side.opposite() && f2.0 != side.opposite() => {
					SubCube::Edge(
						f1.transform_by_to(side, dir, twice),
						f2.transform_by_to(side, dir, twice)
						)
				},
				false if side == &FaceDirection::Middle && (f1.0 != FaceDirection::Right && f2.0 != FaceDirection::Right) && (f1.0 != FaceDirection::Left && f2.0 != FaceDirection::Left) => {
					SubCube::Edge(
						f1.transform_by_to(side, dir, twice),
						f2.transform_by_to(side, dir, twice),
						)
				}
				_ => self.clone()
			},
			SubCube::Corner(ref f1, ref f2, ref f3) => if f1.0 == *side || f2.0 == *side || f3.0 == *side {
					SubCube::Corner(
						f1.transform_by_to(side, dir, twice),
						f2.transform_by_to(side, dir, twice),
						f3.transform_by_to(side, dir, twice)
						)
			} else { self.clone() },
		};
		// println!("{:?} -> {:?}", self, res);
		res
	}
}

impl Default for SubCube {
	fn default() -> SubCube {
		SubCube::Center( Face( FaceDirection::Middle, FaceColor::White ) )
	}
}

#[derive(Debug, PartialEq)]
pub struct RubiksCube([SubCube; 26]);

impl Clone for RubiksCube {
	fn clone(&self) -> RubiksCube {
		let mut new_cubes = RubiksCube::default();
		for (new_cube, old_cube) in new_cubes.0.iter_mut().zip( self.0.iter() ) {
			*new_cube = old_cube.clone();
		}
		return new_cubes;
	}
}

impl RubiksCube {
	pub fn new(front: Face, top: Face, left: Face) -> RubiksCube {
		let back = front.opposite();
		let bottom = top.opposite();
		let right = left.opposite();

		RubiksCube( [
			SubCube::Corner( front.clone(), top.clone(), left.clone() ),
			SubCube::Edge( front.clone(), top.clone() ),
			SubCube::Corner( front.clone(), top.clone(), right.clone() ),
			SubCube::Edge( front.clone(), left.clone() ),
			SubCube::Center( front.clone() ),
			SubCube::Edge( front.clone(), right.clone() ),
			SubCube::Corner( front.clone(), left.clone(), bottom.clone() ),
			SubCube::Edge( front.clone(), bottom.clone() ),
			SubCube::Corner( front.clone(), bottom.clone(), right.clone() ),
			/*--*/
			SubCube::Edge( top.clone(), left.clone() ),
			SubCube::Center( top.clone() ),
			SubCube::Edge( top.clone(), right.clone() ),
			SubCube::Center( left.clone() ),

			SubCube::Center( right.clone() ),
			SubCube::Edge( bottom.clone(), left.clone() ),
			SubCube::Center( bottom.clone() ),
			SubCube::Edge( bottom.clone(), right.clone() ),
			/*--*/
			SubCube::Corner( back.clone(), top.clone(), left.clone() ),
			SubCube::Edge( back.clone(), top.clone() ),
			SubCube::Corner( back.clone(), top.clone(), right.clone() ),
			SubCube::Edge( back.clone(), left.clone() ),
			SubCube::Center( back.clone() ),
			SubCube::Edge( back.clone(), right.clone() ),
			SubCube::Corner( back.clone(), left.clone(), bottom.clone() ),
			SubCube::Edge( back.clone(), bottom.clone() ),
			SubCube::Corner( back.clone(), bottom.clone(), right.clone() ),
		] )
	}

	pub fn serialize(&self) -> String {
		let ser: RubiksCubeSerializable = self.into();
		json::encode(&ser).unwrap()
	}

	pub fn deserialize(s: &str) -> RubiksCube {
		let rcs: RubiksCubeSerializable = json::decode(s).unwrap();
		let rc: Result<RubiksCube, ()> = rcs.into();
		rc.unwrap()
	}

	pub fn is_solved(&self) -> Result<bool, ()> {
		let solved = if let SubCube::Corner(ref f1, ref f2, ref f3) = self.0[0] {
			match (f1, f2, f3) {
				(&Face(FaceDirection::Top, _), &Face(FaceDirection::Front, _), &Face(FaceDirection::Left, _)) => RubiksCube::new( f2.clone(), f1.clone(), f3.clone() ),
				(&Face(FaceDirection::Top, _), &Face(FaceDirection::Left, _), &Face(FaceDirection::Front, _)) => RubiksCube::new( f3.clone(), f1.clone(), f2.clone() ),
				(&Face(FaceDirection::Front, _), &Face(FaceDirection::Top, _), &Face(FaceDirection::Left, _)) => RubiksCube::new( f1.clone(), f2.clone(), f3.clone() ),
				(&Face(FaceDirection::Front, _), &Face(FaceDirection::Left, _), &Face(FaceDirection::Top, _)) => RubiksCube::new( f1.clone(), f3.clone(), f2.clone() ),
				(&Face(FaceDirection::Left, _), &Face(FaceDirection::Front, _), &Face(FaceDirection::Top, _)) => RubiksCube::new( f2.clone(), f3.clone(), f1.clone() ),
				(&Face(FaceDirection::Left, _), &Face(FaceDirection::Top, _), &Face(FaceDirection::Front, _)) => RubiksCube::new( f3.clone(), f2.clone(), f1.clone() ),
				_ => return Err(()),
			}
		} else { return Err(()) };

		fn eq_cubes(c1: &SubCube, c2: &SubCube) -> Result<bool, ()> {
			match (c1, c2) {
				(&SubCube::Corner(ref f1, ref f2, ref f3), &SubCube::Corner(ref f4, ref f5, ref f6)) if (f1 != f2 &&f2 != f3 && f3 != f1 && f4 != f5 && f5 != f6 && f6 != f4) => 
					Ok((f1 == f4 || f1 == f5 || f1 == f6) && (f2 == f4 || f2 == f5 || f2 == f6) && (f3 == f4 || f3 == f5 || f3 == f6)),
				(&SubCube::Edge(ref f1, ref f2), &SubCube::Edge(ref f3, ref f4)) if (f1 != f2 && f3 != f4) => 
					Ok((f1 == f3 || f1 == f4) && (f2 == f3 || f2 == f4)),
				(&SubCube::Center(ref f1), &SubCube::Center(ref f2)) => Ok(f1 == f2),
				_ => Err(())
			}
		}

		self.0.iter().zip( solved.0.iter() ).fold(Result::Ok(true), |res, (check, correct)| match res {
			Ok(v) => if v { eq_cubes(check, correct) } else { Ok(false) },
			Err(e) => Err(e)
		})
	}
}

pub trait TransformCube<T> {
	fn transform(&mut self, &T);
}

impl TransformCube<CubeTransform> for RubiksCube {
	fn transform(&mut self, ct: &CubeTransform) {
		let mut new_cubes = self.clone();
		for (i, new_cube) in new_cubes.0.iter_mut().enumerate() {
			let cy = match *ct {
				CubeTransform( FaceDirection::Top, _, false, _ ) => ( 2 ),
				CubeTransform( FaceDirection::Top, _, true, _ ) => ( if in_cycle(i as u8, 2) { 2 } else { 8 } ), 
				CubeTransform( FaceDirection::Bottom, _, false, _ ) => ( 3 ),
				CubeTransform( FaceDirection::Bottom, _, true, _ ) => ( if in_cycle(i as u8, 3) { 3 } else { 9 } ), 
				CubeTransform( FaceDirection::Left, _, false, _ ) => ( 1 ),
				CubeTransform( FaceDirection::Left, _, true, _ ) => ( if in_cycle(i as u8, 1) { 1 } else { 7 } ), 
				CubeTransform( FaceDirection::Right, _, false, _ ) => ( 0 ),
				CubeTransform( FaceDirection::Right, _, true, _ ) => ( if in_cycle(i as u8, 0) { 0 } else { 6 } ), 
				CubeTransform( FaceDirection::Back, _, false, _ ) => ( 5 ),
				CubeTransform( FaceDirection::Back, _, true, _ ) => ( if in_cycle(i as u8, 5) { 5 } else { 11 } ), 
				CubeTransform( FaceDirection::Front, _, false, _ ) => ( 4 ),
				CubeTransform( FaceDirection::Front, _, true, _ ) => ( if in_cycle(i as u8, 4) { 4 } else { 10 } ), 
				CubeTransform( FaceDirection::Middle, _, false, _ ) => ( 6 ),
				CubeTransform( FaceDirection::Middle, _, true, _ ) => ( 6 ), 
			};
			// println!("{:?}: {} -> {}", ct, i, cy);
			let new_i = match (&ct.1, ct.3) {
				(&RotationDirection::Clockwise, false) => cycle_next(i as u8, cy),
				(&RotationDirection::Clockwise, true) => cycle_next_twice(i as u8, cy),
				(&RotationDirection::Anticlockwise, false) => cycle_prev(i as u8, cy),
				(&RotationDirection::Anticlockwise, true) => cycle_prev_twice(i as u8, cy),
			};
			self.0[new_i as usize] = new_cube.transform_to(&ct);
		}
	}
}

impl TransformCube<Algorithm> for RubiksCube {
	fn transform(&mut self, alg: &Algorithm) {
		for step in &alg.0 {
			self.transform(step);
		}
	}
}

static TRANSFORM_CYCLES: [[u8; 8]; 12] = [
	[9, 6, 3, 12, 20, 23, 26, 17], //R 0
	[1, 4, 7, 15, 24, 21, 18, 10], //L 1
	[3, 2, 1, 10, 18, 19, 20, 12], //U 2
	[7, 8, 9, 17, 26, 25, 24, 15], //D 3
	[1, 2, 3, 6, 9, 8, 7, 4], //F 4
	[26, 23, 20, 19, 18, 21, 24, 25], //B 5
	[8, 5, 2, 11, 19, 22, 25, 16], //r/M 6
	[2, 5, 8, 16, 25, 22, 19, 11], //l 7
	[6, 5, 4, 13, 21, 22, 23, 14], //u 8
	[4, 5, 6, 14, 23, 22, 21, 13], //d 9
	[10, 11, 12, 14, 17, 16, 15, 13], //f 10
	[10, 13, 15, 16, 17, 14, 12, 11] //b 11
];

fn in_cycle(curr: u8, cycle: usize) -> bool {
	TRANSFORM_CYCLES[cycle].iter().any(|x| (*x - 1) == curr)
}

fn cycle_next(curr: u8, cycle: usize) -> u8 {
	for i in 0..8 {
		if TRANSFORM_CYCLES[cycle][i] == (curr + 1) {
			return if i == 7 { TRANSFORM_CYCLES[cycle][1] - 1 } 
			else if i == 6 { TRANSFORM_CYCLES[cycle][0] - 1 } 
			else { TRANSFORM_CYCLES[cycle][i+2] - 1 }
		}
	}
	return curr;
}

fn cycle_prev(curr: u8, cycle: usize) -> u8 {
	for i in 0..8 {
		if TRANSFORM_CYCLES[cycle][i] == (curr + 1) {
			return if i == 0 { TRANSFORM_CYCLES[cycle][6] - 1 } 
			else if i == 1 { TRANSFORM_CYCLES[cycle][7] - 1 }
			else { TRANSFORM_CYCLES[cycle][i-2] - 1 }
		}
	}
	return curr;
}

fn cycle_next_twice(curr: u8, cycle: usize) -> u8 {
	cycle_next( cycle_next(curr, cycle), cycle )
}

fn cycle_prev_twice(curr: u8, cycle: usize) -> u8 {
	cycle_prev( cycle_prev(curr, cycle), cycle )
}

impl Display for RubiksCube {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		fn print_cube(f: &mut fmt::Formatter, cube: &SubCube) -> Result<(), fmt::Error> {
			match *cube {
				SubCube::Edge(ref f1, ref f2) => try!( 
					f.write_str( &format!("{:^45}", &format!( "{:?} {:?} - {:?} {:?}", f1.1, f1.0, f2.1, f2.0 ) ) ) 
				),
				SubCube::Corner(ref f1, ref f2, ref f3) => try!( 
					f.write_str( &format!("{:^45}", &format!( "{:?} {:?} - {:?} {:?} - {:?} {:?}", f1.1, f1.0, f2.1, f2.0, f3.1, f3.0 ) ) ) 
				),
				SubCube::Center(ref f1) => try!( 
					f.write_str( &format!("{:^45}", &format!( "{:?} {:?} Center", f1.1, f1.0 ) ) ) 
				),
			};
			Ok(())
		}

		let cubes = &self.0;

		try!( f.write_str("-------------------------------------------------------------------------------------------------------------------------------------------\n") );

		for r in 0..3 {
			for cube in &cubes[(0 + (3*r))..(3 + (3*r))] {
				try!( f.write_str("|") );
				try!( print_cube(f, cube) );
			}
			try!( f.write_str("|\n") );
		}

		try!( f.write_str("-------------------------------------------------------------------------------------------------------------------------------------------\n") );

		{
			for cube in &cubes[9..12] {
				try!( f.write_str("|") 	);
				try!( print_cube(f, cube) );
			}
			try!( f.write_str("|\n") );

			try!( f.write_str("|") );
			try!( print_cube(f, &cubes[12]) );
			try!( f.write_str(&format!("|{:^45}|", "")) );
			try!( print_cube(f, &cubes[13]) );
			try!( f.write_str("|\n") );

			for cube in &cubes[14..17] {
				try!( f.write_str("|") 	);
				try!( print_cube(f, cube) );
			}
			try!( f.write_str("|\n") );
		}

		try!( f.write_str("-------------------------------------------------------------------------------------------------------------------------------------------\n") );

		for r in 0..3 {
			for cube in &cubes[(17 + (3*r))..(20 + (3*r))] {
				try!( f.write_str("|") );
				try!( print_cube(f, cube) );
			}
			try!( f.write_str("|\n") );
		}

		try!( f.write_str("-------------------------------------------------------------------------------------------------------------------------------------------\n") );

		Ok(())
	}
}

impl Default for RubiksCube {
	fn default() -> RubiksCube {
		let face_front = Face(FaceDirection::Front, FaceColor::Red); 
		let face_top = Face(FaceDirection::Top, FaceColor::Yellow); 
		let face_left = Face(FaceDirection::Left, FaceColor::Blue); 
		RubiksCube::new(face_front, face_top, face_left)
	}
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct RubiksCubeSerializable(Vec<SubCube>);

// impl RubiksCubeSerializable {
// 	fn from(rc: &RubiksCube) -> RubiksCubeSerializable {
// 		RubiksCubeSerializable( (&rc.0).to_vec() )
// 	}

// 	fn into(self) -> RubiksCubeSerializable {

// 	}
// }

impl<'a> From<&'a RubiksCube> for RubiksCubeSerializable {
	fn from(rc: &RubiksCube) -> RubiksCubeSerializable {
		RubiksCubeSerializable( (&rc.0).to_vec() )
	}
}

impl From<RubiksCubeSerializable> for Result<RubiksCube, ()> {
	fn from(rcs: RubiksCubeSerializable) -> Result<RubiksCube, ()> {
		if rcs.0.len() < 26 { return Err(()); }
		let mut rc = RubiksCube::default();
		for (c, s) in rc.0.iter_mut().zip( rcs.0.into_iter() ) {
			*c = s;
		}
		return Ok(rc);
	}
}

static FACE_INDICES: [[usize; 9]; 6] = [
	[1, 2, 3, 4, 5, 6, 7, 8, 9],
	[20, 19, 18, 23, 22, 21, 26, 25, 24],
	[3, 12, 20, 6, 14, 23, 9, 17, 26],
	[18, 10, 1, 21, 13, 4, 24, 15, 7],
	[18, 19, 20, 10, 11, 12, 1, 2, 3],
	[7, 8, 9, 15, 16, 17, 24, 25, 26]
];

#[derive(Debug)]
pub struct CubeFace(pub [Face; 9]);
#[derive(Debug)]
pub struct CubeFaces(pub [CubeFace; 6]);

impl RubiksCube {
	pub fn faces(&self) -> Result<CubeFaces, String> {
		Ok( CubeFaces([
			try!( self.face( FaceDirection::Top ) ),
			try!( self.face( FaceDirection::Bottom ) ),
			try!( self.face( FaceDirection::Left ) ),
			try!( self.face( FaceDirection::Right ) ),
			try!( self.face( FaceDirection::Back ) ),
			try!( self.face( FaceDirection::Front ) ),
		]) )
	}

	pub fn face(&self, face: FaceDirection) -> Result<CubeFace, String> {
		let mut arr: [Face; 9] = unsafe{ mem::uninitialized() };
		// println!("LOL");
		fn check_and_push(v: &mut [Face; 9], c: &RubiksCube, vi: usize, ci: usize, fd: &FaceDirection) -> Result<(), String> {
			// println!("{:?} - {:?} - {:?} - {:?}", vi, ci, fd, c.0[ci]);
			v[vi] = match c.0[ci] {
				SubCube::Center(ref f1) => 
					if &f1.0 == fd { f1.clone() } 
					else { return Err("Center Wrong".to_owned()); },
				SubCube::Edge(ref f1, ref f2) => 
					if &f1.0 == fd { f1.clone() } 
					else if &f2.0 == fd { f2.clone() }
					else { return Err("Edge Wrong".to_owned()); },
				SubCube::Corner(ref f1, ref f2, ref f3) => 
					if &f1.0 == fd { f1.clone() } 
					else if &f2.0 == fd { f2.clone() } 
					else if &f3.0 == fd { f3.clone() } 
					else { return Err("Corner Wrong".to_owned()); },
			};
			Ok(())
		}
		match face {
			FaceDirection::Top => for (vi, &ci) in FACE_INDICES[4].iter().enumerate() { try!( check_and_push(&mut arr, self, vi, ci - 1, &face) ) }, 
			FaceDirection::Bottom => for (vi, &ci) in FACE_INDICES[5].iter().enumerate() { try!( check_and_push(&mut arr, self, vi, ci - 1, &face) ) }, 
			FaceDirection::Left => for (vi, &ci) in FACE_INDICES[3].iter().enumerate() { try!( check_and_push(&mut arr, self, vi, ci - 1, &face) ) }, 
			FaceDirection::Right => for (vi, &ci) in FACE_INDICES[2].iter().enumerate() { try!( check_and_push(&mut arr, self, vi, ci - 1, &face) ) }, 
			FaceDirection::Back => for (vi, &ci) in FACE_INDICES[1].iter().enumerate() { try!( check_and_push(&mut arr, self, vi, ci - 1, &face) ) }, 
			FaceDirection::Front => for (vi, &ci) in FACE_INDICES[0].iter().enumerate() { try!( check_and_push(&mut arr, self, vi, ci - 1, &face) ) }, 
			FaceDirection::Middle => return Err("FaceDirection::Middle".to_owned()),
		};
		Ok( CubeFace( arr ) )
	}
}

impl Display for CubeFace {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		try!( f.write_str("----------------------------------------------------------------\n") );
		for i in 0..3 {
			for j in (0 + (i * 3)) .. (3+ (i * 3)) {
				try!( f.write_str("|") );
				try!( f.write_str( &format!("{:^20}", format!("{:?} {:?}", self.0[j].0, self.0[j].1)) ) );
			}
			try!( f.write_str("|\n") );
		}
		try!( f.write_str("----------------------------------------------------------------\n") );
		Ok(())
	}
}

impl Display for CubeFaces {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		for face in &self.0 {
			try!( f.write_str(&format!("{}\n", face)) );
		}
		Ok(())
	}
}