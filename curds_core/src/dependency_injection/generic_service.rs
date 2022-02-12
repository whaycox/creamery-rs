#[cfg(test)]
mod tests {
    use super::super::*;
    use std::{collections::HashMap, cell};

    trait Repository<TKey, TValue> {
        fn store(&self, item: TValue) -> TKey;
        fn retrieve(&self, key: TKey) -> TValue;
    }

    #[injected]
    struct GenericRepository<TValue> {
        #[defaulted]
        seed: Cell<u32>,
        #[defaulted]
        store: Cell<HashMap<u32, TValue>>,
    }

    impl<TValue> Repository<u32, TValue> for GenericRepository<TValue>
    where TValue : Copy {
        fn store(&self, item: TValue) -> u32 {
            let key = self.seed.get();
            let mut store = self.store.take();
            store.insert(key, item);
            self.store.set(store);
            self.seed.set(key + 1);
            key
        }

        fn retrieve(&self, key: u32) -> TValue {
            let store = self.store.take();
            let value = *store.get(&key).unwrap();
            self.store.set(store);
            value
        }
    }

    #[service_provider]
    #[generates_singleton(dyn Repository<u32, bool> ~ GenericRepository<bool>)]
    #[generates_singleton(dyn Repository<u32, u32> ~ GenericRepository<u32>)]
    struct RepositoryProvider {}

    #[test]
    fn generates_generic_struct() {
        let provider = RepositoryProvider::construct();

        test_bool_repository(provider.generate());
        test_u32_repository(provider.generate());
    }
    fn test_bool_repository(repo: Rc<dyn Repository<u32, bool>>) {
        let first_key = repo.store(true);
        let second_key = repo.store(false);

        assert_eq!(true, repo.retrieve(first_key));
        assert_eq!(false, repo.retrieve(second_key));
    }
    fn test_u32_repository(repo: Rc<dyn Repository<u32, u32>>) {
        let first_key = repo.store(10);
        let second_key = repo.store(400);

        assert_eq!(10, repo.retrieve(first_key));
        assert_eq!(400, repo.retrieve(second_key));
    }
}
