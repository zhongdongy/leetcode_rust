//! A module defining several useful macros
//! 
//! The `new_case!` macro is very useful to simplify TestCase generation 
//! process and you should definitely try it out.
//! 
//! All macros have been exported to crate level. You might want to use it via
//! `leetcode_rust::macro_name!`

////////////////////////////////////////////////////////////////////////////////

/// Helper macro to call `create` method of CaseGroup<T, G, P> instance
/// 
/// # Note
/// You must pass in the casegroup.create method as first parameter of the
/// macro.
#[macro_export]
macro_rules! new_case {
  ($self:ident.$func:ident, $input:expr, $($output:expr),+) => {
      $self.$func($input,vec![$($output),+]);
  };
}

/// Generate `create` & `create_param` implementation for different types
///
/// # Note
/// The `<String, G, P>` generics combination has been implemented individually,
/// so you probably should not implement on your own.
#[macro_export]
macro_rules! codegen_case_create_impl {
    ($t:ty, $g:ty, $p:ty) => {
        /// Implement two handy methods on CaseGroup struct.
        impl CaseGroup<$t, $g, $p> {
            /// Create a new test case (no input parameters) matching selected
            /// generic types.
            pub fn create(&mut self, ipt: $t, exp: Vec<$g>) {
                self.add(Case::new(ipt, exp));
            }

            /// Create a new test case (with input parameters) matching
            /// selected generic types.
            pub fn create_param(&mut self, ipt: $t, exp: Vec<$g>, params: Vec<$p>) {
                self.add(Case::new_params(ipt, params, exp));
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use new_case;

#[allow(unused_imports)]
pub(crate) use codegen_case_create_impl;
