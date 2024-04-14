// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod timeline {
    pub mod raw_events {
      
      #[allow(clippy::all)]
      pub mod api {
        #[used]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        #[derive(Clone)]
        pub enum EventValue{
          StringValue(wit_bindgen::rt::string::String),
          IntValue(i64),
          FloatValue(f64),
          BoolValue(bool),
        }
        impl ::core::fmt::Debug for EventValue {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
              EventValue::StringValue(e) => {
                f.debug_tuple("EventValue::StringValue").field(e).finish()
              }
              EventValue::IntValue(e) => {
                f.debug_tuple("EventValue::IntValue").field(e).finish()
              }
              EventValue::FloatValue(e) => {
                f.debug_tuple("EventValue::FloatValue").field(e).finish()
              }
              EventValue::BoolValue(e) => {
                f.debug_tuple("EventValue::BoolValue").field(e).finish()
              }
            }
          }
        }
        #[derive(Clone)]
        pub struct Event {
          pub time: u64,
          pub event: EventValue,
        }
        impl ::core::fmt::Debug for Event {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Event").field("time", &self.time).field("event", &self.event).finish()
          }
        }
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "timeline:raw-events/api#add-event"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_add_event(arg0: i64,arg1: i32,arg2: i64,arg3: i32,) {
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
            
            let v1 = match arg1 {
              0 => {
                let e1 = {
                  let len0 = arg3 as usize;
                  let bytes0 = Vec::from_raw_parts(arg2 as i32 as *mut _, len0, len0);
                  
                  wit_bindgen::rt::string_lift(bytes0)
                };
                EventValue::StringValue(e1)
              }
              1 => {
                let e1 = arg2;
                EventValue::IntValue(e1)
              }
              2 => {
                let e1 = f64::from_bits(arg2 as u64);
                EventValue::FloatValue(e1)
              }
              n => {
                debug_assert_eq!(n, 3, "invalid enum discriminant");
                let e1 = wit_bindgen::rt::bool_lift(arg2 as i32 as u8);
                EventValue::BoolValue(e1)
              }
            };
            <_GuestImpl as Guest>::add_event(Event{
              time: arg0 as u64,
              event: v1,
            });
          }
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "timeline:raw-events/api#get-event"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_event(arg0: i64,) -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::get_event(arg0 as u64);
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let Event{ time:time2, event:event2, } = result0;
            *((ptr1 + 0) as *mut i64) = wit_bindgen::rt::as_i64(time2);
            match event2 {
              EventValue::StringValue(e) => {
                *((ptr1 + 8) as *mut u8) = (0i32) as u8;
                let vec3 = (e.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((ptr1 + 20) as *mut i32) = len3;
                *((ptr1 + 16) as *mut i32) = ptr3;
              },
              EventValue::IntValue(e) => {
                *((ptr1 + 8) as *mut u8) = (1i32) as u8;
                *((ptr1 + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
              },
              EventValue::FloatValue(e) => {
                *((ptr1 + 8) as *mut u8) = (2i32) as u8;
                *((ptr1 + 16) as *mut f64) = wit_bindgen::rt::as_f64(e);
              },
              EventValue::BoolValue(e) => {
                *((ptr1 + 8) as *mut u8) = (3i32) as u8;
                *((ptr1 + 16) as *mut u8) = (match e { true => 1, false => 0 }) as u8;
              },
            }
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_timeline:raw-events/api#get-event"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_event(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 8) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 16) as *const i32);
                  let l2 = *((arg0 + 20) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                },
                1 => (),
                2 => (),
                _ => (),
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "timeline:raw-events/api#get-events"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_get_events() -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::get_events();
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            let vec4 = result0;
            let len4 = vec4.len() as i32;
            let layout4 = alloc::Layout::from_size_align_unchecked(vec4.len() * 24, 8);
            let result4 = if layout4.size() != 0
            {
              let ptr = alloc::alloc(layout4);
              if ptr.is_null()
              {
                alloc::handle_alloc_error(layout4);
              }
              ptr
            }else {{
              ::core::ptr::null_mut()
            }};
            for (i, e) in vec4.into_iter().enumerate() {
              let base = result4 as i32 + (i as i32) * 24;
              {
                let Event{ time:time2, event:event2, } = e;
                *((base + 0) as *mut i64) = wit_bindgen::rt::as_i64(time2);
                match event2 {
                  EventValue::StringValue(e) => {
                    *((base + 8) as *mut u8) = (0i32) as u8;
                    let vec3 = (e.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr() as i32;
                    let len3 = vec3.len() as i32;
                    ::core::mem::forget(vec3);
                    *((base + 20) as *mut i32) = len3;
                    *((base + 16) as *mut i32) = ptr3;
                  },
                  EventValue::IntValue(e) => {
                    *((base + 8) as *mut u8) = (1i32) as u8;
                    *((base + 16) as *mut i64) = wit_bindgen::rt::as_i64(e);
                  },
                  EventValue::FloatValue(e) => {
                    *((base + 8) as *mut u8) = (2i32) as u8;
                    *((base + 16) as *mut f64) = wit_bindgen::rt::as_f64(e);
                  },
                  EventValue::BoolValue(e) => {
                    *((base + 8) as *mut u8) = (3i32) as u8;
                    *((base + 16) as *mut u8) = (match e { true => 1, false => 0 }) as u8;
                  },
                }
              }
            }
            *((ptr1 + 4) as *mut i32) = len4;
            *((ptr1 + 0) as *mut i32) = result4 as i32;
            ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_timeline:raw-events/api#get-events"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_get_events(arg0: i32,) {
              let l3 = *((arg0 + 0) as *const i32);
              let l4 = *((arg0 + 4) as *const i32);
              let base5 = l3;
              let len5 = l4;
              for i in 0..len5 {
                let base = base5 + i *24;
                {
                  let l0 = i32::from(*((base + 8) as *const u8));
                  match l0 {
                    0 => {
                      let l1 = *((base + 16) as *const i32);
                      let l2 = *((base + 20) as *const i32);
                      wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                    },
                    1 => (),
                    2 => (),
                    _ => (),
                  }
                }
              }
              wit_bindgen::rt::dealloc(base5, (len5 as usize) * 24, 8);
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn add_event(order: Event,);
          fn get_event(time: u64,) -> Event;
          fn get_events() -> wit_bindgen::rt::vec::Vec::<Event>;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(8))]
        struct _RetArea([u8; 24]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 24]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:raw-events"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 610] = [3, 0, 10, 114, 97, 119, 45, 101, 118, 101, 110, 116, 115, 0, 97, 115, 109, 13, 0, 1, 0, 7, 213, 1, 1, 65, 2, 1, 66, 11, 1, 113, 4, 12, 115, 116, 114, 105, 110, 103, 45, 118, 97, 108, 117, 101, 1, 115, 0, 9, 105, 110, 116, 45, 118, 97, 108, 117, 101, 1, 120, 0, 11, 102, 108, 111, 97, 116, 45, 118, 97, 108, 117, 101, 1, 117, 0, 10, 98, 111, 111, 108, 45, 118, 97, 108, 117, 101, 1, 127, 0, 4, 0, 11, 101, 118, 101, 110, 116, 45, 118, 97, 108, 117, 101, 3, 0, 0, 1, 114, 2, 4, 116, 105, 109, 101, 119, 5, 101, 118, 101, 110, 116, 1, 4, 0, 5, 101, 118, 101, 110, 116, 3, 0, 2, 1, 64, 1, 5, 111, 114, 100, 101, 114, 3, 1, 0, 4, 0, 9, 97, 100, 100, 45, 101, 118, 101, 110, 116, 1, 4, 1, 64, 1, 4, 116, 105, 109, 101, 119, 0, 3, 4, 0, 9, 103, 101, 116, 45, 101, 118, 101, 110, 116, 1, 5, 1, 112, 3, 1, 64, 0, 0, 6, 4, 0, 10, 103, 101, 116, 45, 101, 118, 101, 110, 116, 115, 1, 7, 4, 1, 23, 116, 105, 109, 101, 108, 105, 110, 101, 58, 114, 97, 119, 45, 101, 118, 101, 110, 116, 115, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 251, 1, 1, 65, 2, 1, 65, 2, 1, 66, 11, 1, 113, 4, 12, 115, 116, 114, 105, 110, 103, 45, 118, 97, 108, 117, 101, 1, 115, 0, 9, 105, 110, 116, 45, 118, 97, 108, 117, 101, 1, 120, 0, 11, 102, 108, 111, 97, 116, 45, 118, 97, 108, 117, 101, 1, 117, 0, 10, 98, 111, 111, 108, 45, 118, 97, 108, 117, 101, 1, 127, 0, 4, 0, 11, 101, 118, 101, 110, 116, 45, 118, 97, 108, 117, 101, 3, 0, 0, 1, 114, 2, 4, 116, 105, 109, 101, 119, 5, 101, 118, 101, 110, 116, 1, 4, 0, 5, 101, 118, 101, 110, 116, 3, 0, 2, 1, 64, 1, 5, 111, 114, 100, 101, 114, 3, 1, 0, 4, 0, 9, 97, 100, 100, 45, 101, 118, 101, 110, 116, 1, 4, 1, 64, 1, 4, 116, 105, 109, 101, 119, 0, 3, 4, 0, 9, 103, 101, 116, 45, 101, 118, 101, 110, 116, 1, 5, 1, 112, 3, 1, 64, 0, 0, 6, 4, 0, 10, 103, 101, 116, 45, 101, 118, 101, 110, 116, 115, 1, 7, 4, 1, 23, 116, 105, 109, 101, 108, 105, 110, 101, 58, 114, 97, 119, 45, 101, 118, 101, 110, 116, 115, 47, 97, 112, 105, 5, 0, 4, 1, 30, 116, 105, 109, 101, 108, 105, 110, 101, 58, 114, 97, 119, 45, 101, 118, 101, 110, 116, 115, 47, 114, 97, 119, 45, 101, 118, 101, 110, 116, 115, 4, 0, 11, 16, 1, 0, 10, 114, 97, 119, 45, 101, 118, 101, 110, 116, 115, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
