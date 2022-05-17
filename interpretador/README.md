# Interpretador 

### Objetivos
Fazer o um interpretador o mais basico possivel. Sem uso de AST ou qualquer conceito similar.

### Funcionamento
Todo o codigo é dividido em 2 partes. Primeiro uma estrutura que representa a memoria e posui as operações de criação e modificação de valores. E a parte de Eval, que recebe o comando, e processa ele.

### Implementações
#### Funções
    Var [nome] [valor] - 
        Adicionar um valor variavel a memoria

    |> var pi 3.1415
    3.1415
    |> pi
    3.1415
    |> set 1.6180 pi
    1.6180
    |> pi
    1.6180
---
    Const [nome] [valor]
        Adiciona um valor constante a memoria
    
    |> const pi 3.1415
    3.1415
    |> pi
    3.1415
    |> set 1.6180 pi
    'pi' é uma constante e não pode ser alterada
---
    Rmv [nome]
        Remove um valor da memoria
    
    |> const pi 3.1415
    3.1415
    |> set 1.6180 pi
    'pi' é uma constante e não pode ser alterada
    |> rmv pi
    3.1415
    |> const pi 1.6180
    1.6180
---
    Set [valor] [nome]
        Seta um novo valor para uma variavel já existente
    
    |> var e 27.182
    27.182
    |> e
    27.182
    |> set 2.7182 e
    2.7182
    |> e
    2.7182
---
    Show
        Lista todos os valores registrados na memoria
    
    |> var e 2.7182
    2.7182
    |> var pi 3.1415
    3.1415
    |> show
    e = var 2.7182
    pi = var 3.1415
    2 valor(es) na memoria
---
    Cls
        Limpa a tela
---
    Type_of
        Indica o tipo de um valor
    
    |> var palavra teste
    teste
    |> var numero 5.12
    teste
    |> type_of palavra
    String
    |> type_of numero
    Float




### To Do
 - [x] Criação de variaveis F32
 - [x] Remoção de valores
 - [x] Criação das funções aritmetica (Sum, Sub, Mult, Div)
 - [x] Criação de tipos String, Float e Int
 - [x] Criação de mutaveis e imutaveis
 - [ ] Criação de do help
 - [ ] Criação Listas
 - [ ] Melhoria na aritmetica (+, -, *, /)

