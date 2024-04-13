// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod golem {
    pub mod timeline {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "golem:timeline/api#get-timelines"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_timelines() -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::get_timelines();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec3 = result0;
            let len3 = vec3.len() as i32;
            let layout3 = alloc::Layout::from_size_align_unchecked(vec3.len() * 8, 4);
            let result3 = if layout3.size() != 0
            {
              let ptr = alloc::alloc(layout3);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout3);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec3.into_iter().enumerate() {
              let base = result3 as i32 + (i as i32) * 8;
              {
                let vec2 = (e.into_bytes()).into_boxed_slice();
                let ptr2 = vec2.as_ptr() as i32;
                let len2 = vec2.len() as i32;
                ::core::mem::forget(vec2);
                *((base + 4) as *mut i32) = len2;
                *((base + 0) as *mut i32) = ptr2;
              }
            }
            *((ptr1 + 4) as *mut i32) = len3;
            *((ptr1 + 0) as *mut i32) = result3 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_golem:timeline/api#get-timelines"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_timelines(arg0: i32,) {
              let l2 = *((arg0 + 0) as *const i32);
              let l3 = *((arg0 + 4) as *const i32);
              let base4 = l2;
              let len4 = l3;
              for i in 0..len4 {
                let base = base4 + i *8;
                {
                  let l0 = *((base + 0) as *const i32);
                  let l1 = *((base + 4) as *const i32);
                  wit_bindgen::rt::dealloc(l0, (l1) as usize, 1);
                }
              }
              wit_bindgen::rt::dealloc(base4, (len4 as usize) * 8, 4);
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn get_timelines() -> wit_bindgen::rt::vec::Vec::<wit_bindgen::rt::string::String>;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(4))]
        struct _RetArea([u8; 8]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 8]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:golem-timeline"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 299] = [3, 0, 14, 103, 111, 108, 101, 109, 45, 116, 105, 109, 101, 108, 105, 110, 101, 0, 97, 115, 109, 13, 0, 1, 0, 7, 55, 1, 65, 2, 1, 66, 3, 1, 112, 115, 1, 64, 0, 0, 0, 4, 0, 13, 103, 101, 116, 45, 116, 105, 109, 101, 108, 105, 110, 101, 115, 1, 1, 4, 1, 18, 103, 111, 108, 101, 109, 58, 116, 105, 109, 101, 108, 105, 110, 101, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 92, 1, 65, 2, 1, 65, 2, 1, 66, 3, 1, 112, 115, 1, 64, 0, 0, 0, 4, 0, 13, 103, 101, 116, 45, 116, 105, 109, 101, 108, 105, 110, 101, 115, 1, 1, 4, 1, 18, 103, 111, 108, 101, 109, 58, 116, 105, 109, 101, 108, 105, 110, 101, 47, 97, 112, 105, 5, 0, 4, 1, 29, 103, 111, 108, 101, 109, 58, 116, 105, 109, 101, 108, 105, 110, 101, 47, 103, 111, 108, 101, 109, 45, 116, 105, 109, 101, 108, 105, 110, 101, 4, 0, 11, 20, 1, 0, 14, 103, 111, 108, 101, 109, 45, 116, 105, 109, 101, 108, 105, 110, 101, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}