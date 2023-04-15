/*

    Write a program to calculate the following:

    Sum=1-x2/2!+ x4/4!-x6/ 6!+x8/8!-x10/10!

    i) A perfect number is a number that is the sum of all its divisors except itself.
        Six is the perfect number. The only numbers that divide 6 evenly are 1,2,3 and 6
        (i.e., 1+2+3=6).
    ii) An abundant number is one that is less than the sum of its divisors (Eg: 12 <1+2+3+4+6).
    iii) Adeficientnumberisonethatisgreaterthanthesumofitsdivisors (Ex: 9 > 1+3).

    Write a program to classify N integers (Read N from keyboard) each as perfect, abundant or deficient.

 */


enum NumberType {
    Deficient,
    Perfect,
    Abundant,
}

impl ToString for NumberType {
    fn to_string(&self) -> String {
        match *self {
            NumberType::Deficient => "Deficient".to_string(),
            NumberType::Perfect => "Perfect".to_string(),
            NumberType::Abundant => "Abundant".to_string(),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let x: f64 = match args.as_slice() {
        [x] => x.parse::<f64>().expect(&format!("{x} is not a 8bit integer")),
        _ => panic!("Please provide a valid 8bit integer"),
    };
    eval_expr(x);
    let number_type = type_fo_number(x as u8);
    println!("Given number {x} is {}", number_type.to_string());
}

fn eval_expr(x: f64) {
    let sum: f64 = 1_f64 - (x.powf(2_f64) / get_factorial(2)) + (x.powf(4_f64) / get_factorial(4)) - (x.powf(6_f64) / get_factorial(6)) + (x.powf(8_f64) / get_factorial(8)) - (x.powf(10_f64) / get_factorial(10));
    println!("1 - (x^2/2!)+ (x^4/4!) - (x^6/6!) + (x^8/8!) - (x^10/10!) where x={x} is {sum}");
}

fn get_factorial(n: u8) -> f64 {
    if n == 0 || n == 1 {
        return 1_f64;
    }
    (n as f64) * get_factorial(n - 1)
}

fn type_fo_number(n: u8) -> NumberType {
    let divisors: Vec<u8> = get_divisors_of_number(n);
    let mut sum: u8 = 0;
    for divisor in divisors {
        sum += divisor
    }
    if sum == n {
        NumberType::Perfect
    } else if sum < n {
        NumberType::Deficient
    } else {
        NumberType::Abundant
    }
}

fn get_divisors_of_number(n: u8) -> Vec<u8> {
    let mut counter: u8 = 1;
    let mut divisors: Vec<u8> = vec![];
    while counter <= n / 2 {
        if n % counter == 0 {
            divisors.push(counter)
        }
        counter += 1;
    }
    divisors
}
