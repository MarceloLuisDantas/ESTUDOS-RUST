use std::*;

fn input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    return buffer.trim().to_string();
}

struct Quadrado {
    lado: f32,
}
impl Quadrado {
    fn area(&self) -> f32 {
        self.lado * self.lado
    }
}

struct Retangulo {
    altura: f32,
    base: f32,
}
impl Retangulo {
    fn area(&self) -> f32 {
        (self.altura * self.base) / 2.0
    }
}

struct Triangulo {
    altura: f32,
    base: f32,
}
impl Triangulo {
    fn area(&self) -> f32 {
        (self.altura * self.base) / 2.0
    }
}

struct Circulo {
    raio: f32,
}
impl Circulo {
    fn area(&self) -> f32 {
        3.14159 * (self.raio * self.raio)
    }
}

struct Trapezio {
    base_menor: f32,
    base_maior: f32,
    altura: f32,
}
impl Trapezio {
    fn area(&self) -> f32 {
        ((self.base_maior + self.base_menor) * self.altura) / 2.0
    }
}

enum Formas {
    Quadrado(Quadrado),
    Retangulo(Retangulo),
    Circulo(Circulo),
    Triangulo(Triangulo),
    Trapezio(Trapezio),
}

impl Formas {
    fn area(&self) -> f32 {
        match self {
            Formas::Triangulo(triangulo) => triangulo.area(),
            Formas::Retangulo(retangulo) => retangulo.area(),
            Formas::Quadrado(quadrado) => quadrado.area(),
            Formas::Trapezio(trapezio) => trapezio.area(),
            Formas::Circulo(circulo) => circulo.area(),
        }
    }
}

fn main() { 
    let buff = input();
    let valores = buff.split(" ")
                      .map( |x| x.parse::<f32>().unwrap() )
                      .collect::<Vec<f32>>();

    let a = valores[0];
    let b = valores[1];
    let c = valores[2];
    let triangulo = Formas::Triangulo(Triangulo{altura: a, base: c});
    let cirulo = Formas::Circulo(Circulo{raio: c});
    let trapezio = Formas::Trapezio(Trapezio{base_maior: a, base_menor: b, altura: c});
    let quadrado = Formas::Quadrado(Quadrado{lado: b});
    let retangulo = Formas::Retangulo(Retangulo{altura:a, base: b});

    println!("TRIANGULO: {:.3}", triangulo.area());
    println!("CIRCULO: {:.3}", cirulo.area());
    println!("TRAPEZIO: {:.3}", trapezio.area());
    println!("QUADRADO: {:.3}", quadrado.area());
    println!("RETANGULO: {:.3}", retangulo.area());

}


