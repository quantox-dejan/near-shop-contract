pub trait VectorUtils<T>
where
    T: Clone,
{
    fn intersect_with_ids<F1: Fn(&T) -> String, F2: Fn(&String, &String) -> bool>(
        &self,
        id_selector: F1,
        ids: &Vec<String>,
        validator: F2,
    ) -> Self;
}

impl<T> VectorUtils<T> for Vec<T>
where
    T: Clone,
{
    fn intersect_with_ids<F1: Fn(&T) -> String, F2: Fn(&String, &String) -> bool>(
        &self,
        id_selector: F1,
        ids: &Vec<String>,
        validator: F2,
    ) -> Self {
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
    #[test]
    fn intersect_with_ids_returns_correctly() {
        let product1 = crate::model::product::Product::new("Product1".to_string(), 0, 0);
        let product2 = crate::model::product::Product::new("Product2".to_string(), 0, 0);
        let product3 = crate::model::product::Product::new("Product2".to_string(), 0, 0);
        let products = vec![product1.clone(), product2.clone(), product3.clone()];

        let ids = vec![String::from(&product2.id), String::from(&product1.id)];
        let intersection = crate::utils::vector_utils::VectorUtils::intersect_with_ids(
            &products,
            |x| String::from(&x.id),
            &ids,
            |left, right| left == right,
        );
        assert_eq!(2, intersection.len());
    }
}
