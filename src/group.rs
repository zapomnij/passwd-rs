use std::ffi::{CStr, CString};
use std::io;
use std::os::raw::*;
use std::ptr;

/// Group struct
pub struct Group {
	/// Group name
	pub name: String,
	/// Group password
	pub password: Option<String>,
	/// Group ID
	pub gid: c_uint,

	/// Members of group
	pub members: Vec<String>,
}

// Raw
#[repr(C)]
struct Raw {
	gr_name: *const c_char,
	gr_passwd: *const c_char,
	gr_gid: c_uint,

	gr_mem: *const *const c_char,
}
extern "C" {
	fn getgrnam(name: *const c_char) -> *const Raw;
	fn getgrgid(gid: c_uint) -> *const Raw;
}

impl Group {
	unsafe fn convert(s: *const Raw) -> io::Result<Self> {
		if s == ptr::null() {
			return Err(io::Error::last_os_error());
		}

		let name = match CStr::from_ptr((*s).gr_name).to_str() {
			Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData)),
			Ok(o) => o.to_string(),
		};
		let passwd = match CStr::from_ptr((*s).gr_passwd).to_str() {
			Err(_) => None,
			Ok(o) => Some(o.to_string()),
		};
		let gid = (*s).gr_gid;

		let mut members = Vec::new();
		if (*s).gr_mem != ptr::null() {
			for i in 0.. {
				let memeber_ptr = *(*s).gr_mem.offset(i);
				if memeber_ptr != ptr::null() {
					match CStr::from_ptr(memeber_ptr).to_str() {
						Ok(o) => members.push(o.to_string()),
						Err(_) => continue,
					}
				} else {
					break;
				}
			}
		}

		Ok(Self {
			name,
			password: passwd,
			gid,
			members,
		})
	}

	/// Get group information by group name
	pub fn new_from_groupname(username: &str) -> io::Result<Self> {
		unsafe {
			let raw = match CString::new(username) {
				Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData)),
				Ok(o) => o,
			};

			Self::convert(getgrnam(raw.as_ptr()))
		}
	}

	/// Get group information by group ID
	pub fn new_from_gid(gid: c_uint) -> io::Result<Self> {
		unsafe { Self::convert(getgrgid(gid)) }
	}

	/// Convert members array (`Vector`) to `String`
	pub fn display_members(&self) -> String {
		let mut m = String::new();
		for member in &self.members {
			m.push_str(format!("{} ", member).as_str());
		}

		m
	}
}
