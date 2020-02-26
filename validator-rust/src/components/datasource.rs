use std::collections::HashMap;
use crate::utilities::properties as property_utils;
use crate::utilities::properties::{Properties, NodeProperties};


use crate::proto;

use crate::components::{Component};

use crate::utilities::serial::{Value};



impl Component for proto::DataSource {
    // modify min, max, n, categories, is_public, non-null, etc. based on the arguments and component
    fn propagate_property(
        &self,
        _public_arguments: &HashMap<String, Value>,
        _properties: &property_utils::NodeProperties,
    ) -> Result<Properties, String> {
        Ok(Properties {
            nullity: true,
            releasable: false,
            nature: None,
            c_stability: vec![1.],
            num_columns: Some(1),
            num_records: vec![None]
        })
    }

    fn is_valid(
        &self,
        _public_arguments: &HashMap<String, Value>,
        _properties: &property_utils::NodeProperties,
    ) -> Result<(), String> {
        Ok(())
    }

    fn get_names(
        &self,
        _properties: &NodeProperties,
    ) -> Result<Vec<String>, String> {
        Err("get_names not implemented".to_string())
    }
}