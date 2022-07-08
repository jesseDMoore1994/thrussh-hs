#[macro_use]
extern crate curryrs;

use curryrs::types::*;
use tokio::runtime::Runtime;

mod client;

#[repr(C)]
pub struct Data {
	pub a: bool,
	pub b: u32,
	pub c: String
}

#[no_mangle]
pub extern fn create_async_runtime() -> *mut tokio::runtime::Runtime {
  let b = Box::into_raw(Box::new(Runtime::new().unwrap()));
  println!("runtime: {:p}", b);
  b
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn release_async_runtime(ptr: *mut tokio::runtime::Runtime) {
    if ptr.is_null() {
        panic!("Cannot release a runtime that does not exist!");
    }
    Box::from_raw(ptr);
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn create_session(runtime: *mut tokio::runtime::Runtime) -> *mut client::Session {
    if runtime.is_null() {
        panic!("I need the tokio runtime to continue!");
    }

	let rt = Box::from_raw(runtime);
	let res = rt.block_on(async {
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
pub unsafe extern fn callex(rt: *mut tokio::runtime::Runtime, session: *mut client::Session) {
	println!("callex");
	let rt = {
		assert!(!rt.is_null());
		Box::from_raw(&mut *rt)
	};
    let session = {
		assert!(!session.is_null());
        &mut *session
    };
	println!("runtime in callex: {:p}", rt);
	println!("session in callex: {:p}", session);

	match rt.block_on(async { 
		session.call("whoami").await
	}) {
		Ok(_) => println!("Ok"),
		Err(e) => println!("{}", e)
	};
}

#[no_mangle]
pub extern fn create_data_c() -> *mut Data {
	Box::into_raw(Box::new(Data{
		a: true,
		b: 1,
		c: String::from("AAA")
	}))
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn destroy_data_c(d: *mut Data) {
	if d.is_null() {
		panic!("cannot destroy empty Data")
	}
	Box::from_raw(d);
}

// Place each function you want exported into the safe_ffi! macro and it will
// export each one and place the pub extern for you!
safe_ffi! (

	fn print_data_c(d: &mut Data) -> () {
		println!("{}", d.c)
	}

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
