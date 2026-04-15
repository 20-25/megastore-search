use megastore_search::{Product, SearchEngine, SearchFilters};

fn sample_engine() -> SearchEngine {
    let mut engine = SearchEngine::new();

    engine.add_product(Product {
        id: 1,
        name: "Notebook Gamer Nitro".to_string(),
        brand: "Acer".to_string(),
        category: "Eletronicos".to_string(),
        price: 4299.90,
    });

    engine.add_product(Product {
        id: 2,
        name: "Smartphone Galaxy".to_string(),
        brand: "Samsung".to_string(),
        category: "Eletronicos".to_string(),
        price: 2599.90,
    });

    engine.add_product(Product {
        id: 3,
        name: "Camiseta Basica Algodao".to_string(),
        brand: "MegaFashion".to_string(),
        category: "Vestuario".to_string(),
        price: 59.90,
    });

    engine
}

#[test]
fn should_index_products() {
    let engine = sample_engine();
    assert_eq!(engine.total_products(), 3);
}

#[test]
fn should_search_by_name() {
    let engine = sample_engine();
    let results = engine.search("notebook", SearchFilters::default());

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].brand, "Acer");
}

#[test]
fn should_filter_by_category() {
    let engine = sample_engine();
    let results = engine.search(
        "galaxy",
        SearchFilters {
            category: Some("Eletronicos".to_string()),
            ..Default::default()
        },
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Smartphone Galaxy");
}

#[test]
fn should_filter_by_price() {
    let engine = sample_engine();
    let results = engine.search(
        "",
        SearchFilters {
            max_price: Some(100.0),
            ..Default::default()
        },
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].category, "Vestuario");
}
