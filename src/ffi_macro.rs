macro_rules! match_args_len {
    ($args_len:ident,  $func_args_type_ptr:expr,  $js_function_ptr:expr, $env:expr, $($num:literal => $closure:ident, $($arg:ident),*),*) => {
        match $args_len {
            $(
                $num => {
                    let lambda = move |$($arg: *mut c_void),*| {
                        let func_args_type_ptr = &*$func_args_type_ptr;
                        let js_function_ptr = &*$js_function_ptr;
                        let arg_arr = [$($arg),*];
                        let value: Vec<JsUnknown> = (0..$num)
                            .map(|index| {
                                let c_param = arg_arr[index as usize];
                                let arg_type = (func_args_type_ptr).get_element::<JsUnknown>(index).unwrap();
                                let param = get_js_function_call_value($env, arg_type, c_param);
                                param
                            })
                            .collect();
                        js_function_ptr.call(None, &value).unwrap();
                    };
                    let closure = Box::into_raw(Box::new($closure::new(lambda)));
                    return std::mem::transmute((*closure).code_ptr());
                }
            )*
            _ => {
                std::ptr::null_mut() as *mut c_void
            },
        }
    };
}
