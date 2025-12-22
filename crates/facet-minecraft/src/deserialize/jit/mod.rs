use cranelift_jit::{JITBuilder, JITModule};
use facet_format::{
    FormatJitParser,
    jit::{FunctionBuilder, JitCursor, JitFormat, JitStringValue, Value},
};

use crate::deserialize::McDeserializer;

mod helpers;

/// An implementation of [`JitFormat`] for [`McDeserializer`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct McJitFormat;

impl<'de> FormatJitParser<'de> for McDeserializer<'de> {
    type FormatJit = McJitFormat;

    fn jit_input(&self) -> &'de [u8] { todo!() }

    fn jit_pos(&self) -> Option<usize> { todo!() }

    fn jit_set_pos(&mut self, _pos: usize) { todo!() }

    fn jit_format(&self) -> Self::FormatJit { McJitFormat }

    fn jit_error(&self, _input: &'de [u8], _error_pos: usize, _error_code: i32) -> Self::Error {
        todo!()
    }
}

impl JitFormat for McJitFormat {
    const MAP_STATE_ALIGN: u32 = 1;
    const MAP_STATE_SIZE: u32 = 0;
    const PROVIDES_SEQ_COUNT: bool = false;
    const SEQ_STATE_ALIGN: u32 = 1;
    const SEQ_STATE_SIZE: u32 = 0;

    fn register_helpers(_builder: &mut JITBuilder) { todo!() }

    fn emit_skip_ws(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> Value {
        todo!()
    }

    fn emit_skip_value(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> Value {
        todo!()
    }

    fn emit_peek_null(&self, _b: &mut FunctionBuilder, _c: &mut JitCursor) -> (Value, Value) {
        todo!()
    }

    fn emit_consume_null(&self, _b: &mut FunctionBuilder, _c: &mut JitCursor) -> Value { todo!() }

    fn emit_parse_bool(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_parse_u8(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_parse_i64(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_parse_u64(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_parse_f64(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_parse_string(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
    ) -> (JitStringValue, Value) {
        todo!()
    }

    fn emit_seq_begin(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_seq_is_end(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_seq_next(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> Value {
        todo!()
    }

    fn emit_map_begin(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> Value {
        todo!()
    }

    fn emit_map_is_end(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> (Value, Value) {
        todo!()
    }

    fn emit_map_read_key(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> (JitStringValue, Value) {
        todo!()
    }

    fn emit_map_kv_sep(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> Value {
        todo!()
    }

    fn emit_map_next(
        &self,
        _module: &mut JITModule,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _state_ptr: Value,
    ) -> Value {
        todo!()
    }

    fn helper_seq_begin() -> Option<&'static str> { None }

    fn helper_seq_is_end() -> Option<&'static str> { None }

    fn helper_seq_next() -> Option<&'static str> { None }

    fn helper_parse_bool() -> Option<&'static str> { None }

    fn helper_parse_i64() -> Option<&'static str> { None }

    fn helper_parse_u64() -> Option<&'static str> { None }

    fn helper_parse_f64() -> Option<&'static str> { None }

    fn helper_parse_string() -> Option<&'static str> { None }

    fn emit_seq_bulk_copy_u8(
        &self,
        _b: &mut FunctionBuilder,
        _c: &mut JitCursor,
        _count: Value,
        _dest_ptr: Value,
    ) -> Option<Value> {
        None
    }

    fn emit_key_normalize(&self, _b: &mut FunctionBuilder, _key: &mut JitStringValue) {}
}
