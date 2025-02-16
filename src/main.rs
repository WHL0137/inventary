fn adicionar_item(inventario: &mut Vec<String>, item: &str) {
    inventario.push(item.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adicionar_item() {
        let mut inventario = Vec::new();
        adicionar_item(&mut inventario, "Caneta");
        assert_eq!(inventario.len(), 1);
        assert_eq!(inventario[0], "Caneta");
    }
}

fn main() {
    let mut inventario = Vec::new();
    adicionar_item(&mut inventario, "Lápis");
    println!("Inventário: {:?}", inventario);
}
