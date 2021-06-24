pub fn flat_to_triples<T: Copy>(a: &[T]) -> Vec<[T; 3]> {
    if a.len() % 3 != 0 {
        panic!("No triples!!!!")
    }
    let n_trips = (a.len() / 3) as usize;
    let mut out = Vec::with_capacity(n_trips);
    for mut i in 0..n_trips {
        i *= 3;
        out.push([a[i], a[i + 1], a[i + 2]])
    }
    out
}
pub fn flat_to_doubles<T: Copy>(a: &[T]) -> Vec<[T; 2]> {
    if a.len() % 2 != 0 {
        panic!("No doubles!!!!")
    }
    let n_trips = (a.len() / 2) as usize;
    let mut out = Vec::with_capacity(n_trips);
    for mut i in 0..n_trips {
        i *= 2;
        out.push([a[i], a[i + 1]])
    }
    out
}
