// Generated by `wit-bindgen` 0.16.0. DO NOT EDIT!
pub mod exports {
  pub mod timeline {
    pub mod event_processor {
      
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
          pub event: wit_bindgen::rt::vec::Vec::<(wit_bindgen::rt::string::String,EventValue,)>,
        }
        impl ::core::fmt::Debug for Event {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Event").field("time", &self.time).field("event", &self.event).finish()
          }
        }
        #[derive(Clone)]
        pub struct WorkerId {
          pub name: wit_bindgen::rt::string::String,
        }
        impl ::core::fmt::Debug for WorkerId {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("WorkerId").field("name", &self.name).finish()
          }
        }
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub struct TimePeriod {
          pub t1: u64,
          pub t2: u64,
        }
        impl ::core::fmt::Debug for TimePeriod {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("TimePeriod").field("t1", &self.t1).field("t2", &self.t2).finish()
          }
        }
        #[derive(Clone)]
        pub struct EventStateResult {
          pub time_period: TimePeriod,
          pub value: EventValue,
        }
        impl ::core::fmt::Debug for EventStateResult {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("EventStateResult").field("time-period", &self.time_period).field("value", &self.value).finish()
          }
        }
        #[derive(Clone)]
        pub struct LatestEventToStateResult {
          pub event_col_name: wit_bindgen::rt::string::String,
          pub event_results: wit_bindgen::rt::vec::Vec::<EventStateResult>,
        }
        impl ::core::fmt::Debug for LatestEventToStateResult {
          fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("LatestEventToStateResult").field("event-col-name", &self.event_col_name).field("event-results", &self.event_results).finish()
          }
        }
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "timeline:event-processor/api#initialize-latest-event-state"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_initialize_latest_event_state(arg0: i32,arg1: i32,arg2: i32,arg3: i32,) -> i32 {
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
            let len1 = arg3 as usize;
            let bytes1 = Vec::from_raw_parts(arg2 as *mut _, len1, len1);
            let result2 = <_GuestImpl as Guest>::initialize_latest_event_state(WorkerId{
              name: wit_bindgen::rt::string_lift(bytes0),
            }, wit_bindgen::rt::string_lift(bytes1));
            let ptr3 = _RET_AREA.0.as_mut_ptr() as i32;
            match result2 {
              Ok(e) => { {
                *((ptr3 + 0) as *mut u8) = (0i32) as u8;
                let vec4 = (e.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                ::core::mem::forget(vec4);
                *((ptr3 + 8) as *mut i32) = len4;
                *((ptr3 + 4) as *mut i32) = ptr4;
              } },
              Err(e) => { {
                *((ptr3 + 0) as *mut u8) = (1i32) as u8;
                let vec5 = (e.into_bytes()).into_boxed_slice();
                let ptr5 = vec5.as_ptr() as i32;
                let len5 = vec5.len() as i32;
                ::core::mem::forget(vec5);
                *((ptr3 + 8) as *mut i32) = len5;
                *((ptr3 + 4) as *mut i32) = ptr5;
              } },
            };ptr3
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_timeline:event-processor/api#initialize-latest-event-state"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_initialize_latest_event_state(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 4) as *const i32);
                  let l2 = *((arg0 + 8) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                },
                _ => {
                  let l3 = *((arg0 + 4) as *const i32);
                  let l4 = *((arg0 + 8) as *const i32);
                  wit_bindgen::rt::dealloc(l3, (l4) as usize, 1);
                },
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "timeline:event-processor/api#add-event"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_add_event(arg0: i64,arg1: i32,arg2: i32,) -> i32 {
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
            
            let base11 = arg1;
            let len11 = arg2;
            let mut result11 = Vec::with_capacity(len11 as usize);
            for i in 0..len11 {
              let base = base11 + i * 24;
              let e11 = {
                let l0 = *((base + 0) as *const i32);
                let l1 = *((base + 4) as *const i32);
                let len2 = l1 as usize;
                let bytes2 = Vec::from_raw_parts(l0 as *mut _, len2, len2);
                let l3 = i32::from(*((base + 8) as *const u8));
                let v10 = match l3 {
                  0 => {
                    let e10 = {
                      let l4 = *((base + 16) as *const i32);
                      let l5 = *((base + 20) as *const i32);
                      let len6 = l5 as usize;
                      let bytes6 = Vec::from_raw_parts(l4 as *mut _, len6, len6);
                      
                      wit_bindgen::rt::string_lift(bytes6)
                    };
                    EventValue::StringValue(e10)
                  }
                  1 => {
                    let e10 = {
                      let l7 = *((base + 16) as *const i64);
                      
                      l7
                    };
                    EventValue::IntValue(e10)
                  }
                  2 => {
                    let e10 = {
                      let l8 = *((base + 16) as *const f64);
                      
                      l8
                    };
                    EventValue::FloatValue(e10)
                  }
                  n => {
                    debug_assert_eq!(n, 3, "invalid enum discriminant");
                    let e10 = {
                      let l9 = i32::from(*((base + 16) as *const u8));
                      
                      wit_bindgen::rt::bool_lift(l9 as u8)
                    };
                    EventValue::BoolValue(e10)
                  }
                };
                
                (wit_bindgen::rt::string_lift(bytes2), v10)
              };
              result11.push(e11);
            }
            wit_bindgen::rt::dealloc(base11, (len11 as usize) * 24, 8);
            let result12 = <_GuestImpl as Guest>::add_event(Event{
              time: arg0 as u64,
              event: result11,
            });
            let ptr13 = _RET_AREA.0.as_mut_ptr() as i32;
            match result12 {
              Ok(e) => { {
                *((ptr13 + 0) as *mut u8) = (0i32) as u8;
                let vec14 = (e.into_bytes()).into_boxed_slice();
                let ptr14 = vec14.as_ptr() as i32;
                let len14 = vec14.len() as i32;
                ::core::mem::forget(vec14);
                *((ptr13 + 8) as *mut i32) = len14;
                *((ptr13 + 4) as *mut i32) = ptr14;
              } },
              Err(e) => { {
                *((ptr13 + 0) as *mut u8) = (1i32) as u8;
                let vec15 = (e.into_bytes()).into_boxed_slice();
                let ptr15 = vec15.as_ptr() as i32;
                let len15 = vec15.len() as i32;
                ::core::mem::forget(vec15);
                *((ptr13 + 8) as *mut i32) = len15;
                *((ptr13 + 4) as *mut i32) = ptr15;
              } },
            };ptr13
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_timeline:event-processor/api#add-event"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_add_event(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 4) as *const i32);
                  let l2 = *((arg0 + 8) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                },
                _ => {
                  let l3 = *((arg0 + 4) as *const i32);
                  let l4 = *((arg0 + 8) as *const i32);
                  wit_bindgen::rt::dealloc(l3, (l4) as usize, 1);
                },
              }
            }
          };
        };
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "timeline:event-processor/api#latest-event-to-state"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_latest_event_to_state(arg0: i64,) -> i32 {
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
            
            let result0 = <_GuestImpl as Guest>::latest_event_to_state(arg0 as u64);
            let ptr1 = _RET_AREA.0.as_mut_ptr() as i32;
            match result0 {
              Ok(e) => { {
                *((ptr1 + 0) as *mut u8) = (0i32) as u8;
                let LatestEventToStateResult{ event_col_name:event_col_name2, event_results:event_results2, } = e;
                let vec3 = (event_col_name2.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                ::core::mem::forget(vec3);
                *((ptr1 + 8) as *mut i32) = len3;
                *((ptr1 + 4) as *mut i32) = ptr3;
                let vec7 = event_results2;
                let len7 = vec7.len() as i32;
                let layout7 = alloc::Layout::from_size_align_unchecked(vec7.len() * 32, 8);
                let result7 = if layout7.size() != 0
                {
                  let ptr = alloc::alloc(layout7);
                  if ptr.is_null()
                  {
                    alloc::handle_alloc_error(layout7);
                  }
                  ptr
                }else {{
                  ::core::ptr::null_mut()
                }};
                for (i, e) in vec7.into_iter().enumerate() {
                  let base = result7 as i32 + (i as i32) * 32;
                  {
                    let EventStateResult{ time_period:time_period4, value:value4, } = e;
                    let TimePeriod{ t1:t15, t2:t25, } = time_period4;
                    *((base + 0) as *mut i64) = wit_bindgen::rt::as_i64(t15);
                    *((base + 8) as *mut i64) = wit_bindgen::rt::as_i64(t25);
                    match value4 {
                      EventValue::StringValue(e) => {
                        *((base + 16) as *mut u8) = (0i32) as u8;
                        let vec6 = (e.into_bytes()).into_boxed_slice();
                        let ptr6 = vec6.as_ptr() as i32;
                        let len6 = vec6.len() as i32;
                        ::core::mem::forget(vec6);
                        *((base + 28) as *mut i32) = len6;
                        *((base + 24) as *mut i32) = ptr6;
                      },
                      EventValue::IntValue(e) => {
                        *((base + 16) as *mut u8) = (1i32) as u8;
                        *((base + 24) as *mut i64) = wit_bindgen::rt::as_i64(e);
                      },
                      EventValue::FloatValue(e) => {
                        *((base + 16) as *mut u8) = (2i32) as u8;
                        *((base + 24) as *mut f64) = wit_bindgen::rt::as_f64(e);
                      },
                      EventValue::BoolValue(e) => {
                        *((base + 16) as *mut u8) = (3i32) as u8;
                        *((base + 24) as *mut u8) = (match e { true => 1, false => 0 }) as u8;
                      },
                    }
                  }
                }
                *((ptr1 + 16) as *mut i32) = len7;
                *((ptr1 + 12) as *mut i32) = result7 as i32;
              } },
              Err(e) => { {
                *((ptr1 + 0) as *mut u8) = (1i32) as u8;
                let vec8 = (e.into_bytes()).into_boxed_slice();
                let ptr8 = vec8.as_ptr() as i32;
                let len8 = vec8.len() as i32;
                ::core::mem::forget(vec8);
                *((ptr1 + 8) as *mut i32) = len8;
                *((ptr1 + 4) as *mut i32) = ptr8;
              } },
            };ptr1
          }
          
          const _: () = {
            #[doc(hidden)]
            #[export_name = "cabi_post_timeline:event-processor/api#latest-event-to-state"]
            #[allow(non_snake_case)]
            unsafe extern "C" fn __post_return_latest_event_to_state(arg0: i32,) {
              let l0 = i32::from(*((arg0 + 0) as *const u8));
              match l0 {
                0 => {
                  let l1 = *((arg0 + 4) as *const i32);
                  let l2 = *((arg0 + 8) as *const i32);
                  wit_bindgen::rt::dealloc(l1, (l2) as usize, 1);
                  let l6 = *((arg0 + 12) as *const i32);
                  let l7 = *((arg0 + 16) as *const i32);
                  let base8 = l6;
                  let len8 = l7;
                  for i in 0..len8 {
                    let base = base8 + i *32;
                    {
                      let l3 = i32::from(*((base + 16) as *const u8));
                      match l3 {
                        0 => {
                          let l4 = *((base + 24) as *const i32);
                          let l5 = *((base + 28) as *const i32);
                          wit_bindgen::rt::dealloc(l4, (l5) as usize, 1);
                        },
                        1 => (),
                        2 => (),
                        _ => (),
                      }
                    }
                  }
                  wit_bindgen::rt::dealloc(base8, (len8 as usize) * 32, 8);
                },
                _ => {
                  let l9 = *((arg0 + 4) as *const i32);
                  let l10 = *((arg0 + 8) as *const i32);
                  wit_bindgen::rt::dealloc(l9, (l10) as usize, 1);
                },
              }
            }
          };
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn initialize_latest_event_state(worker: WorkerId,event_col_name: wit_bindgen::rt::string::String,) -> Result<wit_bindgen::rt::string::String,wit_bindgen::rt::string::String>;
          fn add_event(event: Event,) -> Result<wit_bindgen::rt::string::String,wit_bindgen::rt::string::String>;
          fn latest_event_to_state(t1: u64,) -> Result<LatestEventToStateResult,wit_bindgen::rt::string::String>;
        }
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        
        #[repr(align(4))]
        struct _RetArea([u8; 20]);
        static mut _RET_AREA: _RetArea = _RetArea([0; 20]);
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:event-processor"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1120] = [3, 0, 15, 101, 118, 101, 110, 116, 45, 112, 114, 111, 99, 101, 115, 115, 111, 114, 0, 97, 115, 109, 13, 0, 1, 0, 7, 202, 3, 1, 65, 2, 1, 66, 23, 1, 113, 4, 12, 115, 116, 114, 105, 110, 103, 45, 118, 97, 108, 117, 101, 1, 115, 0, 9, 105, 110, 116, 45, 118, 97, 108, 117, 101, 1, 120, 0, 11, 102, 108, 111, 97, 116, 45, 118, 97, 108, 117, 101, 1, 117, 0, 10, 98, 111, 111, 108, 45, 118, 97, 108, 117, 101, 1, 127, 0, 4, 0, 11, 101, 118, 101, 110, 116, 45, 118, 97, 108, 117, 101, 3, 0, 0, 1, 111, 2, 115, 1, 1, 112, 2, 1, 114, 2, 4, 116, 105, 109, 101, 119, 5, 101, 118, 101, 110, 116, 3, 4, 0, 5, 101, 118, 101, 110, 116, 3, 0, 4, 1, 114, 1, 4, 110, 97, 109, 101, 115, 4, 0, 9, 119, 111, 114, 107, 101, 114, 45, 105, 100, 3, 0, 6, 1, 114, 2, 2, 116, 49, 119, 2, 116, 50, 119, 4, 0, 11, 116, 105, 109, 101, 45, 112, 101, 114, 105, 111, 100, 3, 0, 8, 1, 114, 2, 11, 116, 105, 109, 101, 45, 112, 101, 114, 105, 111, 100, 9, 5, 118, 97, 108, 117, 101, 1, 4, 0, 18, 101, 118, 101, 110, 116, 45, 115, 116, 97, 116, 101, 45, 114, 101, 115, 117, 108, 116, 3, 0, 10, 1, 112, 11, 1, 114, 2, 14, 101, 118, 101, 110, 116, 45, 99, 111, 108, 45, 110, 97, 109, 101, 115, 13, 101, 118, 101, 110, 116, 45, 114, 101, 115, 117, 108, 116, 115, 12, 4, 0, 28, 108, 97, 116, 101, 115, 116, 45, 101, 118, 101, 110, 116, 45, 116, 111, 45, 115, 116, 97, 116, 101, 45, 114, 101, 115, 117, 108, 116, 3, 0, 13, 1, 106, 1, 115, 1, 115, 1, 64, 2, 6, 119, 111, 114, 107, 101, 114, 7, 14, 101, 118, 101, 110, 116, 45, 99, 111, 108, 45, 110, 97, 109, 101, 115, 0, 15, 4, 0, 29, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 45, 108, 97, 116, 101, 115, 116, 45, 101, 118, 101, 110, 116, 45, 115, 116, 97, 116, 101, 1, 16, 1, 64, 1, 5, 101, 118, 101, 110, 116, 5, 0, 15, 4, 0, 9, 97, 100, 100, 45, 101, 118, 101, 110, 116, 1, 17, 1, 106, 1, 14, 1, 115, 1, 64, 1, 2, 116, 49, 119, 0, 18, 4, 0, 21, 108, 97, 116, 101, 115, 116, 45, 101, 118, 101, 110, 116, 45, 116, 111, 45, 115, 116, 97, 116, 101, 1, 19, 4, 1, 28, 116, 105, 109, 101, 108, 105, 110, 101, 58, 101, 118, 101, 110, 116, 45, 112, 114, 111, 99, 101, 115, 115, 111, 114, 47, 97, 112, 105, 5, 0, 11, 9, 1, 0, 3, 97, 112, 105, 3, 0, 0, 7, 250, 3, 1, 65, 2, 1, 65, 2, 1, 66, 23, 1, 113, 4, 12, 115, 116, 114, 105, 110, 103, 45, 118, 97, 108, 117, 101, 1, 115, 0, 9, 105, 110, 116, 45, 118, 97, 108, 117, 101, 1, 120, 0, 11, 102, 108, 111, 97, 116, 45, 118, 97, 108, 117, 101, 1, 117, 0, 10, 98, 111, 111, 108, 45, 118, 97, 108, 117, 101, 1, 127, 0, 4, 0, 11, 101, 118, 101, 110, 116, 45, 118, 97, 108, 117, 101, 3, 0, 0, 1, 111, 2, 115, 1, 1, 112, 2, 1, 114, 2, 4, 116, 105, 109, 101, 119, 5, 101, 118, 101, 110, 116, 3, 4, 0, 5, 101, 118, 101, 110, 116, 3, 0, 4, 1, 114, 1, 4, 110, 97, 109, 101, 115, 4, 0, 9, 119, 111, 114, 107, 101, 114, 45, 105, 100, 3, 0, 6, 1, 114, 2, 2, 116, 49, 119, 2, 116, 50, 119, 4, 0, 11, 116, 105, 109, 101, 45, 112, 101, 114, 105, 111, 100, 3, 0, 8, 1, 114, 2, 11, 116, 105, 109, 101, 45, 112, 101, 114, 105, 111, 100, 9, 5, 118, 97, 108, 117, 101, 1, 4, 0, 18, 101, 118, 101, 110, 116, 45, 115, 116, 97, 116, 101, 45, 114, 101, 115, 117, 108, 116, 3, 0, 10, 1, 112, 11, 1, 114, 2, 14, 101, 118, 101, 110, 116, 45, 99, 111, 108, 45, 110, 97, 109, 101, 115, 13, 101, 118, 101, 110, 116, 45, 114, 101, 115, 117, 108, 116, 115, 12, 4, 0, 28, 108, 97, 116, 101, 115, 116, 45, 101, 118, 101, 110, 116, 45, 116, 111, 45, 115, 116, 97, 116, 101, 45, 114, 101, 115, 117, 108, 116, 3, 0, 13, 1, 106, 1, 115, 1, 115, 1, 64, 2, 6, 119, 111, 114, 107, 101, 114, 7, 14, 101, 118, 101, 110, 116, 45, 99, 111, 108, 45, 110, 97, 109, 101, 115, 0, 15, 4, 0, 29, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 45, 108, 97, 116, 101, 115, 116, 45, 101, 118, 101, 110, 116, 45, 115, 116, 97, 116, 101, 1, 16, 1, 64, 1, 5, 101, 118, 101, 110, 116, 5, 0, 15, 4, 0, 9, 97, 100, 100, 45, 101, 118, 101, 110, 116, 1, 17, 1, 106, 1, 14, 1, 115, 1, 64, 1, 2, 116, 49, 119, 0, 18, 4, 0, 21, 108, 97, 116, 101, 115, 116, 45, 101, 118, 101, 110, 116, 45, 116, 111, 45, 115, 116, 97, 116, 101, 1, 19, 4, 1, 28, 116, 105, 109, 101, 108, 105, 110, 101, 58, 101, 118, 101, 110, 116, 45, 112, 114, 111, 99, 101, 115, 115, 111, 114, 47, 97, 112, 105, 5, 0, 4, 1, 40, 116, 105, 109, 101, 108, 105, 110, 101, 58, 101, 118, 101, 110, 116, 45, 112, 114, 111, 99, 101, 115, 115, 111, 114, 47, 101, 118, 101, 110, 116, 45, 112, 114, 111, 99, 101, 115, 115, 111, 114, 4, 0, 11, 21, 1, 0, 15, 101, 118, 101, 110, 116, 45, 112, 114, 111, 99, 101, 115, 115, 111, 114, 3, 2, 0, 0, 16, 12, 112, 97, 99, 107, 97, 103, 101, 45, 100, 111, 99, 115, 0, 123, 125, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 56, 46, 50, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 54, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
