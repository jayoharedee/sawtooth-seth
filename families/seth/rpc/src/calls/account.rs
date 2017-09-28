/*
 * Copyright 2017 Intel Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

use jsonrpc_core::{Params, Value, Error};

use client::{
    ValidatorClient,
    BlockKey,
    BlockKeyParseError,
    num_to_hex,
    hex_prefix,
    bytes_to_hex_str,
};

use sawtooth_sdk::messaging::stream::MessageSender;
use error;
use requests::{RequestHandler};

pub fn get_method_list<T>() -> Vec<(String, RequestHandler<T>)> where T: MessageSender {
    let mut methods: Vec<(String, RequestHandler<T>)> = Vec::new();

    methods.push((String::from("eth_getBalance"), get_balance));
    methods.push((String::from("eth_getStorageAt"), get_storage_at));
    methods.push((String::from("eth_getCode"), get_code));
    methods.push((String::from("eth_sign"), sign));
    methods.push((String::from("eth_call"), call));
    methods.push((String::from("eth_accounts"), accounts));

    methods
}

fn validate_block_key(block: String) -> Result<BlockKey, Error> {
    match block.parse() {
        Ok(k) => Ok(k),
        Err(BlockKeyParseError::Invalid) => {
            return Err(Error::invalid_params("Failed to parse block number"));
        },
        Err(BlockKeyParseError::Unsupported) => {
            return Err(error::not_implemented());
        },
    }
}

fn validate_account_address(address: String) -> Result<String, Error> {
    if address.len() != 42 {
        Err(Error::invalid_params(
            format!("Invalid address length: {} != {}", address.len(), 42)))
    } else {
        Ok(String::from(&address[2..]))
    }
}

fn validate_storage_address(address: String) -> Result<String, Error> {
    if address.len() < 4 || address.len() % 2 != 0{
        Err(Error::invalid_params(format!("Invalid storage position: {}", address)))
    } else {
        Ok(String::from(&address[2..]))
    }
}

pub fn get_balance<T>(params: Params, mut client: ValidatorClient<T>) -> Result<Value, Error> where T: MessageSender {
    info!("eth_getBalance");
    let (address, block): (String, String) = match params.parse() {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::invalid_params("Takes [address: DATA(20), block: QUANTITY|TAG]"));
        },
    };

    let key = validate_block_key(block)?;
    let address = validate_account_address(address)?;

    match client.get_account(address, key) {
        Ok(Some(account)) => Ok(num_to_hex(&account.balance)),
        Ok(None) => Ok(Value::Null),
        Err(error) => {
            error!("{}", error);
            Err(Error::internal_error())
        },
    }
}

pub fn get_storage_at<T>(params: Params, mut client: ValidatorClient<T>) -> Result<Value, Error> where T: MessageSender {
    info!("eth_getStorageAt");
    let (address, position, block): (String, String, String) = match params.parse() {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::invalid_params("Takes [address: DATA(20), position: QUANTITY, block: QUANTITY|TAG]"));
        },
    };

    let key = validate_block_key(block)?;
    let account_address = validate_account_address(address)?;
    let storage_address = validate_storage_address(position)?;

    match client.get_storage_at(account_address, storage_address, key) {
        Ok(Some(value)) => Ok(hex_prefix(&bytes_to_hex_str(&value))),
        Ok(None) => Ok(Value::Null),
        Err(error) => {
            error!("{}", error);
            Err(Error::internal_error())
        },
    }
}

pub fn get_code<T>(params: Params, mut client: ValidatorClient<T>) -> Result<Value, Error> where T: MessageSender {
    info!("eth_getCode");
    let (address, block): (String, String) = match params.parse() {
        Ok(t) => t,
        Err(_) => {
            return Err(Error::invalid_params("Takes [address: DATA(20), block: QUANTITY|TAG]"));
        },
    };

    let key = validate_block_key(block)?;
    let address = validate_account_address(address)?;

    match client.get_account(address, key) {
        Ok(Some(account)) => Ok(hex_prefix(&bytes_to_hex_str(&account.code))),
        Ok(None) => Ok(Value::Null),
        Err(error) => {
            error!("{}", error);
            Err(Error::internal_error())
        },
    }
}
pub fn sign<T>(_params: Params, mut _client: ValidatorClient<T>) -> Result<Value, Error> where T: MessageSender {
    Err(error::not_implemented())
}
pub fn call<T>(_params: Params, mut _client: ValidatorClient<T>) -> Result<Value, Error> where T: MessageSender {
    Err(error::not_implemented())
}
pub fn accounts<T>(_params: Params, mut _client: ValidatorClient<T>) -> Result<Value, Error> where T: MessageSender {
    Err(error::not_implemented())
}