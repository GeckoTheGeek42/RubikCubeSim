use piston::window::{ WindowSettings, Size };
use piston::event::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL, glyph_cache };
use graphics::Viewport;
use std::path::Path;
use piston::input::{Button, MouseButton, Key};
use cube::{RubiksCube, FaceColor, Algorithm, TransformCube};
use std::borrow::ToOwned;
use std::rc::Rc;
use std::cell::RefCell;

pub struct AppWindow<'a>(App<'a>, Window);

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    character_cache: glyph_cache::GlyphCache<'a>,
    state: AppState,
}

impl<'a> AppWindow<'a> {
	pub fn display(mut self) {
		let AppWindow(mut app, mut window) = self;
		for e in window.events() {
			if let Some(r) = e.render_args() {
			    app.render(&r);
			}

			if let Some(u) = e.update_args() {
			    app.update(&u);
			}

			if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
				app.state.clicked = true;
			}
			if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
				app.state.clicked = false;
				app.state.worked = false;
			}

			if let Some(t) = e.text_args() {
				app.state.alg_str.push_str(&t);
			}
			if let Some(Button::Keyboard(Key::Backspace)) = e.press_args() {
				app.state.alg_str.pop();
			}

			if let Some(Button::Keyboard(Key::Return)) = e.press_args() {
				app.state.alg = Some( app.state.alg_str.parse().unwrap() );
				app.state.alg_loaded = Some( app.state.alg_str.clone() );
				app.state.alg_str = String::new();
			}
		}
	}
}

impl<'a> App<'a> {
	pub fn new(cube: Rc<RefCell<RubiksCube>>) -> AppWindow<'a> {
		let gl = OpenGL::_3_2;
		let window = Window::new(
			gl,
			WindowSettings::new(
				"Rubik's Cube".to_owned(), 
				Size { width: 1280, height: 720 }
			).exit_on_esc(true)
		);
		let app = App {
			gl: GlGraphics::new(gl),
	        character_cache: glyph_cache::GlyphCache::new(&Path::new("C:\\windows\\fonts\\arial.ttf")).unwrap(),
			state: AppState::new(cube),
		};
		AppWindow(app, window)
	}

    fn render(&mut self, args: &RenderArgs) {
    	use graphics::*;
    	use self::color::*;

    	const ZERO: f32 = 0.0;
    	const HALF: f32 = 0.5;
    	const THQU: f32 = 0.75;
    	const FULL: f32 = 1.0;
    	const LIME	    : RGB = RGB( ZERO,  FULL,  ZERO  );
    	const GREEN 	: RGB = RGB( ZERO,  HALF,  ZERO  );
    	const RED  	    : RGB = RGB( FULL,  ZERO,  ZERO  );
    	const BLUE 	    : RGB = RGB( ZERO,  ZERO,  FULL  );
    	const YELLOW    : RGB = RGB( FULL,  FULL,  ZERO  );
    	const PINK      : RGB = RGB( FULL,  ZERO,  FULL  );
    	const CYAN		: RGB = RGB( ZERO,  FULL,  FULL  );
    	const WHITE     : RGB = RGB( FULL,  FULL,  FULL  );
    	const BLACK		: RGB = RGB( ZERO,  ZERO,  ZERO  );
    	const GRAY		: RGB = RGB( HALF,	HALF,  HALF  );
    	const SILVER	: RGB = RGB( THQU,  THQU,  THQU  );
    	const PURPLE	: RGB = RGB( HALF,  ZERO,  HALF  );
    	const MAROON	: RGB = RGB( HALF,  ZERO,  ZERO  );
    	const TEAL		: RGB = RGB( ZERO,  HALF,  HALF  );
    	const NAVY		: RGB = RGB( ZERO,  ZERO,  HALF  );
    	const ORANGE	: RGB = RGB( FULL,  HALF,  ZERO  );

    	fn face_color(c: &FaceColor) -> RGB {
    		match *c {
				FaceColor::Red => RED, 
				FaceColor::Orange => ORANGE, 
				FaceColor::Yellow => YELLOW, 
				FaceColor::White => WHITE, 
				FaceColor::Green => GREEN, 
				FaceColor::Blue => BLUE, 
    		}
    	}

    	//Setup
        let context = &Context::abs(args.width as f64, args.height as f64);
        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2) as f64);
        let mouse_context = &context.trans(self.state.mouse_cursor.0, self.state.mouse_cursor.1);
        let prev_mouse_context = &mouse_context.trans(-self.state.mouse_movement.0, -self.state.mouse_movement.1);

        const SUB_FACE_SIZE	: f64 = 40.0;
        const SUB_FACE_SPACE: f64 = 5.0;
        const FACE_SPACE 	: f64 = 5.0;
        const FACE_SIZE 	: f64 = (SUB_FACE_SIZE * 3.0) + FACE_SPACE;
        const CUBE_SIZE		: f64 = FACE_SIZE * 3.0;
        const CUBE_PADDING	: f64 = 100.0;

        fn get_sub_face_contexts(base_context: Context) -> [Context; 9] {[
       		base_context, 									base_context.trans(SUB_FACE_SIZE, 0.0), 					base_context.trans(SUB_FACE_SIZE * 2.0  , 0.0),
       		base_context.trans(0.0, SUB_FACE_SIZE), 		base_context.trans(SUB_FACE_SIZE, SUB_FACE_SIZE),			base_context.trans(SUB_FACE_SIZE * 2.0, SUB_FACE_SIZE),
       		base_context.trans(0.0, SUB_FACE_SIZE * 2.0), 	base_context.trans(SUB_FACE_SIZE, SUB_FACE_SIZE * 2.0), 	base_context.trans(SUB_FACE_SIZE * 2.0  , SUB_FACE_SIZE * 2.0), 
        ]}

        let cube_context 		= &context.trans(CUBE_PADDING, CUBE_PADDING);
        let cube_contexts = [
        	get_sub_face_contexts(cube_context.trans(FACE_SIZE 			, 0.0 	 			)),
        	get_sub_face_contexts(cube_context.trans(FACE_SIZE  		, FACE_SIZE * 2.0	)),
        	get_sub_face_contexts(cube_context.trans(0.0 				, FACE_SIZE 		)),
        	get_sub_face_contexts(cube_context.trans(FACE_SIZE * 2.0 	, FACE_SIZE 		)),
        	get_sub_face_contexts(cube_context.trans(FACE_SIZE * 2.0 	, FACE_SIZE * 2.0 	)),
        	get_sub_face_contexts(cube_context.trans(FACE_SIZE  		, FACE_SIZE 		)),
        ];

        let cube_faces = self.state.cube.borrow().faces().unwrap();

        let character_cache = &mut self.character_cache;

        let alg_str = &format!( "Algorithm: {}",  self.state.alg_str );
        let alg_loaded = &format!( "Algorithm Loaded: {}", self.state.alg_loaded.clone().unwrap_or("None".to_owned()) );
        let cube_solved = &format!( "Cube Solved?: {}", if self.state.solved {"Yes"} else {"No"} );

        let ui_context = cube_context.trans(CUBE_SIZE + CUBE_PADDING, 0.0);
        let alg_str_context = ui_context;
        let alg_loaded_context = ui_context.trans(0.0, 36.0);
        let cube_solved_context = alg_loaded_context.trans(0.0, 24.0);

        fn shrink_rect(rect: &types::Rectangle, factor: f64) -> types::Rectangle {
        	let factor2 = factor * 2.0;
        	[ rect[0] + factor, rect[1] + factor, rect[2] - factor2, rect[3] - factor2 ]
        }

    	self.gl.draw(viewport(args.width, args.height), |_, gl| {
    		clear(WHITE.into(), gl);

    		let rect = [0.0, 0.0, SUB_FACE_SIZE, SUB_FACE_SIZE];
    		let rect_inner = shrink_rect(&rect, SUB_FACE_SPACE);
    		for (sub_contexts, sub_faces) in cube_contexts.iter().zip( cube_faces.0.iter() ) {
    			for (sub_context, face) in sub_contexts.iter().zip( sub_faces.0.iter() ) {
    				rectangle( BLACK.into(), rect, sub_context.transform, gl );
    				rectangle( face_color(&face.1).into(), rect_inner, sub_context.transform, gl );
    			}
    		}

    		text::Text::colored(BLACK.into(), 48).draw("Rubiks Cube Simulator", character_cache, default_draw_state(), cube_context.trans(0.0, -48.0).transform, gl);
    		text::Text::colored(BLACK.into(), 36).draw(alg_str, character_cache, default_draw_state(), alg_str_context.transform, gl);
    		text::Text::colored(BLACK.into(), 24).draw(alg_loaded, character_cache, default_draw_state(), alg_loaded_context.transform, gl);
    		text::Text::colored(BLACK.into(), 24).draw(cube_solved, character_cache, default_draw_state(), cube_solved_context.transform, gl);
    	})
	}

	fn update(&mut self, args: &UpdateArgs) {
		if self.state.clicked && !self.state.worked && self.state.alg.is_some() {
			let ref mut alg = self.state.alg.clone().unwrap();
			self.state.cube.borrow_mut().transform(alg);
			// println!("{}", *self.state.cube.borrow());
			self.state.solved = self.state.cube.borrow().is_solved().unwrap();
			self.state.worked = true;
		}
	}
}

fn viewport(width: u32, height: u32) -> Viewport {
	Viewport {
    	rect: [0, 0, width as i32, height as i32],
    	draw_size: [width as u32, height as u32],
    	window_size: [width as u32, height as u32],
    }
}

pub struct AppState {
	cube: Rc<RefCell<RubiksCube>>,
	mouse_cursor: (f64, f64),
	mouse_movement: (f64, f64),
	// button: Option<Button>,
	clicked: bool, worked: bool, solved: bool,
	alg_str: String, alg_loaded: Option<String>, alg: Option<Algorithm>,
}

impl AppState {
	fn new(cube: Rc<RefCell<RubiksCube>>) -> AppState {
		AppState {
			cube: cube,
			mouse_cursor: (0.0, 0.0),
			mouse_movement: (0.0, 0.0),
			// button: None,
			clicked: false, worked: false,
			alg_str: String::new(), alg_loaded: None, alg: None,
			solved: true
		}
	}
}

pub mod color {
	use std::convert::{From, Into};
	use std::ops::Add;
	use std::default::Default;

	#[derive(Clone, Debug)]
	pub struct Color(pub f32, pub f32, pub f32, pub f32);
	#[derive(Clone, Debug)]
	pub struct RGB(pub f32, pub f32, pub f32);

	#[derive(Debug)] pub struct Red(pub f32);
	#[derive(Debug)] pub struct Blue(pub f32);
	#[derive(Debug)] pub struct Green(pub f32);
	#[derive(Debug)] pub struct Alpha(pub f32);

	impl Default for Color {
		fn default() -> Color {
			(Red::default(), Green::default(), Blue::default(), Alpha::default()).into()
		}
	}
	impl Default for RGB {
		fn default() -> RGB {
			(Red::default(), Green::default(), Blue::default()).into()
		}
	}
	impl Default for Alpha {
		fn default() -> Alpha {
			Alpha(1.0)
		}
	}
	impl Default for Red {
		fn default() -> Red {
			Red(1.0)
		}
	}
	impl Default for Green {
		fn default() -> Green {
			Green(1.0)
		}
	}
	impl Default for Blue {
		fn default() -> Blue {
			Blue(1.0)
		}
	}

	impl From<(f32, f32, f32)> for RGB {
		fn from((r, g, b): (f32, f32, f32)) -> RGB {
			RGB(r, g, b)
		}
	}
	impl From<Red> for RGB {
		fn from(Red(v): Red) -> RGB {
			RGB(v, 0.0, 0.0)
		}
	}
	impl From<Green> for RGB {
		fn from(Green(v): Green) -> RGB {
			RGB(0.0, v, 0.0)
		}
	}
	impl From<Blue> for RGB {
		fn from(Blue(v): Blue) -> RGB {
			RGB(0.0, 0.0, v)
		}
	}
	impl From<(Red, Green, Blue)> for RGB {
		fn from((Red(r), Green(g), Blue(b)): (Red, Green, Blue)) -> RGB {
			RGB(r, g, b)
		}
	}

	impl From<(f32, f32, f32, f32)> for Color {
		fn from((r, g, b, a): (f32, f32, f32, f32)) -> Color {
			Color(r, g, b, a)
		}
	}
	impl From<(Red, Alpha)> for Color {
		fn from((Red(c), Alpha(a)): (Red, Alpha)) -> Color {
			Color(c, 0.0, 0.0, a)
		}
	}
	impl From<(Green, Alpha)> for Color {
		fn from((Green(c), Alpha(a)): (Green, Alpha)) -> Color {
			Color(0.0, c, 0.0, a)
		}
	}
	impl From<(Blue, Alpha)> for Color {
		fn from((Blue(c), Alpha(a)): (Blue, Alpha)) -> Color {
			Color(0.0, 0.0, c, a)
		}
	}
	impl From<(RGB, Alpha)> for Color {
		fn from((RGB(r, g, b), Alpha(a)): (RGB, Alpha)) -> Color {
			Color(r, g, b, a)
		}
	}
	impl From<Red> for Color {
		fn from(Red(c): Red) -> Color {
			Color(c, 0.0, 0.0, Alpha::default().0)
		}
	}
	impl From<Green> for Color {
		fn from(Green(c): Green) -> Color {
			Color(0.0, c, 0.0, Alpha::default().0)
		}
	}
	impl From<Blue> for Color {
		fn from(Blue(c): Blue) -> Color {
			Color(0.0, 0.0, c, Alpha::default().0)
		}
	}
	impl From<RGB> for Color {
		fn from(RGB(r, g, b): RGB) -> Color {
			Color(r, g, b, Alpha::default().0)
		}
	}
	impl From<(Red, Green, Blue, Alpha)> for Color {
		fn from((Red(r), Green(g), Blue(b), Alpha(a)): (Red, Green, Blue, Alpha)) -> Color {
			Color(r, g, b, a)
		}
	}

	impl From<Color> for [f32; 4] {
		fn from(Color(r, g, b, a): Color) -> [f32; 4] {
			[r, g, b, a]
		}
	}
	impl From<RGB> for [f32; 4] {
		fn from(RGB(r, g, b): RGB) -> [f32; 4] {
			[r, g, b, Alpha::default().0]
		}
	}

	impl <'a, C: Into<Color>> Add<C> for &'a Color {
		type Output = Color;

		fn add(self, rhs: C) -> Color {
			let Color(r_fg, g_fg, b_fg, a_fg): Color = rhs.into();
			let Color(r_bg, g_bg, b_bg, a_bg): Color = *self;

			let alpha_f = a_bg + a_fg - (a_bg / a_fg);
			if alpha_f < 1.0e-6 { return Color(0.0, 0.0, 0.0, 0.0) }

			let (r_bg_a, g_bg_a, b_bg_a) = (r_bg * a_bg, g_bg * a_bg, b_bg * a_bg);
			let (r_fg_a, g_fg_a, b_fg_a) = (r_fg * a_fg, g_fg * a_fg, b_fg * a_fg);

			let red_f   = r_fg_a + r_bg_a * (1.0 - a_fg);
			let green_f = g_fg_a + g_bg_a * (1.0 - a_fg);
			let blue_f  = b_fg_a + b_bg_a * (1.0 - a_fg);

			(red_f / alpha_f, green_f / alpha_f, blue_f / alpha_f, alpha_f).into()
		}
	}

	impl<'a, C: Into<RGB>> Add<C> for &'a RGB {
		type Output = RGB;

		fn add(self, rhs: C) -> RGB {
			let rhs: RGB = rhs.into();
			RGB((self.0 + rhs.0).min(1.0), (self.1 + rhs.1).min(1.0), (self.2 + rhs.2).min(1.0))
		}
	}

	pub trait Mix<FG = Self> {
		type Output;

		fn mix(self, fg: FG) -> <Self as Mix>::Output;
	}

	impl<F: Into<RGB>, B: Into<RGB>> Mix<F> for B {
		type Output = RGB;

		fn mix(self, fg: F) -> RGB {
			fn ave(f: f32, b: f32) -> f32 { (f + b) / 2.0 }

			let RGB(rb, gb, bb) = self.into();
			let RGB(rf, gf, bf) = fg.into();

			RGB( ave(rb, rf), ave(gb, gf), ave(bb, bf) )
		}
	}
}