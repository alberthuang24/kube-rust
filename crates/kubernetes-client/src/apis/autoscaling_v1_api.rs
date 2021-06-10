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


/// struct for typed errors of method `create_namespaced_horizontal_pod_autoscaler`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNamespacedHorizontalPodAutoscalerError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_collection_namespaced_horizontal_pod_autoscaler`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCollectionNamespacedHorizontalPodAutoscalerError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `delete_namespaced_horizontal_pod_autoscaler`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteNamespacedHorizontalPodAutoscalerError {
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

/// struct for typed errors of method `list_horizontal_pod_autoscaler_for_all_namespaces`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHorizontalPodAutoscalerForAllNamespacesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_namespaced_horizontal_pod_autoscaler`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNamespacedHorizontalPodAutoscalerError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_namespaced_horizontal_pod_autoscaler`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchNamespacedHorizontalPodAutoscalerError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `patch_namespaced_horizontal_pod_autoscaler_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchNamespacedHorizontalPodAutoscalerStatusError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `read_namespaced_horizontal_pod_autoscaler`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadNamespacedHorizontalPodAutoscalerError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `read_namespaced_horizontal_pod_autoscaler_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadNamespacedHorizontalPodAutoscalerStatusError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_namespaced_horizontal_pod_autoscaler`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceNamespacedHorizontalPodAutoscalerError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `replace_namespaced_horizontal_pod_autoscaler_status`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceNamespacedHorizontalPodAutoscalerStatusError {
    Status401(),
    UnknownValue(serde_json::Value),
}


/// create a HorizontalPodAutoscaler
pub async fn create_namespaced_horizontal_pod_autoscaler(configuration: &configuration::Configuration, namespace: &str, body: crate::models::V1HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::V1HorizontalPodAutoscaler, Error<CreateNamespacedHorizontalPodAutoscalerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers", configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<CreateNamespacedHorizontalPodAutoscalerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// delete collection of HorizontalPodAutoscaler
pub async fn delete_collection_namespaced_horizontal_pod_autoscaler(configuration: &configuration::Configuration, namespace: &str, pretty: Option<&str>, _continue: Option<&str>, dry_run: Option<&str>, field_selector: Option<&str>, grace_period_seconds: Option<i32>, label_selector: Option<&str>, limit: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, body: Option<crate::models::V1DeleteOptions>) -> Result<crate::models::V1Status, Error<DeleteCollectionNamespacedHorizontalPodAutoscalerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers", configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<DeleteCollectionNamespacedHorizontalPodAutoscalerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// delete a HorizontalPodAutoscaler
pub async fn delete_namespaced_horizontal_pod_autoscaler(configuration: &configuration::Configuration, name: &str, namespace: &str, pretty: Option<&str>, dry_run: Option<&str>, grace_period_seconds: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, body: Option<crate::models::V1DeleteOptions>) -> Result<crate::models::V1Status, Error<DeleteNamespacedHorizontalPodAutoscalerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<DeleteNamespacedHorizontalPodAutoscalerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// get available resources
pub async fn get_api_resources(configuration: &configuration::Configuration, ) -> Result<crate::models::V1ApiResourceList, Error<GetApiResourcesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/", configuration.base_path);
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

/// list or watch objects of kind HorizontalPodAutoscaler
pub async fn list_horizontal_pod_autoscaler_for_all_namespaces(configuration: &configuration::Configuration, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, pretty: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Result<crate::models::V1HorizontalPodAutoscalerList, Error<ListHorizontalPodAutoscalerForAllNamespacesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/horizontalpodautoscalers", configuration.base_path);
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
        let local_var_entity: Option<ListHorizontalPodAutoscalerForAllNamespacesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// list or watch objects of kind HorizontalPodAutoscaler
pub async fn list_namespaced_horizontal_pod_autoscaler(configuration: &configuration::Configuration, namespace: &str, pretty: Option<&str>, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Result<crate::models::V1HorizontalPodAutoscalerList, Error<ListNamespacedHorizontalPodAutoscalerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers", configuration.base_path, namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ListNamespacedHorizontalPodAutoscalerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// partially update the specified HorizontalPodAutoscaler
pub async fn patch_namespaced_horizontal_pod_autoscaler(configuration: &configuration::Configuration, name: &str, namespace: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Result<crate::models::V1HorizontalPodAutoscaler, Error<PatchNamespacedHorizontalPodAutoscalerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<PatchNamespacedHorizontalPodAutoscalerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// partially update status of the specified HorizontalPodAutoscaler
pub async fn patch_namespaced_horizontal_pod_autoscaler_status(configuration: &configuration::Configuration, name: &str, namespace: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Result<crate::models::V1HorizontalPodAutoscaler, Error<PatchNamespacedHorizontalPodAutoscalerStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<PatchNamespacedHorizontalPodAutoscalerStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// read the specified HorizontalPodAutoscaler
pub async fn read_namespaced_horizontal_pod_autoscaler(configuration: &configuration::Configuration, name: &str, namespace: &str, pretty: Option<&str>) -> Result<crate::models::V1HorizontalPodAutoscaler, Error<ReadNamespacedHorizontalPodAutoscalerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ReadNamespacedHorizontalPodAutoscalerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// read status of the specified HorizontalPodAutoscaler
pub async fn read_namespaced_horizontal_pod_autoscaler_status(configuration: &configuration::Configuration, name: &str, namespace: &str, pretty: Option<&str>) -> Result<crate::models::V1HorizontalPodAutoscaler, Error<ReadNamespacedHorizontalPodAutoscalerStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ReadNamespacedHorizontalPodAutoscalerStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// replace the specified HorizontalPodAutoscaler
pub async fn replace_namespaced_horizontal_pod_autoscaler(configuration: &configuration::Configuration, name: &str, namespace: &str, body: crate::models::V1HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::V1HorizontalPodAutoscaler, Error<ReplaceNamespacedHorizontalPodAutoscalerError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ReplaceNamespacedHorizontalPodAutoscalerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// replace status of the specified HorizontalPodAutoscaler
pub async fn replace_namespaced_horizontal_pod_autoscaler_status(configuration: &configuration::Configuration, name: &str, namespace: &str, body: crate::models::V1HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Result<crate::models::V1HorizontalPodAutoscaler, Error<ReplaceNamespacedHorizontalPodAutoscalerStatusError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status", configuration.base_path, name=crate::apis::urlencode(name), namespace=crate::apis::urlencode(namespace));
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
        let local_var_entity: Option<ReplaceNamespacedHorizontalPodAutoscalerStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

