use crate::err::B::X;
use crate::err::Container;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::err::LlccB;

const PRJ_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Debug,)]
pub struct Dest {
	uuid:    Uuid,
	prefix:  Option<String,>,
	postfix: Option<String,>,
}

impl Default for Dest {
	fn default() -> Self {
		Self::new(None, Uuid::new_v4(), None,).unwrap()
		// let rslt =
		// 	Self { uuid: Uuid::new_v4(), prefix: None, postfix: None, };
		// let out_path = rslt.path(DestKind::OutDir,).into();
		// if !out_path.exists() {
		// 	fs::create_dir_all(&out_path,).unwrap_or_else(|e| {
		// 		panic!(
		// 			"failed to create output directory at `{}`. not able to \
		// 			 proceed compilation. abort\ndetail: {e:#?}",
		// 			out_path.display()
		// 		)
		// 	},);
		// }
		//
		// rslt
	}
}

impl Dest {
	pub fn new(
		prefix: Option<String,>,
		uuid: Uuid,
		postfix: Option<String,>,
	) -> LlccB<Self,> {
		let rslt = Self { uuid, prefix, postfix, };
		let out_path = rslt.path(DestKind::OutDir,).into();
		if !out_path.exists() {
			fs::create_dir_all(&out_path,)?;
		}

		X(rslt,)
	}

	pub fn path(&self, kind: DestKind,) -> impl Into<PathBuf,> {
		self.base_of(kind,).into().join(kind.path(),)
	}

	pub fn base_of(&self, kind: DestKind,) -> impl Into<PathBuf,> {
		let mut assets = PathBuf::from(PRJ_DIR,).join("assets",);
		if kind == DestKind::Src {
			return assets.join(kind.path(),);
		}
		assets.push("out",);
		assets.push(self.uuid_part().into(),);

		assets
	}

	pub fn uuid_part(&self,) -> impl Into<PathBuf,> {
		let mut dir_name = vec![];
		if let Some(prefix,) = self.prefix.clone() {
			dir_name.push(prefix,);
		}
		dir_name.push(self.uuid.to_string(),);
		if let Some(postfix,) = self.postfix.clone() {
			dir_name.push(postfix,);
		}

		dir_name.join("-",)
	}
}

#[derive(PartialEq, Eq, Clone, Copy,)]
pub enum DestKind {
	Src,
	Asm,
	Obj,
	Exe,
	OutDir,
}

impl DestKind {
	fn path(self,) -> PathBuf {
		let path_str = match self {
			Self::Src => "main.c",
			Self::Asm => "out.s",
			Self::Obj => "out.o",
			Self::Exe => "out",
			Self::OutDir => "",
		};
		PathBuf::from(path_str,)
	}
}
