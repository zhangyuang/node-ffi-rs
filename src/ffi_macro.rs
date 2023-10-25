macro_rules! match_args_len {
     ($args_len:expr, $func_args_type:expr, $js_function:expr, $env:expr, $($num:literal => $closure:ident, $($arg:ident),*),*) => {
         match $args_len {
             $(
                 $num => {
                     use libffi::high::$closure;
                     let lambda = |$($arg: *mut c_void),*| {
                         let value: Vec<JsUnknown> = (0..$args_len)
                             .map(|index| {
                                 let c_param = match index {
                                     $(
                                         idx if idx == index => $arg,
                                     )*
                                     _ => unreachable!(),
                                 };
                                 let arg_type = $func_args_type.get_element::<JsUnknown>(index).unwrap();
                                 let param = get_js_function_call_value($env, arg_type, c_param);
                                 param
                             })
                             .collect();
                         $js_function.call(None, &value).unwrap();
                     };
                     let closure = Box::into_raw(Box::new($closure::new(&lambda)));
                     return std::mem::transmute((*closure).code_ptr());
                 }
             )*
             _ => panic!("func_args get array error"),
         }
     };
 }
