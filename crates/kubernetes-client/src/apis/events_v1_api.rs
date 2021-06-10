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


/// struct for typed errors of method `create_namespaced_event`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNamespacedEventError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_collection_namespaced_event`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCollectionNamespacedEventError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_namespaced_event`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNamespacedEventError {
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

/// struct for typed errors of method `list_event_for_all_namespaces`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventForAllNamespacesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_namespaced_event`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNamespacedEventError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_namespaced_event`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchNamespacedEventError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `read_namespaced_event`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadNamespacedEventError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_namespaced_event`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceNamespacedEventError {
    Status401(),
    UnknownValue(serde_json::Value),
}


/// create an Event
pub async fn create_namespaced_event(configuration: &configuration::Configuration, namespace: &str, body: crate::models::EventsV1Event, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::EventsV1Event, Error<CreateNamespacedEventError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/namespaces/{namespace}/events", configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<CreateNamespacedEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// delete collection of Event
pub async fn delete_collection_namespaced_event(configuration: &configuration::Configuration, namespace: &str, pretty: Option<&str>, _continue: Option<&str>, dry_run: Option<&str>, field_selector: Option<&str>, grace_period_seconds: Option<i32>, label_selector: Option<&str>, limit: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, body: Option<crate::models::V1DeleteOptions>) -> Result<crate::models::V1Status, Error<DeleteCollectionNamespacedEventError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/namespaces/{namespace}/events", configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<DeleteCollectionNamespacedEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// delete an Event
pub async fn delete_namespaced_event(configuration: &configuration::Configuration, name: &str, namespace: &str, pretty: Option<&str>, dry_run: Option<&str>, grace_period_seconds: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, body: Option<crate::models::V1DeleteOptions>) -> Result<crate::models::V1Status, Error<DeleteNamespacedEventError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/namespaces/{namespace}/events/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<DeleteNamespacedEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// get available resources
pub async fn get_api_resources(configuration: &configuration::Configuration, ) -> Result<crate::models::V1ApiResourceList, Error<GetApiResourcesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/", configuration.base_path);
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

/// list or watch objects of kind Event
pub async fn list_event_for_all_namespaces(configuration: &configuration::Configuration, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, pretty: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Result<crate::models::EventsV1EventList, Error<ListEventForAllNamespacesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/events", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
    if let Some(ref local_var_str) = pretty {
        local_var_req_builder = local_var_req_builder.query(&[("pretty", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListEventForAllNamespacesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// list or watch objects of kind Event
pub async fn list_namespaced_event(configuration: &configuration::Configuration, namespace: &str, pretty: Option<&str>, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Result<crate::models::EventsV1EventList, Error<ListNamespacedEventError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/namespaces/{namespace}/events", configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ListNamespacedEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// partially update the specified Event
pub async fn patch_namespaced_event(configuration: &configuration::Configuration, name: &str, namespace: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Result<crate::models::EventsV1Event, Error<PatchNamespacedEventError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/namespaces/{namespace}/events/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<PatchNamespacedEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// read the specified Event
pub async fn read_namespaced_event(configuration: &configuration::Configuration, name: &str, namespace: &str, pretty: Option<&str>) -> Result<crate::models::EventsV1Event, Error<ReadNamespacedEventError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/namespaces/{namespace}/events/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ReadNamespacedEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// replace the specified Event
pub async fn replace_namespaced_event(configuration: &configuration::Configuration, name: &str, namespace: &str, body: crate::models::EventsV1Event, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::EventsV1Event, Error<ReplaceNamespacedEventError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/events.k8s.io/v1/namespaces/{namespace}/events/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ReplaceNamespacedEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

