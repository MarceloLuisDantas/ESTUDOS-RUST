use std::collections::HashMap;
use std::fmt;

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
        let mutavel = if self.mutavel { "const" } else { "var" };
        write!(f, "{} {}", mutavel, self.tipo)
    }
}

impl Valor {
    pub fn new(string: String, mutavel: bool) -> Self {
        let mut dots = 0;
        for c in string.chars() {
            if c == '.' {
                dots += 1;
            } else if !c.is_digit(10) || dots > 1 {                
                return Valor { 
                    mutavel: mutavel,
                    tipo: Tipo::Str { valor: string.to_string() } 
                }
            }
        }
        if dots == 1 {
            Valor { 
                mutavel: mutavel,
                tipo: Tipo::Float { valor: string.parse::<f64>().unwrap() }
            }
        } else {
            Valor { 
                mutavel: mutavel,
                tipo: Tipo::Int { valor: string.parse::<i64>().unwrap() }
            }
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
        format!("{} valores na memoria", self.valores.len())
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

    /// Sintaxe - rmv [nome]
    /// Remove um valor da memoria
    pub fn rmv(&mut self, nome: &str) -> Result<Tipo, String>{
        if self.valores.contains_key(nome) {
            match self.valores.remove(nome) {
                Some(x) => Ok(x.tipo),
                None => Err(format!("Erro ao tentar remover '{}'", nome)),
            }
        } else {
            Err(format!("Variavel '{}' não encontrada na memoria", nome))
        }
    }
}

#[cfg(test)]
mod test_memoria {
    use super::*;
    #[test]
    fn test() {
        let mut memoria = Memoria::new();

        let _ = memoria.var("valor1", Valor::new("32.0".to_string(), true));
        assert_eq!(memoria.valores["valor1"], Valor { 
            mutavel: true,
            tipo: Tipo::Float { valor: 32.0 }, 
        });

        let _ = memoria.var("valor2", Valor::new("42".to_string(), true));
        assert_eq!(memoria.valores["valor2"], Valor { 
            mutavel: true,
            tipo: Tipo::Int { valor: 42 }, 
        });

        let _ = memoria.var("valor3", Valor::new("teste".to_string(), true));
        assert_eq!(memoria.valores["valor3"], Valor { 
            mutavel: true,
            tipo: Tipo::Str { valor: "teste".to_string() }, 
        });

        let _ = memoria.var("valor4", Valor::new("27.03.2002".to_string(), true));
        assert_eq!(memoria.valores["valor4"], Valor { 
            mutavel: true,
            tipo: Tipo::Str { valor: "27.03.2002".to_string() }, 
        });
    }
}