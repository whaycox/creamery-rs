use super::*;

pub struct DispatchDefaults {
    message: SerialTemplate,
    request: SerialTemplate,
    chain: ParallelTemplate,
}

impl DispatchDefaults {
    pub fn new(message: SerialTemplate, request: SerialTemplate, chain: ParallelTemplate) -> Self {
        Self {
            message: message,
            request: request,
            chain: chain,
        }
    }

    pub fn expand_pipeline(&self, parameters: RoutingParameters) -> SerialRoute {
        if parameters.response_type.is_some() {
            self.request.expand_with(parameters)
        }
        else {
            self.message.expand_with(parameters)
        }
    }

    pub fn expand_chain(&self) -> ParallelRoute {
        self.chain.expand()
    }
}