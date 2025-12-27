use crate::err::B::X;
use crate::err::LlccB;
use crate::file_io::Dest;
use crate::file_io::DestKind;
use crate::register::*;
use core::str;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::iter::Peekable;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;
use std::str::Chars;

/// syscall number of exit
const EXIT: u16 = 93;

macro_rules! ret_val {
	($value:expr) => {
		Instruction::Mov { target: Register::X8, value: $value, }
	};
}

pub struct Assembler {
	dest: Dest,
}

impl Assembler {}

pub fn asm_str(src: impl Into<String,>,) -> LlccB<impl Into<String,>,> {
	use Instruction::*;
	use Register::*;

	let chars = src.into();
	let mut chars = chars.chars().peekable();
	let mut inst_list = vec![
		Section(SectionKind::Text,),
		Global(&["_start",],),
		Symbol("_start",),
	];

	let first_num = parse_number(&mut chars,)?;
	inst_list.push(Mov {
		target: X0,
		value:  RegisterOrImmediate::try_from(first_num,)?,
	},);

	while let Some(c,) = chars.next() {
		match c {
			'+' => {
				let num = parse_number(&mut chars,)?;
				inst_list.push(Add {
					target: X0,
					lhs:    X0,
					rhs:    RegisterOrImmediate::try_from(num,)?,
				},);
			},
			'-' => {
				let num = parse_number(&mut chars,)?;
				inst_list.push(Sub {
					target: X0,
					lhs:    X0,
					rhs:    RegisterOrImmediate::try_from(num,)?,
				},);
			},
			a => todo!("unexpected char `{a}`"),
		}
	}

	inst_list.push(ret_val!(RegisterOrImmediate::try_from(EXIT as i32)?),);
	inst_list.push(Svc { syscall: EXIT, },);
	X(ReadableAsm::from_instructions(inst_list,),)
}

#[cfg(target_arch = "aarch64")]
enum Instruction<'a,> {
	Section(SectionKind,),
	Global(&'a [&'a str],),
	Symbol(&'a str,),
	Svc {
		/// this number is ignored on aarch64 linux
		syscall: u16,
	},
	Mov {
		target: Register,
		value:  RegisterOrImmediate<12, false,>,
	},
	Add {
		target: Register,
		lhs:    Register,
		rhs:    RegisterOrImmediate<12, false,>,
	},
	Sub {
		target: Register,
		lhs:    Register,
		rhs:    RegisterOrImmediate<12, false,>,
	},
}

impl<'a,> From<Instruction<'a,>,> for String {
	fn from(val: Instruction<'a,>,) -> Self {
		const SEPARATOR: &str = ", ";
		use Instruction::*;
		let mut val = match val {
			Section(section_kind,) => {
				let kind: String = section_kind.into();
				format!(".{kind}")
			},
			Global(items,) => {
				let items = items.join(" ",);
				format!(".global {items}")
			},
			Symbol(s,) => format!("{s}:"),
			Svc { syscall, } => format!("svc #{syscall}"),
			Mov { target, value, } => {
				[format!("mov {}", target), value.to_string(),].join(SEPARATOR,)
			},
			Add { target, lhs, rhs, } => {
				[format!("add {}", target,), lhs.to_string(), rhs.to_string(),]
					.join(SEPARATOR,)
			},
			Sub { target, lhs, rhs, } => {
				[format!("sub {}", target), lhs.to_string(), rhs.to_string(),]
					.join(SEPARATOR,)
			},
		};

		val.push('\n',);
		val
	}
}

struct ReadableAsm<'a,>(Vec<Instruction<'a,>,>,);

impl<'a,> ReadableAsm<'a,> {
	fn from_instructions(inst_list: Vec<Instruction<'a,>,>,) -> Self {
		Self(inst_list,)
	}
}

impl<'a,> From<ReadableAsm<'a,>,> for String {
	fn from(val: ReadableAsm<'a,>,) -> Self {
		val.0
			.into_iter()
			.map(|inst| {
				let inst: String = inst.into();
				inst
			},)
			.collect::<Vec<String,>>()
			.concat()
	}
}

enum SectionKind {
	Text,
}

impl From<SectionKind,> for String {
	fn from(val: SectionKind,) -> Self {
		let value = match val {
			SectionKind::Text => "text",
		};
		value.to_string()
	}
}

fn parse_number(chars: &mut Peekable<Chars,>,) -> LlccB<i32,> {
	let mut num = "".to_string();
	while let Some(c,) = chars.peek() {
		if c.is_numeric() {
			num.push(*c,);
		} else {
			let num = num.parse()?;
			return X(num,);
		}

		chars.next();
	}

	let num = num.parse()?;
	X(num,)
}

/// # Return
///
/// returns path to generated assembly file
pub fn write_asm(
	asm_src: impl Into<String,>,
	asm_path: impl Into<PathBuf,>,
) -> LlccB<impl Into<PathBuf,>,> {
	let asm_path = asm_path.into();
	let mut output = fs::OpenOptions::new()
		.truncate(true,)
		.create(true,)
		.write(true,)
		.open(&asm_path,)?;
	output.write_all(asm_src.into().as_bytes(),)?;
	X(asm_path,)
}

pub fn run_cmd<I, S,>(cmd: impl Into<String,>, args: I,) -> LlccB<ExitStatus,>
where
	I: IntoIterator<Item = S,>,
	S: AsRef<OsStr,>,
{
	let status = Command::new(cmd.into(),).args(args,).status()?;
	X(status,)
}

pub fn clear_out(dest: &Dest,) -> LlccB<(),> {
	let out = dest.path(DestKind::OutDir,).into();
	if fs::exists(&out,)? {
		fs::remove_dir_all(out,)?;
	}

	X((),)
}
