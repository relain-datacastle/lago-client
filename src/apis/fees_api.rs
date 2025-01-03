/*
 * Lago API documentation
 *
 * Lago API allows your application to push customer information and metrics (events) from your application to the billing application.
 *
 * The version of the OpenAPI document: 1.17.1
 * Contact: tech@getlago.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`delete_fee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFeeError {
    Status401(models::ApiErrorUnauthorized),
    Status404(models::ApiErrorNotFound),
    Status405(models::ApiErrorNotAllowed),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`find_all_fees`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindAllFeesError {
    Status401(models::ApiErrorUnauthorized),
    Status422(models::ApiErrorUnprocessableEntity),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`find_fee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindFeeError {
    Status401(models::ApiErrorUnauthorized),
    Status404(models::ApiErrorNotFound),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_fee`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateFeeError {
    Status400(models::ApiErrorBadRequest),
    Status401(models::ApiErrorUnauthorized),
    Status404(models::ApiErrorNotFound),
    Status405(models::ApiErrorNotAllowed),
    Status422(models::ApiErrorUnprocessableEntity),
    UnknownValue(serde_json::Value),
}


/// This endpoint is used for deleting a specific fee that has not yet been invoiced
pub async fn delete_fee(configuration: &configuration::Configuration, lago_id: &str) -> Result<models::Fee, Error<DeleteFeeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fees/{lago_id}", local_var_configuration.base_path, lago_id=crate::apis::urlencode(lago_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteFeeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint is used for retrieving all fees that has been issued.
pub async fn find_all_fees(configuration: &configuration::Configuration, page: Option<i32>, per_page: Option<i32>, external_customer_id: Option<&str>, external_subscription_id: Option<&str>, event_transaction_id: Option<&str>, currency: Option<models::Currency>, fee_type: Option<&str>, billable_metric_code: Option<&str>, payment_status: Option<&str>, created_at_from: Option<String>, created_at_to: Option<String>, succeeded_at_from: Option<String>, succeeded_at_to: Option<String>, failed_at_from: Option<String>, failed_at_to: Option<String>, refunded_at_from: Option<String>, refunded_at_to: Option<String>) -> Result<models::FeesPaginated, Error<FindAllFeesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fees", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = external_customer_id {
        local_var_req_builder = local_var_req_builder.query(&[("external_customer_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = external_subscription_id {
        local_var_req_builder = local_var_req_builder.query(&[("external_subscription_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = event_transaction_id {
        local_var_req_builder = local_var_req_builder.query(&[("event_transaction_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = currency {
        local_var_req_builder = local_var_req_builder.query(&[("currency", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fee_type {
        local_var_req_builder = local_var_req_builder.query(&[("fee_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = billable_metric_code {
        local_var_req_builder = local_var_req_builder.query(&[("billable_metric_code", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = payment_status {
        local_var_req_builder = local_var_req_builder.query(&[("payment_status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_at_from {
        local_var_req_builder = local_var_req_builder.query(&[("created_at_from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = created_at_to {
        local_var_req_builder = local_var_req_builder.query(&[("created_at_to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = succeeded_at_from {
        local_var_req_builder = local_var_req_builder.query(&[("succeeded_at_from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = succeeded_at_to {
        local_var_req_builder = local_var_req_builder.query(&[("succeeded_at_to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = failed_at_from {
        local_var_req_builder = local_var_req_builder.query(&[("failed_at_from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = failed_at_to {
        local_var_req_builder = local_var_req_builder.query(&[("failed_at_to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = refunded_at_from {
        local_var_req_builder = local_var_req_builder.query(&[("refunded_at_from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = refunded_at_to {
        local_var_req_builder = local_var_req_builder.query(&[("refunded_at_to", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FindAllFeesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint is used for retrieving a specific fee that has been issued.
pub async fn find_fee(configuration: &configuration::Configuration, lago_id: &str) -> Result<models::Fee, Error<FindFeeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fees/{lago_id}", local_var_configuration.base_path, lago_id=crate::apis::urlencode(lago_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<FindFeeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint is used for updating a specific fee that has been issued.
pub async fn update_fee(configuration: &configuration::Configuration, lago_id: &str, fee_update_input: Option<models::FeeUpdateInput>) -> Result<models::Fee, Error<UpdateFeeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/fees/{lago_id}", local_var_configuration.base_path, lago_id=crate::apis::urlencode(lago_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&fee_update_input);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateFeeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

