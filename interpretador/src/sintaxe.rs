use crate::erros;

pub fn sintaxe_var_const(parametros: &[&str]) -> Result<bool, String> {
    if parametros.len() != 3 {
        Err(erros::sintaxe_var_const_len())
    } else {
        match parametros[1].parse::<f64>() {
            Ok(_) => Err(erros::sintaxe_var_const_nome(parametros[1])),
            Err(_) => Ok(true),
        }
    }
}

pub fn sintaxe_set(parametros: &[&str]) -> Result<bool, String> {
    if parametros.len() != 3 {
        Err(erros::sintaxe_set_len())
    } else {
        Ok(true)
    }
}

pub fn sintaxe_type_of(parametros: &[&str]) -> Result<bool, String> {
    if parametros.len() != 2 {
        Err(erros::sintaxe_type_of_len())
    } else {
        Ok(true)
    }
}

pub fn sintaxe_rmv(parametros: &[&str]) -> Result<bool, String> {
    if parametros.len() != 2 {
        Err(erros::sintaxe_rmv_len())
    } else {
        Ok(true)
    }
}

pub fn sintaxe_aritmetica(parametros: &[&str]) -> Result<bool, String> {
    let operadores: Vec<f32> = parametros
        .iter()
        .filter_map(|x| x.parse().ok())
        .collect();

    if operadores.len() != parametros.len() - 1 {
        Err(erros::sintaxe_aritmetica())
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod test_sintaxe {

    use super::*;
    #[test]
    fn test_sintaxe_aritmetica() {
        let linha: Vec<&str> = "sum 10 10".split(" ").collect();
        assert_eq!(sintaxe_aritmetica(&linha), Ok(true));
        let linha: Vec<&str> = "div 10 10 10".split(" ").collect();
        assert_eq!(sintaxe_aritmetica(&linha), Ok(true));
        let linha: Vec<&str> = "mult 10 10 10 10".split(" ").collect();
        assert_eq!(sintaxe_aritmetica(&linha), Ok(true));
        let linha: Vec<&str> = "sum 10".split(" ").collect();
        assert_eq!(sintaxe_aritmetica(&linha), Err(erros::sintaxe_aritmetica()));
    }
    #[test]
    fn test_sintaxe_var_const() {
        
    }
    #[test]
    fn test_sintaxe_rmv() {
        
    }
    #[test]
    fn test_sintaxe_set() {
        
    }
    #[test]
    fn test_sintaxe_type_of() {
        
    }
}