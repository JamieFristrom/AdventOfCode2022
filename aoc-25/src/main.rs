fn main() {
    println!("Hello, world!");
    let answer = do_the_thing(get_puzzle_input());
    println!("boom? {answer}");
}

fn do_the_thing(input: &str) -> String {
    let mut sum : i128 = 0;
    for line in input.lines() {
        let decimal = snafu_to_decimal(line);
        println!("{decimal}");
        sum += decimal;
    }
    
    decimal_to_snafu(sum)
}

fn snafu_to_decimal(snafu: &str) -> i128 {
    let mut total = 0i128;
    for (i, c) in snafu.chars().rev().enumerate() {
        let digit = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => { panic!(); }
        };
        total += digit * 5i128.pow( i as u32 );
    }

    total
}

fn snafu_digit(decimal: i8) -> char {
    match decimal {
        -2 => {'='},
        -1 => {'-'},
        0 => {'0'},
        1 => {'1'},
        2 => {'2'},
        _ => {panic!();}
    }
}

fn decimal_to_snafu(decimal: i128) -> String {
    if decimal < 0 { panic!(); }
    if decimal==0 { return "0".to_string() };
    let mut digits: Vec<i8> = vec![];
    let mut rest_of_it = decimal;
    while rest_of_it>0 {
        let mut digit : i128 = rest_of_it as i128 % 5;
        rest_of_it = rest_of_it / 5;
        if digit == 3 {
            digit = -2;
            rest_of_it += 1;
        }
        else if digit == 4{
            digit = -1;
            rest_of_it += 1;
        }
        digits.push(digit as i8);
    }

    digits.iter().rev().map(|x| snafu_digit(*x)).collect::<String>()
}

#[test]
fn test_sample() {
    let answer = do_the_thing(&get_sample_input());
    assert_eq!("2=-1=0", answer);
}

#[test]
fn test_snafu_to_decimal() {
    assert_eq!(1, snafu_to_decimal("1"));
    assert_eq!(0, snafu_to_decimal("0"));
    assert_eq!(2, snafu_to_decimal("2"));
    assert_eq!(3, snafu_to_decimal("1="));
    assert_eq!(4, snafu_to_decimal("1-"));
    assert_eq!(5, snafu_to_decimal("10"));
    assert_eq!(6, snafu_to_decimal("11"));
    assert_eq!(7, snafu_to_decimal("12"));
    assert_eq!(8, snafu_to_decimal("2="));
    assert_eq!(9, snafu_to_decimal("2-"));
    assert_eq!(10, snafu_to_decimal("20"));
    assert_eq!(15, snafu_to_decimal("1=0"));
    assert_eq!(20, snafu_to_decimal("1-0"));
    assert_eq!(2022, snafu_to_decimal("1=11-2"));
    assert_eq!(12345, snafu_to_decimal("1-0---0"));
    assert_eq!(314159265, snafu_to_decimal("1121-1110-1=0"));
}

#[test]
fn test_decimal_to_snafu() {
    assert_eq!(decimal_to_snafu(1),"1");  
    assert_eq!(decimal_to_snafu(0),"0");
    assert_eq!(decimal_to_snafu(2),"2");
    assert_eq!(decimal_to_snafu(3),"1=");   // 3
    assert_eq!(decimal_to_snafu(4),"1-");
    assert_eq!(decimal_to_snafu(5),"10");
    assert_eq!(decimal_to_snafu(6),"11");
    assert_eq!(decimal_to_snafu(7),"12");
    assert_eq!(decimal_to_snafu(8),"2=");
    assert_eq!(decimal_to_snafu(9),"2-");
    assert_eq!(decimal_to_snafu(10),"20");
    assert_eq!(decimal_to_snafu(15),"1=0");
    assert_eq!(decimal_to_snafu(20),"1-0");
    assert_eq!(decimal_to_snafu(2022),"1=11-2");
    assert_eq!(decimal_to_snafu(12345),"1-0---0");
    assert_eq!(decimal_to_snafu(314159265),"1121-1110-1=0");
}

fn get_sample_input() -> &'static str {
"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
}

fn get_puzzle_input() -> &'static str {
"1=---001012-
1=0
1-2=20---
22--021102
102=10=
2-==-
11=--=21-2==0202
2=----2
1==---20-22=00-
1===---=10
1=0=-10-=
222-==2==1-1=10
1=--20=2-=1-2=220-0
1=0122-21==21-1
200
10==-02-00-=011=0
1=1-====00=100-1
1===-01-==02120-
2==01=--=2=0=0-2112
1-2
112-21=21-210=
2221-22
202==0=0-211-=
1-01-20122
122=12212=2
2-010
1-0=0022
1=002-10100011-12-21
1=2-1
1==0-01-1-02=
11-2==
1-10-=11
11==0=-10000001--0
2==122
102121=-=021
1=000--
120=00112
10=-
112
2=2-0==1=2=12
100--01022-=1-
1=1
1111=00211212
1---11201=2-=
22=-01=0=12102
2--2=0-01022-12
21==210=1=
1---1
2=1
1=-11=
1-=2=100-2=2
102=
2-2-
102
10==02
1=221=2=0=0=2100
10=2-0000==1001-01
20
212=
122022-02-==-=1=-2
2-1
2-1212==0==21===2=
1-2=11-0
1-00
1=2-202021202=
2=
2
22102==0--=-220
20=-2-2=2-112-11
1-=00-0-1
1001-0-==2
120--=-2
1=22-=--0-=2-0-01
2-20000
1-1201=
2==2=0--20=-0100
1=00
101==
11--=2-01-
1-12100
1=2=
1=0-0122-0-2-2-0=0
2-=022-===-022
1=-002220
1==1==0211-2--
10=0021--=-11=-=122
12=02=-=1
1=2221==1-0-011=
1122010==0-0
1=1110120
1=11020
2==
11=110121=01
1112=01==-00==0
1=012-12-20=-
222==2=-
2-==12-
212-21
10022=10
111-
11--21==--0==
11-=000=-
10=001=-
2--0=
2---2-021==0210-11
1102=2--02112=00=
2==2=100
102-=
22-1=20=-2=02"
}