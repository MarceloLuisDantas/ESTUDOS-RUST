use std::collections::HashMap;

pub struct Memoria {
    valores: HashMap<String, i32>,
}

impl Memoria {
    pub fn new() -> Self {
        Memoria { valores: HashMap::new() }
    }

    pub fn lista_valores(&self) {
        for (key, value) in self.valores.iter() {
            println!("{} -> {}", key, value);
        }
    }

    pub fn var(&mut self, nome: &str, valor: i32) -> i32 {
        if self.valores.contains_key(nome) {
            *self.valores.get_mut(nome).unwrap() = valor;
        } else  {
            self.valores.insert(nome.to_string(), valor);
        }
        return valor;
    }

    pub fn rmv(&mut self, nome: &str) -> Result<i32, String>{
        if self.valores.contains_key(nome) {
            match self.valores.remove(nome) {
                Some(x) => Ok(x),
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
        let _ = memoria.var("valor1", 32);
        assert_eq!(memoria.valores["valor1"], 32);

        let _ = memoria.var("valor1", 23);
        assert_eq!(memoria.valores["valor1"], 23);

        assert_eq!(memoria.rmv("valor1"), Ok(23));

        let _ = memoria.var("valor2", 10);
        assert_eq!(memoria.valores["valor2"], 10);

        match memoria.rmv("valor1") {
            Ok(_) => {},
            Err(msg) => {
                assert_eq!(msg, "Variavel 'valor1' não encontrada na memoria")
            }
        }
    }
}