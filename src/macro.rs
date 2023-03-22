use paste::paste;
macro_rules! define_api {
    ($name:ident,$text:literal,$($arg_name:ident,$arg_type:ident),*) => {
        paste!{
        pub(crate) fn [<call_ $name>]($(
        $arg_name:$arg_type,
        )*){
                let args = stringify!($($arg_name={},)*);
                println!("{}?{}",args);


            }

        }
    }
}

define_api!(api,"hello world",name,String);

#[cfg(test)]
mod test {
    use reqwest::{Method, Request};
    use crate::r#macro::call_api;

    #[test]
    fn t() {

        call_api(String::from("happy"))
    }
}