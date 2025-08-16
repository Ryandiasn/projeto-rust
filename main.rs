use std::collections::HashMap;

// Estrutura de um produto
#[derive(Debug, Clone)]
struct Product {
    name: String,
    category: String,
    brand: String,
}

// Estrutura da loja com índices de busca
struct Store {
    products: Vec<Product>,
    name_index: HashMap<String, Vec<usize>>,
    category_index: HashMap<String, Vec<usize>>,
    brand_index: HashMap<String, Vec<usize>>,
}

impl Store {
    fn new(products: Vec<Product>) -> Self {
        let mut store = Store {
            products,
            name_index: HashMap::new(),
            category_index: HashMap::new(),
            brand_index: HashMap::new(),
        };
        store.build_indices();
        store
    }

    fn build_indices(&mut self) {
        for (i, product) in self.products.iter().enumerate() {
            self.name_index
                .entry(product.name.clone())
                .or_insert(Vec::new())
                .push(i);
            self.category_index
                .entry(product.category.clone())
                .or_insert(Vec::new())
                .push(i);
            self.brand_index
                .entry(product.brand.clone())
                .or_insert(Vec::new())
                .push(i);
        }
    }

    // Função de busca corrigida - busca combinada (AND entre critérios)
    fn search(&self, query: &str, category: &str, brand: &str) -> Vec<Product> {
        let mut results = Vec::new();
        
        // Se todos os parâmetros estão vazios, retorna vazio
        if query.is_empty() && category.is_empty() && brand.is_empty() {
            return results;
        }

        // Percorre todos os produtos e verifica se atendem aos critérios
        for product in &self.products {
            let mut matches = true;
            
            // Verifica nome se query não estiver vazia
            if !query.is_empty() {
                matches = matches && product.name.contains(query);
            }
            
            // Verifica categoria se category não estiver vazia
            if !category.is_empty() {
                matches = matches && product.category == category;
            }
            
            // Verifica marca se brand não estiver vazia
            if !brand.is_empty() {
                matches = matches && product.brand == brand;
            }
            
            if matches {
                results.push(product.clone());
            }
        }
        
        results
    }

    // Busca otimizada usando os índices
    fn search_optimized(&self, query: &str, category: &str, brand: &str) -> Vec<Product> {
        let mut candidate_indices: Option<Vec<usize>> = None;

        // Buscar por nome usando índice
        if !query.is_empty() {
            if let Some(indices) = self.name_index.get(query) {
                candidate_indices = Some(indices.clone());
            } else {
                return Vec::new(); // Se nome específico não existe, retorna vazio
            }
        }

        // Filtrar por categoria
        if !category.is_empty() {
            if let Some(cat_indices) = self.category_index.get(category) {
                if let Some(ref mut candidates) = candidate_indices {
                    // Intersecção: manter apenas produtos que estão em ambas as listas
                    candidates.retain(|&index| cat_indices.contains(&index));
                } else {
                    candidate_indices = Some(cat_indices.clone());
                }
            } else {
                return Vec::new(); // Categoria não existe
            }
        }

        // Filtrar por marca
        if !brand.is_empty() {
            if let Some(brand_indices) = self.brand_index.get(brand) {
                if let Some(ref mut candidates) = candidate_indices {
                    // Intersecção: manter apenas produtos que estão em ambas as listas
                    candidates.retain(|&index| brand_indices.contains(&index));
                } else {
                    candidate_indices = Some(brand_indices.clone());
                }
            } else {
                return Vec::new(); // Marca não existe
            }
        }

        // Se não há candidatos, retorna vazio
        let indices = match candidate_indices {
            Some(indices) => indices,
            None => return Vec::new(),
        };

        // Converte índices em produtos
        indices
            .iter()
            .map(|&i| self.products[i].clone())
            .collect()
    }
}

fn main() {
    // Produtos de exemplo
    let products = vec![
        Product {
            name: "Laptop".to_string(),
            category: "Eletrônicos".to_string(),
            brand: "MarcaA".to_string(),
        },
        Product {
            name: "Smartphone".to_string(),
            category: "Eletrônicos".to_string(),
            brand: "MarcaB".to_string(),
        },
        Product {
            name: "Cadeira".to_string(),
            category: "Móveis".to_string(),
            brand: "MarcaC".to_string(),
        },
        Product {
            name: "Mesa".to_string(),
            category: "Móveis".to_string(),
            brand: "MarcaC".to_string(),
        },
    ];

    let store = Store::new(products);

    println!("=== EXEMPLOS DE BUSCA ===\n");

    // Exemplo 1: Busca por nome específico
    println!("1. Busca por nome 'Laptop':");
    let results = store.search("Laptop", "", "");
    for product in &results {
        println!("   {:?}", product);
    }
    println!("   Encontrados: {} produto(s)\n", results.len());

    // Exemplo 2: Busca por categoria
    println!("2. Busca por categoria 'Eletrônicos':");
    let results = store.search("", "Eletrônicos", "");
    for product in &results {
        println!("   {:?}", product);
    }
    println!("   Encontrados: {} produto(s)\n", results.len());

    // Exemplo 3: Busca por marca
    println!("3. Busca por marca 'MarcaC':");
    let results = store.search("", "", "MarcaC");
    for product in &results {
        println!("   {:?}", product);
    }
    println!("   Encontrados: {} produto(s)\n", results.len());

    // Exemplo 4: Busca combinada (nome + categoria + marca)
    println!("4. Busca combinada (Laptop + Eletrônicos + MarcaA):");
    let results = store.search("Laptop", "Eletrônicos", "MarcaA");
    for product in &results {
        println!("   {:?}", product);
    }
    println!("   Encontrados: {} produto(s)\n", results.len());

    // Exemplo 5: Busca que não encontra nada
    println!("5. Busca que não encontra (produto inexistente):");
    let results = store.search("Tablet", "", "");
    for product in &results {
        println!("   {:?}", product);
    }
    println!("   Encontrados: {} produto(s)\n", results.len());

    // Comparando busca normal vs otimizada
    println!("=== COMPARAÇÃO: BUSCA NORMAL vs OTIMIZADA ===\n");
    
    println!("Busca normal por categoria 'Móveis':");
    let results1 = store.search("", "Móveis", "");
    println!("   Encontrados: {} produto(s)", results1.len());
    
    println!("Busca otimizada por categoria 'Móveis':");
    let results2 = store.search_optimized("", "Móveis", "");
    println!("   Encontrados: {} produto(s)", results2.len());
}