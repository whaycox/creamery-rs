use std::marker::PhantomData;

use super::*;

pub trait ArgumentFactory {
    fn has_arguments(&self) -> bool;
    fn next(&self) -> String;
}

#[cfg(test)]
pub trait ArgumentFactorySetup {   
    fn has_arguments(&self, setup: Box<dyn Setup<(), bool>>);
    fn consume_has_arguments(&self) -> bool;
    fn next(&self, setup: Box<dyn Setup<(), String>>);
    fn consume_next(&self) -> String;
}
#[injected]
#[cfg(test)]
pub struct WheyArgumentFactorySetup<TDisambiguator: 'static> {
    #[defaulted]
    phantom: PhantomData<TDisambiguator>,
    #[defaulted]
    has_arguments_setups: Cell<Vec<Box<dyn Setup<(), bool>>>>,
    #[defaulted]
    next_setups: Cell<Vec<Box<dyn Setup<(), String>>>>,
}
#[cfg(test)]
impl<TDisambiguator> ArgumentFactorySetup for WheyArgumentFactorySetup<TDisambiguator> {
    fn has_arguments(&self, setup: Box<dyn Setup<(), bool>>) {
        let mut setups = self.has_arguments_setups.take();
        setups.insert(0, setup);
        self.has_arguments_setups.set(setups)
    }
    fn consume_has_arguments(&self) -> bool {
        let mut setups = self.has_arguments_setups.take();
        match setups.pop() {
            Some(setup) => {
                match setup.consume(()) {
                    Ok(mocked) => { 
                        if !setup.is_exhausted() {
                            setups.push(setup);
                        }
                        self.has_arguments_setups.set(setups);
                        mocked
                    },
                    Err(error) => {
                        setup.set_times(0);
                        for unconsumed_setup in setups {
                            unconsumed_setup.set_times(0)
                        }
                        match error {
                            SetupError::ExhaustedConsumption => panic!("has_arguments consumed an exhausted setup"),
                            SetupError::InputComparison => panic!("has_arguments was supplied an input that was not expected"),
                        }
                    }
                }
            },
            None => panic!("has_arguments has no setups"),
        }
    }

    fn next(&self, setup: Box<dyn Setup<(), String>>) {
        let mut setups = self.next_setups.take();
        setups.insert(0, setup);
        self.next_setups.set(setups)
    }
    fn consume_next(&self) -> String {
        let mut setups = self.next_setups.take();
        match setups.pop() {
            Some(setup) => {
                match setup.consume(()) {
                    Ok(mocked) => { 
                        if !setup.is_exhausted() {
                            setups.push(setup);
                        }
                        self.next_setups.set(setups);
                        mocked
                    },
                    Err(error) => {
                        setup.set_times(0);
                        for unconsumed_setup in setups {
                            unconsumed_setup.set_times(0)
                        }
                        match error {
                            SetupError::ExhaustedConsumption => panic!("next consumed an exhausted setup"),
                            SetupError::InputComparison => panic!("next was supplied an input that was not expected"),
                        }
                    }
                }
            },
            None => panic!("next has no setups"),
        }
    }
}

#[injected]
#[cfg(test)]
pub struct WheyArgumentFactory<TDisambiguator: 'static> {
    setups: Rc<WheyArgumentFactorySetup<TDisambiguator>>,
}
#[cfg(test)]
impl<TDisambiguator> ArgumentFactory for WheyArgumentFactory<TDisambiguator> {
    fn has_arguments(&self) -> bool { self.setups.consume_has_arguments() }
    fn next(&self) -> String { self.setups.consume_next() }
}