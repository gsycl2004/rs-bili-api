

#[macro_export] macro_rules! define_api {
    ($name:ident,$url:literal,$($arg_name:ident,$arg_type:ty),*) => {

        paste!{
        pub(crate) fn [<call_ $name>]($(
        $arg_name:$arg_type,
        )*)->Request{
                let mut args = String::from($url);
                args += "?";
                $(
                    args += stringify!($arg_name);
                    args += "=";
                    args += $arg_name.borrow();
                    args += "&";
                )*
                Request::new(Method::GET,Url::parse(&args).unwrap())


            }

        }
    }
}



