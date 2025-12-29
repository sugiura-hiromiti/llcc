pub trait HasOut {
	type OutInfo;
	type OutInfoRef<'a,>
	where Self: 'a;
	fn out_info(&self,) -> Option<Self::OutInfoRef<'_,>,>;
	fn out_info_owned(&self,) -> Option<Self::OutInfo,>;
	fn exist_out(&self,) -> bool {
		self.out_info().is_some()
	}
}

pub trait HasIn {
	type InInfo;
	type InInfoRef<'a,>
	where Self: 'a;
	fn in_info(&self,) -> Option<Self::InInfoRef<'_,>,>;
	fn in_info_owned(&self,) -> Option<Self::InInfo,>;
	fn exist_in(&self,) -> bool {
		self.in_info().is_some()
	}
}
