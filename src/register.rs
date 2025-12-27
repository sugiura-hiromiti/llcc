use crate::err::LlccError;
use std::fmt::Display;
use std::str::FromStr;

#[derive(strum::Display,)]
pub enum RegisterOrImmediate<
	const BIT: u8,
	const IS_SIGNED: bool,
	R = Register,
	I = Immediate<BIT, IS_SIGNED,>,
> where
	R: Display,
	I: Display,
{
	#[strum(to_string = "{0}")]
	Register(R,),
	#[strum(to_string = "#{0}")]
	Immediate(I,),
}

impl<const BIT: u8, const IS_SIGNED: bool,> TryFrom<&str,>
	for RegisterOrImmediate<BIT, IS_SIGNED,>
{
	type Error = LlccError;

	fn try_from(value: &str,) -> Result<Self, Self::Error,> {
		Ok(Self::Register(Register::from_str(value,)?,),)
	}
}

impl<const BIT: u8, const IS_SIGNED: bool,> TryFrom<i32,>
	for RegisterOrImmediate<BIT, IS_SIGNED,>
{
	type Error = LlccError;

	/// if `value` overflows, max value allowed fot the immediate
	fn try_from(value: i32,) -> Result<Self, Self::Error,> {
		Ok(Self::Immediate(Immediate(value,),),)
	}
}

//  TODO: shift演算どうする?
pub struct Immediate<const BIT: u8, const IS_SIGNED: bool,>(i32,);

impl<const BIT: u8, const IS_SIGNED: bool,> TryFrom<i32,>
	for Immediate<BIT, IS_SIGNED,>
{
	type Error = LlccError;

	fn try_from(value: i32,) -> Result<Self, Self::Error,> {
		// 型チェックフラグ達
		let undesired_sign = !IS_SIGNED && value < 0;
		let overflowing = {
			let unsignated_value =
				if IS_SIGNED { value & i32::MAX } else { value };
			(unsignated_value >> BIT) != 0
		};

		if undesired_sign || overflowing {
			Err(LlccError::mismatch_imm(value, BIT, IS_SIGNED,),)
		} else {
			Ok(Self(value,),)
		}
	}
}

impl<const BIT: u8, const IS_SIGNED: bool,> Display
	for Immediate<BIT, IS_SIGNED,>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
		let formatted = format_args!("{}", self.0);
		f.write_fmt(formatted,)
	}
}

#[derive(strum::Display, strum::EnumString,)]
pub enum Register {
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
