pub trait VectorUtils<T>
where T: Clone {
    fn intersect_with_ids<F1: Fn(&T) -> String, F2: Fn(&String, &String) -> bool>(&self, id_selector: F1, ids: &Vec<String>, validator: F2) -> Self;
}

impl<T> VectorUtils<T> for Vec<T>
where T : Clone {
    fn intersect_with_ids<F1: Fn(&T) -> String, F2: Fn(&String, &String) -> bool>(&self, id_selector: F1, ids: &Vec<String>, validator: F2) -> Self {
        let mut out: Self = vec![];
        let length = ids.len();
        for item in self.iter() {
            for i in 0..length {
                let source_id = id_selector(item);
                if validator(&source_id, &ids[i]) {
                    out.push(item.clone());
                    break;
                }
            }
        }

        out
    }
}

mod tests {
    use crate::model::product::Product;
    use super::VectorUtils;

    #[test]
    fn intersect_with_ids_returns_correctly() {
        let product1 = Product::new("Product1".to_string(), 0.00);
        let product2 = Product::new("Product2".to_string(), 0.00);
        let product3 = Product::new("Product2".to_string(), 0.00);
        let products = vec![product1.clone(), product2.clone(), product3.clone()];

        let ids = vec![String::from(&product2.id), String::from(&product1.id)];
        let intersection = products.intersect_with_ids(|x| String::from(&x.id), &ids, |left, right| left == right);
        assert_eq!(2, intersection.len());
    }
}