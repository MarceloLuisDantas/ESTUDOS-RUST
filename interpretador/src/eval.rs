// use crate::io::print;
use crate::memoria::Memoria;
use crate::sintaxe::{sintaxe_rmv, sintaxe_var, sintaxe_aritmetica};

const FUNCOES: [&'static str; 4] = ["var", "rmv", "cls", "show"];
const ARITMETICA: [&'static str; 4] = ["sum", "sub", "mult", "div"];

#[derive(PartialEq, Debug)]
pub enum Resultado {
    Valor{r: f32},
    Msg{r: String},
}

impl Resultado {
    pub fn valor(&self) -> String {
        match self {
            Resultado::Msg { r } => r.to_string(),
            Resultado::Valor { r } => r.to_string(),
        }
    }
}

pub fn eval(linha: &str, memoria: &mut Memoria) -> Result<Resultado, String>{
    let tokens: Vec<&str> = linha.split(" ").collect();
    let comando = tokens[0];
    if FUNCOES.contains(&comando) {
        match comando {
            "var" => { 
                match sintaxe_var(&tokens) {
                    Ok(v) => {
                        Ok(Resultado::Valor{r: memoria.var(tokens[1], v)}) 
                    },
                    Err(msg) => { Err(msg) }
                }
            }
            "rmv" => {
                match sintaxe_rmv(&tokens) {
                    Ok(_) => {
                        match memoria.rmv(tokens[1]) {
                            Ok(v) => Ok(Resultado::Valor{r: v}),
                            Err(msg) => Err(msg),
                        }
                    },
                    Err(msg) => Err(msg),
                }
            }
            "cls" => { Ok(Resultado::Msg{r: "cls".to_string()}) }
            "show" => { Ok(Resultado::Msg{r: memoria.lista_valores()}) }
            _ => { Err("Erro inesperado".to_string()) },
        }
    } else if ARITMETICA.contains(&comando) {
        match sintaxe_aritmetica(&tokens) {
            Ok(_) => { 
                let valores: Vec<f32> = tokens[1..].iter().map(|x| x.parse::<f32>().unwrap()).collect();
                match comando {
                    "sum" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc + x).unwrap();
                        Ok(Resultado::Valor{r: resultado})
                    },
                    "sub" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc - x).unwrap();
                        Ok(Resultado::Valor{r: resultado})
                    },
                    "mult" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc * x).unwrap();
                        Ok(Resultado::Valor{r: resultado})
                    },
                    "div" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc / x).unwrap();
                        Ok(Resultado::Valor{r: resultado})
                    },
                    _ => {
                        Err("Teoricamente seria impossivel de você chegar aqui".to_string())
                    },
                }
            },
            Err(msg) => { Err(msg) }
        }
    } else {
        if memoria.contains(comando) {
            Ok(Resultado::Valor{r: memoria.get_valor(comando)})
        } else {
            Err(format!("'{}' não é uma função ou esta declarado na memoria", comando))
        }
    }
}

#[cfg(test)]
mod teste_eval {
    use crate::eval;

    use super::*;
    
    #[test]
    fn test_var() {
        let mut memoria = Memoria::new();
        
        assert_eq!(eval("var", &mut memoria), 
            Err("A função 'Var' recebe exatamente 2 parmetros".to_string()));
        
        assert_eq!(eval("var num 40", &mut memoria), Ok(Resultado::Valor { r: 40.0 }));

        assert_eq!(eval("var num dromedarios", &mut memoria), 
            Err("'dromedarios' não é um parametro valido para Var".to_string()));        
        
        assert_eq!(eval("var num 20", &mut memoria), Ok(Resultado::Valor { r: 20.0 }));
    }
    
    #[test]
    fn test_rmv() {
        let mut memoria = Memoria::new();
        let _ = eval("var valor1 30", &mut memoria);
        let _ = eval("var valor2 21", &mut memoria);
    
        assert_eq!(eval("rmv", &mut memoria), 
            Err("A função 'Rmv' recebe exatamente 1 parmetro".to_string()));
    
        assert_eq!(eval("rmv valor1", &mut memoria),  Ok(Resultado::Valor { r: 30.0 }));

        assert_eq!(eval("rmv valor1", &mut memoria), 
            Err("Variavel 'valor1' não encontrada na memoria".to_string()));
    }

    #[test]
    fn test_show_variavel() {
        let mut memoria = Memoria::new();
        let _ = eval("var valor1 30", &mut memoria);
        let _ = eval("var valor2 21", &mut memoria);

        assert_eq!(eval("valor1", &mut memoria), Ok(Resultado::Valor { r: 30.0 }));
        assert_eq!(eval("valor3", &mut memoria), 
            Err("'valor3' não é uma função ou esta declarado na memoria".to_string()));
        
        let _ = eval("rmv valor1", &mut memoria);
        assert_eq!(eval("valor1", &mut memoria), 
            Err("'valor1' não é uma função ou esta declarado na memoria".to_string()));

        memoria.var("valor3", 42.0);
        assert_eq!(eval("valor3", &mut memoria),  Ok(Resultado::Valor { r: 42.0 }));
    }

    #[test]
    fn test_lista_memoria() {
        let mut memoria = Memoria::new();
        let _ = eval("var valor1 10", &mut memoria);
        let _ = eval("var valor2 21", &mut memoria);
        let _ = eval("var valor3 42", &mut memoria);
        assert_eq!(eval("show", &mut memoria), Ok(Resultado::Msg { r: "3 valores na memoria".to_string() }));

        let _ = eval("rmv valor3", &mut memoria);
        assert_eq!(eval("show", &mut memoria), Ok(Resultado::Msg { r: "2 valores na memoria".to_string() }));
    }

    #[test]
    fn test_aritmetica() {
        let mut memoria = Memoria::new();
        assert_eq!(eval("sum 30 30", &mut memoria), Ok(Resultado::Valor { r: 60.0 }));
        assert_eq!(eval("sub 30 30", &mut memoria), Ok(Resultado::Valor { r: 0.0 }));
        assert_eq!(eval("div 30 2", &mut memoria), Ok(Resultado::Valor { r: 15.0 }));
        assert_eq!(eval("mult 2 10", &mut memoria), Ok(Resultado::Valor { r: 20.0 }));
        assert_eq!(eval("div 5 2", &mut memoria), Ok(Resultado::Valor { r: 2.5 }));
    }
}