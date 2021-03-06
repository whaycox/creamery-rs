#[cfg(test)]
mod tests {
    use super::super::*;
    use std::collections::HashMap;

    trait Repository<TKey, TValue> {
        fn store(&mut self, item: TValue) -> TKey;
        fn retrieve(&self, key: TKey) -> TValue;
    }

    #[injected]
    struct GenericRepository<TValue> {
        #[defaulted]
        seed: u32,
        #[defaulted]
        store: HashMap<u32, TValue>,
    }

    impl<TValue> Repository<u32, TValue> for GenericRepository<TValue>
    where TValue : Copy {
        fn store(&mut self, item: TValue) -> u32 {
            let key = self.seed;
            self.store.insert(key, item);
            self.seed = key + 1;
            key
        }

        fn retrieve(&self, key: u32) -> TValue {
            let value = self.store[&key];
            value
        }
    }

    #[service_provider]
    #[generates(dyn Repository<u32, bool> ~ GenericRepository<bool>)]
    #[generates(dyn Repository<u32, u32> ~ GenericRepository<u32>)]
    struct RepositoryProvider {}

    #[test]
    fn generates_generic_struct() {
        let provider = RepositoryProvider::construct();

        test_bool_repository(provider.generate());
        test_u32_repository(provider.generate());
    }
    fn test_bool_repository(mut repo: Box<dyn Repository<u32, bool>>) {
        let first_key = repo.store(true);
        let second_key = repo.store(false);
        
        assert_eq!(true, repo.retrieve(first_key));
        assert_eq!(false, repo.retrieve(second_key));
    }
    fn test_u32_repository(mut repo: Box<dyn Repository<u32, u32>>) {
        let first_key = repo.store(10);
        let second_key = repo.store(400);

        assert_eq!(10, repo.retrieve(first_key));
        assert_eq!(400, repo.retrieve(second_key));
    }
}
