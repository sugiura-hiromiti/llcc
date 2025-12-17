use crate::err::B::X;
use crate::err::LlccB;
use crate::file_io::Dest;
use crate::file_io::DestKind;
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
		value:  RegisterOrImmediate::Immediate(first_num,),
	},);

	while let Some(c,) = chars.next() {
		match c {
			'+' => {
				let num = parse_number(&mut chars,)?;
				inst_list.push(Add {
					target: X0,
					lhs:    X0,
					rhs:    RegisterOrImmediate::Immediate(num,),
				},);
			},
			'-' => {
				let num = parse_number(&mut chars,)?;
				inst_list.push(Sub {
					target: X0,
					lhs:    X0,
					rhs:    RegisterOrImmediate::Immediate(num,),
				},);
			},
			a => todo!("unexpected char `{a}`"),
		}
	}

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
		value:  RegisterOrImmediate,
	},
	Add {
		target: Register,
		lhs:    Register,
		rhs:    RegisterOrImmediate,
	},
	Sub {
		target: Register,
		lhs:    Register,
		rhs:    RegisterOrImmediate,
	},
}

impl<'a,> From<Instruction<'a,>,> for String {
	fn from(val: Instruction<'a,>,) -> Self {
		use Instruction::*;
		match val {
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
				[format!("mov {}", target), value.to_string(),].join(",",)
			},
			Add { target, lhs, rhs, } => {
				[format!("add {}", target,), lhs.to_string(), rhs.to_string(),]
					.join(",",)
			},
			Sub { target, lhs, rhs, } => {
				[format!("sub {}", target), lhs.to_string(), rhs.to_string(),]
					.join(",",)
			},
		}
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
			.join("\n",)
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

#[derive(strum::Display,)]
enum RegisterOrImmediate {
	#[strum(to_string = "{0}")]
	Register(Register,),
	#[strum(to_string = "#{0}")]
	Immediate(i32,),
}

#[derive(strum::Display,)]
enum Register {
	X31,
	X30,
	X29,
	X28,
	X27,
	X26,
	X25,
	X24,
	X23,
	X22,
	X21,
	X20,
	X19,
	X18,
	X17,
	X16,
	X15,
	X14,
	X13,
	X12,
	X11,
	X10,
	X9,
	X8,
	X7,
	X6,
	X5,
	X4,
	X3,
	X2,
	X1,
	X0,
	W30,
	W29,
	W28,
	W27,
	W26,
	W25,
	W24,
	W23,
	W22,
	W21,
	W20,
	W19,
	W18,
	W17,
	W16,
	W15,
	W14,
	W13,
	W12,
	W11,
	W10,
	W9,
	W8,
	W7,
	W6,
	W5,
	W4,
	W3,
	W2,
	W1,
	W0,
	Sp,
	Xzr,
	Wzr,
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
