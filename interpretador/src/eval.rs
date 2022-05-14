// use crate::io::print;
use crate::memoria::{Memoria, Tipo, Valor};
use crate::sintaxe::*;

const FUNCOES: [&'static str; 5] = ["var", "const", "rmv", "cls", "show"];
const ARITMETICA: [&'static str; 4] = ["sum", "sub", "mult", "div"];

#[derive(PartialEq, Debug)]
pub enum Resultado {
    Valor{r: Tipo},
    Msg{r: String},
    Num{r: f64},
}

pub fn eval(linha: &str, memoria: &mut Memoria) -> Result<Resultado, String>{
    let tokens: Vec<&str> = linha.split(" ").collect();
    let comando = tokens[0];
    if FUNCOES.contains(&comando) {
        match comando {
            "var" | "const" => { 
                match sintaxe_var_const(&tokens) {
                    Ok(_) => {
                        if FUNCOES.contains(&tokens[1]) {
                            Err("Nomes de funções são palavras reservadas e não podem ser utilizadas".to_string())
                        } else {
                            if memoria.contains(tokens[1]) {
                                Err(format!("Valor '{}' já registrado na memoria", tokens[1]))
                            } else {
                                let valor = Valor::new(tokens[2].to_string(), comando == "var" );
                                Ok(Resultado::Valor { r: memoria.var(tokens[1], valor) })
                            }
                        }
                    },
                    Err(msg) => { Err(msg) }
                }
            }
            "rmv" => {
                match sintaxe_rmv(&tokens) {
                    Ok(_) => {
                        match memoria.rmv(tokens[1]) {
                            Ok(v) => {
                                Ok(Resultado::Valor { r: v })
                            },
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
                let valores: Vec<f64> = tokens[1..].iter().map(|x| x.parse::<f64>().unwrap()).collect();
                match comando {
                    "sum" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc + x).unwrap();
                        Ok(Resultado::Num{r: resultado})
                    },
                    "sub" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc - x).unwrap();
                        Ok(Resultado::Num{r: resultado})
                    },
                    "mult" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc * x).unwrap();
                        Ok(Resultado::Num{r: resultado})
                    },
                    "div" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc / x).unwrap();
                        Ok(Resultado::Num{r: resultado})
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
            let valor = memoria.get_valor(comando);
            Ok(Resultado::Valor{r: valor.tipo})
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
        Err("A função 'Var'/'Const' recebem exatamente 2 parmetros \n 
             -- var [nome] [valor] \n
             -- const [nome] [valor] \n".trim().to_string()));
        
        assert_eq!(eval("var num 40", &mut memoria), Ok(Resultado::Valor {  
            r: Tipo::Int { valor: 40 }
        }));

        assert_eq!(eval("var num 30", &mut memoria), 
            Err("Valor 'num' já registrado na memoria".to_string()));

        assert_eq!(eval("var eu dromedario", &mut memoria), Ok(Resultado::Valor {  
            r: Tipo::Str { valor: "dromedario".to_string() }
        }));

        assert_eq!(eval("var eu 53.12", &mut memoria), 
            Err("Valor 'eu' já registrado na memoria".to_string()));
    }
    
    #[test]
    fn test_rmv() {
        let mut memoria = Memoria::new();
        let _ = eval("var valor1 30", &mut memoria);
        let _ = eval("var valor2 chapeus", &mut memoria);
    
        assert_eq!(eval("rmv", &mut memoria), 
        Err("A função 'Rmv' recebe exatamente 1 parmetro \n
             -- rmv [nome] \n".trim().to_string()));
    
        assert_eq!(eval("rmv valor1", &mut memoria),  Ok(Resultado::Valor { 
            r: Tipo::Int { valor: 30 }
        }));

        assert_eq!(eval("rmv valor1", &mut memoria), 
            Err("Variavel 'valor1' não encontrada na memoria".to_string()));
        
        assert_eq!(eval("rmv valor2", &mut memoria),  Ok(Resultado::Valor { 
            r: Tipo::Str { valor: "chapeus".to_string() }
        }));
    }

    #[test]
    fn test_show_variavel() {
        let mut memoria = Memoria::new();
        let _ = eval("var valor1 30", &mut memoria);
        let _ = eval("var valor2 21", &mut memoria);

        assert_eq!(eval("valor1", &mut memoria), Ok(Resultado::Valor {
            r: Tipo::Int { valor: 30 }
        }));

        assert_eq!(eval("valor3", &mut memoria), 
            Err("'valor3' não é uma função ou esta declarado na memoria".to_string()));
        
        let _ = eval("rmv valor1", &mut memoria);
        assert_eq!(eval("valor1", &mut memoria), 
            Err("'valor1' não é uma função ou esta declarado na memoria".to_string()));

        let _ = eval("var valor1 42", &mut memoria);
        assert_eq!(eval("valor1", &mut memoria),  Ok(Resultado::Valor { 
            r: Tipo::Int { valor: 42 }
        }));
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
        assert_eq!(eval("sum 30 30", &mut memoria), Ok(Resultado::Num { r: 60.0 }));
        assert_eq!(eval("sub 30 30", &mut memoria), Ok(Resultado::Num { r: 0.0 }));
        assert_eq!(eval("div 30 2", &mut memoria), Ok(Resultado::Num { r: 15.0 }));
        assert_eq!(eval("mult 2 10", &mut memoria), Ok(Resultado::Num { r: 20.0 }));
        assert_eq!(eval("div 5 2", &mut memoria), Ok(Resultado::Num { r: 2.5 }));
    }
}