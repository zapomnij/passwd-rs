use std::ffi::{CStr, CString};
use std::io;
use std::os::raw::*;

/// User information struct
pub struct User {
	/// Username
	pub name: String,
	/// Full name of the user
	pub fullname: String,
	/// Home directory
	pub homedir: String,
	/// Path to the shell in use
	pub shell: String,
	/// User password. Can be equal to 'x' what means that, the password is stored in shadow. Check `Shadow` struct.
	pub passwd: Option<String>,

	/// User ID
	pub uid: c_uint,
	/// Group ID, which is created for that user
	pub gid: c_uint,
}

// Raw
#[repr(C)]
struct Raw {
	pw_name: *const c_char,
	pw_passwd: *const c_char,
	pw_uid: c_uint,
	pw_gid: c_uint,
	pw_gecos: *const c_char,
	pw_dir: *const c_char,
	pw_shell: *const c_char,
}
extern "C" {
	fn getpwnam(name: *const c_char) -> *const Raw;
	fn getpwuid(uid: c_uint) -> *const Raw;
}
unsafe fn unref(ptr: *const c_char) -> String {
	CStr::from_ptr(ptr).to_str().unwrap_or("").to_string()
}

impl User {
	unsafe fn convert(raw: *const Raw) -> io::Result<Self> {
		if raw == std::ptr::null() {
			return Err(io::Error::last_os_error());
		}

		let name = unref((*raw).pw_name);
		let passwd = {
			let p = unref((*raw).pw_passwd);
			let ret;
			if p.eq("") {
				ret = None
			} else {
				ret = Some(p);
			}
			ret
		};
		let uid = (*raw).pw_uid;
		let gid = (*raw).pw_gid;
		let fullname = unref((*raw).pw_gecos);
		let homedir = unref((*raw).pw_dir);
		let shell = unref((*raw).pw_shell);

		Ok(Self {
			name,
			fullname,
			homedir,
			shell,
			passwd,
			uid,
			gid,
		})
	}

	/// Get user information by user ID
	pub fn new_from_uid(uid: c_uint) -> io::Result<Self> {
		unsafe { Self::convert(getpwuid(uid)) }
	}

	/// Get user information by it's username
	pub fn new_from_name(name: &str) -> io::Result<Self> {
		unsafe {
			let cstr = match CString::new(name) {
				Ok(o) => o,
				Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData)),
			};

			Self::convert(getpwnam(cstr.as_ptr()))
		}
	}

	/// Fetch information about current user
	pub fn current_user() -> io::Result<Self> {
		unsafe {
			let uid = crate::getuid();

			Self::convert(getpwuid(uid))
		}
	}
}