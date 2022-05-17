use std::collections::HashMap;
use std::fmt::{self};
use crate::erros;

#[derive(PartialEq, Debug, Clone)]
pub enum Tipo {
    Int{valor: i64},
    Float{valor: f64},
    Str{valor: String},
}

impl fmt::Display for Tipo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tipo::Int { valor } => write!(f, "{}", valor),
            Tipo::Float { valor } => write!(f, "{}", valor),
            Tipo::Str { valor } => write!(f, "{}", valor),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Valor {
    pub mutavel: bool,
    pub tipo: Tipo,    
}

impl fmt::Display for Valor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mutavel = if self.mutavel { "var" } else { "const" };
        write!(f, "{} {}", mutavel, self.tipo)
    }
}

impl Valor {
    pub fn new(string: String, mutavel: bool) -> Result<Self, String> {
        let mut chars: Vec<char> = string.chars().collect();
        if chars[0] == '"' && chars[chars.len() - 1] == '"' {
            if chars.len() == 2 {
                return Err(erros::str_vazia())
            } else {
                chars.pop();
                chars.remove(0);
                return Ok( Valor { 
                    mutavel: mutavel,
                    tipo: Tipo::Str { valor: String::from_iter(chars) } 
                })
            }
        }
        let mut dots = 0;
        for c in string.chars() {
            if c == '.' {
                dots += 1;
            } else if !c.is_digit(10) || dots > 1 {                
                return Err(erros::valor_invalido())
            }
        }
        if dots == 1 {
            Ok( Valor { 
                mutavel: mutavel,
                tipo: Tipo::Float { valor: string.parse::<f64>().unwrap() }
            })
        } else {
            Ok( Valor { 
                mutavel: mutavel,
                tipo: Tipo::Int { valor: string.parse::<i64>().unwrap() }
            })
        }
    }
    pub fn get_tipo(&self) -> String {
        match self.tipo {
            Tipo::Int { valor: _ } => "Int".to_string(),
            Tipo::Float { valor: _ } => "Float".to_string(),
            Tipo::Str { valor: _ } => "Str".to_string(), 
        }
    }
}

pub struct Memoria {
    valores: HashMap<String, Valor>,
}

impl Memoria {
    pub fn new() -> Self {
        Memoria { valores: HashMap::new() }
    }

    /// Verifica se um valor esta registrado pelo nome
    pub fn contains(&self, nome: &str) -> bool {
        self.valores.contains_key(nome)
    }

    /// Retorna o valor registrado na memoria pelo nome
    pub fn get_valor(&self, nome: &str) -> Valor {
        self.valores.get(nome).unwrap().clone()
    }

    /// Mostra todos os valores guardados na memoria
    pub fn lista_valores(&self) -> String{
        for (key, value) in self.valores.iter() {
            println!("{} = {}", key, value);
        }
        format!("{} valor(es) na memoria", self.valores.len())
    }

    /// Sintaxe - var [nome] [valor]
    /// Declara um novo valor na memoria
    pub fn var(&mut self, nome: &str, valor: Valor) -> Tipo {
        if self.valores.contains_key(nome) {
            *self.valores.get_mut(nome).unwrap() = valor.clone();
        } else  {
            self.valores.insert(nome.to_string(), valor.clone());
        }
        return valor.tipo;
    }

    /// Sintaxe - set [valor] [nome]
    /// Altera um valor já existente na memoria
    pub fn set(&mut self, nome: &str, valor: &str) -> Result<Valor, String> {
        let valor_antigo = self.get_valor(nome);
        if !valor_antigo.mutavel {
            Err(erros::valor_constante(nome))
        } else {
            match Valor::new(valor.to_string(), true) {
                Ok(novo_valor) => {
                    if novo_valor.get_tipo() == valor_antigo.get_tipo() {
                        let _ = self.var(nome, novo_valor.clone());
                        Ok(novo_valor)
                    } else {
                        Err(erros::tipos_diferentes())
                    }
                }
                Err(err) => Err(err)
            }
        }
    }

    /// Sintaxe - rmv [nome]
    /// Remove um valor da memoria
    pub fn rmv(&mut self, nome: &str) -> Result<Tipo, String>{
        if self.valores.contains_key(nome) {
            match self.valores.remove(nome) {
                Some(x) => Ok(x.tipo),
                None => Err(erros::nao_pode_remover(nome)),
            }
        } else {
            Err(erros::nao_encontrado(nome))
        }
    }
}

// #[cfg(test)]
// mod test_memoria {
//     use super::*;
    
// }