use super::*;

pub struct WheySetup<TInput, TMocked> {
    calls: Cell<u32>,
    comparisons: Cell<Vec<CompareExpectation<TInput>>>,
    generators: Cell<Vec<GenerateExpectation<TMocked>>>,
}
impl<TInput, TMocked> Default for WheySetup<TInput, TMocked> {
    fn default() -> Self {
        Self { 
            calls: Default::default(), 
            comparisons: Default::default(), 
            generators: Default::default() 
        }
    }
}

impl<TInput, TMocked> WheySetup<TInput, TMocked> {
    pub fn consume(&self, input: TInput) -> TMocked {
        if !self.perform_compare(input) {
            panic!("Input was not expected");
        }
        let generated = self.perform_generate();
        self.calls.set(self.calls.get() + 1);
        generated
    }
    fn perform_compare(&self, input: TInput) -> bool {
        let mut comparisons = self.comparisons.take();
        if comparisons.len() == 0 {
            panic!("There are no more input expectations");
        }
        let comparer = comparisons.pop().unwrap();
        let comparison = comparer.consume(input);
        if !comparer.is_exhausted() {
            comparisons.push(comparer);
        }
        self.comparisons.set(comparisons);
        comparison
    }
    fn perform_generate(&self) -> TMocked {
        let mut generators = self.generators.take();
        if generators.len() == 0 {
            panic!("There are no more generate expectations");
        }
        let generator = generators.pop().unwrap();
        let generated = generator.consume();
        if !generator.is_exhausted() {
            generators.push(generator);
        }
        self.generators.set(generators);
        generated
    }
}