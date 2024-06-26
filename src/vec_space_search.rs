use std::collections::HashMap;

pub struct VecSpaceSearchEngin {
    pub doc: Vec<String>,
    pub src: String,
    pub concordances: HashMap<usize, f64>,
}

impl VecSpaceSearchEngin {
    pub fn new() -> Self {
        Self {
            doc: vec![],
            src: String::new(),
            concordances: HashMap::new(),
        }
    }
    // preprocess
    pub fn init(self, doc: Vec<String>, src: String) -> Self {
        let mut hp = HashMap::new();
        for (idx, s) in doc.iter().enumerate() {
            let r = self.relation(src.clone(), s.to_string());
            hp.insert(idx, r);
        }
        Self {
            doc,
            src,
            concordances: hp,
        }
    }

    pub fn module(&self, c: &HashMap<String, usize>) -> f64 {
        let mut total = 0;
        for (_, v) in c {
            total += v.pow(2);
        }
        (total as f64).sqrt()
    }

    // relation
    pub fn relation(&self, s1: String, s2: String) -> f64 {
        let c1 = self.concordance(s1);
        let c2 = self.concordance(s2);

        let mut products = 0;
        for (k, v) in &c1 {
            if let Some(v2) = c2.get(k) {
                products += v * v2;
            }
        }

        if self.module(&c1) * self.module(&c2) != 0 as f64 {
            return products as f64 / (self.module(&c1) * self.module(&c2));
        } else {
            return 0.0;
        }
    }

    pub fn concordance(&self, s: String) -> HashMap<String, usize> {
        let mut hp = HashMap::new();
        for word in s.split(" ") {
            if let Some(v) = hp.get_mut(word) {
                *v += 1;
            } else {
                hp.insert(word.to_string(), 1);
            }
        }
        return hp;
    }

    pub fn sorted_concordances(&self) -> Vec<(usize, f64)> {
        let mut vec = Vec::new();
        for (k, v) in &self.concordances {
            vec.push((*k, *v));
        }
        vec.sort_by(|a, b| b.1.total_cmp(&a.1));
        return vec;
    }

    pub fn print(&self) {
        let res_sorted = self.sorted_concordances();
        for i in &res_sorted[0..2] {
            println!("{} :    {}", i.1, &self.doc[i.0][0..100])
        }
    }
}
