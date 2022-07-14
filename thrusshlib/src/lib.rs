#[macro_use]
extern crate curryrs;
extern crate lazy_static;

use curryrs::types::*;
use tokio::runtime::Runtime;
use lazy_static::lazy_static;

mod client;

#[repr(C)]
pub struct Data {
	pub a: bool,
	pub b: u32,
	pub c: String
}

lazy_static! {
	    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn create_session() -> *mut client::Session {
	let res = RUNTIME.block_on(async {
		  client::Session::connect("127.0.0.1:22", "test", "test").await
	});
	let b = Box::into_raw(Box::new(res.unwrap()));
	println!("session: {:p}", b);
	b
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn destroy_session(s: *mut client::Session) {
	if s.is_null() {
		panic!("cannot destroy null session!")
	}

	Box::from_raw(s);
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn callex(session: *mut client::Session) {
    let session = {
		assert!(!session.is_null());
        &mut *session
    };
	match RUNTIME.block_on(async { 
		session.call("whoami").await
	}) {
		Ok(res) => println!("{}", res.output()),
		Err(e) => println!("{}", e)
	};
}

// Place each function you want exported into the safe_ffi! macro and it will
// export each one and place the pub extern for you!
safe_ffi! (

	fn hello() -> () {
		println!("Hello")
	}

	fn double(x: I32) -> I32 {
		2 * x
	}

	fn square(x: U64) -> U64 {
		x * x
	}

	fn cube(x: I64) -> I64 {
		x * x * x
	}

);
