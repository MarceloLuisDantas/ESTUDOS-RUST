struct Pedido {
    codigo: i64,
    quantidade: i64,
    preco: f32,
}

impl Pedido {
    fn get_total(&self) -> f32 {
        self.preco * (self.quantidade as f32)
    }
}

fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada");
    buffer
}

fn get_pedido() -> Pedido {
    let entrada = input();
    let valores: Vec<String> = entrada.split(" ")
                                      .map(| x | x.to_string())
                                      .collect();

    let codigo     = valores[0].trim().parse::<i64>().expect("Erro ao converter Codigo");
    let quantidade = valores[1].trim().parse::<i64>().expect("Erro ao converter a Quantidade");
    let preco      = valores[2].trim().parse::<f32>().expect("Erro ao converter o Valor");
    let pedido = Pedido { codigo, quantidade, preco, };

    pedido
}

fn main() {
    let pedido1: Pedido = get_pedido();
    let pedido2: Pedido = get_pedido();
    println!("VALOR A PAGAR: R$ {:.2}", pedido1.get_total() + pedido2.get_total());
}

