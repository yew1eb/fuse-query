// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use crate::datatypes::{DataArrayRef, DataSchema};
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct DataBlock {
    schema: DataSchema,
    columns: Vec<DataArrayRef>,
}

impl DataBlock {
    pub fn new(schema: DataSchema, columns: Vec<DataArrayRef>) -> Self {
        DataBlock { schema, columns }
    }

    pub fn empty() -> Self {
        DataBlock {
            schema: DataSchema::empty(),
            columns: vec![],
        }
    }

    pub fn schema(&self) -> &DataSchema {
        &self.schema
    }

    pub fn rows(&self) -> u64 {
        if self.columns.is_empty() {
            0
        } else {
            self.columns[0].len() as u64
        }
    }

    pub fn column_by_name(&self, name: &str) -> Result<&DataArrayRef> {
        let idx = self.schema.index_of(name)?;
        Ok(&self.columns[idx])
    }
}
