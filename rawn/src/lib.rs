//! # Rawn
//! 
//! Helper of raw pointers cleaning.
//! 
//! Trait [BoxRaw] is blanket implemented for tuples of mut raw pointers.
//! 
//! Struct [BoxRaws] is new type struct wrapping [BoxRaw] implemented type.
//!  
//! # Example
//! ```rust
//! use rawn::{BoxRaw, BoxRaws};
//! 
//! // declare a raw pointer
//! let x: *mut u8 = Box::into_raw(Box::new(0u8));
//! // destruct it with [`clean()`] method of trait [`BoxRaw`]
//! x.clean();
//! 
//! // below code would work, but it's accessing dangling pointer,
//! // which is already cleaned out.
//! // rust's miri test would not pass this case.
//! #[cfg_attr(miri, ignore)]
//! {
//!   unsafe {
//!     let _ = *x;
//!   }
//! }
//! 
//! // BoxRaw is implemented for tuples of mut raw pointers.
//! // Available tuple size is from 1 to 12.
//! let a = Box::into_raw(Box::new(String::from("raw")));
//! let b = Box::into_raw(Box::new(12.));
//! let c: Box<Vec<u8>> = Box::new(vec![0, 1, 2]);
//! let c: *mut Vec<u8> = Box::into_raw(c);
//! 
//! let x = (a, b, c);
//! x.clean();
//! 
//! // [BoxRaws] is new type struct wrapping a tuple of raw pointers.
//! // It implements [BowRaw] too.
//! let a = Box::into_raw(Box::new(0u8));
//! let b = Box::into_raw(Box::new(1u8));
//! let raws = BoxRaws::new((a, b));
//! raws.clean();
//! 
//! // [BoxRaw] is only for raw pointers which are declared using `Box::into_raw()`.
//! // Thus, Using coerced mutable references would make errors.
//! // Below code will panic:
//! /*
//! let mut c: Vec<u8> = vec![0, 1, 2];
//! let c: *mut Vec<u8> = &mut c;
//! c.clean();
//! */
//! ```

pub trait BoxRaw {
  /// clean up multiple raw pointers contained inside a tuple.
  /// 
  /// It's only for raw pointers which are declared using
  /// `Box::into_raw()`.
  /// 
  /// Coerced mutable references would cause errors.
  /// 
  fn clean(self);
}

impl<X> BoxRaw for *mut X {
  fn clean(self) {
    unsafe {
      let _ = Box::from_raw(self);
    }
  }
}

macro_rules! impl_mut_raws_for_tuple {
  ($($T:tt),*) => {
    paste::paste! {
      impl<$($T,)*> BoxRaw for ($($T,)*)
      where
        $($T: BoxRaw,)*
      {
        fn clean(self) {
          let ($([<$T:lower>],)*) = self;
          $(
            [<$T:lower>].clean();
          )*
        }
      }
    }
  }
}

impl_mut_raws_for_tuple!(A);
impl_mut_raws_for_tuple!(A, B);
impl_mut_raws_for_tuple!(A, B, C);
impl_mut_raws_for_tuple!(A, B, C, D);
impl_mut_raws_for_tuple!(A, B, C, D, E);
impl_mut_raws_for_tuple!(A, B, C, D, E, F);
impl_mut_raws_for_tuple!(A, B, C, D, E, F, G);
impl_mut_raws_for_tuple!(A, B, C, D, E, F, G, H);
impl_mut_raws_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_mut_raws_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_mut_raws_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_mut_raws_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);


pub struct BoxRaws<X: BoxRaw>(pub X);

impl<X: BoxRaw> BoxRaw for BoxRaws<X> {
  fn clean(self) {
    self.0.clean();
  }
}

impl<X: BoxRaw> BoxRaws<X> {
  pub fn new(raws: X) -> Self { Self(raws) }
}


#[cfg(test)]
mod tests {
  use super::*;

  /// Conduct `cargo miri test`
  #[test]
  fn pass_miri() {

    let x = Box::into_raw(Box::new(2u8));
    x.clean();

    let x = (
      Box::into_raw(Box::new(2u8)),
      Box::into_raw(Box::new(2u32))
    );
    x.clean();

    let i = Box::into_raw(Box::new(0));

    let x = (
      i,
      Box::into_raw(Box::new(2u32)),
      Box::into_raw(Box::new(String::from("raw")))
    );
    x.clean();
  }

  #[test]
  #[cfg_attr(miri, ignore)]
  fn not_pass_miri() {

    let i = Box::into_raw(Box::new(0));

    let x = (
      i,
      Box::into_raw(Box::new(14u32))
    );

    x.clean();

    unsafe {
      // accessing dangling pointer
      let y = *i + 10;
      assert_eq!(y, 10);
    }
  }

  #[test]
  fn box_raws() {
    let x: Box<Vec<String>> = Box::new(vec![String::from("Rome")]);
    let x = Box::into_raw(x);
    let y = Box::into_raw(Box::new(10i32));

    (x, y).clean();
    
    // use struct `BoxRaws`
    let x: Box<Vec<String>> = Box::new(vec![String::from("Rome")]);
    let x = Box::into_raw(x);
    let y = Box::into_raw(Box::new(10i32));

    let raws = BoxRaws::new((x, y));
    raws.clean();
  }
}
