use super::*;

//#[whey_mock]
pub trait Terminal {
    fn write(&self, message: &str) -> bool;
}

    pub struct WheyTerminal<'a> {
        write_setup: curds_core_abstraction::whey::WheySetup<(&'a str), bool>,
    }
    impl<TProvider> curds_core_abstraction::dependency_injection::Injected<TProvider>
    for WheyTerminal {
        fn inject(provider: &TProvider) -> Self {
            Self::construct()
        }
    }
    impl WheyTerminal {
        pub fn construct() -> Self {
            Self {
                write_setup: std::default::Default::default(),
            }
        }
    }
    impl Terminal for WheyTerminal {
        fn write(&self, message: &str) -> bool {
            self.write_setup.consume((message))
        }
    }