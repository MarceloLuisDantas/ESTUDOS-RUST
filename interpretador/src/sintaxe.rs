#[allow(dead_code)]
pub fn help_var() -> String { "var [nome] [valor]".to_string() }
pub fn sintaxe_var(parametros: &[&str]) -> Result<f32, String> {
    if parametros.len() != 3 {
        Err("A função 'Var' recebe exatamente 2 parmetros".to_string())
    } else {
        match parametros[2].parse::<f32>() {
            Ok(v) => { Ok(v) },
            Err(_) => { 
                Err(format!("'{}' não é um parametro valido para Var", parametros[2]))
            },
        }
    }
}

#[allow(dead_code)]
pub fn help_rmv() -> String { "rmv [nome]".to_string() }
pub fn sintaxe_rmv(parametros: &[&str]) -> Result<String, String> {
    if parametros.len() != 2 {
        Err("A função 'Rmv' recebe exatamente 1 parmetro".to_string())
    } else {
        Ok("ok".to_string())
    }
}

#[allow(dead_code)]
pub fn help_aritmetica() -> String { "[operação] [valor] [valor] ...".to_string() }
pub fn sintaxe_aritmetica(parametros: &[&str]) -> Result<String, String> {
    let operadores: Vec<f32> = parametros
        .iter()
        .filter_map(|x| x.parse().ok())
        .collect();

    if operadores.len() != parametros.len() - 1 {
        Err("Funções de aritmetica recebem apenas numeros".to_string())
    } else {
        Ok("ok".to_string())
    }
}