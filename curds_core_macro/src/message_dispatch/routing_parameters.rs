use super::*;

#[derive(Clone, Debug)]
pub struct RoutingParameters {
    pub message_type: Type,
    pub context_type: Type,
    pub response_type: Option<Type>,
}