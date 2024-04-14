// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod timeline {
    pub mod driver {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "timeline:driver/api#run"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_run(arg0: i32,arg1: i32,) {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            let len0 = arg1 as usize;
            let bytes0 = Vec::from_raw_parts(arg0 as *mut _, len0, len0);
            <_GuestImpl as Guest>::run(wit_bindgen::rt::string_lift(bytes0));
          }
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn run(value: wit_bindgen::rt::string::String,);
        }
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:driver"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 266] = [3, 0, 6, 100, 114, 105, 118, 101, 114, 0, 97, 115, 109, 13, 0, 1, 0, 7, 50, 1, 65, 2, 1, 66, 2, 1, 64, 1, 5, 118, 97, 108, 117, 101, 115, 1, 0, 4, 0, 3, 114, 117, 110, 1, 0, 4, 1, 19, 116, 105, 109, 101, 108, 105, 110, 101, 58, 100, 114, 105, 118, 101, 114, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 80, 1, 65, 2, 1, 65, 2, 1, 66, 2, 1, 64, 1, 5, 118, 97, 108, 117, 101, 115, 1, 0, 4, 0, 3, 114, 117, 110, 1, 0, 4, 1, 19, 116, 105, 109, 101, 108, 105, 110, 101, 58, 100, 114, 105, 118, 101, 114, 47, 97, 112, 105, 5, 0, 4, 1, 22, 116, 105, 109, 101, 108, 105, 110, 101, 58, 100, 114, 105, 118, 101, 114, 47, 100, 114, 105, 118, 101, 114, 4, 0, 11, 12, 1, 0, 6, 100, 114, 105, 118, 101, 114, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}