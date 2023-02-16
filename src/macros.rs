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

            /// Create a new test case (no input parameters but multi-inputs)
            /// matching selected generic types.
            pub fn create_multi(&mut self, ipts: Vec<$t>, exp: Vec<$g>) {
                self.add(Case::new_multi(ipts, exp));
            }

            /// Create a new test case (with input parameters and multi-inputs)
            /// matching selected generic types.
            pub fn create_param_multi(&mut self, ipts: Vec<$t>, exp: Vec<$g>, params: Vec<$p>) {
                self.add(Case::new_params_multi(ipts, params, exp));
            }
        }
    };
}

#[macro_export]
macro_rules! codegen_vector_case_create_impl {
    ($t:ty, $g:ty, $p:ty) => {
        /// Implement two handy methods on CaseGroup struct.
        impl VectorCaseGroup<$t, $g, $p> {
            /// Create a new test case (no input parameters) matching selected
            /// generic types.
            pub fn create(&mut self, ipt: $t, exp: Vec<$g>) {
                self.add(VectorCase::new(vec![ipt], exp));
            }

            /// Create a new test case (with input parameters) matching
            /// selected generic types.
            pub fn create_param(&mut self, ipt: $t, exp: Vec<$g>, params: Vec<$p>) {
                self.add(VectorCase::new_params(vec![ipt], params, exp));
            }

            /// Create a new test case (no input parameters but multi-inputs)
            /// matching selected generic types.
            pub fn create_multi(&mut self, ipts: Vec<$t>, exp: Vec<$g>) {
                self.add(VectorCase::new_multi(ipts, exp));
            }

            /// Create a new test case (with input parameters and multi-inputs)
            /// matching selected generic types.
            pub fn create_param_multi(&mut self, ipts: Vec<$t>, exp: Vec<$g>, params: Vec<$p>) {
                self.add(VectorCase::new_params_multi(ipts, params, exp));
            }
        }
    };
}

/// Use a simplified syntax to create nested Vectors (as per needed by several 
/// LeetCode problems.)
/// 
/// # Examples:
/// 
/// ```rust
/// use leetcode_rust::vec2d;
/// 
/// let v = vec2d![[1,2,3],[2,3,4]];
/// assert_eq!(v[0], vec![1,2,3]);
/// ```
#[macro_export]
macro_rules! vec2d {
    ($([$($x:expr),* $(,)*]),+ $(,)*) => {{
        vec![$(vec![$($x,)*],)*]
    }};
}

#[allow(unused_imports)]
pub(crate) use new_case;

#[allow(unused_imports)]
pub(crate) use vec2d;

#[allow(unused_imports)]
pub(crate) use codegen_case_create_impl;

#[allow(unused_imports)]
pub(crate) use codegen_vector_case_create_impl;
