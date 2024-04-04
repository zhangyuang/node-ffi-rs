macro_rules! match_args_len {
    ($args_len:ident, $tsfn:expr, $func_args_type_rs:expr,  $js_function:expr,  $($num:literal => $closure:ident, $($arg:ident),*),*) => {
        match $args_len {
            $(
                $num => {
                    let lambda = move |$($arg: *mut c_void),*| {
                        let arg_arr = [$($arg),*];
                        let value: Vec<RsArgsValue> = (0..$num)
                            .map(|index| {
                                let c_param = arg_arr[index as usize];
                                 let arg_type = $func_args_type_rs.get(&index.to_string()).unwrap();
                                let param = get_js_function_call_value( arg_type, c_param);
                                param
                            })
                            .collect();
                      $tsfn.call(value, ThreadsafeFunctionCallMode::NonBlocking);
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
