use megastore_search::{Product, SearchIndex, SearchQuery};

fn main() {
    let mut index = SearchIndex::new();

    index.add_product(Product {
        id: 1,
        name: "Mouse Gamer RGB".to_string(),
        brand: "HyperTech".to_string(),
        category: "Periféricos".to_string(),
        description: "Mouse com sensor preciso e iluminação RGB".to_string(),
        price: 129.90,
    });

    index.add_product(Product {
        id: 2,
        name: "Teclado Mecânico Pro".to_string(),
        brand: "HyperTech".to_string(),
        category: "Periféricos".to_string(),
        description: "Teclado mecânico com switches táteis".to_string(),
        price: 299.90,
    });

    index.add_product(Product {
        id: 3,
        name: "Headset Surround X".to_string(),
        brand: "SoundMax".to_string(),
        category: "Áudio".to_string(),
        description: "Headset gamer com som imersivo".to_string(),
        price: 249.90,
    });

    index.add_co_purchase(1, 2).unwrap();
    index.add_co_purchase(1, 2).unwrap();
    index.add_co_purchase(1, 3).unwrap();

    let query = SearchQuery {
        text: "mouse gamer".to_string(),
        brand: Some("HyperTech".to_string()),
        category: Some("Periféricos".to_string()),
        limit: 5,
    };

    let results = index.search(&query);
    println!("Resultados da busca:");
    for product in results {
        println!(
            "- {} | {} | {} | R$ {:.2}",
            product.name, product.brand, product.category, product.price
        );
    }

    let recommendations = index.recommend(1, 3);
    println!("\nRecomendações para o produto 1:");
    for product in recommendations {
        println!("- {}", product.name);
    }
}
