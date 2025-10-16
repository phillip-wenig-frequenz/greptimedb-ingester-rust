// Copyright 2023 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Table schema and data structures for GreptimeDB bulk insert operations

use derive_builder::Builder;

use crate::api::v1::{ColumnDataType, SemanticType};

/// Extended data type information for columns that need additional parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTypeExtension {
    /// Decimal128 with specific precision and scale
    Decimal128 { precision: u8, scale: i8 },
}

/// Represents a time-series data table with schema
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct TableSchema {
    /// Table name
    name: String,
    /// Table columns
    #[builder(default)]
    columns: Vec<Column>,
}

impl TableSchema {
    /// Create a new table schema builder
    pub fn builder() -> TableSchemaBuilder {
        TableSchemaBuilder::default()
    }

    /// Get the table name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the table columns
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Add a tag column (for indexing and grouping)
    pub fn add_tag<T: Into<String>>(
        mut self,
        name: T,
        data_type: ColumnDataType,
        nullable: bool,
    ) -> Self {
        self.columns.push(Column {
            name: name.into(),
            data_type,
            semantic_type: SemanticType::Tag,
            data_type_extension: None,
            nullable,
        });
        self
    }

    /// Add a timestamp column (timeline for time series)
    pub fn add_timestamp<T: Into<String>>(mut self, name: T, data_type: ColumnDataType) -> Self {
        self.columns.push(Column {
            name: name.into(),
            data_type,
            semantic_type: SemanticType::Timestamp,
            data_type_extension: None,
            nullable: false,
        });
        self
    }

    /// Add a field column (measurement values)
    pub fn add_field<T: Into<String>>(
        mut self,
        name: T,
        data_type: ColumnDataType,
        nullable: bool,
    ) -> Self {
        self.columns.push(Column {
            name: name.into(),
            data_type,
            semantic_type: SemanticType::Field,
            data_type_extension: None,
            nullable,
        });
        self
    }

    /// Add a decimal128 field column with specific precision and scale
    pub fn add_decimal128_field<T: Into<String>>(
        mut self,
        name: T,
        precision: u8,
        scale: i8,
        nullable: bool,
    ) -> Self {
        self.columns.push(Column {
            name: name.into(),
            data_type: ColumnDataType::Decimal128,
            semantic_type: SemanticType::Field,
            data_type_extension: Some(DataTypeExtension::Decimal128 { precision, scale }),
            nullable,
        });
        self
    }
}

/// Table column definition
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: ColumnDataType,
    pub semantic_type: SemanticType,
    /// Extended type information for data types that need additional parameters
    pub data_type_extension: Option<DataTypeExtension>,
    pub nullable: bool,
}

/// Represents a data row with type-safe value access
#[derive(Debug, Clone, Default)]
pub struct Row {
    values: Vec<Value>,
}

impl Row {
    /// Create a new empty row
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new row with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }

    /// Get the number of values in the row
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if the row is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Create a row directly from values (more efficient than chaining add_value calls)
    pub fn from_values(values: Vec<Value>) -> Self {
        Self { values }
    }

    /// Add a value to the row
    pub fn add_value(mut self, value: Value) -> Self {
        self.values.push(value);
        self
    }

    /// Add multiple values to the row
    pub fn add_values(mut self, values: Vec<Value>) -> Self {
        self.values.extend(values);
        self
    }

    /// Add multiple values from an iterator
    pub fn add_values_iter(mut self, values: impl IntoIterator<Item = Value>) -> Self {
        self.values.extend(values);
        self
    }

    /// Get boolean value at index (safe version with bounds checking)
    pub fn get_bool(&self, index: usize) -> Option<bool> {
        match self.values.get(index)? {
            Value::Boolean(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "boolean", other),
        }
    }

    /// Get boolean value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_bool_unchecked(&self, index: usize) -> Option<bool> {
        match self.values.get_unchecked(index) {
            Value::Boolean(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "boolean", other),
        }
    }

    /// Get i8 value at index (safe version with bounds checking)
    pub fn get_i8(&self, index: usize) -> Option<i8> {
        match self.values.get(index)? {
            Value::Int8(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i8", other),
        }
    }

    /// Get i8 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_i8_unchecked(&self, index: usize) -> Option<i8> {
        match self.values.get_unchecked(index) {
            Value::Int8(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i8", other),
        }
    }

    /// Get i16 value at index (safe version with bounds checking)
    pub fn get_i16(&self, index: usize) -> Option<i16> {
        match self.values.get(index)? {
            Value::Int16(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i16", other),
        }
    }

    /// Get i16 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_i16_unchecked(&self, index: usize) -> Option<i16> {
        match self.values.get_unchecked(index) {
            Value::Int16(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i16", other),
        }
    }

    /// Get i32 value at index (safe version with bounds checking)
    pub fn get_i32(&self, index: usize) -> Option<i32> {
        match self.values.get(index)? {
            Value::Int32(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i32", other),
        }
    }

    /// Get i32 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_i32_unchecked(&self, index: usize) -> Option<i32> {
        match self.values.get_unchecked(index) {
            Value::Int32(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i32", other),
        }
    }

    /// Get i64 value at index (safe version with bounds checking)
    pub fn get_i64(&self, index: usize) -> Option<i64> {
        match self.values.get(index)? {
            Value::Int64(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i64", other),
        }
    }

    /// Get i64 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_i64_unchecked(&self, index: usize) -> Option<i64> {
        match self.values.get_unchecked(index) {
            Value::Int64(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "i64", other),
        }
    }

    /// Get u8 value at index (safe version with bounds checking)
    pub fn get_u8(&self, index: usize) -> Option<u8> {
        match self.values.get(index)? {
            Value::Uint8(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u8", other),
        }
    }

    /// Get u8 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_u8_unchecked(&self, index: usize) -> Option<u8> {
        match self.values.get_unchecked(index) {
            Value::Uint8(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u8", other),
        }
    }

    /// Get u16 value at index (safe version with bounds checking)
    pub fn get_u16(&self, index: usize) -> Option<u16> {
        match self.values.get(index)? {
            Value::Uint16(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u16", other),
        }
    }

    /// Get u16 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_u16_unchecked(&self, index: usize) -> Option<u16> {
        match self.values.get_unchecked(index) {
            Value::Uint16(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u16", other),
        }
    }

    /// Get u32 value at index (safe version with bounds checking)
    pub fn get_u32(&self, index: usize) -> Option<u32> {
        match self.values.get(index)? {
            Value::Uint32(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u32", other),
        }
    }

    /// Get u32 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_u32_unchecked(&self, index: usize) -> Option<u32> {
        match self.values.get_unchecked(index) {
            Value::Uint32(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u32", other),
        }
    }

    /// Get u64 value at index (safe version with bounds checking)
    pub fn get_u64(&self, index: usize) -> Option<u64> {
        match self.values.get(index)? {
            Value::Uint64(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u64", other),
        }
    }

    /// Get u64 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_u64_unchecked(&self, index: usize) -> Option<u64> {
        match self.values.get_unchecked(index) {
            Value::Uint64(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "u64", other),
        }
    }

    /// Get f32 value at index (safe version with bounds checking)
    pub fn get_f32(&self, index: usize) -> Option<f32> {
        match self.values.get(index)? {
            Value::Float32(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "f32", other),
        }
    }

    /// Get f32 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_f32_unchecked(&self, index: usize) -> Option<f32> {
        match self.values.get_unchecked(index) {
            Value::Float32(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "f32", other),
        }
    }

    /// Get f64 value at index (safe version with bounds checking)
    pub fn get_f64(&self, index: usize) -> Option<f64> {
        match self.values.get(index)? {
            Value::Float64(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "f64", other),
        }
    }

    /// Get f64 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_f64_unchecked(&self, index: usize) -> Option<f64> {
        match self.values.get_unchecked(index) {
            Value::Float64(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "f64", other),
        }
    }

    /// Get binary value at index (safe version with bounds checking)
    pub fn get_binary(&self, index: usize) -> Option<Vec<u8>> {
        match self.values.get(index)? {
            Value::Binary(v) => Some(v.clone()),
            Value::String(v) => Some(v.as_bytes().to_vec()), // JSON type
            Value::Null => None,
            other => handle_type_mismatch(index, "binary", other),
        }
    }

    /// Get binary value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_binary_unchecked(&self, index: usize) -> Option<Vec<u8>> {
        match self.values.get_unchecked(index) {
            Value::Binary(v) => Some(v.clone()),
            Value::String(v) => Some(v.as_bytes().to_vec()), // JSON type
            Value::Null => None,
            other => handle_type_mismatch(index, "binary", other),
        }
    }

    /// Take binary value at index (safe version with bounds checking)
    pub fn take_binary(&mut self, index: usize) -> Option<Vec<u8>> {
        if index >= self.values.len() {
            return None;
        }
        unsafe { self.take_binary_unchecked(index) }
    }

    /// Take binary value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn take_binary_unchecked(&mut self, index: usize) -> Option<Vec<u8>> {
        match std::mem::replace(self.values.get_unchecked_mut(index), Value::Null) {
            Value::Binary(v) => Some(v),
            Value::String(v) => Some(v.into_bytes()), // JSON type
            Value::Null => None,
            other => handle_type_mismatch(index, "binary", &other),
        }
    }

    /// Get string value at index (safe version with bounds checking)
    pub fn get_string(&self, index: usize) -> Option<String> {
        match self.values.get(index)? {
            Value::String(v) => Some(v.clone()),
            Value::Null => None,
            other => handle_type_mismatch(index, "string", other),
        }
    }

    /// Get string value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_string_unchecked(&self, index: usize) -> Option<String> {
        match self.values.get_unchecked(index) {
            Value::String(v) => Some(v.clone()),
            Value::Null => None,
            other => handle_type_mismatch(index, "string", other),
        }
    }

    /// Take string value at index (safe version with bounds checking)
    pub fn take_string(&mut self, index: usize) -> Option<String> {
        if index >= self.values.len() {
            return None;
        }
        unsafe { self.take_string_unchecked(index) }
    }

    /// Take string value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn take_string_unchecked(&mut self, index: usize) -> Option<String> {
        match std::mem::replace(self.values.get_unchecked_mut(index), Value::Null) {
            Value::String(v) => Some(v),
            Value::Null => None,
            other => handle_type_mismatch(index, "string", &other),
        }
    }

    /// Get date value at index (safe version with bounds checking)
    pub fn get_date(&self, index: usize) -> Option<i32> {
        match self.values.get(index)? {
            Value::Date(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "date", other),
        }
    }

    /// Get date value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_date_unchecked(&self, index: usize) -> Option<i32> {
        match self.values.get_unchecked(index) {
            Value::Date(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "date", other),
        }
    }

    /// Get datetime value at index (safe version with bounds checking)
    pub fn get_datetime(&self, index: usize) -> Option<i64> {
        match self.values.get(index)? {
            Value::Datetime(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "datetime", other),
        }
    }

    /// Get datetime value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_datetime_unchecked(&self, index: usize) -> Option<i64> {
        match self.values.get_unchecked(index) {
            Value::Datetime(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "datetime", other),
        }
    }

    /// Get timestamp value at index (generic, supports all timestamp types, safe version with bounds checking)
    pub fn get_timestamp(&self, index: usize) -> Option<i64> {
        match self.values.get(index)? {
            Value::TimestampSecond(v) => Some(*v),
            Value::TimestampMillisecond(v) => Some(*v),
            Value::TimestampMicrosecond(v) => Some(*v),
            Value::TimestampNanosecond(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "timestamp", other),
        }
    }

    /// Get timestamp value at index (generic, supports all timestamp types, unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_timestamp_unchecked(&self, index: usize) -> Option<i64> {
        match self.values.get_unchecked(index) {
            Value::TimestampSecond(v) => Some(*v),
            Value::TimestampMillisecond(v) => Some(*v),
            Value::TimestampMicrosecond(v) => Some(*v),
            Value::TimestampNanosecond(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "timestamp", other),
        }
    }

    /// Get time32 value at index (safe version with bounds checking)
    pub fn get_time32(&self, index: usize) -> Option<i32> {
        match self.values.get(index)? {
            Value::TimeSecond(v) => Some(*v),
            Value::TimeMillisecond(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "time32", other),
        }
    }

    /// Get time32 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_time32_unchecked(&self, index: usize) -> Option<i32> {
        match self.values.get_unchecked(index) {
            Value::TimeSecond(v) => Some(*v),
            Value::TimeMillisecond(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "time32", other),
        }
    }

    /// Get time64 value at index (safe version with bounds checking)
    pub fn get_time64(&self, index: usize) -> Option<i64> {
        match self.values.get(index)? {
            Value::TimeMicrosecond(v) => Some(*v),
            Value::TimeNanosecond(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "time64", other),
        }
    }

    /// Get time64 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_time64_unchecked(&self, index: usize) -> Option<i64> {
        match self.values.get_unchecked(index) {
            Value::TimeMicrosecond(v) => Some(*v),
            Value::TimeNanosecond(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "time64", other),
        }
    }

    /// Get decimal128 value at index (safe version with bounds checking)
    pub fn get_decimal128(&self, index: usize) -> Option<i128> {
        match self.values.get(index)? {
            Value::Decimal128(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "decimal128", other),
        }
    }

    /// Get decimal128 value at index (unsafe version without bounds checking)
    /// # Safety
    /// The caller must ensure that `index < self.values.len()`
    pub unsafe fn get_decimal128_unchecked(&self, index: usize) -> Option<i128> {
        match self.values.get_unchecked(index) {
            Value::Decimal128(v) => Some(*v),
            Value::Null => None,
            other => handle_type_mismatch(index, "decimal128", other),
        }
    }
}

/// Handle type mismatch with debug assertion
#[inline]
fn handle_type_mismatch<T>(index: usize, expected: &str, actual: &Value) -> Option<T> {
    if cfg!(debug_assertions) {
        panic!("Expected `{expected}` value at index {index}, got {actual:?}")
    }
    None
}

/// Type-safe value wrapper for all GreptimeDB data types
#[derive(Debug, Clone)]
pub enum Value {
    // Boolean
    Boolean(bool),

    // Integer types
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),

    // Float types
    Float32(f32),
    Float64(f64),

    // String and Binary types
    Binary(Vec<u8>),
    String(String),

    // Date and Time types
    Date(i32),     // Days since Unix epoch
    Datetime(i64), // Milliseconds since Unix epoch

    // Timestamp types
    TimestampSecond(i64),
    TimestampMillisecond(i64),
    TimestampMicrosecond(i64),
    TimestampNanosecond(i64),

    // Time types (time of day without date)
    TimeSecond(i32),
    TimeMillisecond(i32),
    TimeMicrosecond(i64),
    TimeNanosecond(i64),

    // Decimal type (`precision` and `scale` are placed in the column schema)
    Decimal128(i128),

    // JSON type (stored as string)
    Json(String),

    // Null value
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bool_correct_types() {
        let row = Row::from_values(vec![
            Value::Boolean(true),
            Value::Boolean(false),
            Value::Null,
        ]);

        // Test correct type
        assert_eq!(row.get_bool(0), Some(true));
        assert_eq!(row.get_bool(1), Some(false));

        // Test null value
        assert_eq!(row.get_bool(2), None);
    }

    #[test]
    fn test_get_bool_unchecked_correct_types() {
        let row = Row::from_values(vec![
            Value::Boolean(false),
            Value::Boolean(true),
            Value::Null,
        ]);

        unsafe {
            // Test correct type
            assert_eq!(row.get_bool_unchecked(0), Some(false));
            assert_eq!(row.get_bool_unchecked(1), Some(true));

            // Test null value
            assert_eq!(row.get_bool_unchecked(2), None);
        }
    }

    #[test]
    #[should_panic(expected = "Expected `boolean` value at index 0, got Int32(42)")]
    fn test_get_bool_type_mismatch_debug_assert() {
        let row = Row::from_values(vec![Value::Int32(42)]);

        // This should trigger debug_assert! in debug mode
        let _ = row.get_bool(0);
    }

    #[test]
    #[should_panic(expected = "Expected `boolean` value at index 0, got String(\"test\")")]
    fn test_get_bool_unchecked_type_mismatch_debug_assert() {
        let row = Row::from_values(vec![Value::String("test".to_string())]);

        unsafe {
            // This should trigger debug_assert! in debug mode
            let _ = row.get_bool_unchecked(0);
        }
    }
}
