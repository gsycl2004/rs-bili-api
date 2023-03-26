#[macro_export] macro_rules! define_api_get {
    ($name:ident,$url:literal,$($arg_name:ident),*) => {
        paste!{
        pub(crate) fn [<call_ $name>]($(
        $arg_name:&String,
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

#[macro_export] macro_rules! define_api_post {
    ($name:ident,$url:literal,$($arg_name:ident),*) => {
        paste!{
        pub(crate) fn [<call_ $name>](client:&Client,
        $(
        $arg_name:&str,
        )*
        )->Request{

                let mut args:HashMap<String,&str> = HashMap::new();
                $(
                    args.insert(stringify!($arg_name).replace("r#",""),$arg_name);
                )*
                println!("{:?}",args);
                let b = client.post($url)
                .form(&args)
                .header("referer","https://www.bilibili.com")
                .build()
                .unwrap();
                b
            }

        }
    }
}





