use rand::Rng; //importar biblioteca para numeros aleatoreos

fn main() {
    println!("*QUICK SORT*");
    let mut vector = generate_random_vector(50, 1, 100);
    println!("Lista inicial: {:?}", vector);

    quick_sort(&mut vector);
    print!("Lista ordenada: {:?}", vector);
}

//PARTICIONAR VECTOR EN DOS CONJUNTOS
fn partition(vector: &[i64], pivot: i64) -> (Vec<i64>, Vec<i64>) {
    let mut less_than: Vec<i64> = vec![];
    let mut greater_than: Vec<i64> = vec![];

    for &x in vector {
        if x < pivot {
            less_than.push(x);
        } else if x > pivot {
            greater_than.push(x);
        }
    }
    (less_than, greater_than)
}

//QUICK SORT
fn quick_sort (vector: &mut Vec<i64>){
    if vector.len() <= 1 {
        return;
    }

    let pivot = choose_pivot(&vector);
    let (mut less_than, mut greater_than) = partition(vector, pivot);

    quick_sort(&mut less_than);
    quick_sort(&mut greater_than);

    vector.clear(); //remuevo todos lo valores del vector
    vector.extend(less_than); //agrego al vector la lista de menores que el pivote
    vector.push(pivot); //agrego al vector el pivote
    vector.extend(greater_than); //agredo al vector la lista de mayores que el pivote
}

//ELEGIR PIVOTE [ELIJO EL DEL MEDIO]
fn choose_pivot (vector: &[i64]) -> i64 {
    vector[vector.len() / 2]
}

//FABRICAR VECTOR ALEATORIO
fn generate_random_vector (size: usize, min: i64, max: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(min..=max)).collect()
}