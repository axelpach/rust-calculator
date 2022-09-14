use regex::Regex;

fn main() {

    //Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_rest = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();

    //Traer datos del usuario
    println!("Porfavor introduce tu expresión: ");
    let mut expression : String = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    //Multiplicacion
    loop {
        //Aplicar operaciones
        let caps = re_mult.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let mult = left_value * right_value;

        expression = expression.replace(cap_expression, &mult.to_string());
        
        //println!("{:?} izq: {} + der: {}", caps, left_value, right_value);
        
    }

    //Division
    loop {
        //Aplicar operaciones
        let caps = re_div.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let div = left_value / right_value;

        expression = expression.replace(cap_expression, &div.to_string());
        
        //println!("{:?} izq: {} + der: {}", caps, left_value, right_value);
        
    }

    //Suma
    loop {
        //Aplicar operaciones
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let addition = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
        
        //println!("{:?} izq: {} + der: {}", caps, left_value, right_value);
        
    }

    //Resta
    loop {
        //Aplicar operaciones
        let caps = re_rest.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let substraction = left_value - right_value;

        expression = expression.replace(cap_expression, &substraction.to_string());
        
        //println!("{:?} izq: {} + der: {}", caps, left_value, right_value);
        
    }
    
    //Mostrar resultados
    println!("Resultado: {}", expression);
    
}
