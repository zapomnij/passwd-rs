# passwd-rs
Crate/Library providing a functions to get information about groups, users and shadow passwords.

## Example
```rust
use passwd_rs::{Group, User, Shadow, AccountStatus};

fn main() -> std::io::Result<()> {
	let user = User::current_user()?;
	let password;
	if user.passwd.as_ref().unwrap().eq("x") {
		// WARN! This works only if program is executed as root
		let shadow = match Shadow::new_from_username(&user.name.clone()) {
			Err(e) => {
				if e.kind() == std::io::ErrorKind::PermissionDenied {
					println!("Must be run as root to access shadow passwords");
				} else { return Err(e) };

				return Ok(());
			},
			Ok(o) => o,
		};
		if let AccountStatus::Active(passwd) = shadow.passwd {
			password = passwd;
		} else {
			password = shadow.passwd.to_string();
		}
	} else {
		password = user.passwd.unwrap();
	}

	let group = Group::new_from_gid(user.gid.clone())?;

	println!("Group details:");
	println!("Name: {}", group.name);
	println!("ID: {}", group.gid);
	println!("Members: {}", group.display_members());
	println!();
	println!("User details:");
	println!("Name: {}", user.name);
	println!("ID: {}", user.uid);
	println!("Password: {}", password);

	Ok(())
}
```