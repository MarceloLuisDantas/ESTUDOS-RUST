// use crate::io::print;
use crate::erros;
use crate::memoria::{Memoria, Tipo, Valor};
use crate::sintaxe::*;

const FUNCOES: [&'static str; 7] = ["var", "const", "rmv", "cls", "show", "set", "type_of"];
const ARITMETICA: [&'static str; 4] = ["sum", "sub", "mult", "div"];

#[derive(PartialEq, Debug)]
pub enum Resultado {
    Valor { r: Tipo },
    Msg { r: String },
    Num { r: f64 },
}

pub fn eval(linha: &str, memoria: &mut Memoria) -> Result<Resultado, String> {
    let tokens: Vec<&str> = linha.split(" ").collect();
    let comando = tokens[0];
    if FUNCOES.contains(&comando) {
        match comando {
            "var" | "const" => match sintaxe_var_const(&tokens) {
                Ok(_) => {
                    if FUNCOES.contains(&tokens[1]) || ARITMETICA.contains(&tokens[1]) {
                        Err(erros::nome_invalido())
                    } else {
                        if memoria.contains(tokens[1]) {
                            Err(erros::valor_ja_existente(&tokens[1]))
                        } else {
                            match Valor::new(tokens[2].to_string(), comando == "var") {
                                Ok(valor) => Ok(Resultado::Valor {
                                    r: memoria.var(tokens[1], valor),
                                }),
                                Err(err) => Err(err),
                            }
                        }
                    }
                }
                Err(msg) => Err(msg),
            },
            "rmv" => match sintaxe_rmv(&tokens) {
                Ok(_) => match memoria.rmv(tokens[1]) {
                    Ok(v) => Ok(Resultado::Valor { r: v }),
                    Err(msg) => Err(msg),
                },
                Err(msg) => Err(msg),
            },
            "set" => match sintaxe_set(&tokens) {
                Ok(_) => {
                    if memoria.contains(tokens[2]) {
                        match memoria.set(tokens[2], tokens[1]) {
                            Ok(v) => Ok(Resultado::Valor { r: v.tipo }),
                            Err(err) => Err(err),
                        }
                    } else {
                        Err(erros::nao_encontrado(&tokens[2]))
                    }
                }
                Err(err) => Err(err),
            },
            "type_of" => match sintaxe_type_of(&tokens) {
                Ok(_) => {
                    if memoria.contains(tokens[1]) {
                        let tipo = memoria.get_valor(tokens[1]).get_tipo();
                        Ok(Resultado::Msg {
                            r: format!("{} = {}", tokens[1], tipo),
                        })
                    } else {
                        Err(erros::nao_encontrado(&tokens[2]))
                    }
                }
                Err(err) => Err(err),
            },
            "cls" => Ok(Resultado::Msg {
                r: "cls".to_string(),
            }),
            "show" => Ok(Resultado::Msg {
                r: memoria.lista_valores(),
            }),
            _ => Err(erros::erro_inesperado()),
        }
    } else if ARITMETICA.contains(&comando) {
        match sintaxe_aritmetica(&tokens) {
            Ok(_) => {
                let valores: Vec<f64> = tokens[1..]
                    .iter()
                    .map(|x| x.parse::<f64>().unwrap())
                    .collect();
                match comando {
                    "sum" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc + x).unwrap();
                        Ok(Resultado::Num { r: resultado })
                    }
                    "sub" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc - x).unwrap();
                        Ok(Resultado::Num { r: resultado })
                    }
                    "mult" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc * x).unwrap();
                        Ok(Resultado::Num { r: resultado })
                    }
                    "div" => {
                        let resultado = valores.into_iter().reduce(|acc, x| acc / x).unwrap();
                        Ok(Resultado::Num { r: resultado })
                    }
                    _ => Err("Teoricamente seria impossivel de você chegar aqui".to_string(),
                }
            }
            Err(msg) => Err(msg),
        }
    } else {
        if memoria.contains(comando) {
            let valor = memoria.get_valor(comando);
            Ok(Resultado::Valor { r: valor.tipo })
        } else {
            Err(erros::valor_desconhecido(comando))
        }
    }
}

// #[cfg(test)]
// mod teste_eval {

// }
