🛒 Lista de Compras em Rust

Este é um projeto simples em Rust que calcula o valor total de uma lista de compras, verifica se há itens com valores inválidos e aplica um desconto caso o total seja maior que R$ 30,00.

O objetivo do código é praticar:

Variáveis

Arrays

Loops (for)

Condicionais (if/else)

Tipagem estática do Rust

Saída formatada (println!)

## 📋 O que o programa faz

1. Define uma lista fixa de itens e seus preços

2. Valida se existe algum preço negativo

3. Soma todos os preços

4.  um desconto de R$ 5,00 caso o total seja maior que 30

5. Exibe tudo de forma organizada no final

## 🧠 Lógica do desconto

Se o total for maior que 30, o programa aplica:

desconto = 5.0

desconto_p = true

Caso contrário:

desconto = 0.0

desconto_p = false

## ▶️ Como executar

Certifique-se de ter o Rust instalado:

rustc --version


Compile o programa:

rustc main.rs


Execute:

./main

## 🧾 Código usado

fn main() {
    
    // lista de itens
    let itens: [&str; 5] = ["feijão", "Arroz", "Repolho", "Morango", "Maçã"];

    // preços
    let precos: [f64; 5] = [26.99, 23.99, 15.87, 8.67, 2.00];

    // validaçao 
    for p in precos {
        if p < 0.00 {
            println!("erro: valores invalidos");
            break;
        }
    }

    // calcula o total
    let mut total: f64 = 0.00;
    for i in precos {
        total +=  i;
    }

    // aplicar desconto se for maior que 30.00 reais
    let mut desconto: f64 = 0.00;
    let mut desconto_p: bool = false;
    if total > 30.00 {
        println!("desconto aplicado!");
        desconto = 5.00;
        desconto_p = true;
    } else {
        println!("sem desconto aplicado!");
    }

    let valor_total: f64 = total - desconto;

    // saida formatada
    println!("=== lista de compras ===");
    for i in 0..itens.len() {
        println!("{} R$ {}", itens[i], precos[i]);
    }

    println!("valor total: {}", valor_total);
    println!("desconto aplicado: {}", desconto_p);
    if desconto_p == true {
        println!("valor do desconto: {}", desconto);
    }
    println!("valor final: {}", valor_total);
}

## 🎯 Objetivo do projeto

Este projeto faz parte da minha jornada de aprendizado em Rust.
Futuramente pretendo transformar isso em um CLI interativo, aceitando entrada do usuário e adicionando mais funcionalidades.
