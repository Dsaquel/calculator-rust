use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    
    let first_arg = match args.nth(1) {
        Some(v) => v,
        _ => panic!("Operation requise")
    };
    let second_arg = match args.nth(0) {
        Some(v) if !v.contains(['/', '+', '-', '*', 'x']) => panic!("Le second argument n'est pas un operateur"),
        Some(v) => v,
        _ => panic!("Pas d'operateur")
    };

    let thrid_arg: String = match args.nth(0) {
        Some(v) => v,
        _ => panic!("Pas de troisieme nombre")
    };

    let res = calculator((first_arg, second_arg, thrid_arg));

    println!("le resultat de l'operation est de : {}", res)
}

fn calculator(operation: (String, String, String)) -> f32 {
    let (first_number, operator, second_number) = operation;
    let first = first_number.parse::<f32>().expect("Le premier argument n'est pas un nombre");
    let second = second_number.parse::<f32>().expect("Le deuxieme argument n'est pas un nombre");

    match operator.as_str() {
        "/" => first / second,
        "+" => first + second,
        "*" | "x" => first * second,
        "-" => first - second,
        _ => panic!("Probleme au niveau de l'opération")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_addition() {
        assert_eq!(calculator(("10".to_string(), "+".to_string(), "5".to_string())), 15f32)
    }

    #[test]
    fn test_sub() {
        assert_eq!(calculator(("10".to_string(), "-".to_string(), "5".to_string())), 5f32)
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculator(("10".to_string(), "x".to_string(), "5".to_string())), 50f32)
    }

    #[test]
    fn test_division() {
        assert_eq!(calculator(("11".to_string(), "/".to_string(), "5".to_string())), 2.2f32)
    }
}
