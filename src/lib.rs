//! # passwd-rs
//! Crate/Library providing a functions to get information about groups, users and shadow passwords.
//!
//! ## Example
//! ```rust
//! use passwd_rs::{Group, User, Shadow, AccountStatus};
//!
//! fn main() -> std::io::Result<()> {
//! 	let user = User::current_user()?;
//! 	let password;
//! 	if user.passwd.as_ref().unwrap().eq("x") {
//! 		// WARN! This works only if program is executed as root
//! 		let shadow = match Shadow::new_from_username(&user.name.clone()) {
//!  			Err(e) => {
//! 				if e.kind() == std::io::ErrorKind::PermissionDenied {
//! 					println!("Must be run as root to access shadow passwords");
//! 				}
//! 				else { return Err(e) };
//!
//!					return Ok(());
//! 			},
//! 			Ok(o) => o,
//! 		};
//! 		if let AccountStatus::Active(passwd) = shadow.passwd {
//! 			password = passwd;
//! 		} else {
//! 			password = shadow.passwd.to_string();
//! 		}
//! 	} else {
//! 		password = user.passwd.unwrap();
//! 	}
//!
//! 	let group = Group::new_from_gid(user.gid.clone())?;
//!
//! 	println!("Group details:");
//! 	println!("Name: {}", group.name);
//! 	println!("ID: {}", group.gid);
//! 	println!("Members: {}", group.display_members());
//! 	println!();
//! 	println!("User details:");
//! 	println!("Name: {}", user.name);
//! 	println!("ID: {}", user.uid);
//! 	println!("Password: {}", password);
//!
//! 	Ok(())
//! }
//! ```

pub mod group;
pub mod shadow;
pub mod user;

use std::os::raw::*;
pub use group::Group;
pub use shadow::AccountStatus;
pub use shadow::Shadow;
pub use user::User;

#[cfg(test)]
mod tests {
	use crate::Group;
	use crate::Shadow;
	use crate::User;
	use std::io;

	#[test]
	fn user_from_name() -> std::io::Result<()> {
		let passwd = User::new_from_name("root")?;

		assert!(passwd.name.eq("root"));

		Ok(())
	}

	#[test]
	fn user_from_uid() -> std::io::Result<()> {
		let passwd = User::new_from_uid(0)?;

		assert_eq!(passwd.uid, 0);

		Ok(())
	}

	#[test]
	fn group_from_name() -> std::io::Result<()> {
		let grp = Group::new_from_groupname("root")?;

		assert!(grp.name.eq("root"));

		Ok(())
	}

	#[test]
	fn group_from_gid() -> std::io::Result<()> {
		let grp = Group::new_from_gid(0)?;

		assert_eq!(grp.gid, 0);

		Ok(())
	}

	// Need to be run as root
	#[test]
	fn shadow_from_username() -> std::io::Result<()> {
		let shadow = Shadow::new_from_username("root");
		if let Err(e) = shadow {
			if e.kind() == io::ErrorKind::PermissionDenied {
				eprintln!("Shadow test must be run as root");
				return Ok(());
			}

			return Err(e);
		}

		assert!(shadow.as_ref().unwrap().name.eq("root"));

		Ok(())
	}
}

extern "C" {
	fn getuid() -> c_uint;
}

/// Get current user's ID
pub fn whoami_uid() -> c_uint {
	unsafe { getuid() }
}

/// Get current user's username
pub fn whoami() -> std::io::Result<String> {
	let user = User::current_user()?;
	Ok(user.name)
}