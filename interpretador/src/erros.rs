pub fn valor_constante(nome: &str) -> String { format!(
"    '{}' é uma constante e não pode ser alterada
", nome
)}

pub fn nao_pode_remover(nome: &str) -> String { format!(
"    Erro ao tentar remover '{}'
", nome
)}

pub fn nao_encontrado(nome: &str) -> String { format!(
"    Valor '{}' não encontrado na memoria
", nome
)}

pub fn valor_ja_existente(nome: &str) -> String { format!(
"    Valor '{}' já registrado na memoria
", nome
)}

pub fn sintaxe_var_const_nome(nome: &str) -> String {format!(
"    {} não é um nome valido
", nome
)}

pub fn valor_desconhecido(comando: &str) -> String { format!(
"    '{}' não é uma função ou esta declarado na memoria
", comando
)}


pub fn str_vazia() -> String { 
"    Strings vazias não são valores validos
".to_string() }

pub fn valor_invalido() -> String { 
"    Valor invalido, tente colocar a entrada entre aspas duplas   
        var valor1 \"teste\"
".to_string() }

pub fn tipos_diferentes() -> String { 
"    Os valores possuem tipos diferentes
".to_string()}

pub fn nome_invalido() -> String { 
"    Nomes de funções são palavras reservadas e não podem ser utilizadas
".to_string()}

pub fn erro_inesperado() -> String {
"    Um erro inesperado/desconhecido aconteceu
".to_string()}


pub fn sintaxe_var_const_len() -> String {
"    A função 'Var'/'Const' recebem exatamente 2 parametros
        -- var [nome] [valor]
        -- const [nome] [valor]
".to_string()}


pub fn sintaxe_set_len() -> String {
"    A função 'Set' recebem exatamente 2 parametros
        -- set [valor] [nome]
".to_string()}

pub fn sintaxe_type_of_len() -> String {
"    A função 'type_of' recebem exatamente 1 parametro
        -- type_of [nome]
".to_string()}

pub fn sintaxe_rmv_len() -> String {
"    A função 'Rmv' recebe exatamente 1 parmetro 
        -- rmv [nome]
".to_string()}

pub fn sintaxe_aritmetica() -> String {
"    Funções de aritmetica recebem apenas numeros
        -- sum 40 23.10 2
        -- div 4.12 2.0 5
".to_string()}