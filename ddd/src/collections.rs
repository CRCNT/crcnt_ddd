pub fn merge_results<T: Clone, E: Clone>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
  let init: Vec<T> = vec![];
  xs.iter().fold(Ok(init), |acc, next| {
             let acc = acc.and_then(|mut xs| {
                            let next = next.clone();
                            next.map(|x| {
                                  xs.push(x.clone());
                                  xs
                                })
                          });
             acc
           })
}
