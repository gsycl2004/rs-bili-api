#[macro_export] macro_rules! define_api_get {
    ($name:ident,$url:literal,$($arg_name:ident),*) => {
        paste::paste!{
        pub(crate) fn [<call_ $name>]($(
        $arg_name:&str,
        )*)->reqwest::Request{
                let mut args = String::from($url);
                args += "?";
                $(
                    args += stringify!($arg_name);
                    args += "=";
                    args += $arg_name;
                    args += "&";
                )*
                reqwest::Request::new(reqwest::Method::GET,reqwest::Url::parse(&args).unwrap())


            }

        }
    }
}

#[macro_export] macro_rules! define_api_post {
    ($name:ident,$url:literal,$($arg_name:ident),*) => {
        paste::paste!{
        pub(crate) fn [<call_ $name>](client:&reqwest::Client,
        $(
        $arg_name:&str,
        )*
        )->reqwest::Request{

                let mut args:HashMap<String,&str> = HashMap::new();
                $(
                    args.insert(stringify!($arg_name).replace("r#",""),$arg_name);
                )*
                println!("{:?}",args);
                let b = client.post($url)
                .form(&args)
                .header("referer","https://www.bilibili.com")
                .header("user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.1661.54")

                .build()
                .unwrap();
                b
            }

        }
    }
}





