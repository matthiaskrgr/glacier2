#![feature(generic_const_exprs)]

pub async fn something(path: &[[usize; N_ISLANDS]; N_ISLANDS]) -> usize {
    async {
        match path {
            [] => 0,

            _ => 1,
        }
    }
    .await
}
