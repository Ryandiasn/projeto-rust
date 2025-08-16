# Sistema de Busca MegaStore 🏪

Sistema simples de busca para catálogo de produtos usando HashMap em Rust.

## 📝 Descrição

Implementação básica de um sistema de busca otimizado para o catálogo da MegaStore, utilizando estruturas de dados HashMap para indexação eficiente de produtos por nome, categoria e marca.

## 🚀 Como Executar

```bash
# Clonar o repositório
git clone <url-do-repositorio>
cd megastore-search

# Executar o programa
cargo run

# Executar testes (se houver)
cargo test
```

## 🎯 Funcionalidades

### Tipos de Busca:
- **Por Nome**: Encontra produtos pelo nome exato
- **Por Categoria**: Filtra produtos por categoria
- **Por Marca**: Filtra produtos por marca específica
- **Busca Combinada**: Combina múltiplos critérios (AND)

### Exemplo de Uso:
```rust
let store = Store::new(products);

// Busca por nome
let results = store.search("Laptop", "", "");

// Busca por categoria
let results = store.search("", "Eletrônicos", "");

// Busca combinada
let results = store.search("Laptop", "Eletrônicos", "MarcaA");
```

## 🏗️ Estrutura do Código

### Product
```rust
struct Product {
    name: String,
    category: String,
    brand: String,
}
```

### Store
```rust
struct Store {
    products: Vec<Product>,                    // Lista de produtos
    name_index: HashMap<String, Vec<usize>>,   // Índice por nome
    category_index: HashMap<String, Vec<usize>>, // Índice por categoria
    brand_index: HashMap<String, Vec<usize>>,  // Índice por marca
}
```

## ⚡ Algoritmos e Estruturas de Dados

### HashMap (Tabela Hash)
- **Complexidade**: O(1) para busca
- **Uso**: Indexação de produtos por diferentes critérios
- **Vantagem**: Acesso rápido aos dados

### Indexação
```rust
fn build_indices(&mut self) {
    for (i, product) in self.products.iter().enumerate() {
        // Mapeia nome -> lista de índices dos produtos
        self.name_index
            .entry(product.name.clone())
            .or_insert(Vec::new())
            .push(i);
        // ... mesmo para categoria e marca
    }
}
```

### Busca Combinada
```rust
fn search(&self, query: &str, category: &str, brand: &str) -> Vec<Product> {
    // Aplica filtros AND: produto deve atender TODOS os critérios não-vazios
    for product in &self.products {
        let mut matches = true;
        
        if !query.is_empty() {
            matches = matches && product.name.contains(query);
        }
        // ... verifica outros critérios
    }
}
```

## 📊 Performance

### Busca Normal
- **Complexidade**: O(n) - percorre todos os produtos
- **Uso**: Busca flexível com `contains()`

### Busca Otimizada  
- **Complexidade**: O(1) + O(k) onde k = produtos encontrados
- **Uso**: Busca exata usando índices HashMap

## 📁 Estrutura do Projeto

```
megastore-search/
├── src/
│   └── main.rs          # Código principal
├── Cargo.toml          # Configuração do projeto
└── README.md           # Este arquivo
```

## 🧪 Exemplos de Execução

O programa inclui exemplos que demonstram:

1. **Busca por nome específico**
   ```
   Busca: "Laptop"
   Resultado: [Produto com nome "Laptop"]
   ```

2. **Busca por categoria**
   ```
   Busca: categoria="Eletrônicos"
   Resultado: [Laptop, Smartphone]
   ```

3. **Busca por marca**
   ```
   Busca: marca="MarcaC"
   Resultado: [Cadeira, Mesa]
   ```

4. **Busca combinada**
   ```
   Busca: "Laptop" + "Eletrônicos" + "MarcaA"
   Resultado: [Laptop da MarcaA na categoria Eletrônicos]
   ```

5. **Busca sem resultados**
   ```
   Busca: "Tablet" (produto inexistente)
   Resultado: []
   ```

## 🎓 Conceitos Implementados

### Estruturas de Dados
- ✅ **HashMap**: Estrutura principal para indexação O(1)
- ✅ **Vec**: Armazenamento sequencial dos produtos
- ✅ **String**: Manipulação eficiente de texto

### Algoritmos
- ✅ **Indexação por Hash**: Criação de índices múltiplos
- ✅ **Busca Linear**: Percorre produtos aplicando filtros
- ✅ **Busca por Índice**: Usa HashMap para acesso direto
- ✅ **Intersecção de Conjuntos**: Para busca combinada otimizada

### Programação em Rust
- ✅ **Ownership**: Gerenciamento seguro de memória
- ✅ **Borrowing**: Referências imutáveis (`&self`)
- ✅ **Pattern Matching**: `if let` e `match`
- ✅ **Iteradores**: `.iter()`, `.enumerate()`, `.map()`

## 🔧 Configuração (Cargo.toml)

```toml
[package]
name = "megastore-search"
version = "0.1.0"
edition = "2021"

[dependencies]
# Nenhuma dependência externa - usa apenas std
```


## 📈 Vantagens da Implementação

### Eficiência
- **HashMap O(1)**: Busca rápida por chave
- **Índices múltiplos**: Acesso otimizado por diferentes critérios
- **Memória controlada**: Rust garante segurança sem garbage collector

### Simplicidade
- **Código limpo**: Estrutura clara e fácil de entender
- **Sem dependências**: Usa apenas biblioteca padrão
- **Exemplos práticos**: Demonstrações funcionais incluídas

### Escalabilidade
- **Estrutura flexível**: Fácil de estender com novos campos
- **Índices expansíveis**: Novos tipos de busca podem ser adicionados
- **Performance consistente**: HashMap mantém O(1) mesmo com muitos dados

## 📄 Licença

Projeto desenvolvido para fins educacionais - Estudo de Caso MegaStore.

---
**Desenvolvido em Rust 🦀**
