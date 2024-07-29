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
    assert_eq!(8, calc::eval("2 **(3)".to_string()).unwrap().0);
}

#[test]
fn pow_test_2(){
    assert_eq!(1, calc::eval("(((19128736)) **(0))".to_string()).unwrap().0);
}

#[test]
fn pow_test_3(){
    assert_eq!(268435456, calc::eval("16 **7".to_string()).unwrap().0);
}

#[test]
fn pow_test_4(){
    assert_eq!(1, calc::eval("1 **839274".to_string()).unwrap().0);
}

#[test]
fn fact_test_1(){
    assert_eq!(6, calc::eval("3!".to_string()).unwrap().0);
}

#[test]
fn fact_test_2(){
    assert_eq!(1, calc::eval("0!".to_string()).unwrap().0);
}

#[test]
fn fact_test_3(){
    assert_eq!(3628800, calc::eval("10!".to_string()).unwrap().0);
}

#[test]
fn fact_test_4(){
    assert_eq!(720, calc::eval("3!!".to_string()).unwrap().0);
}

#[test]
fn sqrt_test_1(){
    assert_eq!(3, calc::eval("@9".to_string()).unwrap().0);
}

#[test]
fn sqrt_test_2(){
    assert_eq!(0, calc::eval("@0".to_string()).unwrap().0);
}

#[test]
fn sqrt_test_3(){
    assert_eq!(20, calc::eval("@400".to_string()).unwrap().0);
}

#[test]
fn sqrt_test_4(){
    assert_eq!(23, calc::eval("@@279841".to_string()).unwrap().0);
}

#[test]
fn and_test_1(){
    assert_eq!(0, calc::eval("124&&0".to_string()).unwrap().0);
}

#[test]
fn and_test_2(){
    assert_eq!(0, calc::eval("0&&2313".to_string()).unwrap().0);
}

#[test]
fn and_test_3(){
    assert_eq!(0, calc::eval("0&&0".to_string()).unwrap().0);
}

#[test]
fn and_test_4(){
    assert_eq!(1, calc::eval("124&&929842".to_string()).unwrap().0);
}

#[test]
fn and_test_5(){
    assert_eq!(0, calc::eval("124&&929842&&0".to_string()).unwrap().0);
}

#[test]
fn or_test_1(){
    assert_eq!(1, calc::eval("124||0".to_string()).unwrap().0);
}

#[test]
fn or_test_2(){
    assert_eq!(1, calc::eval("0||2313".to_string()).unwrap().0);
}

#[test]
fn or_test_3(){
    assert_eq!(0, calc::eval("0||0".to_string()).unwrap().0);
}

#[test]
fn or_test_4(){
    assert_eq!(1, calc::eval("124||929842".to_string()).unwrap().0);
}

#[test]
fn or_test_5(){
    assert_eq!(1, calc::eval("124||929842||0".to_string()).unwrap().0);
}

#[test]
fn xor_test_1(){
    assert_eq!(1, calc::eval("124^^0".to_string()).unwrap().0);
}

#[test]
fn xor_test_2(){
    assert_eq!(1, calc::eval("0^^2313".to_string()).unwrap().0);
}

#[test]
fn xor_test_3(){
    assert_eq!(0, calc::eval("0^^0".to_string()).unwrap().0);
}

#[test]
fn xor_test_4(){
    assert_eq!(0, calc::eval("124^^929842".to_string()).unwrap().0);
}

#[test]
fn xor_test_5(){
    assert_eq!(0, calc::eval("124^^929842^^0".to_string()).unwrap().0);
}

#[test]
fn band_test_1(){
    assert_eq!(0, calc::eval("124&0".to_string()).unwrap().0);
}

#[test]
fn band_test_2(){
    assert_eq!(1, calc::eval("5&3".to_string()).unwrap().0);
}

#[test]
fn band_test_3(){
    assert_eq!(0, calc::eval("0&0".to_string()).unwrap().0);
}

#[test]
fn band_test_4(){
    assert_eq!(4145, calc::eval("12345&54321".to_string()).unwrap().0);
}

#[test]
fn band_test_5(){
    assert_eq!(0, calc::eval("124&929842&0".to_string()).unwrap().0);
}

#[test]
fn bor_test_1(){
    assert_eq!(124, calc::eval("124|0".to_string()).unwrap().0);
}

#[test]
fn bor_test_2(){
    assert_eq!(7, calc::eval("5|3".to_string()).unwrap().0);
}

#[test]
fn bor_test_3(){
    assert_eq!(0, calc::eval("0|0".to_string()).unwrap().0);
}

#[test]
fn bor_test_4(){
    assert_eq!(62521, calc::eval("12345|54321".to_string()).unwrap().0);
}

#[test]
fn bor_test_5(){
    assert_eq!(13, calc::eval("1|8|4".to_string()).unwrap().0);
}

#[test]
fn bxor_test_1(){
    assert_eq!(124, calc::eval("124^0".to_string()).unwrap().0);
}

#[test]
fn bxor_test_2(){
    assert_eq!(6, calc::eval("5^3".to_string()).unwrap().0);
}

#[test]
fn bxor_test_3(){
    assert_eq!(0, calc::eval("0^0".to_string()).unwrap().0);
}

#[test]
fn bxor_test_4(){
    assert_eq!(58376, calc::eval("12345^54321".to_string()).unwrap().0);
}

#[test]
fn bxor_test_5(){
    assert_eq!(0, calc::eval("30^20^10".to_string()).unwrap().0);
}

#[test]
fn mod_test_1(){
    assert_eq!(0, calc::eval("10%1".to_string()).unwrap().0);
}

#[test]
fn mod_test_2(){
    assert_eq!(0, calc::eval("10%5".to_string()).unwrap().0);
}

#[test]
fn mod_test_3(){
    assert_eq!(3, calc::eval("3%4".to_string()).unwrap().0);
}

#[test]
fn mod_test_4(){
    assert_eq!(1, calc::eval("4%3%2".to_string()).unwrap().0);
}

#[test]
fn equals_test_1(){
    assert_eq!(1, calc::eval("0==0".to_string()).unwrap().0);
}

#[test]
fn equals_test_2(){
    assert_eq!(0, calc::eval("1==0".to_string()).unwrap().0);
}

#[test]
fn equals_test_3(){
    assert_eq!(0, calc::eval("1000==29634".to_string()).unwrap().0);
}

#[test]
fn equals_test_4(){
    assert_eq!(1, calc::eval("1000==29634==0".to_string()).unwrap().0);
}

#[test]
fn not_test_1(){
    assert_eq!(0, calc::eval("~3".to_string()).unwrap().0);
}

#[test]
fn not_test_2(){
    assert_eq!(1, calc::eval("~0".to_string()).unwrap().0);
}

#[test]
fn not_test_3(){
    assert_eq!(0, calc::eval("~987343".to_string()).unwrap().0);
}

#[test]
fn not_test_4(){
    assert_eq!(1, calc::eval("~~987343".to_string()).unwrap().0);
}

#[test]
fn gt_test_1(){
    assert_eq!(1, calc::eval("3>1".to_string()).unwrap().0);
}

#[test]
fn gt_test_2(){
    assert_eq!(0, calc::eval("4>5".to_string()).unwrap().0);
}

#[test]
fn gt_test_3(){
    assert_eq!(0, calc::eval("4>4".to_string()).unwrap().0);
}

#[test]
fn gt_test_4(){
    assert_eq!(1, calc::eval("4>3>0".to_string()).unwrap().0);
}

#[test]
fn ge_test_1(){
    assert_eq!(1, calc::eval("3>=1".to_string()).unwrap().0);
}

#[test]
fn ge_test_2(){
    assert_eq!(0, calc::eval("4>=5".to_string()).unwrap().0);
}

#[test]
fn ge_test_3(){
    assert_eq!(1, calc::eval("4>=4".to_string()).unwrap().0);
}

#[test]
fn ge_test_4(){
    assert_eq!(1, calc::eval("4>=4>=1>=1>=0".to_string()).unwrap().0);
}

#[test]
fn lt_test_1(){
    assert_eq!(0, calc::eval("3<1".to_string()).unwrap().0);
}

#[test]
fn lt_test_2(){
    assert_eq!(1, calc::eval("4<5".to_string()).unwrap().0);
}

#[test]
fn lt_test_3(){
    assert_eq!(0, calc::eval("4<4".to_string()).unwrap().0);
}

#[test]
fn lt_test_4(){
    assert_eq!(1, calc::eval("1<2<2<2<2".to_string()).unwrap().0);
}

#[test]
fn le_test_1(){
    assert_eq!(0, calc::eval("3<=1".to_string()).unwrap().0);
}

#[test]
fn le_test_2(){
    assert_eq!(1, calc::eval("4<=5".to_string()).unwrap().0);
}

#[test]
fn le_test_3(){
    assert_eq!(1, calc::eval("4<=4".to_string()).unwrap().0);
}

#[test]
fn le_test_4(){
    assert_eq!(1, calc::eval("2<=2<=1<=1<=1".to_string()).unwrap().0);
}

#[test]
fn ne_test_1(){
    assert_eq!(1, calc::eval("3~=1".to_string()).unwrap().0);
}

#[test]
fn ne_test_2(){
    assert_eq!(1, calc::eval("4~=5".to_string()).unwrap().0);
}

#[test]
fn ne_test_3(){
    assert_eq!(0, calc::eval("4~=4".to_string()).unwrap().0);
}

#[test]
fn ne_test_4(){
    assert_eq!(0, calc::eval("2~=2~=4~=1".to_string()).unwrap().0);
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
    assert_eq!(1, calc::eval("(((2))**3)/((4)+2*(200/(2**6+8*4)))".to_string()).unwrap().0);
}

#[test]
fn expr_test_1(){
    assert_eq!(4, calc::eval("(((2))**3)/((4)+2*(200/(2**6+8*4)))+(((2))**3)/((4)+2*(200/(2**6+8*4)))+(((2))**3)/((4)+2*(200/(2**6+8*4)))+(((2))**3)/((4)+2*(200/(2**6+8*4)))".to_string()).unwrap().0);
}

#[test]
fn expr_test_2(){
    assert_eq!(164531, calc::eval("(1+2+3+4+5)**4*13/4".to_string()).unwrap().0);
}

#[test]
fn expr_test_3(){
    assert_eq!(1, calc::eval("(@@160 000*3+4)/8**2!".to_string()).unwrap().0);
}

#[test]
fn expr_test_4(){
    assert_eq!(1, calc::eval("3*1-3+2 ~= @16/2!-(6&3) && @(2**3*50)>=19".to_string()).unwrap().0);
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

#[test]
fn var_test_1(){
    assert_eq!(2, calc::eval("a=(2+3)-3".to_string()).unwrap().0);
    assert_eq!(5, calc::eval("1+a*2".to_string()).unwrap().0);
    assert_eq!(18, calc::eval("asd09 = 3*(1+2+3)".to_string()).unwrap().0);
    assert_eq!(22, calc::eval("a*(a+asd09)-asd09".to_string()).unwrap().0);
    assert_eq!(3, calc::eval("a=3".to_string()).unwrap().0);
}