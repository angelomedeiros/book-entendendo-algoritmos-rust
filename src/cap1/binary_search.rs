pub fn binary_search(list: &Vec<i8>, item: i8) -> Option<usize> {
    let mut baixo = 0;
    let mut alto = list.len();

    while baixo <= alto {
        let meio = (baixo + alto) / 2;
        let chute = list[meio];

        if chute == item {
            return Some(meio);
        } else if chute > item {
            alto = meio - 1;
        } else {
            baixo = meio + 1;
        }
    }

    return None;
}

#[test]
fn test_binary_search() {
    let list = vec![1, 3, 4, 5, 7, 8, 11, 13, 15, 18, 32];
    let index = binary_search(&list, 1).unwrap();

    println!("Index: {}", index);
}

#[test]
fn test_binary_search_with_match() {
    let list = vec![1, 3, 4, 5, 7, 8, 11, 13, 15, 18, 32];
    let index = binary_search(&list, 6);

    match index {
        Some(value) => println!("Indexx: {}", value),
        None => println!("O elemento não está na lista"),
    }
}
