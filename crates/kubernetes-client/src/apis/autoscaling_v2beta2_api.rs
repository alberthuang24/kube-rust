/*
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1.21.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct AutoscalingV2beta2ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AutoscalingV2beta2ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AutoscalingV2beta2ApiClient<C> {
        AutoscalingV2beta2ApiClient {
            configuration,
        }
    }
}

pub trait AutoscalingV2beta2Api {
    fn create_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, body: crate::models::V2beta2HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>>;
    fn delete_collection_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: Option<&str>, _continue: Option<&str>, dry_run: Option<&str>, field_selector: Option<&str>, grace_period_seconds: Option<i32>, label_selector: Option<&str>, limit: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, body: Option<crate::models::V1DeleteOptions>) -> Box<dyn Future<Item = crate::models::V1Status, Error = Error<serde_json::Value>>>;
    fn delete_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, pretty: Option<&str>, dry_run: Option<&str>, grace_period_seconds: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, body: Option<crate::models::V1DeleteOptions>) -> Box<dyn Future<Item = crate::models::V1Status, Error = Error<serde_json::Value>>>;
    fn get_api_resources(&self, ) -> Box<dyn Future<Item = crate::models::V1ApiResourceList, Error = Error<serde_json::Value>>>;
    fn list_horizontal_pod_autoscaler_for_all_namespaces(&self, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, pretty: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscalerList, Error = Error<serde_json::Value>>>;
    fn list_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: Option<&str>, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscalerList, Error = Error<serde_json::Value>>>;
    fn patch_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>>;
    fn patch_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>>;
    fn read_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, pretty: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>>;
    fn read_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, pretty: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>>;
    fn replace_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: crate::models::V2beta2HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>>;
    fn replace_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: crate::models::V2beta2HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>AutoscalingV2beta2Api for AutoscalingV2beta2ApiClient<C> {
    fn create_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, body: crate::models::V2beta2HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = dry_run {
            req = req.with_query_param("dryRun".to_string(), s.to_string());
        }
        if let Some(ref s) = field_manager {
            req = req.with_query_param("fieldManager".to_string(), s.to_string());
        }
        req = req.with_path_param("namespace".to_string(), namespace.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn delete_collection_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: Option<&str>, _continue: Option<&str>, dry_run: Option<&str>, field_selector: Option<&str>, grace_period_seconds: Option<i32>, label_selector: Option<&str>, limit: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, body: Option<crate::models::V1DeleteOptions>) -> Box<dyn Future<Item = crate::models::V1Status, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = _continue {
            req = req.with_query_param("continue".to_string(), s.to_string());
        }
        if let Some(ref s) = dry_run {
            req = req.with_query_param("dryRun".to_string(), s.to_string());
        }
        if let Some(ref s) = field_selector {
            req = req.with_query_param("fieldSelector".to_string(), s.to_string());
        }
        if let Some(ref s) = grace_period_seconds {
            req = req.with_query_param("gracePeriodSeconds".to_string(), s.to_string());
        }
        if let Some(ref s) = label_selector {
            req = req.with_query_param("labelSelector".to_string(), s.to_string());
        }
        if let Some(ref s) = limit {
            req = req.with_query_param("limit".to_string(), s.to_string());
        }
        if let Some(ref s) = orphan_dependents {
            req = req.with_query_param("orphanDependents".to_string(), s.to_string());
        }
        if let Some(ref s) = propagation_policy {
            req = req.with_query_param("propagationPolicy".to_string(), s.to_string());
        }
        if let Some(ref s) = resource_version {
            req = req.with_query_param("resourceVersion".to_string(), s.to_string());
        }
        if let Some(ref s) = resource_version_match {
            req = req.with_query_param("resourceVersionMatch".to_string(), s.to_string());
        }
        if let Some(ref s) = timeout_seconds {
            req = req.with_query_param("timeoutSeconds".to_string(), s.to_string());
        }
        req = req.with_path_param("namespace".to_string(), namespace.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn delete_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, pretty: Option<&str>, dry_run: Option<&str>, grace_period_seconds: Option<i32>, orphan_dependents: Option<bool>, propagation_policy: Option<&str>, body: Option<crate::models::V1DeleteOptions>) -> Box<dyn Future<Item = crate::models::V1Status, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = dry_run {
            req = req.with_query_param("dryRun".to_string(), s.to_string());
        }
        if let Some(ref s) = grace_period_seconds {
            req = req.with_query_param("gracePeriodSeconds".to_string(), s.to_string());
        }
        if let Some(ref s) = orphan_dependents {
            req = req.with_query_param("orphanDependents".to_string(), s.to_string());
        }
        if let Some(ref s) = propagation_policy {
            req = req.with_query_param("propagationPolicy".to_string(), s.to_string());
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("namespace".to_string(), namespace.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn get_api_resources(&self, ) -> Box<dyn Future<Item = crate::models::V1ApiResourceList, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/apis/autoscaling/v2beta2/".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;

        req.execute(self.configuration.borrow())
    }

    fn list_horizontal_pod_autoscaler_for_all_namespaces(&self, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, pretty: Option<&str>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscalerList, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/apis/autoscaling/v2beta2/horizontalpodautoscalers".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = allow_watch_bookmarks {
            req = req.with_query_param("allowWatchBookmarks".to_string(), s.to_string());
        }
        if let Some(ref s) = _continue {
            req = req.with_query_param("continue".to_string(), s.to_string());
        }
        if let Some(ref s) = field_selector {
            req = req.with_query_param("fieldSelector".to_string(), s.to_string());
        }
        if let Some(ref s) = label_selector {
            req = req.with_query_param("labelSelector".to_string(), s.to_string());
        }
        if let Some(ref s) = limit {
            req = req.with_query_param("limit".to_string(), s.to_string());
        }
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = resource_version {
            req = req.with_query_param("resourceVersion".to_string(), s.to_string());
        }
        if let Some(ref s) = resource_version_match {
            req = req.with_query_param("resourceVersionMatch".to_string(), s.to_string());
        }
        if let Some(ref s) = timeout_seconds {
            req = req.with_query_param("timeoutSeconds".to_string(), s.to_string());
        }
        if let Some(ref s) = watch {
            req = req.with_query_param("watch".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn list_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: Option<&str>, allow_watch_bookmarks: Option<bool>, _continue: Option<&str>, field_selector: Option<&str>, label_selector: Option<&str>, limit: Option<i32>, resource_version: Option<&str>, resource_version_match: Option<&str>, timeout_seconds: Option<i32>, watch: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscalerList, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = allow_watch_bookmarks {
            req = req.with_query_param("allowWatchBookmarks".to_string(), s.to_string());
        }
        if let Some(ref s) = _continue {
            req = req.with_query_param("continue".to_string(), s.to_string());
        }
        if let Some(ref s) = field_selector {
            req = req.with_query_param("fieldSelector".to_string(), s.to_string());
        }
        if let Some(ref s) = label_selector {
            req = req.with_query_param("labelSelector".to_string(), s.to_string());
        }
        if let Some(ref s) = limit {
            req = req.with_query_param("limit".to_string(), s.to_string());
        }
        if let Some(ref s) = resource_version {
            req = req.with_query_param("resourceVersion".to_string(), s.to_string());
        }
        if let Some(ref s) = resource_version_match {
            req = req.with_query_param("resourceVersionMatch".to_string(), s.to_string());
        }
        if let Some(ref s) = timeout_seconds {
            req = req.with_query_param("timeoutSeconds".to_string(), s.to_string());
        }
        if let Some(ref s) = watch {
            req = req.with_query_param("watch".to_string(), s.to_string());
        }
        req = req.with_path_param("namespace".to_string(), namespace.to_string());

        req.execute(self.configuration.borrow())
    }

    fn patch_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = dry_run {
            req = req.with_query_param("dryRun".to_string(), s.to_string());
        }
        if let Some(ref s) = field_manager {
            req = req.with_query_param("fieldManager".to_string(), s.to_string());
        }
        if let Some(ref s) = force {
            req = req.with_query_param("force".to_string(), s.to_string());
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("namespace".to_string(), namespace.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn patch_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: serde_json::Value, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>, force: Option<bool>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers/{name}/status".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = dry_run {
            req = req.with_query_param("dryRun".to_string(), s.to_string());
        }
        if let Some(ref s) = field_manager {
            req = req.with_query_param("fieldManager".to_string(), s.to_string());
        }
        if let Some(ref s) = force {
            req = req.with_query_param("force".to_string(), s.to_string());
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("namespace".to_string(), namespace.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn read_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, pretty: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("namespace".to_string(), namespace.to_string());

        req.execute(self.configuration.borrow())
    }

    fn read_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, pretty: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers/{name}/status".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("namespace".to_string(), namespace.to_string());

        req.execute(self.configuration.borrow())
    }

    fn replace_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: crate::models::V2beta2HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers/{name}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = dry_run {
            req = req.with_query_param("dryRun".to_string(), s.to_string());
        }
        if let Some(ref s) = field_manager {
            req = req.with_query_param("fieldManager".to_string(), s.to_string());
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("namespace".to_string(), namespace.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn replace_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: crate::models::V2beta2HorizontalPodAutoscaler, pretty: Option<&str>, dry_run: Option<&str>, field_manager: Option<&str>) -> Box<dyn Future<Item = crate::models::V2beta2HorizontalPodAutoscaler, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/apis/autoscaling/v2beta2/namespaces/{namespace}/horizontalpodautoscalers/{name}/status".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "authorization".to_owned(),
            }))
        ;
        if let Some(ref s) = pretty {
            req = req.with_query_param("pretty".to_string(), s.to_string());
        }
        if let Some(ref s) = dry_run {
            req = req.with_query_param("dryRun".to_string(), s.to_string());
        }
        if let Some(ref s) = field_manager {
            req = req.with_query_param("fieldManager".to_string(), s.to_string());
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("namespace".to_string(), namespace.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

}
