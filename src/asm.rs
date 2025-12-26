use crate::err::B::X;
use crate::err::LlccB;
use crate::err::LlccError;
use crate::file_io::Dest;
use crate::file_io::DestKind;
use core::str;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::iter::Peekable;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;
use std::str::Chars;
use std::str::FromStr;

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

	inst_list.push(ret_val!(RegisterOrImmediate::Immediate(EXIT)),);
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

macro_rules! ret_val {
	($value:expr) => {
		Instruction::Mov { target: Register::X8, value: $value, }
	};
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

//  TODO: ---
#[derive(strum::Display,)]
enum RegisterOrImmediate<const BIT: u8 = 16, const IS_SIGNED: bool = true,> {
	#[strum(to_string = "{0}")]
	Register(Register,),
	#[strum(to_string = "#{0}")]
	Immediate(Immediate<BIT, IS_SIGNED,>,),
}

impl TryFrom<Into<String,>,> for RegisterOrImmediate {
	type Error = LlccError;

	fn try_from(value: Into<String,>,) -> Result<Self, Self::Error,> {
		Ok(Self::Register(Register::from_str(value.into().as_str(),)?,),)
	}
}

impl<I: num::Integer,> TryFrom<I,> for RegisterOrImmediate {
	type Error = LlccError;

	/// if `value` overflows, max value allowed fot the immediate
	fn try_from(value: I,) -> Result<Self, Self::Error,> {
		todo!()
	}
}

struct Immediate<const BIT: u8, const IS_SIGNED: bool,>(i32,);

impl<const BIT: u8, const IS_SIGNED: bool,> Display
	for Immediate<BIT, IS_SIGNED,>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
		let formatted = format_args!("{}", self.0);
		f.write_fmt(formatted,)
	}
}

#[derive(strum::Display, strum::EnumString,)]
enum Register {
	#[strum(ascii_case_insensitive)]
	X31,
	#[strum(ascii_case_insensitive)]
	X30,
	#[strum(ascii_case_insensitive)]
	X29,
	#[strum(ascii_case_insensitive)]
	X28,
	#[strum(ascii_case_insensitive)]
	X27,
	#[strum(ascii_case_insensitive)]
	X26,
	#[strum(ascii_case_insensitive)]
	X25,
	#[strum(ascii_case_insensitive)]
	X24,
	#[strum(ascii_case_insensitive)]
	X23,
	#[strum(ascii_case_insensitive)]
	X22,
	#[strum(ascii_case_insensitive)]
	X21,
	#[strum(ascii_case_insensitive)]
	X20,
	#[strum(ascii_case_insensitive)]
	X19,
	#[strum(ascii_case_insensitive)]
	X18,
	#[strum(ascii_case_insensitive)]
	X17,
	#[strum(ascii_case_insensitive)]
	X16,
	#[strum(ascii_case_insensitive)]
	X15,
	#[strum(ascii_case_insensitive)]
	X14,
	#[strum(ascii_case_insensitive)]
	X13,
	#[strum(ascii_case_insensitive)]
	X12,
	#[strum(ascii_case_insensitive)]
	X11,
	#[strum(ascii_case_insensitive)]
	X10,
	#[strum(ascii_case_insensitive)]
	X9,
	#[strum(ascii_case_insensitive)]
	X8,
	#[strum(ascii_case_insensitive)]
	X7,
	#[strum(ascii_case_insensitive)]
	X6,
	#[strum(ascii_case_insensitive)]
	X5,
	#[strum(ascii_case_insensitive)]
	X4,
	#[strum(ascii_case_insensitive)]
	X3,
	#[strum(ascii_case_insensitive)]
	X2,
	#[strum(ascii_case_insensitive)]
	X1,
	#[strum(ascii_case_insensitive)]
	X0,
	#[strum(ascii_case_insensitive)]
	W30,
	#[strum(ascii_case_insensitive)]
	W29,
	#[strum(ascii_case_insensitive)]
	W28,
	#[strum(ascii_case_insensitive)]
	W27,
	#[strum(ascii_case_insensitive)]
	W26,
	#[strum(ascii_case_insensitive)]
	W25,
	#[strum(ascii_case_insensitive)]
	W24,
	#[strum(ascii_case_insensitive)]
	W23,
	#[strum(ascii_case_insensitive)]
	W22,
	#[strum(ascii_case_insensitive)]
	W21,
	#[strum(ascii_case_insensitive)]
	W20,
	#[strum(ascii_case_insensitive)]
	W19,
	#[strum(ascii_case_insensitive)]
	W18,
	#[strum(ascii_case_insensitive)]
	W17,
	#[strum(ascii_case_insensitive)]
	W16,
	#[strum(ascii_case_insensitive)]
	W15,
	#[strum(ascii_case_insensitive)]
	W14,
	#[strum(ascii_case_insensitive)]
	W13,
	#[strum(ascii_case_insensitive)]
	W12,
	#[strum(ascii_case_insensitive)]
	W11,
	#[strum(ascii_case_insensitive)]
	W10,
	#[strum(ascii_case_insensitive)]
	W9,
	#[strum(ascii_case_insensitive)]
	W8,
	#[strum(ascii_case_insensitive)]
	W7,
	#[strum(ascii_case_insensitive)]
	W6,
	#[strum(ascii_case_insensitive)]
	W5,
	#[strum(ascii_case_insensitive)]
	W4,
	#[strum(ascii_case_insensitive)]
	W3,
	#[strum(ascii_case_insensitive)]
	W2,
	#[strum(ascii_case_insensitive)]
	W1,
	#[strum(ascii_case_insensitive)]
	W0,
	#[strum(ascii_case_insensitive)]
	Sp,
	#[strum(ascii_case_insensitive)]
	Xzr,
	#[strum(ascii_case_insensitive)]
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
