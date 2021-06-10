/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `create_custom_resource_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCustomResourceDefinitionError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_collection_custom_resource_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCollectionCustomResourceDefinitionError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_custom_resource_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCustomResourceDefinitionError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_api_resources`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApiResourcesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_custom_resource_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCustomResourceDefinitionError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_custom_resource_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchCustomResourceDefinitionError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_custom_resource_definition_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchCustomResourceDefinitionStatusError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `read_custom_resource_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadCustomResourceDefinitionError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `read_custom_resource_definition_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadCustomResourceDefinitionStatusError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_custom_resource_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceCustomResourceDefinitionError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_custom_resource_definition_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceCustomResourceDefinitionStatusError {
    Status401(),
    UnknownValue(serde_json::Value),
}


/// create a CustomResourceDefinition
pub async fn create_custom_resource_definition(configuration: &configuration::Configuration, body: crate::models::V1CustomResourceDefinition, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::V1CustomResourceDefinition, Error<CreateCustomResourceDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("dryRun", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_manager {
        local_var_req_builder = local_var_req_builder.query(&[("fieldManager", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCustomResourceDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// delete collection of CustomResourceDefinition
pub async fn delete_collection_custom_resource_definition(configuration: &configuration::Configuration, pretty: Option<&str>, _continue: Option<&str>, dry_run: Option<&str>, field_selector: Option<&str>, grace_period_seconds: Option<i32>, label_selector: Option<&str>, limit: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, body: Option<crate::models::V1DeleteOptions>) -> Result<crate::models::V1Status, Error<DeleteCollectionCustomResourceDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _continue {
        local_var_req_builder = local_var_req_builder.query(&[("continue", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("dryRun", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_selector {
        local_var_req_builder = local_var_req_builder.query(&[("fieldSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = grace_period_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("gracePeriodSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("labelSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = orphan_dependents {
        local_var_req_builder = local_var_req_builder.query(&[("orphanDependents", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = propagation_policy {
        local_var_req_builder = local_var_req_builder.query(&[("propagationPolicy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resource_version_match {
        local_var_req_builder = local_var_req_builder.query(&[("resourceVersionMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("timeoutSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteCollectionCustomResourceDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// delete a CustomResourceDefinition
pub async fn delete_custom_resource_definition(configuration: &configuration::Configuration, name: &str, pretty: Option<&str>, dry_run: Option<&str>, grace_period_seconds: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, body: Option<crate::models::V1DeleteOptions>) -> Result<crate::models::V1Status, Error<DeleteCustomResourceDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions/{name}", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("dryRun", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = grace_period_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("gracePeriodSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = orphan_dependents {
        local_var_req_builder = local_var_req_builder.query(&[("orphanDependents", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = propagation_policy {
        local_var_req_builder = local_var_req_builder.query(&[("propagationPolicy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteCustomResourceDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// get available resources
pub async fn get_api_resources(configuration: &configuration::Configuration, ) -> Result<crate::models::V1ApiResourceList, Error<GetApiResourcesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetApiResourcesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// list or watch objects of kind CustomResourceDefinition
pub async fn list_custom_resource_definition(configuration: &configuration::Configuration, pretty: Option<&str>, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Result<crate::models::V1CustomResourceDefinitionList, Error<ListCustomResourceDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = allow_watch_bookmarks {
        local_var_req_builder = local_var_req_builder.query(&[("allowWatchBookmarks", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _continue {
        local_var_req_builder = local_var_req_builder.query(&[("continue", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_selector {
        local_var_req_builder = local_var_req_builder.query(&[("fieldSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = label_selector {
        local_var_req_builder = local_var_req_builder.query(&[("labelSelector", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resource_version {
        local_var_req_builder = local_var_req_builder.query(&[("resourceVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resource_version_match {
        local_var_req_builder = local_var_req_builder.query(&[("resourceVersionMatch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timeout_seconds {
        local_var_req_builder = local_var_req_builder.query(&[("timeoutSeconds", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = watch {
        local_var_req_builder = local_var_req_builder.query(&[("watch", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListCustomResourceDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// partially update the specified CustomResourceDefinition
pub async fn patch_custom_resource_definition(configuration: &configuration::Configuration, name: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Result<crate::models::V1CustomResourceDefinition, Error<PatchCustomResourceDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions/{name}", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("dryRun", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_manager {
        local_var_req_builder = local_var_req_builder.query(&[("fieldManager", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force {
        local_var_req_builder = local_var_req_builder.query(&[("force", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PatchCustomResourceDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// partially update status of the specified CustomResourceDefinition
pub async fn patch_custom_resource_definition_status(configuration: &configuration::Configuration, name: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Result<crate::models::V1CustomResourceDefinition, Error<PatchCustomResourceDefinitionStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions/{name}/status", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("dryRun", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_manager {
        local_var_req_builder = local_var_req_builder.query(&[("fieldManager", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force {
        local_var_req_builder = local_var_req_builder.query(&[("force", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PatchCustomResourceDefinitionStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// read the specified CustomResourceDefinition
pub async fn read_custom_resource_definition(configuration: &configuration::Configuration, name: &str, pretty: Option<&str>) -> Result<crate::models::V1CustomResourceDefinition, Error<ReadCustomResourceDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions/{name}", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReadCustomResourceDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// read status of the specified CustomResourceDefinition
pub async fn read_custom_resource_definition_status(configuration: &configuration::Configuration, name: &str, pretty: Option<&str>) -> Result<crate::models::V1CustomResourceDefinition, Error<ReadCustomResourceDefinitionStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions/{name}/status", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReadCustomResourceDefinitionStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// replace the specified CustomResourceDefinition
pub async fn replace_custom_resource_definition(configuration: &configuration::Configuration, name: &str, body: crate::models::V1CustomResourceDefinition, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::V1CustomResourceDefinition, Error<ReplaceCustomResourceDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions/{name}", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("dryRun", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_manager {
        local_var_req_builder = local_var_req_builder.query(&[("fieldManager", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceCustomResourceDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// replace status of the specified CustomResourceDefinition
pub async fn replace_custom_resource_definition_status(configuration: &configuration::Configuration, name: &str, body: crate::models::V1CustomResourceDefinition, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::V1CustomResourceDefinition, Error<ReplaceCustomResourceDefinitionStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/apiextensions.k8s.io/v1/customresourcedefinitions/{name}/status", configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = dry_run {
        local_var_req_builder = local_var_req_builder.query(&[("dryRun", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_manager {
        local_var_req_builder = local_var_req_builder.query(&[("fieldManager", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceCustomResourceDefinitionStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

