use std::ffi::{CStr, CString};
use std::io;
use std::os::raw::*;

/// Account status
pub enum AccountStatus {
	/// Account is enabled/active and password is stored in `String` attached to `Active`
	Active(String),
	/// Account has no password
	NoPassword,
	/// A regular user can't enter that account
	NoLogin,
	/// Unknown status
	Unknown,
}
impl AccountStatus {
	/// Convert `AccountStatus` to `String`
	pub fn to_string(&self) -> String {
		match self {
			Self::Unknown => "Unknown password".to_string(),
			Self::Active(p) => p.to_string(),
			Self::NoLogin => "Disabled account".to_string(),
			Self::NoPassword => "No password".to_string(),
		}
	}
}

/// Struct for information about shadow passwords
pub struct Shadow {
	/// Account name
	pub name: String,
	/// Password. Stored in `AccountStatus`
	pub passwd: AccountStatus,

	/// Date of last change (measured in days since 1970-01-01 00:00:00 +0000 (UTC))
	pub last_chage: c_long,
	/// Min number of days between changes
	pub min: c_long,
	/// Max number of days between changes
	pub max: c_long,

	/// Number of days before password expires to warn user to change it
	pub warn: c_long,
	/// Number of days after password expires until account is disabled
	pub inactive: c_long,
	/// Date when account expires (measured in days since 1970-01-01 00:00:00 +0000 (UTC))
	pub expires: c_long,
}

// Raw
#[repr(C)]
struct Spwd {
	sp_namp: *const c_char,
	sp_pwdp: *const c_char,

	sp_lstchg: c_long,
	sp_min: c_long,
	sp_max: c_long,
	sp_warn: c_long,

	sp_inact: c_long,
	sp_expire: c_long,

	sp_flag: c_ulong,
}
extern "C" {
	fn getspnam(name: *const c_char) -> *const Spwd;
}

impl Shadow {
	unsafe fn convert(raw: *const Spwd) -> io::Result<Self> {
		if raw == std::ptr::null() {
			return Err(io::Error::last_os_error());
		}

		let name = match CStr::from_ptr((*raw).sp_namp).to_str() {
			Ok(o) => o.to_string(),
			Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData)),
		};

		let passwd: AccountStatus = {
			match CStr::from_ptr((*raw).sp_pwdp).to_str() {
				Err(_) => AccountStatus::Unknown,
				Ok(o) => {
					if o.eq("*") {
						AccountStatus::NoLogin
					} else if o.eq("!") {
						AccountStatus::NoPassword
					} else {
						AccountStatus::Active(o.to_string())
					}
				}
			}
		};

		Ok(Self {
			name,
			passwd,
			last_chage: (*raw).sp_lstchg,
			min: (*raw).sp_min,
			max: (*raw).sp_max,
			warn: (*raw).sp_warn,
			inactive: (*raw).sp_inact,
			expires: (*raw).sp_expire,
		})
	}

	/// Get information about the shadow passwords of the specified user.
	pub fn new_from_username(name: &str) -> io::Result<Self> {
		unsafe {
			let ptr = match CString::new(name) {
				Err(_) => return Err(io::Error::from(io::ErrorKind::InvalidData)),
				Ok(o) => o,
			};

			Self::convert(getspnam(ptr.as_ptr()))
		}
	}
}
