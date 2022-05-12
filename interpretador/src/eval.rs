use crate::memoria::Memoria;

fn eval_var(parametros: &[&str]) -> Result<i32, String> {
    if parametros.len() != 3 {
        Err("A função 'Var' recebe exatamente 2 parmetros".to_string())
    } else {
        match parametros[2].parse::<i32>() {
            Ok(v) => { Ok(v) },
            Err(_) => { 
                Err(format!("'{}' não é um parametro valido para Var", parametros[2]))
            },
        }
    }
}

fn eval_rmv(parametros: &[&str]) -> Result<String, String> {
    if parametros.len() != 2 {
        Err("A função 'Rmv' recebe exatamente 1 parmetro".to_string())
    } else {
        Ok("ok".to_string())
    }
}

pub fn eval(linha: &str, memoria: &mut Memoria) -> Result<i32, String>{
    let funcoes = vec!["var", "rmv", "cls"];
    let tokens: Vec<&str> = linha.split(" ").collect();
    if funcoes.contains(&tokens[0]) {
        match tokens[0] {
            "var" => { 
                match eval_var(&tokens) {
                    Ok(v) => { Ok(memoria.var(tokens[1], v)) },
                    Err(msg) => { Err(msg) }
                }
            },
            "rmv" => {
                match eval_rmv(&tokens) {
                    Ok(_) => {
                        match memoria.rmv(tokens[1]) {
                            Ok(v) => Ok(v),
                            Err(msg) => Err(msg),
                        }
                    },
                    Err(msg) => Err(msg),
                }
            }
            "cls" => {
                Err("cls".to_string())
            }
            _ => { Err("Erro inesperado".to_string()) },
        }
    } else {
        Err("Não implementado".to_string())
    }
}

#[cfg(test)]
mod teste_eval {
    use super::*;
    #[test]
    fn test_var() {
        let mut memoria = Memoria::new();
        
        assert_eq!(eval("var", &mut memoria), 
            Err("A função 'Var' recebe exatamente 2 parmetros".to_string()));
        
            assert_eq!(eval("var num 40", &mut memoria), Ok(40));

        assert_eq!(eval("var num dromedarios", &mut memoria), 
            Err("'dromedarios' não é um parametro valido para Var".to_string()));        
        
        assert_eq!(eval("var num 20", &mut memoria), Ok(20));
    }
    #[test]
    fn test_rmv() {
        let mut memoria = Memoria::new();
        memoria.var("valor1", 30);
        memoria.var("valor2", 21);
    
        assert_eq!(eval("rmv", &mut memoria), 
            Err("A função 'Rmv' recebe exatamente 1 parmetro".to_string()));
    
        assert_eq!(eval("rmv valor1", &mut memoria), Ok(30));

        assert_eq!(eval("rmv valor1", &mut memoria), 
            Err("Variavel 'valor1' não encontrada na memoria".to_string()));
    }
}