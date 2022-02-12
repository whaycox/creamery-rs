use super::*;

pub trait Terminal {
    
}

#[cfg(test)]
pub trait TerminalSetup {

}

#[cfg(test)]
pub struct WheyTerminalSetup {

}
#[cfg(test)]
impl TerminalSetup for WheyTerminalSetup {

}

#[injected]
#[cfg(test)]
pub struct WheyTerminal {}

#[cfg(test)]
impl Terminal for WheyTerminal {

}