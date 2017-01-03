// Copyright 2016 Aerospike, Inc.
//
// Portions may be licensed to Aerospike, Inc. under one or more contributor
// license agreements.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

#![feature(test)]

#[macro_use]
extern crate aerospike;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate test;

use aerospike::{ReadPolicy, WritePolicy};
use aerospike::{Key, Bin};
use aerospike::value::*;

use test::Bencher;

mod common1;

#[bench]
fn single_key_read(b: &mut Bencher) {
    let ref client = common1::GLOBAL_CLIENT;
    let namespace: &str = &common1::AEROSPIKE_NAMESPACE;
    let set_name: &str = &common1::AEROSPIKE_SET;
    let key = as_key!(namespace, set_name, common1::rand_str(10));
    let wbin = as_bin!("i", 1);
    let bins = vec![&wbin];
    let rpolicy = ReadPolicy::default();
    let wpolicy = WritePolicy::default();
    client.put(&wpolicy, &key, &bins).unwrap();

    b.iter(|| {
        client.get(&rpolicy, &key, None).unwrap()
    });
}