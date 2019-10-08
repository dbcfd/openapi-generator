/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct UserApiClient<C: hyper::client::connect::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> UserApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> UserApiClient<C> {
        UserApiClient {
            configuration: configuration,
        }
    }
}

pub trait UserApi {
    fn create_user(&self, body: crate::models::User) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_users_with_array_input(&self, body: Vec<crate::models::User>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn create_users_with_list_input(&self, body: Vec<crate::models::User>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_user(&self, username: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_user_by_name(&self, username: &str) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>>;
    fn login_user(&self, username: &str, password: &str) -> Box<Future<Item = String, Error = Error<serde_json::Value>>>;
    fn logout_user(&self, ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_user(&self, username: &str, body: crate::models::User) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::connect::Connect>UserApi for UserApiClient<C> {
    fn create_user(&self, body: crate::models::User) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/user".to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn create_users_with_array_input(&self, body: Vec<crate::models::User>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/user/createWithArray".to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn create_users_with_list_input(&self, body: Vec<crate::models::User>) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Post, "/user/createWithList".to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn delete_user(&self, username: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Delete, "/user/{username}".to_string())
            .with_path_param("username".to_string(), username.to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn get_user_by_name(&self, username: &str) -> Box<Future<Item = crate::models::User, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/user/{username}".to_string())
            .with_path_param("username".to_string(), username.to_string())
            .execute(self.configuration.borrow())
    }

    fn login_user(&self, username: &str, password: &str) -> Box<Future<Item = String, Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/user/login".to_string())
            .with_query_param("username".to_string(), username.to_string())
            .with_query_param("password".to_string(), password.to_string())
            .execute(self.configuration.borrow())
    }

    fn logout_user(&self, ) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Get, "/user/logout".to_string())
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

    fn update_user(&self, username: &str, body: crate::models::User) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        __internal_request::Request::new(hyper::Method::Put, "/user/{username}".to_string())
            .with_path_param("username".to_string(), username.to_string())
            .with_body_param(body)
            .returns_nothing()
            .execute(self.configuration.borrow())
    }

}
