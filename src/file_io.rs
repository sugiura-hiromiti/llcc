use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const PRJ_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Debug,)]
pub struct Dest(Uuid,);

impl Default for Dest {
	fn default() -> Self {
		let rslt = Self(Uuid::new_v4(),);
		let out_path = rslt.path(DestKind::OutDir,).into();
		if !out_path.exists() {
			fs::create_dir_all(&out_path,).unwrap_or_else(|e| {
				panic!(
					"failed to create output directory at `{}`. not able to \
					 proceed compilation. abort\ndetail: {e:#?}",
					out_path.display()
				)
			},);
		}

		dbg!(rslt)
	}
}

impl Dest {
	pub fn path(&self, kind: DestKind,) -> impl Into<PathBuf,> {
		self.base_of(kind,).into().join(kind.path(),)
	}

	pub fn base_of(&self, kind: DestKind,) -> impl Into<PathBuf,> {
		let mut assets = PathBuf::from(PRJ_DIR,).join("assets/out",);
		if kind != DestKind::Src {
			assets.push(self.uuid_part().into(),);
		}

		assets
	}

	pub fn uuid_part(&self,) -> impl Into<PathBuf,> {
		PathBuf::from(self.0.to_string(),)
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
