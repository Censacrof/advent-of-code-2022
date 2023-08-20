pub struct BinaryTree {
    v: i32,
    l: Option<Box<BinaryTree>>,
    r: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    pub fn new(v: i32) -> Self {
        Self {
            v,
            l: None,
            r: None,
        }
    }

    pub fn binary_insert_desc(&mut self, v: i32) {
        if v > self.v {
            match &mut self.l {
                Some(l) => l.binary_insert_desc(v),
                None => self.l = Some(Box::new(BinaryTree::new(v))),
            };

            return;
        }

        match &mut self.r {
            Some(r) => r.binary_insert_desc(v),
            None => self.r = Some(Box::new(BinaryTree::new(v))),
        };
    }

    pub fn traverse(&self) -> Vec<i32> {
        let mut res = Vec::<i32>::new();
        self._traverse_inner(&mut res);
        return res;
    }

    fn _traverse_inner(&self, res: &mut Vec<i32>) {
        if let Some(l) = &self.l {
            l._traverse_inner(res)
        }

        res.push(self.v.clone());

        if let Some(r) = &self.r {
            r._traverse_inner(res)
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinaryTree;

    #[test]
    fn it_binary_inserts_correctly() {
        let mut root = BinaryTree::new(3);
        assert_eq!(root.v, 3);

        root.binary_insert_desc(1);
        assert_eq!(root.r.as_mut().unwrap().v, 1);

        root.binary_insert_desc(5);
        assert_eq!(root.l.as_mut().unwrap().v, 5);

        root.binary_insert_desc(4);
        assert_eq!(root.l.as_mut().unwrap().r.as_mut().unwrap().v, 4);

        root.binary_insert_desc(7);
        assert_eq!(root.l.as_mut().unwrap().l.as_mut().unwrap().v, 7);
    }

    #[test]
    fn it_traverses_correctly() {
        let mut root = BinaryTree::new(3);
        root.binary_insert_desc(1);
        root.binary_insert_desc(5);
        root.binary_insert_desc(4);
        root.binary_insert_desc(7);

        let traversed = root.traverse();

        assert_eq!(traversed.get(0).unwrap().to_owned(), 7);
        assert_eq!(traversed.get(1).unwrap().to_owned(), 5);
        assert_eq!(traversed.get(2).unwrap().to_owned(), 4);
        assert_eq!(traversed.get(3).unwrap().to_owned(), 3);
        assert_eq!(traversed.get(4).unwrap().to_owned(), 1);
    }
}
