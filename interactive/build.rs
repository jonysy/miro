use std::{env, fs, io};
use std::io::Write;

fn main() {

	if let Err(error) = copy("main.rs") {

		panic!("{}", error);
	}
}

fn copy(name: &str) -> io::Result<()> {

	let mut inp = fs::File::open(name)?;

	let mut f = {

		let dir = env::var("OUT_DIR").unwrap();
		let path = dir + "/main.rs";
		fs::File::create(&path)?
	};

	writeln!(f, "{{")?;

	let _ = io::copy(&mut inp, &mut f)?;

	writeln!(f, "")?;
	writeln!(f, "Ok(())")?;
	writeln!(f, "}}")?;

	Ok(())
}