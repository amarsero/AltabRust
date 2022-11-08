
pub struct ResultList {
    pub head: std::sync::atomic::AtomicPtr<Node>,
}

struct Node {
    next: Option<&ResultEntry>,
    prev: Option<&ResultEntry>,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

pub impl ResultList {
    fn add(&self, entry: &ResultEntry) {
        let node = Box::new(Node {
            next: Some(entry),
            prev: None,
        });
        let head = self.head.get_mut();
        self.head.compare_exchange(head,node, success, failure)
    }
}