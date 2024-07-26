use super::calc;

#[test]
fn get_number_test_1(){
    assert_eq!(1, calc::eval("1".to_string()).unwrap().0);
}

#[test]
fn get_number_test_2(){
    assert_eq!(1, calc::eval("((1))".to_string()).unwrap().0);
}

#[test]
fn get_number_test_3(){
    assert_eq!(110200, calc::eval("1   1  0 2 00".to_string()).unwrap().0);
}

#[test]
fn add_test_1(){
    assert_eq!(3, calc::eval("1+2".to_string()).unwrap().0);
}

#[test]
fn add_test_2(){
    assert_eq!(3, calc::eval("(1+2)".to_string()).unwrap().0);
}

#[test]
fn add_test_3(){
    assert_eq!(3, calc::eval("(1)+(2)".to_string()).unwrap().0);
}

#[test]
fn add_test_4(){
    assert_eq!(17, calc::eval("5+1 0  +2".to_string()).unwrap().0);
}

#[test]
fn sub_test_1(){
    assert_eq!(1, calc::eval("2-1".to_string()).unwrap().0);
}

#[test]
fn sub_test_2(){
    assert_eq!(10, calc::eval("(10)-0".to_string()).unwrap().0);
}

#[test]
fn sub_test_3(){
    assert_eq!(11, calc::eval("((2 0)  -( (9)))".to_string()).unwrap().0);
}

#[test]
fn sub_test_4(){
    assert_eq!(4, calc::eval("1 0- 4-2".to_string()).unwrap().0);
}

#[test]
fn mul_test_1(){
    assert_eq!(2, calc::eval("2*1".to_string()).unwrap().0);
}

#[test]
fn mul_test_2(){
    assert_eq!(0, calc::eval("(10)*0".to_string()).unwrap().0);
}

#[test]
fn mul_test_3(){
    assert_eq!(180, calc::eval("((2 0)  *( (9)))".to_string()).unwrap().0);
}

#[test]
fn mul_test_4(){
    assert_eq!(60, calc::eval("2*1 0*3".to_string()).unwrap().0);
}

#[test]
fn div_test_1(){
    assert_eq!(5, calc::eval("1 0/ 2".to_string()).unwrap().0);
}

#[test]
fn div_test_2(){
    assert_eq!(15, calc::eval("(1 5)/ 1".to_string()).unwrap().0);
}

#[test]
fn div_test_3(){
    assert_eq!(61839061, calc::eval("((123678122)/2)".to_string()).unwrap().0);
}

#[test]
fn div_test_4(){
    assert_eq!(2, calc::eval("1 6/ 2/   4".to_string()).unwrap().0);
}

#[test]
fn pow_test_1(){
    assert_eq!(8, calc::eval("2 ^(3)".to_string()).unwrap().0);
}

#[test]
fn pow_test_2(){
    assert_eq!(1, calc::eval("(((19128736)) ^(0))".to_string()).unwrap().0);
}

#[test]
fn pow_test_3(){
    assert_eq!(268435456, calc::eval("16 ^7".to_string()).unwrap().0);
}

#[test]
fn pow_test_4(){
    assert_eq!(1, calc::eval("1 ^839274".to_string()).unwrap().0);
}

#[test]
fn precedence_test(){
    assert_eq!(16, calc::eval("5*3+10-18/2".to_string()).unwrap().0);
}

#[test]
fn parenthesis_test_1(){
    assert_eq!(25, calc::eval("10*(2+8)/4".to_string()).unwrap().0);
}

#[test]
fn parenthesis_test_2(){
    assert_eq!(25, calc::eval("10*(2+8)/4".to_string()).unwrap().0);
}

#[test]
fn parenthesis_test_3(){
    assert_eq!(1, calc::eval("(((2))^3)/((4)+2*(200/(2^6+8*4)))".to_string()).unwrap().0);
}

#[test]
fn expr_test_1(){
    assert_eq!(4, calc::eval("(((2))^3)/((4)+2*(200/(2^6+8*4)))+(((2))^3)/((4)+2*(200/(2^6+8*4)))+(((2))^3)/((4)+2*(200/(2^6+8*4)))+(((2))^3)/((4)+2*(200/(2^6+8*4)))".to_string()).unwrap().0);
}

#[test]
fn expr_test_2(){
    assert_eq!(164531, calc::eval("(1+2+3+4+5)^4*13/4".to_string()).unwrap().0);
}

#[test]
fn divided_by_zero_test(){
    assert_eq!("Divided by zero", calc::eval("1/0".to_string()).unwrap_err().0);
}

#[test]
fn wrong_paren_test_1(){
    assert_eq!("Wrong parenthesis found", calc::eval("(".to_string()).unwrap_err().0);
}

#[test]
fn wrong_paren_test_2(){
    assert_eq!("Wrong parenthesis found", calc::eval("())".to_string()).unwrap_err().0);
}

#[test]
fn no_result_test_1(){
    assert_eq!("No Result", calc::eval("()".to_string()).unwrap_err().0);
}

#[test]
fn arg_miss_test_1(){
    assert_eq!("Argumentum is missing", calc::eval("()+1".to_string()).unwrap_err().0);
}

#[test]
fn arg_miss_test_2(){
    assert_eq!("Argumentum is missing", calc::eval("()+1+()".to_string()).unwrap_err().0);
}

#[test]
fn arg_miss_test_3(){
    assert_eq!("Argumentum is missing", calc::eval("1^()".to_string()).unwrap_err().0);
}

#[test]
fn arg_miss_test_4(){
    assert_eq!("Argumentum is missing", calc::eval("()/()".to_string()).unwrap_err().0);
}

#[test]
fn arg_miss_test_5(){
    assert_eq!("Argumentum is missing", calc::eval("()*()".to_string()).unwrap_err().0);
}

#[test]
fn arg_miss_test_6(){
    assert_eq!("Argumentum is missing", calc::eval("()-()".to_string()).unwrap_err().0);
}

#[test]
fn token_parse_err_test_6(){
    assert_eq!("a: Token cannot be parsed", calc::eval("a".to_string()).unwrap_err().0);
}

#[test]
fn token_parse_err_test_7(){
    assert_eq!("]: Token cannot be parsed", calc::eval("]".to_string()).unwrap_err().0);
}

#[test]
fn token_parse_err_test_8(){
    assert_eq!("': Token cannot be parsed", calc::eval("'".to_string()).unwrap_err().0);
}