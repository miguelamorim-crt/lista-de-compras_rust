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
                desconto = 0.00;
                desconto_p = false;
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
