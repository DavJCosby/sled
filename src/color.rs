use std::fmt::Debug;
pub trait ColorType: Debug + Default + Copy {}

impl<T: Debug + Default + Copy> ColorType for T {}
