use proc_macro;

//#[macro_export]
//macro_rules! vec {
//    ( $( $x:expr ), *) => {
//        {
//            let mut temp_vec = Vec::new();
//            $(
//                temp_vec.push($x);
//            )*
//            temp_vec
//        }
//    };
//}
//
//#[some_attributes]
//pub fn some_name(input: TokenStream) -> TokenStream {}
//
//pub fn add(left: u64, right: u64) -> u64 {
//    left + right
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
