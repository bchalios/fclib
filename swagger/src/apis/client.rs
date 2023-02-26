use std::rc::Rc;

use hyper;

use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
    configuration: Rc<Configuration<C>>,
    default_api: Box<dyn (::apis::DefaultApi)>,
}

impl<C: hyper::client::Connect> APIClient<C> {
    pub fn new(configuration: Configuration<C>) -> APIClient<C> {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            default_api: Box::new(::apis::DefaultApiClient::new(rc)),
        }
    }

    pub fn default_api(&self) -> &dyn (::apis::DefaultApi) {
        self.default_api.as_ref()
    }
}
