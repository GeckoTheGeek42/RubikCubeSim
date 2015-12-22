// extern crate threadpool;
extern crate rustc_serialize;

extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

pub mod cube;
pub mod gfx2d;
// use rustc_serialize::json;
use std::default::Default;
use std::rc::Rc;
use std::cell::RefCell;

use cube::*;
use gfx2d::*;

fn main() {
	let cube_a = Rc::new( RefCell::new( RubiksCube::default() ) );

	App::new(cube_a.clone()).display();

	println!("{}", *cube_a.borrow());
	// println!("{}", cube_a.borrow().faces().unwrap());

	let alg_a: CubeTransform = "r".parse().unwrap();
	cube_a.borrow_mut().transform(&alg_a);

	println!("{}", *cube_a.borrow());
	println!("{}", cube_a.borrow().faces().unwrap());

	// let alg_a: Algorithm = "r2 F B2 M".parse().unwrap();
	// let ct_a: CubeTransform = "R".parse().unwrap();
	// println!("{:?}", alg_a);

	// cube_a.transform( &ct_a );
	// println!("{}", cube_a);
	// cube_a.transform( &alg_a );
	// println!("{}", cube_a);

	// let cube_a_ser: RubiksCubeSerializable = (&cube_a).into();
	// let cube_a_ser_str = json::encode(&cube_a_ser).unwrap();
	// println!("{}", cube_a_ser_str);
	// let cube_a_re: RubiksCube = {
	// 	let rcs: RubiksCubeSerializable = json::decode(&cube_a_ser_str).unwrap();
	// 	let rc: Result<RubiksCube, ()> = rcs.into();
	// 	rc.unwrap()
	// };
	// println!("{}", cube_a_re);
	// assert_eq!(cube_a, cube_a_re);
	// assert_eq!(cube_a, RubiksCube::deserialize( &cube_a.serialize() ));
	// assert!(RubiksCube::default().is_solved().unwrap());

	// let mut cube_b = RubiksCube::default();
	// let mut cube_c = RubiksCube::default();
	// let alg_b: Algorithm = "R2 R2".parse().unwrap();
	// let alg_c: Algorithm = "R R'".parse().unwrap();
	// cube_b.transform( &alg_b );
	// cube_c.transform( &alg_c );
	// assert!(cube_b.is_solved().unwrap() && cube_c.is_solved().unwrap());
	// println!("{}\n{}", cube_b, cube_c);
	// assert_eq!(cube_a, cube_b);
	// assert_eq!(cube_b, cube_c);
	// assert_eq!(cube_c, cube_a);

	// let mut cube_d = RubiksCube::default();
	// println!("{}", cube_d);
	// let alg_d: Algorithm = "R2 U R U R' U' R' U' R' U R' R U' R U R U R U' R' U' R2".parse().unwrap();
	// cube_d.transform(&alg_d);
	// println!("{}", cube_d);
	// assert!(cube_d.is_solved().unwrap());
	// assert_eq!(RubiksCube::default(), cube_d);
}