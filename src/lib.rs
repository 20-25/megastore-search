use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub description: String,
    pub price: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    pub text: String,
    pub brand: Option<String>,
    pub category: Option<String>,
    pub limit: usize,
}

#[derive(Debug, Default)]
pub struct SearchIndex {
    products: HashMap<u32, Product>,
    token_index: HashMap<String, HashSet<u32>>,
    brand_index: HashMap<String, HashSet<u32>>,
    category_index: HashMap<String, HashSet<u32>>,
    graph: HashMap<u32, HashMap<u32, usize>>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_product(&mut self, product: Product) {
        let id = product.id;

        for token in tokenize(&format!("{} {}", product.name, product.description)) {
            self.token_index.entry(token).or_default().insert(id);
        }

        self.brand_index
            .entry(normalize(&product.brand))
            .or_default()
            .insert(id);

        self.category_index
            .entry(normalize(&product.category))
            .or_default()
            .insert(id);

        self.products.insert(id, product);
    }

    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn add_co_purchase(&mut self, a: u32, b: u32) -> Result<(), String> {
        if !self.products.contains_key(&a) || !self.products.contains_key(&b) {
            return Err("Um ou mais produtos informados não existem no catálogo.".to_string());
        }

        if a == b {
            return Ok(());
        }

        *self.graph.entry(a).or_default().entry(b).or_insert(0) += 1;
        *self.graph.entry(b).or_default().entry(a).or_insert(0) += 1;
        Ok(())
    }

    pub fn search(&self, query: &SearchQuery) -> Vec<Product> {
        let tokens = tokenize(&query.text);
        let mut scores: HashMap<u32, usize> = HashMap::new();

        if tokens.is_empty() {
            for id in self.products.keys() {
                scores.insert(*id, 0);
            }
        } else {
            for token in &tokens {
                if let Some(product_ids) = self.token_index.get(token) {
                    for id in product_ids {
                        *scores.entry(*id).or_insert(0) += 1;
                    }
                }
            }
        }

        let brand_filter = query
            .brand
            .as_ref()
            .and_then(|brand| self.brand_index.get(&normalize(brand)));

        let category_filter = query
            .category
            .as_ref()
            .and_then(|category| self.category_index.get(&normalize(category)));

        let mut results: Vec<(u32, usize)> = scores
            .into_iter()
            .filter(|(id, _)| {
                brand_filter.is_none_or(|allowed| allowed.contains(id))
                    && category_filter.is_none_or(|allowed| allowed.contains(id))
            })
            .collect();

        results.sort_by_key(|(id, score)| (Reverse(*score), *id));

        results
            .into_iter()
            .take(query.limit.max(1))
            .filter_map(|(id, _)| self.products.get(&id).cloned())
            .collect()
    }

    pub fn recommend(&self, product_id: u32, limit: usize) -> Vec<Product> {
        let mut neighbors: Vec<(u32, usize)> = self
            .graph
            .get(&product_id)
            .into_iter()
            .flat_map(|adjacent| adjacent.iter().map(|(id, weight)| (*id, *weight)))
            .collect();

        neighbors.sort_by_key(|(id, weight)| (Reverse(*weight), *id));

        neighbors
            .into_iter()
            .take(limit.max(1))
            .filter_map(|(id, _)| self.products.get(&id).cloned())
            .collect()
    }
}

fn normalize(input: &str) -> String {
    input.trim().to_lowercase()
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .split(|c: char| !c.is_alphanumeric())
        .filter(|token| !token.trim().is_empty())
        .map(normalize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_and_finds_product_by_text() {
        let mut index = SearchIndex::new();
        index.add_product(Product {
            id: 1,
            name: "Mouse Gamer RGB".to_string(),
            brand: "HyperTech".to_string(),
            category: "Periféricos".to_string(),
            description: "Mouse com sensor preciso e iluminação RGB".to_string(),
            price: 129.90,
        });

        let results = index.search(&SearchQuery {
            text: "mouse rgb".to_string(),
            brand: None,
            category: None,
            limit: 10,
        });

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 1);
    }
}
