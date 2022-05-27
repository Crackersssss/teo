use std::fmt::Debug;
use async_trait::async_trait;
use serde_json::{Map, Value as JsonValue};
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::error::ActionError;


#[async_trait]
pub(crate) trait Connector: Debug + Send + Sync {

    async fn save_object(&self, object: &Object) -> Result<(), ActionError>;

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError>;

    async fn find_unique(&self, graph: &'static Graph, model: &'static Model, finder: &JsonValue) -> Result<Object, ActionError>;

    async fn find_first(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError>;

    async fn find_many(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Vec<Object>, ActionError>;

    async fn count(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<usize, ActionError>;

}

#[async_trait]
pub(crate) trait ConnectorBuilder: Debug + Send + Sync {

    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector>;
}
