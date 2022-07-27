use std;
use regex::Regex;

fn main() {
    // regex - regular expression
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_rst = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_prd = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(?P<left>\d+)\s?/\s?(?P<right>\d+)").unwrap();

    // user data
    println!("Please enter a expression");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // code for product
    loop {
        let caps = re_prd.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }

    let caps = caps.unwrap();

    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let rigth_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value * rigth_value;

    expression = expression.replace(cap_expression, &addition.to_string());
    }    
    // code for divition
    loop {
        let caps = re_div.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }

    let caps = caps.unwrap();

    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let rigth_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value / rigth_value;

    expression = expression.replace(cap_expression, &addition.to_string());
    }   
    // code for rest
    loop {
        let caps = re_rst.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }


    let caps = caps.unwrap();

    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let rigth_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value - rigth_value;

    expression = expression.replace(cap_expression, &addition.to_string());
    } 

    // code for sum 
    loop {
        let caps = re_add.captures(&expression.as_str());

        if caps.is_none() {
            break;
        }


    let caps = caps.unwrap();

    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let rigth_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value + rigth_value;

    expression = expression.replace(cap_expression, &addition.to_string());
    }
    

    // show the expression
    println!("Result: {}", expression);
}
