pub trait ArgumentFactory {
    fn args(&self) -> Vec<String>;
}

pub struct CronArgumentFactory;

impl ArgumentFactory for CronArgumentFactory {
    fn args(&self) -> Vec<String> {
        let mut arguments: Vec<String> = std::env::args().collect();
        arguments.remove(0);

        arguments
    }
}