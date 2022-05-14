use std::collections::HashMap;

pub struct Memoria {
    valores: HashMap<String, f32>,
}

impl Memoria {
    pub fn new() -> Self {
        Memoria { valores: HashMap::new() }
    }

    pub fn contains(&self, nome: &str) -> bool {
        self.valores.contains_key(nome)
    }

    pub fn get_valor(&self, nome: &str) -> f32 {
        *self.valores.get(nome).unwrap()
    }

    pub fn lista_valores(&self) -> String{
        for (key, value) in self.valores.iter() {
            println!("{} = {}", key, value);
        }
        format!("{} valores na memoria", self.valores.len())
    }

    pub fn var(&mut self, nome: &str, valor: f32) -> f32 {
        if self.valores.contains_key(nome) {
            *self.valores.get_mut(nome).unwrap() = valor;
        } else  {
            self.valores.insert(nome.to_string(), valor);
        }
        return valor;
    }

    pub fn rmv(&mut self, nome: &str) -> Result<f32, String>{
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
        let _ = memoria.var("valor1", 32.0);
        assert_eq!(memoria.valores["valor1"], 32.0);

        let _ = memoria.var("valor1", 23.0);
        assert_eq!(memoria.valores["valor1"], 23.0);

        assert_eq!(memoria.rmv("valor1"), Ok(23.0));

        let _ = memoria.var("valor2", 10.0);
        assert_eq!(memoria.valores["valor2"], 10.0);

        match memoria.rmv("valor1") {
            Ok(_) => {},
            Err(msg) => {
                assert_eq!(msg, "Variavel 'valor1' não encontrada na memoria")
            }
        }
    }
}