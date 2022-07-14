extern crate lazy_static;

use libc::c_char;
use tokio::runtime::Runtime;
use lazy_static::lazy_static;
use std::ffi::CStr;

mod client;

lazy_static! {
	    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

unsafe fn convert_to_r_str<'a>(c: *const c_char) -> &'a str {
    assert!(!c.is_null());
    CStr::from_ptr(c).to_str().unwrap()
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern fn create_session(
    target: *const c_char,
    username: *const c_char,
    password: *const c_char,
) -> *mut client::Session {
	let res = RUNTIME.block_on(async {
            client::Session::connect(
                convert_to_r_str(target),
                convert_to_r_str(username),
                convert_to_r_str(password),
            ).await
	});
	Box::into_raw(Box::new(res.unwrap()))
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
pub unsafe extern fn callex(
	session: *mut client::Session,
    command: *const c_char,
) {
    let session = {
		assert!(!session.is_null());
        &mut *session
    };
	match RUNTIME.block_on(async { 
		session.call(convert_to_r_str(command)).await
	}) {
		Ok(res) => println!("{}", res.output()),
		Err(e) => println!("{}", e)
	};
}
