use std::ops::Add;

pub trait TimesTwo
   where *const Self: Add<*const Self>,
{
   extern "C" fn t2_ptr(slf: *const Self)
   -> <*const Self as Add<*const Self>>::Output {
       slf + slf
   }
}

fn main(){}
