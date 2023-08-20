pub struct BinaryTree {
    v: i32,
    l: Option<Box<BinaryTree>>,
    r: Option<Box<BinaryTree>>,
}

impl<'a> BinaryTree {
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
}
