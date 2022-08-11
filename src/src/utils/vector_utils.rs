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