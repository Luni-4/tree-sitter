#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, core_intrinsics, extern_types,
           label_break_value, ptr_wrapping_offset_from, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec)
     -> libc::c_int;
    #[no_mangle]
    fn iswalnum(__wc: wint_t) -> libc::c_int;
    #[no_mangle]
    fn iswspace(__wc: wint_t) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type TSSymbol = uint16_t;
pub type TSFieldId = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLanguage {
    pub version: uint32_t,
    pub symbol_count: uint32_t,
    pub alias_count: uint32_t,
    pub token_count: uint32_t,
    pub external_token_count: uint32_t,
    pub symbol_names: *mut *const libc::c_char,
    pub symbol_metadata: *const TSSymbolMetadata,
    pub parse_table: *const uint16_t,
    pub parse_actions: *const TSParseActionEntry,
    pub lex_modes: *const TSLexMode,
    pub alias_sequences: *const TSSymbol,
    pub max_alias_sequence_length: uint16_t,
    pub lex_fn: Option<unsafe extern "C" fn(_: *mut TSLexer, _: TSStateId)
                           -> bool>,
    pub keyword_lex_fn: Option<unsafe extern "C" fn(_: *mut TSLexer,
                                                    _: TSStateId) -> bool>,
    pub keyword_capture_token: TSSymbol,
    pub external_scanner: C2RustUnnamed,
    pub field_count: uint32_t,
    pub field_map_slices: *const TSFieldMapSlice,
    pub field_map_entries: *const TSFieldMapEntry,
    pub field_names: *mut *const libc::c_char,
    pub large_state_count: uint32_t,
    pub small_parse_table: *const uint16_t,
    pub small_parse_table_map: *const uint32_t,
    pub public_symbol_map: *const TSSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSFieldMapEntry {
    pub field_id: TSFieldId,
    pub child_index: uint8_t,
    pub inherited: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSFieldMapSlice {
    pub index: uint16_t,
    pub length: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub states: *const bool,
    pub symbol_map: *const TSSymbol,
    pub create: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    pub scan: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: *mut TSLexer, _: *const bool)
                         -> bool>,
    pub serialize: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                               _: *mut libc::c_char)
                              -> libc::c_uint>,
    pub deserialize: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: *const libc::c_char,
                                                 _: libc::c_uint) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLexer {
    pub lookahead: int32_t,
    pub result_symbol: TSSymbol,
    pub advance: Option<unsafe extern "C" fn(_: *mut TSLexer, _: bool) -> ()>,
    pub mark_end: Option<unsafe extern "C" fn(_: *mut TSLexer) -> ()>,
    pub get_column: Option<unsafe extern "C" fn(_: *mut TSLexer) -> uint32_t>,
    pub is_at_included_range_start: Option<unsafe extern "C" fn(_:
                                                                    *const TSLexer)
                                               -> bool>,
    pub eof: Option<unsafe extern "C" fn(_: *const TSLexer) -> bool>,
}
pub type TSStateId = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLexMode {
    pub lex_state: uint16_t,
    pub external_lex_state: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union TSParseActionEntry {
    pub action: TSParseAction,
    pub entry: C2RustUnnamed_0,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub count: uint8_t,
    #[bitfield(name = "reusable", ty = "bool", bits = "0..=0")]
    pub reusable: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct TSParseAction {
    pub params: C2RustUnnamed_1,
    #[bitfield(name = "type_0", ty = "TSParseActionType", bits = "0..=3")]
    pub type_0: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
pub type TSParseActionType = libc::c_uint;
pub const TSParseActionTypeRecover: TSParseActionType = 3;
pub const TSParseActionTypeAccept: TSParseActionType = 2;
pub const TSParseActionTypeReduce: TSParseActionType = 1;
pub const TSParseActionTypeShift: TSParseActionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub shift: C2RustUnnamed_3,
    pub reduce: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub symbol: TSSymbol,
    pub dynamic_precedence: int16_t,
    pub child_count: uint8_t,
    pub production_id: uint8_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub state: TSStateId,
    #[bitfield(name = "extra", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "repetition", ty = "bool", bits = "1..=1")]
    pub extra_repetition: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct TSSymbolMetadata {
    #[bitfield(name = "visible", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "named", ty = "bool", bits = "1..=1")]
    pub visible_named: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSParser {
    pub lexer: Lexer,
    pub stack: *mut Stack,
    pub tree_pool: SubtreePool,
    pub language: *const TSLanguage,
    pub reduce_actions: ReduceActionSet,
    pub finished_tree: Subtree,
    pub scratch_tree_data: SubtreeHeapData,
    pub scratch_tree: MutableSubtree,
    pub token_cache: TokenCache,
    pub reusable_node: ReusableNode,
    pub external_scanner_payload: *mut libc::c_void,
    pub dot_graph_file: *mut FILE,
    pub end_clock: TSClock,
    pub timeout_duration: TSDuration,
    pub accept_count: libc::c_uint,
    pub operation_count: libc::c_uint,
    pub cancellation_flag: *const size_t,
    pub old_tree: Subtree,
    pub included_range_differences: TSRangeArray,
    pub included_range_difference_index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSRangeArray {
    pub contents: *mut TSRange,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSRange {
    pub start_point: TSPoint,
    pub end_point: TSPoint,
    pub start_byte: uint32_t,
    pub end_byte: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSPoint {
    pub row: uint32_t,
    pub column: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Subtree {
    pub data: SubtreeInlineData,
    pub ptr: *const SubtreeHeapData,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SubtreeHeapData {
    pub ref_count: uint32_t,
    pub padding: Length,
    pub size: Length,
    pub lookahead_bytes: uint32_t,
    pub error_cost: uint32_t,
    pub child_count: uint32_t,
    pub symbol: TSSymbol,
    pub parse_state: TSStateId,
    #[bitfield(name = "visible", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "named", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "extra", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "fragile_left", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "fragile_right", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "has_changes", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "has_external_tokens", ty = "bool", bits = "6..=6")]
    #[bitfield(name = "is_missing", ty = "bool", bits = "7..=7")]
    #[bitfield(name = "is_keyword", ty = "bool", bits = "8..=8")]
    pub visible_named_extra_fragile_left_fragile_right_has_changes_has_external_tokens_is_missing_is_keyword: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub c2rust_unnamed: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub external_scanner_state: ExternalScannerState,
    pub lookahead_char: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalScannerState {
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub length: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub long_data: *mut libc::c_char,
    pub short_data: [libc::c_char; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub children: *mut Subtree,
    pub visible_child_count: uint32_t,
    pub named_child_count: uint32_t,
    pub node_count: uint32_t,
    pub repeat_depth: uint32_t,
    pub dynamic_precedence: int32_t,
    pub production_id: uint16_t,
    pub first_leaf: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub symbol: TSSymbol,
    pub parse_state: TSStateId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Length {
    pub bytes: uint32_t,
    pub extent: TSPoint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SubtreeInlineData {
    #[bitfield(name = "is_inline", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "visible", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "named", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "extra", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "has_changes", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "is_missing", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "is_keyword", ty = "bool", bits = "6..=6")]
    pub is_inline_visible_named_extra_has_changes_is_missing_is_keyword: [u8; 1],
    pub symbol: uint8_t,
    pub padding_bytes: uint8_t,
    pub size_bytes: uint8_t,
    pub padding_columns: uint8_t,
    #[bitfield(name = "padding_rows", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "lookahead_bytes", ty = "uint8_t", bits = "4..=7")]
    pub padding_rows_lookahead_bytes: [u8; 1],
    pub parse_state: uint16_t,
}
pub type TSDuration = uint64_t;
pub type TSClock = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReusableNode {
    pub stack: C2RustUnnamed_8,
    pub last_external_token: Subtree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub contents: *mut StackEntry,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackEntry {
    pub tree: Subtree,
    pub child_index: uint32_t,
    pub byte_offset: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenCache {
    pub token: Subtree,
    pub last_external_token: Subtree,
    pub byte_index: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MutableSubtree {
    pub data: SubtreeInlineData,
    pub ptr: *mut SubtreeHeapData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReduceActionSet {
    pub contents: *mut ReduceAction,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReduceAction {
    pub count: uint32_t,
    pub symbol: TSSymbol,
    pub dynamic_precedence: libc::c_int,
    pub production_id: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtreePool {
    pub free_trees: MutableSubtreeArray,
    pub tree_stack: MutableSubtreeArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MutableSubtreeArray {
    pub contents: *mut MutableSubtree,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stack {
    pub heads: C2RustUnnamed_10,
    pub slices: StackSliceArray,
    pub iterators: C2RustUnnamed_9,
    pub node_pool: StackNodeArray,
    pub base_node: *mut StackNode,
    pub subtree_pool: *mut SubtreePool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackNode {
    pub state: TSStateId,
    pub position: Length,
    pub links: [StackLink; 8],
    pub link_count: libc::c_ushort,
    pub ref_count: uint32_t,
    pub error_cost: libc::c_uint,
    pub node_count: libc::c_uint,
    pub dynamic_precedence: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackLink {
    pub node: *mut StackNode,
    pub subtree: Subtree,
    pub is_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackNodeArray {
    pub contents: *mut *mut StackNode,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub contents: *mut StackIterator,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackIterator {
    pub node: *mut StackNode,
    pub subtrees: SubtreeArray,
    pub subtree_count: uint32_t,
    pub is_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtreeArray {
    pub contents: *mut Subtree,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackSliceArray {
    pub contents: *mut StackSlice,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackSlice {
    pub subtrees: SubtreeArray,
    pub version: StackVersion,
}
pub type StackVersion = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub contents: *mut StackHead,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackHead {
    pub node: *mut StackNode,
    pub last_external_token: Subtree,
    pub summary: *mut StackSummary,
    pub node_count_at_last_error: libc::c_uint,
    pub lookahead_when_paused: TSSymbol,
    pub status: StackStatus,
}
pub type StackStatus = libc::c_uint;
pub const StackStatusHalted: StackStatus = 2;
pub const StackStatusPaused: StackStatus = 1;
pub const StackStatusActive: StackStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackSummary {
    pub contents: *mut StackSummaryEntry,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackSummaryEntry {
    pub position: Length,
    pub depth: libc::c_uint,
    pub state: TSStateId,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lexer {
    pub data: TSLexer,
    pub current_position: Length,
    pub token_start_position: Length,
    pub token_end_position: Length,
    pub included_ranges: *mut TSRange,
    pub included_range_count: size_t,
    pub current_included_range_index: size_t,
    pub chunk: *const libc::c_char,
    pub chunk_start: uint32_t,
    pub chunk_size: uint32_t,
    pub lookahead_size: uint32_t,
    pub input: TSInput,
    pub logger: TSLogger,
    pub debug_buffer: [libc::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLogger {
    pub payload: *mut libc::c_void,
    pub log: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: TSLogType,
                                         _: *const libc::c_char) -> ()>,
}
pub type TSLogType = libc::c_uint;
pub const TSLogTypeLex: TSLogType = 1;
pub const TSLogTypeParse: TSLogType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSInput {
    pub payload: *mut libc::c_void,
    pub read: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: uint32_t,
                                          _: TSPoint, _: *mut uint32_t)
                         -> *const libc::c_char>,
    pub encoding: TSInputEncoding,
}
pub type TSInputEncoding = libc::c_uint;
pub const TSInputEncodingUTF16: TSInputEncoding = 1;
pub const TSInputEncodingUTF8: TSInputEncoding = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSTree {
    pub root: Subtree,
    pub language: *const TSLanguage,
    pub parent_cache: *mut ParentCacheEntry,
    pub parent_cache_start: uint32_t,
    pub parent_cache_size: uint32_t,
    pub included_ranges: *mut TSRange,
    pub included_range_count: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ParentCacheEntry {
    pub child: *const Subtree,
    pub parent: *const Subtree,
    pub position: Length,
    pub alias_symbol: TSSymbol,
}
/*
 * TSQuery - A tree query, compiled from a string of S-expressions. The query
 * itself is immutable. The mutable state used in the process of executing the
 * query is stored in a `TSQueryCursor`.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQuery {
    pub captures: SymbolTable,
    pub predicate_values: SymbolTable,
    pub steps: C2RustUnnamed_15,
    pub pattern_map: C2RustUnnamed_14,
    pub predicate_steps: C2RustUnnamed_13,
    pub predicates_by_pattern: C2RustUnnamed_12,
    pub start_bytes_by_pattern: C2RustUnnamed_11,
    pub language: *const TSLanguage,
    pub wildcard_root_pattern_count: uint16_t,
    pub symbol_map: *mut TSSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub contents: *mut uint32_t,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub contents: *mut Slice,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Slice {
    pub offset: uint32_t,
    pub length: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub contents: *mut TSQueryPredicateStep,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryPredicateStep {
    pub type_0: TSQueryPredicateStepType,
    pub value_id: uint32_t,
}
pub type TSQueryPredicateStepType = libc::c_uint;
pub const TSQueryPredicateStepTypeString: TSQueryPredicateStepType = 2;
pub const TSQueryPredicateStepTypeCapture: TSQueryPredicateStepType = 1;
pub const TSQueryPredicateStepTypeDone: TSQueryPredicateStepType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub contents: *mut PatternEntry,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PatternEntry {
    pub step_index: uint16_t,
    pub pattern_index: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub contents: *mut QueryStep,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct QueryStep {
    pub symbol: TSSymbol,
    pub field: TSFieldId,
    pub capture_ids: [uint16_t; 3],
    pub alternative_index: uint16_t,
    pub depth: uint16_t,
    #[bitfield(name = "contains_captures", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "is_pattern_start", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "is_immediate", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "is_last_child", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "is_pass_through", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "is_dead_end", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "alternative_is_immediate", ty = "bool", bits =
               "6..=6")]
    pub contains_captures_is_pattern_start_is_immediate_is_last_child_is_pass_through_is_dead_end_alternative_is_immediate: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolTable {
    pub characters: C2RustUnnamed_17,
    pub slices: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub contents: *mut Slice,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub contents: *mut libc::c_char,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
/*
 * TSQueryCursor - A stateful struct used to execute a query on a tree.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryCursor {
    pub query: *const TSQuery,
    pub cursor: TSTreeCursor,
    pub states: C2RustUnnamed_19,
    pub finished_states: C2RustUnnamed_18,
    pub capture_list_pool: CaptureListPool,
    pub depth: uint32_t,
    pub start_byte: uint32_t,
    pub end_byte: uint32_t,
    pub next_state_id: uint32_t,
    pub start_point: TSPoint,
    pub end_point: TSPoint,
    pub ascending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaptureListPool {
    pub list: [CaptureList; 32],
    pub empty_list: CaptureList,
    pub usage_map: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaptureList {
    pub contents: *mut TSQueryCapture,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryCapture {
    pub node: TSNode,
    pub index: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSNode {
    pub context: [uint32_t; 4],
    pub id: *const libc::c_void,
    pub tree: *const TSTree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub contents: *mut QueryState,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct QueryState {
    pub id: uint32_t,
    pub start_depth: uint16_t,
    pub step_index: uint16_t,
    pub pattern_index: uint16_t,
    pub capture_list_id: uint16_t,
    #[bitfield(name = "consumed_capture_count", ty = "uint16_t", bits =
               "0..=13")]
    #[bitfield(name = "seeking_immediate_match", ty = "bool", bits =
               "14..=14")]
    #[bitfield(name = "has_in_progress_alternatives", ty = "bool", bits =
               "15..=15")]
    pub consumed_capture_count_seeking_immediate_match_has_in_progress_alternatives: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub contents: *mut QueryState,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSTreeCursor {
    pub tree: *const libc::c_void,
    pub id: *const libc::c_void,
    pub context: [uint32_t; 2],
}
pub type TSSymbolType = libc::c_uint;
pub const TSSymbolTypeAuxiliary: TSSymbolType = 2;
pub const TSSymbolTypeAnonymous: TSSymbolType = 1;
pub const TSSymbolTypeRegular: TSSymbolType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSInputEdit {
    pub start_byte: uint32_t,
    pub old_end_byte: uint32_t,
    pub new_end_byte: uint32_t,
    pub start_point: TSPoint,
    pub old_end_point: TSPoint,
    pub new_end_point: TSPoint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryMatch {
    pub id: uint32_t,
    pub pattern_index: uint16_t,
    pub capture_count: uint16_t,
    pub captures: *const TSQueryCapture,
}
/* ***************************/
/* Section - ABI Versioning */
/* ***************************/
/* *
 * The latest ABI version that is supported by the current version of the
 * library. When Languages are generated by the Tree-sitter CLI, they are
 * assigned an ABI version number that corresponds to the current CLI version.
 * The Tree-sitter library is generally backwards-compatible with languages
 * generated using older CLI versions, but is not forwards-compatible.
 */
/* *
 * The earliest ABI version that is supported by the current version of the
 * library.
 */
/* ******************/
/* Section - Types */
/* ******************/
pub type TSQueryError = libc::c_uint;
pub const TSQueryErrorCapture: TSQueryError = 4;
pub const TSQueryErrorField: TSQueryError = 3;
pub const TSQueryErrorNodeType: TSQueryError = 2;
pub const TSQueryErrorSyntax: TSQueryError = 1;
pub const TSQueryErrorNone: TSQueryError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeCursorEntry {
    pub subtree: *const Subtree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackEntry_0 {
    pub tree: *mut Subtree,
    pub edit: Edit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Edit {
    pub start: Length,
    pub old_end: Length,
    pub new_end: Length,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
pub type UnicodeDecodeFunction
    =
    Option<unsafe extern "C" fn(_: *const uint8_t, _: uint32_t,
                                _: *mut int32_t) -> uint32_t>;
/* *
 * Define UChar32 as a type for single Unicode code points.
 * UChar32 is a signed 32-bit integer (same as int32_t).
 *
 * The Unicode code point range is 0..0x10ffff.
 * All other values (negative or >=0x110000) are illegal as Unicode code points.
 * They may be used as sentinel values to indicate "done", "error"
 * or similar non-code point conditions.
 *
 * Before ICU 2.4 (Jitterbug 2146), UChar32 was defined
 * to be wchar_t if that is 32 bits wide (wchar_t may be signed or unsigned)
 * or else to be uint32_t.
 * That is, the definition of UChar32 was platform-dependent.
 *
 * @see U_SENTINEL
 * @stable ICU 2.4
 */
pub type UChar32 = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub contents: *mut *mut StackNode,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SummarizeStackSession {
    pub summary: *mut StackSummary,
    pub max_depth: libc::c_uint,
}
pub type StackAction = libc::c_uint;
pub const StackActionNone: C2RustUnnamed_24 = 0;
pub const StackActionStop: C2RustUnnamed_24 = 1;
pub type StackCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *const StackIterator)
               -> StackAction>;
pub const StackActionPop: C2RustUnnamed_24 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableEntry {
    pub actions: *const TSParseAction,
    pub action_count: uint32_t,
    pub is_reusable: bool,
}
pub const ErrorComparisonTakeRight: ErrorComparison = 4;
pub const ErrorComparisonPreferRight: ErrorComparison = 3;
pub const ErrorComparisonNone: ErrorComparison = 2;
pub const ErrorComparisonPreferLeft: ErrorComparison = 1;
pub const ErrorComparisonTakeLeft: ErrorComparison = 0;
pub type ErrorComparison = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorStatus {
    pub cost: libc::c_uint,
    pub node_count: libc::c_uint,
    pub dynamic_precedence: libc::c_int,
    pub is_in_error: bool,
}
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSStringInput {
    pub string: *const libc::c_char,
    pub length: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub contents: *mut StackEntry_0,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub contents: *mut TreeCursorEntry,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeCursor {
    pub tree: *const TSTree,
    pub stack: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Iterator_0 {
    pub cursor: TreeCursor,
    pub language: *const TSLanguage,
    pub visible_depth: libc::c_uint,
    pub in_padding: bool,
}
pub const IteratorDiffers: IteratorComparison = 0;
pub const IteratorMayDiffer: IteratorComparison = 1;
pub const IteratorMatches: IteratorComparison = 2;
pub type IteratorComparison = libc::c_uint;
pub const _ISprint: C2RustUnnamed_25 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeChildIterator {
    pub parent: Subtree,
    pub tree: *const TSTree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
    pub alias_sequence: *const TSSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CursorChildIterator {
    pub parent: Subtree,
    pub tree: *const TSTree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
    pub alias_sequence: *const TSSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub input: *const libc::c_char,
    pub end: *const libc::c_char,
    pub next: int32_t,
    pub next_size: uint8_t,
}
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub contents: *mut uint32_t,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
// Create a stack.
// Release the memory reserved for a given stack.
// Get the stack's current number of versions.
// Get the state at the top of the given version of the stack. If the stack is
// empty, this returns the initial state, 0.
// Get the last external token associated with a given version of the stack.
// Set the last external token associated with a given version of the stack.
// Get the position of the given version of the stack within the document.
// Push a tree and state onto the given version of the stack.
//
// This transfers ownership of the tree to the Stack. Callers that
// need to retain ownership of the tree for their own purposes should
// first retain the tree.
// Pop the given number of entries from the given version of the stack. This
// operation can increase the number of stack versions by revealing multiple
// versions which had previously been merged. It returns an array that
// specifies the index of each revealed version and the trees that were
// removed from that version.
// Remove an error at the top of the given version of the stack.
// Remove any pending trees from the top of the given version of the stack.
// Remove any all trees from the given version of the stack.
// Get the maximum number of tree nodes reachable from this version of the stack
// since the last error was detected.
// Compute a summary of all the parse states near the top of the given
// version of the stack and store the summary for later retrieval.
// Retrieve a summary of all the parse states near the top of the
// given version of the stack.
// Get the total cost of all errors on the given version of the stack.
// Merge the given two stack versions if possible, returning true
// if they were successfully merged and false otherwise.
// Determine whether the given two stack versions can be merged.
// Remove the given version from the stack.
pub type StackIterateCallback
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: TSStateId,
                                _: uint32_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackIterateSession {
    pub payload: *mut libc::c_void,
    pub callback: StackIterateCallback,
}
pub type C2RustUnnamed_24 = libc::c_uint;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_25 = 8;
pub const _ISpunct: C2RustUnnamed_25 = 4;
pub const _IScntrl: C2RustUnnamed_25 = 2;
pub const _ISblank: C2RustUnnamed_25 = 1;
pub const _ISgraph: C2RustUnnamed_25 = 32768;
pub const _ISspace: C2RustUnnamed_25 = 8192;
pub const _ISxdigit: C2RustUnnamed_25 = 4096;
pub const _ISdigit: C2RustUnnamed_25 = 2048;
pub const _ISalpha: C2RustUnnamed_25 = 1024;
pub const _ISlower: C2RustUnnamed_25 = 512;
pub const _ISupper: C2RustUnnamed_25 = 256;
static mut TS_TREE_STATE_NONE: TSStateId =
    (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as TSStateId;
#[inline]
unsafe extern "C" fn point_lte(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row < b.row || a.row == b.row && a.column <= b.column;
}
#[inline]
unsafe extern "C" fn point_lt(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row < b.row || a.row == b.row && a.column < b.column;
}
#[inline]
unsafe extern "C" fn ts_language_field_map(mut self_0: *const TSLanguage,
                                           mut production_id: uint32_t,
                                           mut start:
                                               *mut *const TSFieldMapEntry,
                                           mut end:
                                               *mut *const TSFieldMapEntry) {
    if (*self_0).version < 10 as libc::c_int as libc::c_uint ||
           (*self_0).field_count == 0 as libc::c_int as libc::c_uint {
        *start = 0 as *const TSFieldMapEntry;
        *end = 0 as *const TSFieldMapEntry;
        return
    }
    let mut slice: TSFieldMapSlice =
        *(*self_0).field_map_slices.offset(production_id as isize);
    *start =
        &*(*self_0).field_map_entries.offset(slice.index as isize) as
            *const TSFieldMapEntry;
    *end =
        (&*(*self_0).field_map_entries.offset(slice.index as isize) as
             *const TSFieldMapEntry).offset(slice.length as libc::c_int as
                                                isize);
}
// #define DEBUG_GET_CHANGED_RANGES
unsafe extern "C" fn iterator_get_visible_state(mut self_0: *const Iterator_0,
                                                mut tree: *mut Subtree,
                                                mut alias_symbol:
                                                    *mut TSSymbol,
                                                mut start_byte:
                                                    *mut uint32_t) {
    let mut i: uint32_t =
        (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint);
    if (*self_0).in_padding {
        if i == 0 as libc::c_int as libc::c_uint { return }
        i = i.wrapping_sub(1)
    }
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) >
              0 as libc::c_int as libc::c_uint {
        let mut entry: TreeCursorEntry =
            *(*self_0).cursor.stack.contents.offset(i as isize);
        if i > 0 as libc::c_int as libc::c_uint {
            let mut parent: *const Subtree =
                (*(*self_0).cursor.stack.contents.offset(i.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                             as
                                                             isize)).subtree;
            let mut alias_sequence: *const TSSymbol =
                ts_language_alias_sequence((*self_0).language,
                                           (*(*parent).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                               as uint32_t);
            if !alias_sequence.is_null() {
                *alias_symbol =
                    *alias_sequence.offset(entry.structural_child_index as
                                               isize)
            }
        }
        if ts_subtree_visible(*entry.subtree) as libc::c_int != 0 ||
               *alias_symbol as libc::c_int != 0 {
            *tree = *entry.subtree;
            *start_byte = entry.position.bytes;
            break ;
        } else { i = i.wrapping_sub(1) }
    };
}
unsafe extern "C" fn iterator_compare(mut old_iter: *const Iterator_0,
                                      mut new_iter: *const Iterator_0)
 -> IteratorComparison {
    let mut old_tree: Subtree = Subtree{ptr: 0 as *const SubtreeHeapData,};
    let mut new_tree: Subtree = Subtree{ptr: 0 as *const SubtreeHeapData,};
    let mut old_start: uint32_t = 0 as libc::c_int as uint32_t;
    let mut new_start: uint32_t = 0 as libc::c_int as uint32_t;
    let mut old_alias_symbol: TSSymbol = 0 as libc::c_int as TSSymbol;
    let mut new_alias_symbol: TSSymbol = 0 as libc::c_int as TSSymbol;
    iterator_get_visible_state(old_iter, &mut old_tree, &mut old_alias_symbol,
                               &mut old_start);
    iterator_get_visible_state(new_iter, &mut new_tree, &mut new_alias_symbol,
                               &mut new_start);
    if old_tree.ptr.is_null() && new_tree.ptr.is_null() {
        return IteratorMatches
    }
    if old_tree.ptr.is_null() || new_tree.ptr.is_null() {
        return IteratorDiffers
    }
    if old_alias_symbol as libc::c_int == new_alias_symbol as libc::c_int &&
           ts_subtree_symbol(old_tree) as libc::c_int ==
               ts_subtree_symbol(new_tree) as libc::c_int {
        if old_start == new_start && !ts_subtree_has_changes(old_tree) &&
               ts_subtree_symbol(old_tree) as libc::c_int !=
                   -(1 as libc::c_int) as TSSymbol as libc::c_int &&
               ts_subtree_size(old_tree).bytes ==
                   ts_subtree_size(new_tree).bytes &&
               ts_subtree_parse_state(old_tree) as libc::c_int !=
                   TS_TREE_STATE_NONE as libc::c_int &&
               ts_subtree_parse_state(new_tree) as libc::c_int !=
                   TS_TREE_STATE_NONE as libc::c_int &&
               (ts_subtree_parse_state(old_tree) as libc::c_int ==
                    0 as libc::c_int) as libc::c_int ==
                   (ts_subtree_parse_state(new_tree) as libc::c_int ==
                        0 as libc::c_int) as libc::c_int {
            return IteratorMatches
        } else { return IteratorMayDiffer }
    }
    return IteratorDiffers;
}
#[inline]
unsafe extern "C" fn length_min(mut len1: Length, mut len2: Length)
 -> Length {
    return if len1.bytes < len2.bytes { len1 } else { len2 };
}
unsafe extern "C" fn iterator_end_position(mut self_0: *mut Iterator_0)
 -> Length {
    if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) <
           (*self_0).cursor.stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./get_changed_ranges.c\x00" as *const u8 as
                          *const libc::c_char,
                      135 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 41],
                                                &[libc::c_char; 41]>(b"Length iterator_end_position(Iterator *)\x00")).as_ptr());
    }
    let mut entry: TreeCursorEntry =
        *(&mut *(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                           as isize) as
              *mut TreeCursorEntry);
    let mut result: Length =
        length_add(entry.position, ts_subtree_padding(*entry.subtree));
    if (*self_0).in_padding {
        return result
    } else { return length_add(result, ts_subtree_size(*entry.subtree)) };
}
unsafe extern "C" fn iterator_descend(mut self_0: *mut Iterator_0,
                                      mut goal_position: uint32_t) -> bool {
    if (*self_0).in_padding { return 0 as libc::c_int != 0 }
    let mut did_descend: bool = false;
    loop  {
        did_descend = 0 as libc::c_int != 0;
        if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) <
               (*self_0).cursor.stack.size {
        } else {
            __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                              as *const u8 as *const libc::c_char,
                          b"lib/src/./get_changed_ranges.c\x00" as *const u8
                              as *const libc::c_char,
                          202 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 45],
                                                    &[libc::c_char; 45]>(b"_Bool iterator_descend(Iterator *, uint32_t)\x00")).as_ptr());
        }
        let mut entry: TreeCursorEntry =
            *(&mut *(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size.wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                               as isize) as
                  *mut TreeCursorEntry);
        let mut position: Length = entry.position;
        let mut structural_child_index: uint32_t =
            0 as libc::c_int as uint32_t;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut n: uint32_t = ts_subtree_child_count(*entry.subtree);
        while i < n {
            let mut child: *const Subtree =
                &mut *(*(*entry.subtree).ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                                as
                                                                                                isize)
                    as *mut Subtree;
            let mut child_left: Length =
                length_add(position, ts_subtree_padding(*child));
            let mut child_right: Length =
                length_add(child_left, ts_subtree_size(*child));
            if child_right.bytes > goal_position {
                array__grow(&mut (*self_0).cursor.stack as
                                *mut C2RustUnnamed_22 as *mut VoidArray,
                            1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh0 = (*self_0).cursor.stack.size;
                (*self_0).cursor.stack.size =
                    (*self_0).cursor.stack.size.wrapping_add(1);
                *(*self_0).cursor.stack.contents.offset(fresh0 as isize) =
                    {
                        let mut init =
                            TreeCursorEntry{subtree: child,
                                            position: position,
                                            child_index: i,
                                            structural_child_index:
                                                structural_child_index,};
                        init
                    };
                if iterator_tree_is_visible(self_0) {
                    if child_left.bytes > goal_position {
                        (*self_0).in_padding = 1 as libc::c_int != 0
                    } else {
                        (*self_0).visible_depth =
                            (*self_0).visible_depth.wrapping_add(1)
                    }
                    return 1 as libc::c_int != 0
                }
                did_descend = 1 as libc::c_int != 0;
                break ;
            } else {
                position = child_right;
                if !ts_subtree_extra(*child) {
                    structural_child_index =
                        structural_child_index.wrapping_add(1)
                }
                i = i.wrapping_add(1)
            }
        }
        if !did_descend { break ; }
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn ts_calloc(mut count: size_t, mut size: size_t)
 -> *mut libc::c_void {
    let mut result: *mut libc::c_void = calloc(count, size);
    if count > 0 as libc::c_int as libc::c_ulong && result.is_null() {
        fprintf(stderr,
                b"tree-sitter failed to allocate %lu bytes\x00" as *const u8
                    as *const libc::c_char, count.wrapping_mul(size));
        exit(1 as libc::c_int);
    }
    return result;
}
#[inline]
unsafe extern "C" fn ts_free(mut buffer: *mut libc::c_void) { free(buffer); }
#[inline]
unsafe extern "C" fn array__grow(mut self_0: *mut VoidArray,
                                 mut count: size_t,
                                 mut element_size: size_t) {
    let mut new_size: size_t =
        ((*self_0).size as libc::c_ulong).wrapping_add(count);
    if new_size > (*self_0).capacity as libc::c_ulong {
        let mut new_capacity: size_t =
            (*self_0).capacity.wrapping_mul(2 as libc::c_int as libc::c_uint)
                as size_t;
        if new_capacity < 8 as libc::c_int as libc::c_ulong {
            new_capacity = 8 as libc::c_int as size_t
        }
        if new_capacity < new_size { new_capacity = new_size }
        array__reserve(self_0, element_size, new_capacity as uint32_t);
    };
}
#[inline]
unsafe extern "C" fn array__reserve(mut self_0: *mut VoidArray,
                                    mut element_size: size_t,
                                    mut new_capacity: uint32_t) {
    if new_capacity > (*self_0).capacity {
        if !(*self_0).contents.is_null() {
            (*self_0).contents =
                ts_realloc((*self_0).contents,
                           (new_capacity as
                                libc::c_ulong).wrapping_mul(element_size))
        } else {
            (*self_0).contents =
                ts_calloc(new_capacity as size_t, element_size)
        }
        (*self_0).capacity = new_capacity
    };
}
#[inline]
unsafe extern "C" fn ts_realloc(mut buffer: *mut libc::c_void,
                                mut size: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = realloc(buffer, size);
    if size > 0 as libc::c_int as libc::c_ulong && result.is_null() {
        fprintf(stderr,
                b"tree-sitter failed to reallocate %lu bytes\x00" as *const u8
                    as *const libc::c_char, size);
        exit(1 as libc::c_int);
    }
    return result;
}
#[inline]
unsafe extern "C" fn ts_subtree_to_mut_unsafe(mut self_0: Subtree)
 -> MutableSubtree {
    let mut result: MutableSubtree =
        MutableSubtree{data:
                           SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                                 [0; 1],
                                             symbol: 0,
                                             padding_bytes: 0,
                                             size_bytes: 0,
                                             padding_columns: 0,
                                             padding_rows_lookahead_bytes:
                                                 [0; 1],
                                             parse_state: 0,},};
    result.data = self_0.data;
    return result;
}
#[inline]
unsafe extern "C" fn array__delete(mut self_0: *mut VoidArray) {
    ts_free((*self_0).contents);
    (*self_0).contents = 0 as *mut libc::c_void;
    (*self_0).size = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn ts_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = malloc(size);
    if size > 0 as libc::c_int as libc::c_ulong && result.is_null() {
        fprintf(stderr,
                b"tree-sitter failed to allocate %lu bytes\x00" as *const u8
                    as *const libc::c_char, size);
        exit(1 as libc::c_int);
    }
    return result;
}
#[inline]
unsafe extern "C" fn length_zero() -> Length {
    let mut result: Length =
        {
            let mut init =
                Length{bytes: 0 as libc::c_int as uint32_t,
                       extent:
                           {
                               let mut init =
                                   TSPoint{row: 0 as libc::c_int as uint32_t,
                                           column:
                                               0 as libc::c_int as uint32_t,};
                               init
                           },};
            init
        };
    return result;
}
#[inline]
unsafe extern "C" fn ts_subtree_dynamic_precedence(mut self_0: Subtree)
 -> int32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 ||
                  (*self_0.ptr).child_count ==
                      0 as libc::c_int as libc::c_uint {
               0 as libc::c_int
           } else {
               (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.dynamic_precedence
           };
}
#[inline]
unsafe extern "C" fn ts_subtree_node_count(mut self_0: Subtree) -> uint32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 ||
                  (*self_0.ptr).child_count ==
                      0 as libc::c_int as libc::c_uint {
               1 as libc::c_int as libc::c_uint
           } else { (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.node_count };
}
#[inline]
unsafe extern "C" fn ts_subtree_total_size(mut self_0: Subtree) -> Length {
    return length_add(ts_subtree_padding(self_0), ts_subtree_size(self_0));
}
#[inline]
unsafe extern "C" fn ts_subtree_size(mut self_0: Subtree) -> Length {
    if self_0.data.is_inline() {
        let mut result: Length =
            {
                let mut init =
                    Length{bytes: self_0.data.size_bytes as uint32_t,
                           extent:
                               {
                                   let mut init =
                                       TSPoint{row:
                                                   0 as libc::c_int as
                                                       uint32_t,
                                               column:
                                                   self_0.data.size_bytes as
                                                       uint32_t,};
                                   init
                               },};
                init
            };
        return result
    } else { return (*self_0.ptr).size };
}
#[inline]
unsafe extern "C" fn ts_subtree_padding(mut self_0: Subtree) -> Length {
    if self_0.data.is_inline() {
        let mut result: Length =
            {
                let mut init =
                    Length{bytes: self_0.data.padding_bytes as uint32_t,
                           extent:
                               {
                                   let mut init =
                                       TSPoint{row:
                                                   self_0.data.padding_rows()
                                                       as uint32_t,
                                               column:
                                                   self_0.data.padding_columns
                                                       as uint32_t,};
                                   init
                               },};
                init
            };
        return result
    } else { return (*self_0.ptr).padding };
}
#[inline]
unsafe extern "C" fn length_add(mut len1: Length, mut len2: Length)
 -> Length {
    let mut result: Length =
        Length{bytes: 0, extent: TSPoint{row: 0, column: 0,},};
    result.bytes = len1.bytes.wrapping_add(len2.bytes);
    result.extent = point_add(len1.extent, len2.extent);
    return result;
}
#[inline]
unsafe extern "C" fn point_add(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if b.row > 0 as libc::c_int as libc::c_uint {
        return point__new(a.row.wrapping_add(b.row), b.column)
    } else { return point__new(a.row, a.column.wrapping_add(b.column)) };
}
#[inline]
unsafe extern "C" fn point__new(mut row: libc::c_uint,
                                mut column: libc::c_uint) -> TSPoint {
    let mut result: TSPoint =
        { let mut init = TSPoint{row: row, column: column,}; init };
    return result;
}
#[inline]
unsafe extern "C" fn ts_subtree_error_cost(mut self_0: Subtree) -> uint32_t {
    if ts_subtree_missing(self_0) {
        return (110 as libc::c_int + 500 as libc::c_int) as uint32_t
    } else {
        return if self_0.data.is_inline() as libc::c_int != 0 {
                   0 as libc::c_int as libc::c_uint
               } else { (*self_0.ptr).error_cost }
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_missing(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.is_missing() as libc::c_int
           } else { (*self_0.ptr).is_missing() as libc::c_int } != 0;
}
unsafe extern "C" fn iterator_advance(mut self_0: *mut Iterator_0) {
    if (*self_0).in_padding {
        (*self_0).in_padding = 0 as libc::c_int != 0;
        if iterator_tree_is_visible(self_0) {
            (*self_0).visible_depth = (*self_0).visible_depth.wrapping_add(1)
        } else { iterator_descend(self_0, 0 as libc::c_int as uint32_t); }
        return
    }
    loop  {
        if iterator_tree_is_visible(self_0) {
            (*self_0).visible_depth = (*self_0).visible_depth.wrapping_sub(1)
        }
        (*self_0).cursor.stack.size =
            (*self_0).cursor.stack.size.wrapping_sub(1);
        let mut entry: TreeCursorEntry =
            *(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size
                                                        as isize);
        if iterator_done(self_0) { return }
        if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint) <
               (*self_0).cursor.stack.size {
        } else {
            __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                              as *const u8 as *const libc::c_char,
                          b"lib/src/./get_changed_ranges.c\x00" as *const u8
                              as *const libc::c_char,
                          255 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 34],
                                                    &[libc::c_char; 34]>(b"void iterator_advance(Iterator *)\x00")).as_ptr());
        }
        let mut parent: *const Subtree =
            (*(&mut *(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size.wrapping_sub(1
                                                                                                         as
                                                                                                         libc::c_int
                                                                                                         as
                                                                                                         libc::c_uint)
                                                                as isize) as
                   *mut TreeCursorEntry)).subtree;
        let mut child_index: uint32_t =
            entry.child_index.wrapping_add(1 as libc::c_int as libc::c_uint);
        if !(ts_subtree_child_count(*parent) > child_index) { continue ; }
        let mut position: Length =
            length_add(entry.position, ts_subtree_total_size(*entry.subtree));
        let mut structural_child_index: uint32_t =
            entry.structural_child_index;
        if !ts_subtree_extra(*entry.subtree) {
            structural_child_index = structural_child_index.wrapping_add(1)
        }
        let mut next_child: *const Subtree =
            &mut *(*(*parent).ptr).c2rust_unnamed.c2rust_unnamed.children.offset(child_index
                                                                                     as
                                                                                     isize)
                as *mut Subtree;
        array__grow(&mut (*self_0).cursor.stack as *mut C2RustUnnamed_22 as
                        *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<TreeCursorEntry>() as
                        libc::c_ulong);
        let fresh1 = (*self_0).cursor.stack.size;
        (*self_0).cursor.stack.size =
            (*self_0).cursor.stack.size.wrapping_add(1);
        *(*self_0).cursor.stack.contents.offset(fresh1 as isize) =
            {
                let mut init =
                    TreeCursorEntry{subtree: next_child,
                                    position: position,
                                    child_index: child_index,
                                    structural_child_index:
                                        structural_child_index,};
                init
            };
        if iterator_tree_is_visible(self_0) {
            if ts_subtree_padding(*next_child).bytes >
                   0 as libc::c_int as libc::c_uint {
                (*self_0).in_padding = 1 as libc::c_int != 0
            } else {
                (*self_0).visible_depth =
                    (*self_0).visible_depth.wrapping_add(1)
            }
        } else { iterator_descend(self_0, 0 as libc::c_int as uint32_t); }
        break ;
    };
}
unsafe extern "C" fn iterator_tree_is_visible(mut self_0: *const Iterator_0)
 -> bool {
    if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) <
           (*self_0).cursor.stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./get_changed_ranges.c\x00" as *const u8 as
                          *const libc::c_char,
                      145 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"_Bool iterator_tree_is_visible(const Iterator *)\x00")).as_ptr());
    }
    let mut entry: TreeCursorEntry =
        *(&mut *(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                           as isize) as
              *mut TreeCursorEntry);
    if ts_subtree_visible(*entry.subtree) { return 1 as libc::c_int != 0 }
    if (*self_0).cursor.stack.size > 1 as libc::c_int as libc::c_uint {
        let mut parent: Subtree =
            *(*(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size.wrapping_sub(2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint)
                                                          as isize)).subtree;
        let mut alias_sequence: *const TSSymbol =
            ts_language_alias_sequence((*self_0).language,
                                       (*parent.ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                           as uint32_t);
        return !alias_sequence.is_null() &&
                   *alias_sequence.offset(entry.structural_child_index as
                                              isize) as libc::c_int !=
                       0 as libc::c_int
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn iterator_ascend(mut self_0: *mut Iterator_0) {
    if iterator_done(self_0) { return }
    if iterator_tree_is_visible(self_0) as libc::c_int != 0 &&
           !(*self_0).in_padding {
        (*self_0).visible_depth = (*self_0).visible_depth.wrapping_sub(1)
    }
    if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) <
           (*self_0).cursor.stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./get_changed_ranges.c\x00" as *const u8 as
                          *const libc::c_char,
                      192 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 33],
                                                &[libc::c_char; 33]>(b"void iterator_ascend(Iterator *)\x00")).as_ptr());
    }
    if (*(&mut *(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                           as isize) as
              *mut TreeCursorEntry)).child_index >
           0 as libc::c_int as libc::c_uint {
        (*self_0).in_padding = 0 as libc::c_int != 0
    }
    (*self_0).cursor.stack.size = (*self_0).cursor.stack.size.wrapping_sub(1);
}
unsafe extern "C" fn iterator_start_position(mut self_0: *mut Iterator_0)
 -> Length {
    if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) <
           (*self_0).cursor.stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./get_changed_ranges.c\x00" as *const u8 as
                          *const libc::c_char,
                      126 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"Length iterator_start_position(Iterator *)\x00")).as_ptr());
    }
    let mut entry: TreeCursorEntry =
        *(&mut *(*self_0).cursor.stack.contents.offset((*self_0).cursor.stack.size.wrapping_sub(1
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    libc::c_uint)
                                                           as isize) as
              *mut TreeCursorEntry);
    if (*self_0).in_padding {
        return entry.position
    } else {
        return length_add(entry.position, ts_subtree_padding(*entry.subtree))
    };
}
unsafe extern "C" fn iterator_done(mut self_0: *mut Iterator_0) -> bool {
    return (*self_0).cursor.stack.size == 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn iterator_new(mut cursor: *mut TreeCursor,
                                  mut tree: *const Subtree,
                                  mut language: *const TSLanguage)
 -> Iterator_0 {
    (*cursor).stack.size = 0 as libc::c_int as uint32_t;
    array__grow(&mut (*cursor).stack as *mut C2RustUnnamed_22 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<TreeCursorEntry>() as libc::c_ulong);
    let fresh2 = (*cursor).stack.size;
    (*cursor).stack.size = (*cursor).stack.size.wrapping_add(1);
    *(*cursor).stack.contents.offset(fresh2 as isize) =
        {
            let mut init =
                TreeCursorEntry{subtree: tree,
                                position: length_zero(),
                                child_index: 0 as libc::c_int as uint32_t,
                                structural_child_index:
                                    0 as libc::c_int as uint32_t,};
            init
        };
    return {
               let mut init =
                   Iterator_0{cursor: *cursor,
                              language: language,
                              visible_depth: 1 as libc::c_int as libc::c_uint,
                              in_padding: 0 as libc::c_int != 0,};
               init
           };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_get_changed_ranges(mut old_tree:
                                                           *const Subtree,
                                                       mut new_tree:
                                                           *const Subtree,
                                                       mut cursor1:
                                                           *mut TreeCursor,
                                                       mut cursor2:
                                                           *mut TreeCursor,
                                                       mut language:
                                                           *const TSLanguage,
                                                       mut included_range_differences:
                                                           *const TSRangeArray,
                                                       mut ranges:
                                                           *mut *mut TSRange)
 -> libc::c_uint {
    let mut results: TSRangeArray =
        {
            let mut init =
                TSRangeArray{contents: 0 as *mut TSRange,
                             size: 0 as libc::c_int as uint32_t,
                             capacity: 0 as libc::c_int as uint32_t,};
            init
        };
    let mut old_iter: Iterator_0 = iterator_new(cursor1, old_tree, language);
    let mut new_iter: Iterator_0 = iterator_new(cursor2, new_tree, language);
    let mut included_range_difference_index: libc::c_uint =
        0 as libc::c_int as libc::c_uint;
    let mut position: Length = iterator_start_position(&mut old_iter);
    let mut next_position: Length = iterator_start_position(&mut new_iter);
    if position.bytes < next_position.bytes {
        ts_range_array_add(&mut results, position, next_position);
        position = next_position
    } else if position.bytes > next_position.bytes {
        ts_range_array_add(&mut results, next_position, position);
        next_position = position
    }
    loop  {
        // Compare the old and new subtrees.
        let mut comparison: IteratorComparison =
            iterator_compare(&mut old_iter, &mut new_iter);
        // Even if the two subtrees appear to be identical, they could differ
    // internally if they contain a range of text that was previously
    // excluded from the parse, and is now included, or vice-versa.
        if comparison as libc::c_uint ==
               IteratorMatches as libc::c_int as libc::c_uint &&
               ts_range_array_intersects(included_range_differences,
                                         included_range_difference_index,
                                         position.bytes,
                                         iterator_end_position(&mut old_iter).bytes)
                   as libc::c_int != 0 {
            comparison = IteratorMayDiffer
        }
        let mut is_changed: bool = 0 as libc::c_int != 0;
        match comparison as libc::c_uint {
            2 => {
                // If the subtrees are definitely identical, move to the end
      // of both subtrees.
                next_position = iterator_end_position(&mut old_iter)
            }
            1 => {
                // If the subtrees might differ internally, descend into both
      // subtrees, finding the first child that spans the current position.
                if iterator_descend(&mut old_iter, position.bytes) {
                    if !iterator_descend(&mut new_iter, position.bytes) {
                        is_changed = 1 as libc::c_int != 0;
                        next_position = iterator_end_position(&mut old_iter)
                    }
                } else if iterator_descend(&mut new_iter, position.bytes) {
                    is_changed = 1 as libc::c_int != 0;
                    next_position = iterator_end_position(&mut new_iter)
                } else {
                    next_position =
                        length_min(iterator_end_position(&mut old_iter),
                                   iterator_end_position(&mut new_iter))
                }
            }
            0 => {
                // If the subtrees are different, record a change and then move
      // to the end of both subtrees.
                is_changed = 1 as libc::c_int != 0;
                next_position =
                    length_min(iterator_end_position(&mut old_iter),
                               iterator_end_position(&mut new_iter))
            }
            _ => { }
        }
        // Ensure that both iterators are caught up to the current position.
        while !iterator_done(&mut old_iter) &&
                  iterator_end_position(&mut old_iter).bytes <=
                      next_position.bytes {
            iterator_advance(&mut old_iter);
        }
        while !iterator_done(&mut new_iter) &&
                  iterator_end_position(&mut new_iter).bytes <=
                      next_position.bytes {
            iterator_advance(&mut new_iter);
        }
        // Ensure that both iterators are at the same depth in the tree.
        while old_iter.visible_depth > new_iter.visible_depth {
            iterator_ascend(&mut old_iter);
        }
        while new_iter.visible_depth > old_iter.visible_depth {
            iterator_ascend(&mut new_iter);
        }
        if is_changed {
            ts_range_array_add(&mut results, position, next_position);
        }
        position = next_position;
        // Keep track of the current position in the included range differences
    // array in order to avoid scanning the entire array on each iteration.
        while included_range_difference_index <
                  (*included_range_differences).size {
            let mut range: *const TSRange =
                &mut *(*included_range_differences).contents.offset(included_range_difference_index
                                                                        as
                                                                        isize)
                    as *mut TSRange;
            if !((*range).end_byte <= position.bytes) { break ; }
            included_range_difference_index =
                included_range_difference_index.wrapping_add(1)
        }
        if !(!iterator_done(&mut old_iter) && !iterator_done(&mut new_iter)) {
            break ;
        }
    }
    let mut old_size: Length = ts_subtree_total_size(*old_tree);
    let mut new_size: Length = ts_subtree_total_size(*new_tree);
    if old_size.bytes < new_size.bytes {
        ts_range_array_add(&mut results, old_size, new_size);
    } else if new_size.bytes < old_size.bytes {
        ts_range_array_add(&mut results, new_size, old_size);
    }
    *cursor1 = old_iter.cursor;
    *cursor2 = new_iter.cursor;
    *ranges = results.contents;
    return results.size;
}
#[inline]
unsafe extern "C" fn ts_subtree_visible_child_count(mut self_0: Subtree)
 -> uint32_t {
    if ts_subtree_child_count(self_0) > 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count
    } else { return 0 as libc::c_int as uint32_t };
}
static mut LENGTH_MAX: Length =
    {
        let mut init =
            Length{bytes: 4294967295 as libc::c_uint,
                   extent:
                       {
                           let mut init =
                               TSPoint{row: 4294967295 as libc::c_uint,
                                       column: 4294967295 as libc::c_uint,};
                           init
                       },};
        init
    };
unsafe extern "C" fn ts_range_array_add(mut self_0: *mut TSRangeArray,
                                        mut start: Length, mut end: Length) {
    if (*self_0).size > 0 as libc::c_int as libc::c_uint {
        if (*self_0).size.wrapping_sub(1 as libc::c_int as libc::c_uint) <
               (*self_0).size {
        } else {
            __assert_fail(b"(uint32_t)(self)->size - 1 < (self)->size\x00" as
                              *const u8 as *const libc::c_char,
                          b"lib/src/./get_changed_ranges.c\x00" as *const u8
                              as *const libc::c_char,
                          12 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 56],
                                                    &[libc::c_char; 56]>(b"void ts_range_array_add(TSRangeArray *, Length, Length)\x00")).as_ptr());
        }
        let mut last_range: *mut TSRange =
            &mut *(*self_0).contents.offset((*self_0).size.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                as isize) as *mut TSRange;
        if start.bytes <= (*last_range).end_byte {
            (*last_range).end_byte = end.bytes;
            (*last_range).end_point = end.extent;
            return
        }
    }
    if start.bytes < end.bytes {
        let mut range: TSRange =
            {
                let mut init =
                    TSRange{start_point: start.extent,
                            end_point: end.extent,
                            start_byte: start.bytes,
                            end_byte: end.bytes,};
                init
            };
        array__grow(self_0 as *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<TSRange>() as libc::c_ulong);
        let fresh3 = (*self_0).size;
        (*self_0).size = (*self_0).size.wrapping_add(1);
        *(*self_0).contents.offset(fresh3 as isize) = range
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_range_array_get_changed_ranges(mut old_ranges:
                                                               *const TSRange,
                                                           mut old_range_count:
                                                               libc::c_uint,
                                                           mut new_ranges:
                                                               *const TSRange,
                                                           mut new_range_count:
                                                               libc::c_uint,
                                                           mut differences:
                                                               *mut TSRangeArray) {
    let mut new_index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut old_index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut current_position: Length = length_zero();
    let mut in_old_range: bool = 0 as libc::c_int != 0;
    let mut in_new_range: bool = 0 as libc::c_int != 0;
    while old_index < old_range_count || new_index < new_range_count {
        let mut old_range: *const TSRange =
            &*old_ranges.offset(old_index as isize) as *const TSRange;
        let mut new_range: *const TSRange =
            &*new_ranges.offset(new_index as isize) as *const TSRange;
        let mut next_old_position: Length =
            Length{bytes: 0, extent: TSPoint{row: 0, column: 0,},};
        if in_old_range {
            next_old_position =
                {
                    let mut init =
                        Length{bytes: (*old_range).end_byte,
                               extent: (*old_range).end_point,};
                    init
                }
        } else if old_index < old_range_count {
            next_old_position =
                {
                    let mut init =
                        Length{bytes: (*old_range).start_byte,
                               extent: (*old_range).start_point,};
                    init
                }
        } else { next_old_position = LENGTH_MAX }
        let mut next_new_position: Length =
            Length{bytes: 0, extent: TSPoint{row: 0, column: 0,},};
        if in_new_range {
            next_new_position =
                {
                    let mut init =
                        Length{bytes: (*new_range).end_byte,
                               extent: (*new_range).end_point,};
                    init
                }
        } else if new_index < new_range_count {
            next_new_position =
                {
                    let mut init =
                        Length{bytes: (*new_range).start_byte,
                               extent: (*new_range).start_point,};
                    init
                }
        } else { next_new_position = LENGTH_MAX }
        if next_old_position.bytes < next_new_position.bytes {
            if in_old_range as libc::c_int != in_new_range as libc::c_int {
                ts_range_array_add(differences, current_position,
                                   next_old_position);
            }
            if in_old_range { old_index = old_index.wrapping_add(1) }
            current_position = next_old_position;
            in_old_range = !in_old_range
        } else if next_new_position.bytes < next_old_position.bytes {
            if in_old_range as libc::c_int != in_new_range as libc::c_int {
                ts_range_array_add(differences, current_position,
                                   next_new_position);
            }
            if in_new_range { new_index = new_index.wrapping_add(1) }
            current_position = next_new_position;
            in_new_range = !in_new_range
        } else {
            if in_old_range as libc::c_int != in_new_range as libc::c_int {
                ts_range_array_add(differences, current_position,
                                   next_new_position);
            }
            if in_old_range { old_index = old_index.wrapping_add(1) }
            if in_new_range { new_index = new_index.wrapping_add(1) }
            in_old_range = !in_old_range;
            in_new_range = !in_new_range;
            current_position = next_new_position
        }
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_is_fragile(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int
           } else {
               ((*self_0.ptr).fragile_left() as libc::c_int != 0 ||
                    (*self_0.ptr).fragile_right() as libc::c_int != 0) as
                   libc::c_int
           } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_range_array_intersects(mut self_0:
                                                       *const TSRangeArray,
                                                   mut start_index:
                                                       libc::c_uint,
                                                   mut start_byte: uint32_t,
                                                   mut end_byte: uint32_t)
 -> bool {
    let mut i: libc::c_uint = start_index;
    while i < (*self_0).size {
        let mut range: *mut TSRange =
            &mut *(*self_0).contents.offset(i as isize) as *mut TSRange;
        if (*range).end_byte > start_byte {
            if (*range).start_byte >= end_byte { break ; }
            return 1 as libc::c_int != 0
        } else { i = i.wrapping_add(1) }
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_total_bytes(mut self_0: Subtree) -> uint32_t {
    return ts_subtree_total_size(self_0).bytes;
}
#[inline]
unsafe extern "C" fn ts_subtree_production_id(mut self_0: Subtree)
 -> uint16_t {
    if ts_subtree_child_count(self_0) > 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id
    } else { return 0 as libc::c_int as uint16_t };
}
#[inline]
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int as libc::c_uint
           } else { (*self_0.ptr).child_count };
}
#[inline]
unsafe extern "C" fn ts_subtree_extra(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.extra() as libc::c_int
           } else { (*self_0.ptr).extra() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_is_error(mut self_0: Subtree) -> bool {
    return ts_subtree_symbol(self_0) as libc::c_int ==
               -(1 as libc::c_int) as TSSymbol as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.symbol as libc::c_int
           } else { (*self_0.ptr).symbol as libc::c_int } as TSSymbol;
}
#[inline]
unsafe extern "C" fn ts_subtree_lookahead_bytes(mut self_0: Subtree)
 -> uint32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.lookahead_bytes() as libc::c_uint
           } else { (*self_0.ptr).lookahead_bytes };
}
#[inline]
unsafe extern "C" fn ts_subtree_repeat_depth(mut self_0: Subtree)
 -> uint32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int as libc::c_uint
           } else {
               (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth
           };
}
#[inline]
unsafe extern "C" fn ts_subtree_has_changes(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.has_changes() as libc::c_int
           } else { (*self_0.ptr).has_changes() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_parse_state(mut self_0: Subtree)
 -> TSStateId {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.parse_state as libc::c_int
           } else { (*self_0.ptr).parse_state as libc::c_int } as TSStateId;
}
#[inline]
unsafe extern "C" fn ts_subtree_fragile_right(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int
           } else { (*self_0.ptr).fragile_right() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_fragile_left(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int
           } else { (*self_0.ptr).fragile_left() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_leaf_parse_state(mut self_0: Subtree)
 -> TSStateId {
    if self_0.data.is_inline() { return self_0.data.parse_state }
    if (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).parse_state
    }
    return (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.first_leaf.parse_state;
}
#[inline]
unsafe extern "C" fn ts_subtree_leaf_symbol(mut self_0: Subtree) -> TSSymbol {
    if self_0.data.is_inline() { return self_0.data.symbol as TSSymbol }
    if (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).symbol
    }
    return (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.first_leaf.symbol;
}
#[inline]
unsafe extern "C" fn ts_subtree_visible(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.visible() as libc::c_int
           } else { (*self_0.ptr).visible() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_has_external_tokens(mut self_0: Subtree)
 -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int
           } else { (*self_0.ptr).has_external_tokens() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_named(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.named() as libc::c_int
           } else { (*self_0.ptr).named() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_language_alias_sequence(mut self_0: *const TSLanguage,
                                                mut production_id: uint32_t)
 -> *const TSSymbol {
    return if production_id > 0 as libc::c_int as libc::c_uint {
               (*self_0).alias_sequences.offset(production_id.wrapping_mul((*self_0).max_alias_sequence_length
                                                                               as
                                                                               libc::c_uint)
                                                    as isize)
           } else { 0 as *const TSSymbol };
}
#[inline]
unsafe extern "C" fn ts_subtree_from_mut(mut self_0: MutableSubtree)
 -> Subtree {
    let mut result: Subtree =
        Subtree{data:
                    SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                          [0; 1],
                                      symbol: 0,
                                      padding_bytes: 0,
                                      size_bytes: 0,
                                      padding_columns: 0,
                                      padding_rows_lookahead_bytes: [0; 1],
                                      parse_state: 0,},};
    result.data = self_0.data;
    return result;
}
#[inline]
unsafe extern "C" fn ts_toggle_allocation_recording(mut value: bool) -> bool {
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn array__erase(mut self_0: *mut VoidArray,
                                  mut element_size: size_t,
                                  mut index: uint32_t) {
    if index < (*self_0).size {
    } else {
        __assert_fail(b"index < self->size\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./array.h\x00" as *const u8 as
                          *const libc::c_char,
                      84 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 49],
                                                &[libc::c_char; 49]>(b"void array__erase(VoidArray *, size_t, uint32_t)\x00")).as_ptr());
    }
    let mut contents: *mut libc::c_char =
        (*self_0).contents as *mut libc::c_char;
    memmove(contents.offset((index as
                                 libc::c_ulong).wrapping_mul(element_size) as
                                isize) as *mut libc::c_void,
            contents.offset((index.wrapping_add(1 as libc::c_int as
                                                    libc::c_uint) as
                                 libc::c_ulong).wrapping_mul(element_size) as
                                isize) as *const libc::c_void,
            ((*self_0).size.wrapping_sub(index).wrapping_sub(1 as libc::c_int
                                                                 as
                                                                 libc::c_uint)
                 as libc::c_ulong).wrapping_mul(element_size));
    (*self_0).size = (*self_0).size.wrapping_sub(1);
}
#[inline]
unsafe extern "C" fn array__splice(mut self_0: *mut VoidArray,
                                   mut element_size: size_t,
                                   mut index: uint32_t,
                                   mut old_count: uint32_t,
                                   mut new_count: uint32_t,
                                   mut elements: *const libc::c_void) {
    let mut new_size: uint32_t =
        (*self_0).size.wrapping_add(new_count).wrapping_sub(old_count);
    let mut old_end: uint32_t = index.wrapping_add(old_count);
    let mut new_end: uint32_t = index.wrapping_add(new_count);
    if old_end <= (*self_0).size {
    } else {
        __assert_fail(b"old_end <= self->size\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./array.h\x00" as *const u8 as
                          *const libc::c_char,
                      124 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"void array__splice(VoidArray *, size_t, uint32_t, uint32_t, uint32_t, const void *)\x00")).as_ptr());
    }
    array__reserve(self_0, element_size, new_size);
    let mut contents: *mut libc::c_char =
        (*self_0).contents as *mut libc::c_char;
    if (*self_0).size > old_end {
        memmove(contents.offset((new_end as
                                     libc::c_ulong).wrapping_mul(element_size)
                                    as isize) as *mut libc::c_void,
                contents.offset((old_end as
                                     libc::c_ulong).wrapping_mul(element_size)
                                    as isize) as *const libc::c_void,
                ((*self_0).size.wrapping_sub(old_end) as
                     libc::c_ulong).wrapping_mul(element_size));
    }
    if new_count > 0 as libc::c_int as libc::c_uint {
        if !elements.is_null() {
            memcpy(contents.offset((index as
                                        libc::c_ulong).wrapping_mul(element_size)
                                       as isize) as *mut libc::c_void,
                   elements,
                   (new_count as libc::c_ulong).wrapping_mul(element_size));
        } else {
            memset(contents.offset((index as
                                        libc::c_ulong).wrapping_mul(element_size)
                                       as isize) as *mut libc::c_void,
                   0 as libc::c_int,
                   (new_count as libc::c_ulong).wrapping_mul(element_size));
        }
    }
    (*self_0).size =
        ((*self_0).size as
             libc::c_uint).wrapping_add(new_count.wrapping_sub(old_count)) as
            uint32_t as uint32_t;
}
#[inline]
unsafe extern "C" fn length_is_undefined(mut length: Length) -> bool {
    return length.bytes == 0 as libc::c_int as libc::c_uint &&
               length.extent.column != 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ts_language_enabled_external_tokens(mut self_0:
                                                             *const TSLanguage,
                                                         mut external_scanner_state:
                                                             libc::c_uint)
 -> *const bool {
    if external_scanner_state == 0 as libc::c_int as libc::c_uint {
        return 0 as *const bool
    } else {
        return (*self_0).external_scanner.states.offset((*self_0).external_token_count.wrapping_mul(external_scanner_state)
                                                            as isize)
    };
}
#[inline]
unsafe extern "C" fn ts_language_next_state(mut self_0: *const TSLanguage,
                                            mut state: TSStateId,
                                            mut symbol: TSSymbol)
 -> TSStateId {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
           ||
           symbol as libc::c_int ==
               -(1 as libc::c_int) as TSSymbol as libc::c_int -
                   1 as libc::c_int {
        return 0 as libc::c_int as TSStateId
    } else if (symbol as libc::c_uint) < (*self_0).token_count {
        let mut count: uint32_t = 0;
        let mut actions: *const TSParseAction =
            ts_language_actions(self_0, state, symbol, &mut count);
        if count > 0 as libc::c_int as libc::c_uint {
            let mut action: TSParseAction =
                *actions.offset(count.wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint) as
                                    isize);
            if action.type_0() as libc::c_int ==
                   TSParseActionTypeShift as libc::c_int {
                return if action.params.shift.extra() as libc::c_int != 0 {
                           state as libc::c_int
                       } else { action.params.shift.state as libc::c_int } as
                           TSStateId
            }
        }
        return 0 as libc::c_int as TSStateId
    } else { return ts_language_lookup(self_0, state, symbol) };
}
#[inline]
unsafe extern "C" fn ts_language_lookup(mut self_0: *const TSLanguage,
                                        mut state: TSStateId,
                                        mut symbol: TSSymbol) -> uint16_t {
    if (*self_0).version >= 11 as libc::c_int as libc::c_uint &&
           state as libc::c_uint >= (*self_0).large_state_count {
        let mut index: uint32_t =
            *(*self_0).small_parse_table_map.offset((state as
                                                         libc::c_uint).wrapping_sub((*self_0).large_state_count)
                                                        as isize);
        let mut data: *const uint16_t =
            &*(*self_0).small_parse_table.offset(index as isize) as
                *const uint16_t;
        let fresh4 = data;
        data = data.offset(1);
        let mut section_count: uint16_t = *fresh4;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < section_count as libc::c_uint {
            let fresh5 = data;
            data = data.offset(1);
            let mut section_value: uint16_t = *fresh5;
            let fresh6 = data;
            data = data.offset(1);
            let mut symbol_count: uint16_t = *fresh6;
            let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while i_0 < symbol_count as libc::c_uint {
                let fresh7 = data;
                data = data.offset(1);
                if *fresh7 as libc::c_int == symbol as libc::c_int {
                    return section_value
                }
                i_0 = i_0.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        return 0 as libc::c_int as uint16_t
    } else {
        return *(*self_0).parse_table.offset((state as
                                                  libc::c_uint).wrapping_mul((*self_0).symbol_count).wrapping_add(symbol
                                                                                                                      as
                                                                                                                      libc::c_uint)
                                                 as isize)
    };
}
#[inline]
unsafe extern "C" fn ts_language_actions(mut self_0: *const TSLanguage,
                                         mut state: TSStateId,
                                         mut symbol: TSSymbol,
                                         mut count: *mut uint32_t)
 -> *const TSParseAction {
    let mut entry: TableEntry =
        TableEntry{actions: 0 as *const TSParseAction,
                   action_count: 0,
                   is_reusable: false,};
    ts_language_table_entry(self_0, state, symbol, &mut entry);
    *count = entry.action_count;
    return entry.actions;
}
static mut LENGTH_UNDEFINED: Length =
    {
        let mut init =
            Length{bytes: 0 as libc::c_int as uint32_t,
                   extent:
                       {
                           let mut init =
                               TSPoint{row: 0 as libc::c_int as uint32_t,
                                       column: 1 as libc::c_int as uint32_t,};
                           init
                       },};
        init
    };
#[inline]
unsafe extern "C" fn ts_subtree_is_keyword(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.is_keyword() as libc::c_int
           } else { (*self_0.ptr).is_keyword() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn length_sub(mut len1: Length, mut len2: Length)
 -> Length {
    let mut result: Length =
        Length{bytes: 0, extent: TSPoint{row: 0, column: 0,},};
    result.bytes = len1.bytes.wrapping_sub(len2.bytes);
    result.extent = point_sub(len1.extent, len2.extent);
    return result;
}
#[inline]
unsafe extern "C" fn point_sub(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if a.row > b.row {
        return point__new(a.row.wrapping_sub(b.row), a.column)
    } else {
        return point__new(0 as libc::c_int as libc::c_uint,
                          a.column.wrapping_sub(b.column))
    };
}
#[inline]
unsafe extern "C" fn ts_language_has_actions(mut self_0: *const TSLanguage,
                                             mut state: TSStateId,
                                             mut symbol: TSSymbol) -> bool {
    let mut entry: TableEntry =
        TableEntry{actions: 0 as *const TSParseAction,
                   action_count: 0,
                   is_reusable: false,};
    ts_language_table_entry(self_0, state, symbol, &mut entry);
    return entry.action_count > 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ts_language_has_reduce_action(mut self_0:
                                                       *const TSLanguage,
                                                   mut state: TSStateId,
                                                   mut symbol: TSSymbol)
 -> bool {
    let mut entry: TableEntry =
        TableEntry{actions: 0 as *const TSParseAction,
                   action_count: 0,
                   is_reusable: false,};
    ts_language_table_entry(self_0, state, symbol, &mut entry);
    return entry.action_count > 0 as libc::c_int as libc::c_uint &&
               (*entry.actions.offset(0 as libc::c_int as isize)).type_0() as
                   libc::c_int == TSParseActionTypeReduce as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_subtree_set_extra(mut self_0: *mut MutableSubtree) {
    if (*self_0).data.is_inline() {
        (*self_0).data.set_extra(1 as libc::c_int != 0)
    } else { (*(*self_0).ptr).set_extra(1 as libc::c_int != 0) };
}
#[inline]
unsafe extern "C" fn ts_subtree_is_eof(mut self_0: Subtree) -> bool {
    return ts_subtree_symbol(self_0) as libc::c_int == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_count(mut self_0:
                                                      *const TSLanguage)
 -> uint32_t {
    return (*self_0).symbol_count.wrapping_add((*self_0).alias_count);
}
/* *
 * Get the ABI version number for this language. This version number is used
 * to ensure that languages were generated by a compatible version of
 * Tree-sitter.
 *
 * See also `ts_parser_set_language`.
 */
#[no_mangle]
pub unsafe extern "C" fn ts_language_version(mut self_0: *const TSLanguage)
 -> uint32_t {
    return (*self_0).version;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_count(mut self_0:
                                                     *const TSLanguage)
 -> uint32_t {
    if (*self_0).version >= 10 as libc::c_int as libc::c_uint {
        return (*self_0).field_count
    } else { return 0 as libc::c_int as uint32_t };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_table_entry(mut self_0:
                                                     *const TSLanguage,
                                                 mut state: TSStateId,
                                                 mut symbol: TSSymbol,
                                                 mut result:
                                                     *mut TableEntry) {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
           ||
           symbol as libc::c_int ==
               -(1 as libc::c_int) as TSSymbol as libc::c_int -
                   1 as libc::c_int {
        (*result).action_count = 0 as libc::c_int as uint32_t;
        (*result).is_reusable = 0 as libc::c_int != 0;
        (*result).actions = 0 as *const TSParseAction
    } else {
        if (symbol as libc::c_uint) < (*self_0).token_count {
        } else {
            __assert_fail(b"symbol < self->token_count\x00" as *const u8 as
                              *const libc::c_char,
                          b"lib/src/./language.c\x00" as *const u8 as
                              *const libc::c_char,
                          33 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 84],
                                                    &[libc::c_char; 84]>(b"void ts_language_table_entry(const TSLanguage *, TSStateId, TSSymbol, TableEntry *)\x00")).as_ptr());
        }
        let mut action_index: uint32_t =
            ts_language_lookup(self_0, state, symbol) as uint32_t;
        let mut entry: *const TSParseActionEntry =
            &*(*self_0).parse_actions.offset(action_index as isize) as
                *const TSParseActionEntry;
        (*result).action_count = (*entry).entry.count as uint32_t;
        (*result).is_reusable = (*entry).entry.reusable();
        (*result).actions =
            entry.offset(1 as libc::c_int as isize) as *const TSParseAction
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_metadata(mut self_0:
                                                         *const TSLanguage,
                                                     mut symbol: TSSymbol)
 -> TSSymbolMetadata {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
       {
        return {
                   let mut init = TSSymbolMetadata{visible_named: [0; 1],};
                   init.set_visible(1 as libc::c_int != 0);
                   init.set_named(1 as libc::c_int != 0);
                   init
               }
    } else if symbol as libc::c_int ==
                  -(1 as libc::c_int) as TSSymbol as libc::c_int -
                      1 as libc::c_int {
        return {
                   let mut init = TSSymbolMetadata{visible_named: [0; 1],};
                   init.set_visible(0 as libc::c_int != 0);
                   init.set_named(0 as libc::c_int != 0);
                   init
               }
    } else { return *(*self_0).symbol_metadata.offset(symbol as isize) };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_public_symbol(mut self_0:
                                                       *const TSLanguage,
                                                   mut symbol: TSSymbol)
 -> TSSymbol {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
       {
        return symbol
    }
    if (*self_0).version >= 11 as libc::c_int as libc::c_uint {
        return *(*self_0).public_symbol_map.offset(symbol as isize)
    } else { return symbol };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_name(mut self_0:
                                                     *const TSLanguage,
                                                 mut symbol: TSSymbol)
 -> *const libc::c_char {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
       {
        return b"ERROR\x00" as *const u8 as *const libc::c_char
    } else if symbol as libc::c_int ==
                  -(1 as libc::c_int) as TSSymbol as libc::c_int -
                      1 as libc::c_int {
        return b"_ERROR\x00" as *const u8 as *const libc::c_char
    } else if (symbol as libc::c_uint) < ts_language_symbol_count(self_0) {
        return *(*self_0).symbol_names.offset(symbol as isize)
    } else { return 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_for_name(mut self_0:
                                                         *const TSLanguage,
                                                     mut string:
                                                         *const libc::c_char,
                                                     mut length: uint32_t,
                                                     mut is_named: bool)
 -> TSSymbol {
    if strncmp(string, b"ERROR\x00" as *const u8 as *const libc::c_char,
               length as libc::c_ulong) == 0 {
        return -(1 as libc::c_int) as TSSymbol
    }
    let mut count: uint32_t = ts_language_symbol_count(self_0);
    let mut i: TSSymbol = 0 as libc::c_int as TSSymbol;
    while (i as libc::c_uint) < count {
        let mut metadata: TSSymbolMetadata =
            ts_language_symbol_metadata(self_0, i);
        if !(!metadata.visible() ||
                 metadata.named() as libc::c_int != is_named as libc::c_int) {
            let mut symbol_name: *const libc::c_char =
                *(*self_0).symbol_names.offset(i as isize);
            if strncmp(symbol_name, string, length as libc::c_ulong) == 0 &&
                   *symbol_name.offset(length as isize) == 0 {
                if (*self_0).version >= 11 as libc::c_int as libc::c_uint {
                    return *(*self_0).public_symbol_map.offset(i as isize)
                } else { return i }
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as TSSymbol;
}
/* *
 * Check whether the given node type id belongs to named nodes, anonymous nodes,
 * or a hidden nodes.
 *
 * See also `ts_node_is_named`. Hidden nodes are never returned from the API.
 */
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_type(mut self_0:
                                                     *const TSLanguage,
                                                 mut symbol: TSSymbol)
 -> TSSymbolType {
    let mut metadata: TSSymbolMetadata =
        ts_language_symbol_metadata(self_0, symbol);
    if metadata.named() {
        return TSSymbolTypeRegular
    } else if metadata.visible() {
        return TSSymbolTypeAnonymous
    } else { return TSSymbolTypeAuxiliary };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_name_for_id(mut self_0:
                                                           *const TSLanguage,
                                                       mut id: TSFieldId)
 -> *const libc::c_char {
    let mut count: uint32_t = ts_language_field_count(self_0);
    if count != 0 && id as libc::c_uint <= count {
        return *(*self_0).field_names.offset(id as isize)
    } else { return 0 as *const libc::c_char };
}
/* *********************/
/* Section - Language */
/* *********************/
/* *
 * Get the number of distinct node types in the language.
 */
/* *
 * Get a node type string for the given numerical id.
 */
/* *
 * Get the numerical id for the given node type string.
 */
/* *
 * Get the number of distinct field names in the language.
 */
/* *
 * Get the field name string for the given numerical id.
 */
/* *
 * Get the numerical id for the given field name string.
 */
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_id_for_name(mut self_0:
                                                           *const TSLanguage,
                                                       mut name:
                                                           *const libc::c_char,
                                                       mut name_length:
                                                           uint32_t)
 -> TSFieldId {
    let mut count: uint32_t = ts_language_field_count(self_0);
    let mut i: TSSymbol = 1 as libc::c_int as TSSymbol;
    while (i as libc::c_uint) <
              count.wrapping_add(1 as libc::c_int as libc::c_uint) {
        match strncmp(name, *(*self_0).field_names.offset(i as isize),
                      name_length as libc::c_ulong) {
            0 => {
                if *(*(*self_0).field_names.offset(i as
                                                       isize)).offset(name_length
                                                                          as
                                                                          isize)
                       as libc::c_int == 0 as libc::c_int {
                    return i
                }
            }
            -1 => { return 0 as libc::c_int as TSFieldId }
            _ => { }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as TSFieldId;
}
static mut BYTE_ORDER_MARK: int32_t = 0xfeff as libc::c_int;
static mut DEFAULT_RANGE: TSRange =
    {
        let mut init =
            TSRange{start_point:
                        {
                            let mut init =
                                TSPoint{row: 0 as libc::c_int as uint32_t,
                                        column:
                                            0 as libc::c_int as uint32_t,};
                            init
                        },
                    end_point:
                        {
                            let mut init =
                                TSPoint{row: 4294967295 as libc::c_uint,
                                        column: 4294967295 as libc::c_uint,};
                            init
                        },
                    start_byte: 0 as libc::c_int as uint32_t,
                    end_byte: 4294967295 as libc::c_uint,};
        init
    };
// Check if the lexer has reached EOF. This state is stored
// by setting the lexer's `current_included_range_index` such that
// it has consumed all of its available ranges.
unsafe extern "C" fn ts_lexer__eof(mut _self: *const TSLexer) -> bool {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    return (*self_0).current_included_range_index ==
               (*self_0).included_range_count;
}
// Clear the currently stored chunk of source code, because the lexer's
// position has changed.
unsafe extern "C" fn ts_lexer__clear_chunk(mut self_0: *mut Lexer) {
    (*self_0).chunk = 0 as *const libc::c_char;
    (*self_0).chunk_size = 0 as libc::c_int as uint32_t;
    (*self_0).chunk_start = 0 as libc::c_int as uint32_t;
}
// Call the lexer's input callback to obtain a new chunk of source code
// for the current position.
unsafe extern "C" fn ts_lexer__get_chunk(mut self_0: *mut Lexer) {
    (*self_0).chunk_start = (*self_0).current_position.bytes;
    (*self_0).chunk =
        (*self_0).input.read.expect("non-null function pointer")((*self_0).input.payload,
                                                                 (*self_0).current_position.bytes,
                                                                 (*self_0).current_position.extent,
                                                                 &mut (*self_0).chunk_size);
    if (*self_0).chunk_size == 0 {
        (*self_0).current_included_range_index =
            (*self_0).included_range_count;
        (*self_0).chunk = 0 as *const libc::c_char
    };
}
// Decode the next unicode character in the current chunk of source code.
// This assumes that the lexer has already retrieved a chunk of source
// code that spans the current position.
unsafe extern "C" fn ts_lexer__get_lookahead(mut self_0: *mut Lexer) {
    let mut position_in_chunk: uint32_t =
        (*self_0).current_position.bytes.wrapping_sub((*self_0).chunk_start);
    let mut chunk: *const uint8_t =
        ((*self_0).chunk as
             *const uint8_t).offset(position_in_chunk as isize);
    let mut size: uint32_t =
        (*self_0).chunk_size.wrapping_sub(position_in_chunk);
    if size == 0 as libc::c_int as libc::c_uint {
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t;
        (*self_0).data.lookahead = '\u{0}' as i32;
        return
    }
    let mut decode: UnicodeDecodeFunction =
        if (*self_0).input.encoding as libc::c_uint ==
               TSInputEncodingUTF8 as libc::c_int as libc::c_uint {
            Some(ts_decode_utf8 as
                     unsafe extern "C" fn(_: *const uint8_t, _: uint32_t,
                                          _: *mut int32_t) -> uint32_t)
        } else {
            Some(ts_decode_utf16 as
                     unsafe extern "C" fn(_: *const uint8_t, _: uint32_t,
                                          _: *mut int32_t) -> uint32_t)
        };
    (*self_0).lookahead_size =
        decode.expect("non-null function pointer")(chunk, size,
                                                   &mut (*self_0).data.lookahead);
    // If this chunk ended in the middle of a multi-byte character,
  // try again with a fresh chunk.
    if (*self_0).data.lookahead == TS_DECODE_ERROR &&
           size < 4 as libc::c_int as libc::c_uint {
        ts_lexer__get_chunk(self_0);
        chunk = (*self_0).chunk as *const uint8_t;
        size = (*self_0).chunk_size;
        (*self_0).lookahead_size =
            decode.expect("non-null function pointer")(chunk, size,
                                                       &mut (*self_0).data.lookahead)
    }
    if (*self_0).data.lookahead == TS_DECODE_ERROR {
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t
    };
}
// Advance to the next character in the source code, retrieving a new
// chunk of source code if needed.
unsafe extern "C" fn ts_lexer__advance(mut _self: *mut TSLexer,
                                       mut skip: bool) {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    if (*self_0).chunk.is_null() { return }
    if skip {
        if (*self_0).logger.log.is_some() {
            snprintf((*self_0).debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     if 32 as libc::c_int <= (*self_0).data.lookahead &&
                            (*self_0).data.lookahead < 127 as libc::c_int {
                         b"skip character:\'%c\'\x00" as *const u8 as
                             *const libc::c_char
                     } else {
                         b"skip character:%d\x00" as *const u8 as
                             *const libc::c_char
                     }, (*self_0).data.lookahead);
            (*self_0).logger.log.expect("non-null function pointer")((*self_0).logger.payload,
                                                                     TSLogTypeLex,
                                                                     (*self_0).debug_buffer.as_mut_ptr());
        }
    } else if (*self_0).logger.log.is_some() {
        snprintf((*self_0).debug_buffer.as_mut_ptr(),
                 1024 as libc::c_int as libc::c_ulong,
                 if 32 as libc::c_int <= (*self_0).data.lookahead &&
                        (*self_0).data.lookahead < 127 as libc::c_int {
                     b"consume character:\'%c\'\x00" as *const u8 as
                         *const libc::c_char
                 } else {
                     b"consume character:%d\x00" as *const u8 as
                         *const libc::c_char
                 }, (*self_0).data.lookahead);
        (*self_0).logger.log.expect("non-null function pointer")((*self_0).logger.payload,
                                                                 TSLogTypeLex,
                                                                 (*self_0).debug_buffer.as_mut_ptr());
    }
    if (*self_0).lookahead_size != 0 {
        (*self_0).current_position.bytes =
            ((*self_0).current_position.bytes as
                 libc::c_uint).wrapping_add((*self_0).lookahead_size) as
                uint32_t as uint32_t;
        if (*self_0).data.lookahead == '\n' as i32 {
            (*self_0).current_position.extent.row =
                (*self_0).current_position.extent.row.wrapping_add(1);
            (*self_0).current_position.extent.column =
                0 as libc::c_int as uint32_t
        } else {
            (*self_0).current_position.extent.column =
                ((*self_0).current_position.extent.column as
                     libc::c_uint).wrapping_add((*self_0).lookahead_size) as
                    uint32_t as uint32_t
        }
    }
    let mut current_range: *const TSRange = 0 as *const TSRange;
    if (*self_0).current_included_range_index < (*self_0).included_range_count
       {
        current_range =
            &mut *(*self_0).included_ranges.offset((*self_0).current_included_range_index
                                                       as isize) as
                *mut TSRange;
        if (*self_0).current_position.bytes == (*current_range).end_byte {
            (*self_0).current_included_range_index =
                (*self_0).current_included_range_index.wrapping_add(1);
            if (*self_0).current_included_range_index <
                   (*self_0).included_range_count {
                current_range = current_range.offset(1);
                (*self_0).current_position =
                    {
                        let mut init =
                            Length{bytes: (*current_range).start_byte,
                                   extent: (*current_range).start_point,};
                        init
                    }
            } else { current_range = 0 as *const TSRange }
        }
    }
    if skip { (*self_0).token_start_position = (*self_0).current_position }
    if !current_range.is_null() {
        if (*self_0).current_position.bytes >=
               (*self_0).chunk_start.wrapping_add((*self_0).chunk_size) {
            ts_lexer__get_chunk(self_0);
        }
        ts_lexer__get_lookahead(self_0);
    } else {
        ts_lexer__clear_chunk(self_0);
        (*self_0).data.lookahead = '\u{0}' as i32;
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t
    };
}
// Mark that a token match has completed. This can be called multiple
// times if a longer match is found later.
unsafe extern "C" fn ts_lexer__mark_end(mut _self: *mut TSLexer) {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    if !ts_lexer__eof(&mut (*self_0).data) {
        // If the lexer is right at the beginning of included range,
    // then the token should be considered to end at the *end* of the
    // previous included range, rather than here.
        let mut current_included_range: *mut TSRange =
            &mut *(*self_0).included_ranges.offset((*self_0).current_included_range_index
                                                       as isize) as
                *mut TSRange;
        if (*self_0).current_included_range_index >
               0 as libc::c_int as libc::c_ulong &&
               (*self_0).current_position.bytes ==
                   (*current_included_range).start_byte {
            let mut previous_included_range: *mut TSRange =
                current_included_range.offset(-(1 as libc::c_int as isize));
            (*self_0).token_end_position =
                {
                    let mut init =
                        Length{bytes: (*previous_included_range).end_byte,
                               extent: (*previous_included_range).end_point,};
                    init
                };
            return
        }
    }
    (*self_0).token_end_position = (*self_0).current_position;
}
unsafe extern "C" fn ts_lexer__get_column(mut _self: *mut TSLexer)
 -> uint32_t {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    let mut goal_byte: uint32_t = (*self_0).current_position.bytes;
    (*self_0).current_position.bytes =
        ((*self_0).current_position.bytes as
             libc::c_uint).wrapping_sub((*self_0).current_position.extent.column)
            as uint32_t as uint32_t;
    (*self_0).current_position.extent.column = 0 as libc::c_int as uint32_t;
    if (*self_0).current_position.bytes < (*self_0).chunk_start {
        ts_lexer__get_chunk(self_0);
    }
    let mut result: uint32_t = 0 as libc::c_int as uint32_t;
    while (*self_0).current_position.bytes < goal_byte {
        ts_lexer__advance(&mut (*self_0).data, 0 as libc::c_int != 0);
        result = result.wrapping_add(1)
    }
    return result;
}
static mut TS_DECODE_ERROR: int32_t = -(1 as libc::c_int);
// Is the lexer at a boundary between two disjoint included ranges of
// source code? This is exposed as an API because some languages' external
// scanners need to perform custom actions at these bounaries.
// The lexer's methods are stored as struct fields so that generated
      // parsers can call them without needing to be linked against this
      // library.
// Move to the first valid position at or after the given position.
// If the current position is outside of the current chunk of text,
    // then clear out the current chunk of text.
// If the given position is beyond any of included ranges, move to the EOF
  // state - past the end of the included ranges.
// Move the lexer to the given position. This doesn't do any work
// if the parser is already at the given position.
// In order to determine that a byte sequence is invalid UTF8 or UTF16,
  // the character decoding algorithm may have looked at the following byte.
  // Therefore, the next byte *after* the current (invalid) character
  // affects the interpretation of the current character.
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_included_ranges(mut self_0: *const Lexer,
                                                  mut count: *mut uint32_t)
 -> *mut TSRange {
    *count = (*self_0).included_range_count as uint32_t;
    return (*self_0).included_ranges;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_reset(mut self_0: *mut Lexer,
                                        mut position: Length) {
    if position.bytes != (*self_0).current_position.bytes {
        ts_lexer_goto(self_0, position);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_delete(mut self_0: *mut Lexer) {
    ts_free((*self_0).included_ranges as *mut libc::c_void);
}
unsafe extern "C" fn ts_lexer__is_at_included_range_start(mut _self:
                                                              *const TSLexer)
 -> bool {
    let mut self_0: *const Lexer = _self as *const Lexer;
    if (*self_0).current_included_range_index < (*self_0).included_range_count
       {
        let mut current_range: *mut TSRange =
            &mut *(*self_0).included_ranges.offset((*self_0).current_included_range_index
                                                       as isize) as
                *mut TSRange;
        return (*self_0).current_position.bytes == (*current_range).start_byte
    } else { return 0 as libc::c_int != 0 };
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_advance_to_end(mut self_0: *mut Lexer) {
    while !(*self_0).chunk.is_null() {
        ts_lexer__advance(&mut (*self_0).data, 0 as libc::c_int != 0);
    };
}
// These functions read one unicode code point from the given string,
// returning the number of bytes consumed.
#[inline]
unsafe extern "C" fn ts_decode_utf8(mut string: *const uint8_t,
                                    mut length: uint32_t,
                                    mut code_point: *mut int32_t)
 -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh8 = i;
    i = i.wrapping_add(1);
    *code_point = *string.offset(fresh8 as isize) as int32_t;
    if !(*code_point & 0x80 as libc::c_int == 0 as libc::c_int) {
        let mut __t: uint8_t = 0 as libc::c_int as uint8_t;
        if !(i != length &&
                 (if *code_point >= 0xe0 as libc::c_int {
                      ((if *code_point < 0xf0 as libc::c_int {
                            *code_point &= 0xf as libc::c_int;
                            __t = *string.offset(i as isize);
                            ((*::std::mem::transmute::<&[u8; 17],
                                                       &[libc::c_char; 17]>(b" 000000000000\x1000\x00"))[*code_point
                                                                                                             as
                                                                                                             usize]
                                 as libc::c_int &
                                 (1 as libc::c_int) <<
                                     (__t as libc::c_int >> 5 as libc::c_int)
                                 != 0 &&
                                 {
                                     __t =
                                         (__t as libc::c_int &
                                              0x3f as libc::c_int) as uint8_t;
                                     (1 as libc::c_int) != 0
                                 }) as libc::c_int
                        } else {
                            *code_point -= 0xf0 as libc::c_int;
                            (*code_point <= 4 as libc::c_int &&
                                 {
                                     __t = *string.offset(i as isize);
                                     ((*::std::mem::transmute::<&[u8; 17],
                                                                &[libc::c_char; 17]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x1e\x0f\x0f\x0f\x00\x00\x00\x00\x00"))[(__t
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_int
                                                                                                                                                                    >>
                                                                                                                                                                    4
                                                                                                                                                                        as
                                                                                                                                                                        libc::c_int)
                                                                                                                                                                   as
                                                                                                                                                                   usize]
                                          as libc::c_int &
                                          (1 as libc::c_int) << *code_point)
                                         != 0
                                 } &&
                                 {
                                     *code_point =
                                         *code_point << 6 as libc::c_int |
                                             __t as libc::c_int &
                                                 0x3f as libc::c_int;
                                     i = i.wrapping_add(1);
                                     (i) != length
                                 } &&
                                 {
                                     __t =
                                         (*string.offset(i as isize) as
                                              libc::c_int -
                                              0x80 as libc::c_int) as uint8_t;
                                     (__t as libc::c_int) <=
                                         0x3f as libc::c_int
                                 }) as libc::c_int
                        }) != 0 &&
                           {
                               *code_point =
                                   *code_point << 6 as libc::c_int |
                                       __t as libc::c_int;
                               i = i.wrapping_add(1);
                               (i) != length
                           }) as libc::c_int
                  } else {
                      (*code_point >= 0xc2 as libc::c_int &&
                           {
                               *code_point &= 0x1f as libc::c_int;
                               (1 as libc::c_int) != 0
                           }) as libc::c_int
                  }) != 0 &&
                 {
                     __t =
                         (*string.offset(i as isize) as libc::c_int -
                              0x80 as libc::c_int) as uint8_t;
                     (__t as libc::c_int) <= 0x3f as libc::c_int
                 } &&
                 {
                     *code_point =
                         *code_point << 6 as libc::c_int | __t as libc::c_int;
                     i = i.wrapping_add(1);
                     (1 as libc::c_int) != 0
                 }) {
            *code_point = -(1 as libc::c_int)
        }
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_set_input(mut self_0: *mut Lexer,
                                            mut input: TSInput) {
    (*self_0).input = input;
    ts_lexer__clear_chunk(self_0);
    ts_lexer_goto(self_0, (*self_0).current_position);
}
#[inline]
unsafe extern "C" fn ts_decode_utf16(mut string: *const uint8_t,
                                     mut length: uint32_t,
                                     mut code_point: *mut int32_t)
 -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh9 = i;
    i = i.wrapping_add(1);
    *code_point =
        *(string as *mut uint16_t).offset(fresh9 as isize) as int32_t;
    if *code_point as libc::c_uint & 0xfffffc00 as libc::c_uint ==
           0xd800 as libc::c_int as libc::c_uint {
        let mut __c2: uint16_t = 0;
        if i != length &&
               {
                   __c2 = *(string as *mut uint16_t).offset(i as isize);
                   (__c2 as libc::c_uint & 0xfffffc00 as libc::c_uint) ==
                       0xdc00 as libc::c_int as libc::c_uint
               } {
            i = i.wrapping_add(1);
            *code_point =
                (*code_point << 10 as libc::c_ulong) + __c2 as UChar32 -
                    (((0xd800 as libc::c_int) << 10 as libc::c_ulong) +
                         0xdc00 as libc::c_int - 0x10000 as libc::c_int)
        }
    }
    return i.wrapping_mul(2 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_mark_end(mut self_0: *mut Lexer) {
    ts_lexer__mark_end(&mut (*self_0).data);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_start(mut self_0: *mut Lexer) {
    (*self_0).token_start_position = (*self_0).current_position;
    (*self_0).token_end_position = LENGTH_UNDEFINED;
    (*self_0).data.result_symbol = 0 as libc::c_int as TSSymbol;
    if !ts_lexer__eof(&mut (*self_0).data) {
        if (*self_0).chunk_size == 0 { ts_lexer__get_chunk(self_0); }
        if (*self_0).lookahead_size == 0 { ts_lexer__get_lookahead(self_0); }
        if (*self_0).current_position.bytes ==
               0 as libc::c_int as libc::c_uint &&
               (*self_0).data.lookahead == BYTE_ORDER_MARK {
            ts_lexer__advance(&mut (*self_0).data, 1 as libc::c_int != 0);
        }
    };
}
unsafe extern "C" fn ts_lexer_goto(mut self_0: *mut Lexer,
                                   mut position: Length) {
    (*self_0).current_position = position;
    let mut found_included_range: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < (*self_0).included_range_count {
        let mut included_range: *mut TSRange =
            &mut *(*self_0).included_ranges.offset(i as isize) as
                *mut TSRange;
        if (*included_range).end_byte > position.bytes {
            if (*included_range).start_byte > position.bytes {
                (*self_0).current_position =
                    {
                        let mut init =
                            Length{bytes: (*included_range).start_byte,
                                   extent: (*included_range).start_point,};
                        init
                    }
            }
            (*self_0).current_included_range_index = i as size_t;
            found_included_range = 1 as libc::c_int != 0;
            break ;
        } else { i = i.wrapping_add(1) }
    }
    if found_included_range {
        if !(*self_0).chunk.is_null() &&
               (position.bytes < (*self_0).chunk_start ||
                    position.bytes >=
                        (*self_0).chunk_start.wrapping_add((*self_0).chunk_size))
           {
            ts_lexer__clear_chunk(self_0);
        }
        (*self_0).lookahead_size = 0 as libc::c_int as uint32_t;
        (*self_0).data.lookahead = '\u{0}' as i32
    } else {
        (*self_0).current_included_range_index =
            (*self_0).included_range_count;
        let mut last_included_range: *mut TSRange =
            &mut *(*self_0).included_ranges.offset((*self_0).included_range_count.wrapping_sub(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                       as isize) as
                *mut TSRange;
        (*self_0).current_position =
            {
                let mut init =
                    Length{bytes: (*last_included_range).end_byte,
                           extent: (*last_included_range).end_point,};
                init
            };
        ts_lexer__clear_chunk(self_0);
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t;
        (*self_0).data.lookahead = '\u{0}' as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_set_included_ranges(mut self_0: *mut Lexer,
                                                      mut ranges:
                                                          *const TSRange,
                                                      mut count: uint32_t)
 -> bool {
    if count == 0 as libc::c_int as libc::c_uint || ranges.is_null() {
        ranges = &DEFAULT_RANGE;
        count = 1 as libc::c_int as uint32_t
    } else {
        let mut previous_byte: uint32_t = 0 as libc::c_int as uint32_t;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < count {
            let mut range: *const TSRange =
                &*ranges.offset(i as isize) as *const TSRange;
            if (*range).start_byte < previous_byte ||
                   (*range).end_byte < (*range).start_byte {
                return 0 as libc::c_int != 0
            }
            previous_byte = (*range).end_byte;
            i = i.wrapping_add(1)
        }
    }
    let mut size: size_t =
        (count as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<TSRange>() as
                                             libc::c_ulong);
    (*self_0).included_ranges =
        ts_realloc((*self_0).included_ranges as *mut libc::c_void, size) as
            *mut TSRange;
    memcpy((*self_0).included_ranges as *mut libc::c_void,
           ranges as *const libc::c_void, size);
    (*self_0).included_range_count = count as size_t;
    ts_lexer_goto(self_0, (*self_0).current_position);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_finish(mut self_0: *mut Lexer,
                                         mut lookahead_end_byte:
                                             *mut uint32_t) {
    if length_is_undefined((*self_0).token_end_position) {
        ts_lexer__mark_end(&mut (*self_0).data);
    }
    let mut current_lookahead_end_byte: uint32_t =
        (*self_0).current_position.bytes.wrapping_add(1 as libc::c_int as
                                                          libc::c_uint);
    if (*self_0).data.lookahead == TS_DECODE_ERROR {
        current_lookahead_end_byte =
            current_lookahead_end_byte.wrapping_add(1)
    }
    if current_lookahead_end_byte > *lookahead_end_byte {
        *lookahead_end_byte = current_lookahead_end_byte
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_init(mut self_0: *mut Lexer) {
    *self_0 =
        {
            let mut init =
                Lexer{data:
                          {
                              let mut init =
                                  TSLexer{lookahead: 0 as libc::c_int,
                                          result_symbol:
                                              0 as libc::c_int as TSSymbol,
                                          advance:
                                              Some(ts_lexer__advance as
                                                       unsafe extern "C" fn(_:
                                                                                *mut TSLexer,
                                                                            _:
                                                                                bool)
                                                           -> ()),
                                          mark_end:
                                              Some(ts_lexer__mark_end as
                                                       unsafe extern "C" fn(_:
                                                                                *mut TSLexer)
                                                           -> ()),
                                          get_column:
                                              Some(ts_lexer__get_column as
                                                       unsafe extern "C" fn(_:
                                                                                *mut TSLexer)
                                                           -> uint32_t),
                                          is_at_included_range_start:
                                              Some(ts_lexer__is_at_included_range_start
                                                       as
                                                       unsafe extern "C" fn(_:
                                                                                *const TSLexer)
                                                           -> bool),
                                          eof:
                                              Some(ts_lexer__eof as
                                                       unsafe extern "C" fn(_:
                                                                                *const TSLexer)
                                                           -> bool),};
                              init
                          },
                      current_position:
                          {
                              let mut init =
                                  Length{bytes: 0 as libc::c_int as uint32_t,
                                         extent:
                                             {
                                                 let mut init =
                                                     TSPoint{row:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     uint32_t,
                                                             column:
                                                                 0 as
                                                                     libc::c_int
                                                                     as
                                                                     uint32_t,};
                                                 init
                                             },};
                              init
                          },
                      token_start_position:
                          Length{bytes: 0,
                                 extent: TSPoint{row: 0, column: 0,},},
                      token_end_position:
                          Length{bytes: 0,
                                 extent: TSPoint{row: 0, column: 0,},},
                      included_ranges: 0 as *mut TSRange,
                      included_range_count: 0 as libc::c_int as size_t,
                      current_included_range_index:
                          0 as libc::c_int as size_t,
                      chunk: 0 as *const libc::c_char,
                      chunk_start: 0 as libc::c_int as uint32_t,
                      chunk_size: 0 as libc::c_int as uint32_t,
                      lookahead_size: 0,
                      input:
                          TSInput{payload: 0 as *mut libc::c_void,
                                  read: None,
                                  encoding: TSInputEncodingUTF8,},
                      logger:
                          {
                              let mut init =
                                  TSLogger{payload: 0 as *mut libc::c_void,
                                           log: None,};
                              init
                          },
                      debug_buffer: [0; 1024],};
            init
        };
    ts_lexer_set_included_ranges(self_0, 0 as *const TSRange,
                                 0 as libc::c_int as uint32_t);
}
// TSNode - constructors
#[no_mangle]
pub unsafe extern "C" fn ts_node_new(mut tree: *const TSTree,
                                     mut subtree: *const Subtree,
                                     mut position: Length,
                                     mut alias: TSSymbol) -> TSNode {
    return {
               let mut init =
                   TSNode{context:
                              [position.bytes, position.extent.row,
                               position.extent.column, alias as uint32_t],
                          id: subtree as *const libc::c_void,
                          tree: tree,};
               init
           };
}
#[inline]
unsafe extern "C" fn ts_node__null() -> TSNode {
    return ts_node_new(0 as *const TSTree, 0 as *const Subtree, length_zero(),
                       0 as libc::c_int as TSSymbol);
}
// TSNode - accessors
#[no_mangle]
pub unsafe extern "C" fn ts_node_start_byte(mut self_0: TSNode) -> uint32_t {
    return self_0.context[0 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_start_point(mut self_0: TSNode) -> TSPoint {
    return {
               let mut init =
                   TSPoint{row: self_0.context[1 as libc::c_int as usize],
                           column:
                               self_0.context[2 as libc::c_int as usize],};
               init
           };
}
#[inline]
unsafe extern "C" fn ts_node__alias(mut self_0: *const TSNode) -> uint32_t {
    return (*self_0).context[3 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn ts_node__subtree(mut self_0: TSNode) -> Subtree {
    return *(self_0.id as *const Subtree);
}
// NodeChildIterator
#[inline]
unsafe extern "C" fn ts_node_iterate_children(mut node: *const TSNode)
 -> NodeChildIterator {
    let mut subtree: Subtree = ts_node__subtree(*node);
    if ts_subtree_child_count(subtree) == 0 as libc::c_int as libc::c_uint {
        return {
                   let mut init =
                       NodeChildIterator{parent:
                                             Subtree{ptr:
                                                         0 as
                                                             *const SubtreeHeapData,},
                                         tree: (*node).tree,
                                         position: length_zero(),
                                         child_index:
                                             0 as libc::c_int as uint32_t,
                                         structural_child_index:
                                             0 as libc::c_int as uint32_t,
                                         alias_sequence:
                                             0 as *const TSSymbol,};
                   init
               }
    }
    let mut alias_sequence: *const TSSymbol =
        ts_language_alias_sequence((*(*node).tree).language,
                                   (*subtree.ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                       as uint32_t);
    return {
               let mut init =
                   NodeChildIterator{parent: subtree,
                                     tree: (*node).tree,
                                     position:
                                         {
                                             let mut init =
                                                 Length{bytes:
                                                            ts_node_start_byte(*node),
                                                        extent:
                                                            ts_node_start_point(*node),};
                                             init
                                         },
                                     child_index:
                                         0 as libc::c_int as uint32_t,
                                     structural_child_index:
                                         0 as libc::c_int as uint32_t,
                                     alias_sequence: alias_sequence,};
               init
           };
}
#[inline]
unsafe extern "C" fn ts_node_child_iterator_done(mut self_0:
                                                     *mut NodeChildIterator)
 -> bool {
    return (*self_0).child_index == (*(*self_0).parent.ptr).child_count;
}
#[inline]
unsafe extern "C" fn ts_node_child_iterator_next(mut self_0:
                                                     *mut NodeChildIterator,
                                                 mut result: *mut TSNode)
 -> bool {
    if (*self_0).parent.ptr.is_null() ||
           ts_node_child_iterator_done(self_0) as libc::c_int != 0 {
        return 0 as libc::c_int != 0
    }
    let mut child: *const Subtree =
        &mut *(*(*self_0).parent.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*self_0).child_index
                                                                                        as
                                                                                        isize)
            as *mut Subtree;
    let mut alias_symbol: TSSymbol = 0 as libc::c_int as TSSymbol;
    if !ts_subtree_extra(*child) {
        if !(*self_0).alias_sequence.is_null() {
            alias_symbol =
                *(*self_0).alias_sequence.offset((*self_0).structural_child_index
                                                     as isize)
        }
        (*self_0).structural_child_index =
            (*self_0).structural_child_index.wrapping_add(1)
    }
    if (*self_0).child_index > 0 as libc::c_int as libc::c_uint {
        (*self_0).position =
            length_add((*self_0).position, ts_subtree_padding(*child))
    }
    *result =
        ts_node_new((*self_0).tree, child, (*self_0).position, alias_symbol);
    (*self_0).position =
        length_add((*self_0).position, ts_subtree_size(*child));
    (*self_0).child_index = (*self_0).child_index.wrapping_add(1);
    return 1 as libc::c_int != 0;
}
// TSNode - private
#[inline]
unsafe extern "C" fn ts_node__is_relevant(mut self_0: TSNode,
                                          mut include_anonymous: bool)
 -> bool {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if include_anonymous {
        return ts_subtree_visible(tree) as libc::c_int != 0 ||
                   ts_node__alias(&mut self_0) != 0
    } else {
        let mut alias: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
        if alias != 0 {
            return ts_language_symbol_metadata((*self_0.tree).language,
                                               alias).named()
        } else {
            return ts_subtree_visible(tree) as libc::c_int != 0 &&
                       ts_subtree_named(tree) as libc::c_int != 0
        }
    };
}
#[inline]
unsafe extern "C" fn ts_node__relevant_child_count(mut self_0: TSNode,
                                                   mut include_anonymous:
                                                       bool) -> uint32_t {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint {
        if include_anonymous {
            return (*tree.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count
        } else {
            return (*tree.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count
        }
    } else { return 0 as libc::c_int as uint32_t };
}
#[inline]
unsafe extern "C" fn ts_node__child(mut self_0: TSNode,
                                    mut child_index: uint32_t,
                                    mut include_anonymous: bool) -> TSNode {
    let mut result: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode =
            TSNode{context: [0; 4],
                   id: 0 as *const libc::c_void,
                   tree: 0 as *const TSTree,};
        let mut index: uint32_t = 0 as libc::c_int as uint32_t;
        let mut iterator: NodeChildIterator =
            ts_node_iterate_children(&mut result);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if ts_node__is_relevant(child, include_anonymous) {
                if index == child_index {
                    if ts_node__is_relevant(self_0, 1 as libc::c_int != 0) {
                        ts_tree_set_cached_parent(self_0.tree, &mut child,
                                                  &mut self_0);
                    }
                    return child
                }
                index = index.wrapping_add(1)
            } else {
                let mut grandchild_index: uint32_t =
                    child_index.wrapping_sub(index);
                let mut grandchild_count: uint32_t =
                    ts_node__relevant_child_count(child, include_anonymous);
                if grandchild_index < grandchild_count {
                    did_descend = 1 as libc::c_int != 0;
                    result = child;
                    child_index = grandchild_index;
                    break ;
                } else {
                    index =
                        (index as libc::c_uint).wrapping_add(grandchild_count)
                            as uint32_t as uint32_t
                }
            }
        }
    }
    return ts_node__null();
}
unsafe extern "C" fn ts_subtree_has_trailing_empty_descendant(mut self_0:
                                                                  Subtree,
                                                              mut other:
                                                                  Subtree)
 -> bool {
    let mut i: libc::c_uint =
        ts_subtree_child_count(self_0).wrapping_sub(1 as libc::c_int as
                                                        libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) >
              0 as libc::c_int as libc::c_uint {
        let mut child: Subtree =
            *(*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i as
                                                                             isize);
        if ts_subtree_total_bytes(child) > 0 as libc::c_int as libc::c_uint {
            break ;
        }
        if child.ptr == other.ptr ||
               ts_subtree_has_trailing_empty_descendant(child, other) as
                   libc::c_int != 0 {
            return 1 as libc::c_int != 0
        }
        i = i.wrapping_sub(1)
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn ts_node__prev_sibling(mut self_0: TSNode,
                                           mut include_anonymous: bool)
 -> TSNode {
    let mut self_subtree: Subtree = ts_node__subtree(self_0);
    let mut self_is_empty: bool =
        ts_subtree_total_bytes(self_subtree) ==
            0 as libc::c_int as libc::c_uint;
    let mut target_end_byte: uint32_t = ts_node_end_byte(self_0);
    let mut node: TSNode = ts_node_parent(self_0);
    let mut earlier_node: TSNode = ts_node__null();
    let mut earlier_node_is_relevant: bool = 0 as libc::c_int != 0;
    while !ts_node_is_null(node) {
        let mut earlier_child: TSNode = ts_node__null();
        let mut earlier_child_is_relevant: bool = 0 as libc::c_int != 0;
        let mut found_child_containing_target: bool = 0 as libc::c_int != 0;
        let mut child: TSNode =
            TSNode{context: [0; 4],
                   id: 0 as *const libc::c_void,
                   tree: 0 as *const TSTree,};
        let mut iterator: NodeChildIterator =
            ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if child.id == self_0.id { break ; }
            if iterator.position.bytes > target_end_byte {
                found_child_containing_target = 1 as libc::c_int != 0;
                break ;
            } else if iterator.position.bytes == target_end_byte &&
                          (!self_is_empty ||
                               ts_subtree_has_trailing_empty_descendant(ts_node__subtree(child),
                                                                        self_subtree)
                                   as libc::c_int != 0) {
                found_child_containing_target = 1 as libc::c_int != 0;
                break ;
            } else if ts_node__is_relevant(child, include_anonymous) {
                earlier_child = child;
                earlier_child_is_relevant = 1 as libc::c_int != 0
            } else if ts_node__relevant_child_count(child, include_anonymous)
                          > 0 as libc::c_int as libc::c_uint {
                earlier_child = child;
                earlier_child_is_relevant = 0 as libc::c_int != 0
            }
        }
        if found_child_containing_target {
            if !ts_node_is_null(earlier_child) {
                earlier_node = earlier_child;
                earlier_node_is_relevant = earlier_child_is_relevant
            }
            node = child
        } else if earlier_child_is_relevant {
            return earlier_child
        } else {
            if !ts_node_is_null(earlier_child) {
                node = earlier_child
            } else if earlier_node_is_relevant {
                return earlier_node
            } else { node = earlier_node }
        }
    }
    return ts_node__null();
}
#[inline]
unsafe extern "C" fn ts_node__next_sibling(mut self_0: TSNode,
                                           mut include_anonymous: bool)
 -> TSNode {
    let mut target_end_byte: uint32_t = ts_node_end_byte(self_0);
    let mut node: TSNode = ts_node_parent(self_0);
    let mut later_node: TSNode = ts_node__null();
    let mut later_node_is_relevant: bool = 0 as libc::c_int != 0;
    while !ts_node_is_null(node) {
        let mut later_child: TSNode = ts_node__null();
        let mut later_child_is_relevant: bool = 0 as libc::c_int != 0;
        let mut child_containing_target: TSNode = ts_node__null();
        let mut child: TSNode =
            TSNode{context: [0; 4],
                   id: 0 as *const libc::c_void,
                   tree: 0 as *const TSTree,};
        let mut iterator: NodeChildIterator =
            ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if iterator.position.bytes < target_end_byte { continue ; }
            if ts_node_start_byte(child) <= ts_node_start_byte(self_0) {
                if ts_node__subtree(child).ptr != ts_node__subtree(self_0).ptr
                   {
                    child_containing_target = child
                }
            } else if ts_node__is_relevant(child, include_anonymous) {
                later_child = child;
                later_child_is_relevant = 1 as libc::c_int != 0;
                break ;
            } else {
                if !(ts_node__relevant_child_count(child, include_anonymous) >
                         0 as libc::c_int as libc::c_uint) {
                    continue ;
                }
                later_child = child;
                later_child_is_relevant = 0 as libc::c_int != 0;
                break ;
            }
        }
        if !ts_node_is_null(child_containing_target) {
            if !ts_node_is_null(later_child) {
                later_node = later_child;
                later_node_is_relevant = later_child_is_relevant
            }
            node = child_containing_target
        } else if later_child_is_relevant {
            return later_child
        } else {
            if !ts_node_is_null(later_child) {
                node = later_child
            } else if later_node_is_relevant {
                return later_node
            } else { node = later_node }
        }
    }
    return ts_node__null();
}
#[inline]
unsafe extern "C" fn ts_node__first_child_for_byte(mut self_0: TSNode,
                                                   mut goal: uint32_t,
                                                   mut include_anonymous:
                                                       bool) -> TSNode {
    let mut node: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode =
            TSNode{context: [0; 4],
                   id: 0 as *const libc::c_void,
                   tree: 0 as *const TSTree,};
        let mut iterator: NodeChildIterator =
            ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if !(ts_node_end_byte(child) > goal) { continue ; }
            if ts_node__is_relevant(child, include_anonymous) {
                return child
            } else {
                if !(ts_node_child_count(child) >
                         0 as libc::c_int as libc::c_uint) {
                    continue ;
                }
                did_descend = 1 as libc::c_int != 0;
                node = child;
                break ;
            }
        }
    }
    return ts_node__null();
}
#[inline]
unsafe extern "C" fn ts_node__descendant_for_byte_range(mut self_0: TSNode,
                                                        mut range_start:
                                                            uint32_t,
                                                        mut range_end:
                                                            uint32_t,
                                                        mut include_anonymous:
                                                            bool) -> TSNode {
    let mut node: TSNode = self_0;
    let mut last_visible_node: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode =
            TSNode{context: [0; 4],
                   id: 0 as *const libc::c_void,
                   tree: 0 as *const TSTree,};
        let mut iterator: NodeChildIterator =
            ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            let mut node_end: uint32_t = iterator.position.bytes;
            // The end of this node must extend far enough forward to touch
      // the end of the range and exceed the start of the range.
            if node_end < range_end { continue ; }
            if node_end <= range_start { continue ; }
            // The start of this node must extend far enough backward to
      // touch the start of the range.
            if range_start < ts_node_start_byte(child) { break ; }
            node = child;
            if ts_node__is_relevant(node, include_anonymous) {
                ts_tree_set_cached_parent(self_0.tree, &mut child,
                                          &mut last_visible_node);
                last_visible_node = node
            }
            did_descend = 1 as libc::c_int != 0;
            break ;
        }
    }
    return last_visible_node;
}
#[inline]
unsafe extern "C" fn ts_node__descendant_for_point_range(mut self_0: TSNode,
                                                         mut range_start:
                                                             TSPoint,
                                                         mut range_end:
                                                             TSPoint,
                                                         mut include_anonymous:
                                                             bool) -> TSNode {
    let mut node: TSNode = self_0;
    let mut last_visible_node: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode =
            TSNode{context: [0; 4],
                   id: 0 as *const libc::c_void,
                   tree: 0 as *const TSTree,};
        let mut iterator: NodeChildIterator =
            ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            let mut node_end: TSPoint = iterator.position.extent;
            // The end of this node must extend far enough forward to touch
      // the end of the range and exceed the start of the range.
            if point_lt(node_end, range_end) { continue ; }
            if point_lte(node_end, range_start) { continue ; }
            // The start of this node must extend far enough backward to
      // touch the start of the range.
            if point_lt(range_start, ts_node_start_point(child)) { break ; }
            node = child;
            if ts_node__is_relevant(node, include_anonymous) {
                ts_tree_set_cached_parent(self_0.tree, &mut child,
                                          &mut last_visible_node);
                last_visible_node = node
            }
            did_descend = 1 as libc::c_int != 0;
            break ;
        }
    }
    return last_visible_node;
}
// TSNode - public
#[no_mangle]
pub unsafe extern "C" fn ts_node_end_byte(mut self_0: TSNode) -> uint32_t {
    return ts_node_start_byte(self_0).wrapping_add(ts_subtree_size(ts_node__subtree(self_0)).bytes);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_end_point(mut self_0: TSNode) -> TSPoint {
    return point_add(ts_node_start_point(self_0),
                     ts_subtree_size(ts_node__subtree(self_0)).extent);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_symbol(mut self_0: TSNode) -> TSSymbol {
    let mut symbol: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
    if symbol == 0 { symbol = ts_subtree_symbol(ts_node__subtree(self_0)) }
    return ts_language_public_symbol((*self_0.tree).language, symbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_type(mut self_0: TSNode)
 -> *const libc::c_char {
    let mut symbol: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
    if symbol == 0 { symbol = ts_subtree_symbol(ts_node__subtree(self_0)) }
    return ts_language_symbol_name((*self_0.tree).language, symbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_string(mut self_0: TSNode)
 -> *mut libc::c_char {
    return ts_subtree_string(ts_node__subtree(self_0),
                             (*self_0.tree).language, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_eq(mut self_0: TSNode, mut other: TSNode)
 -> bool {
    return self_0.tree == other.tree && self_0.id == other.id;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_null(mut self_0: TSNode) -> bool {
    return self_0.id.is_null();
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_extra(mut self_0: TSNode) -> bool {
    return ts_subtree_extra(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_named(mut self_0: TSNode) -> bool {
    let mut alias: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
    return if alias as libc::c_int != 0 {
               ts_language_symbol_metadata((*self_0.tree).language,
                                           alias).named() as libc::c_int
           } else {
               ts_subtree_named(ts_node__subtree(self_0)) as libc::c_int
           } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_missing(mut self_0: TSNode) -> bool {
    return ts_subtree_missing(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_has_changes(mut self_0: TSNode) -> bool {
    return ts_subtree_has_changes(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_has_error(mut self_0: TSNode) -> bool {
    return ts_subtree_error_cost(ts_node__subtree(self_0)) >
               0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_parent(mut self_0: TSNode) -> TSNode {
    let mut node: TSNode =
        ts_tree_get_cached_parent(self_0.tree, &mut self_0);
    if !node.id.is_null() { return node }
    node = ts_tree_root_node(self_0.tree);
    let mut end_byte: uint32_t = ts_node_end_byte(self_0);
    if node.id == self_0.id { return ts_node__null() }
    let mut last_visible_node: TSNode = node;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode =
            TSNode{context: [0; 4],
                   id: 0 as *const libc::c_void,
                   tree: 0 as *const TSTree,};
        let mut iterator: NodeChildIterator =
            ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if ts_node_start_byte(child) > ts_node_start_byte(self_0) ||
                   child.id == self_0.id {
                break ;
            }
            if !(iterator.position.bytes >= end_byte) { continue ; }
            node = child;
            if ts_node__is_relevant(child, 1 as libc::c_int != 0) {
                ts_tree_set_cached_parent(self_0.tree, &mut node,
                                          &mut last_visible_node);
                last_visible_node = node
            }
            did_descend = 1 as libc::c_int != 0;
            break ;
        }
    }
    return last_visible_node;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child(mut self_0: TSNode,
                                       mut child_index: uint32_t) -> TSNode {
    return ts_node__child(self_0, child_index, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_child(mut self_0: TSNode,
                                             mut child_index: uint32_t)
 -> TSNode {
    return ts_node__child(self_0, child_index, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child_by_field_id(mut self_0: TSNode,
                                                   mut field_id: TSFieldId)
 -> TSNode {
    'c_36107:
        loop  {
            if field_id == 0 ||
                   ts_node_child_count(self_0) ==
                       0 as libc::c_int as libc::c_uint {
                return ts_node__null()
            }
            let mut field_map: *const TSFieldMapEntry =
                0 as *const TSFieldMapEntry;
            let mut field_map_end: *const TSFieldMapEntry =
                0 as *const TSFieldMapEntry;
            ts_language_field_map((*self_0.tree).language,
                                  (*ts_node__subtree(self_0).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                      as uint32_t, &mut field_map,
                                  &mut field_map_end);
            if field_map == field_map_end { return ts_node__null() }
            // The field mappings are sorted by their field id. Scan all
  // the mappings to find the ones for the given field id.
            while ((*field_map).field_id as libc::c_int) <
                      field_id as libc::c_int {
                field_map = field_map.offset(1);
                if field_map == field_map_end { return ts_node__null() }
            }
            while (*field_map_end.offset(-(1 as libc::c_int) as
                                             isize)).field_id as libc::c_int >
                      field_id as libc::c_int {
                field_map_end = field_map_end.offset(-1);
                if field_map == field_map_end { return ts_node__null() }
            }
            let mut child: TSNode =
                TSNode{context: [0; 4],
                       id: 0 as *const libc::c_void,
                       tree: 0 as *const TSTree,};
            let mut iterator: NodeChildIterator =
                ts_node_iterate_children(&mut self_0);
            while ts_node_child_iterator_next(&mut iterator, &mut child) {
                if ts_subtree_extra(ts_node__subtree(child)) { continue ; }
                let mut index: uint32_t =
                    iterator.structural_child_index.wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint);
                if index < (*field_map).child_index as libc::c_uint {
                    continue ;
                }
                // Hidden nodes' fields are "inherited" by their visible parent.
                if (*field_map).inherited {
                    // If this is the *last* possible child node for this field,
        // then perform a tail call to avoid recursion.
                    if field_map.offset(1 as libc::c_int as isize) ==
                           field_map_end {
                        self_0 = child;
                        continue 'c_36107 ;
                    } else {
                        // Otherwise, descend into this child, but if it doesn't contain
        // the field, continue searching subsequent children.
                        let mut result: TSNode =
                            ts_node_child_by_field_id(child, field_id);
                        if !result.id.is_null() { return result }
                        field_map = field_map.offset(1);
                        if field_map == field_map_end {
                            return ts_node__null()
                        }
                    }
                } else if ts_node__is_relevant(child, 1 as libc::c_int != 0) {
                    return child
                } else {
                    // If the field refers to a hidden node, return its first visible
      // child.
                    return ts_node_child(child, 0 as libc::c_int as uint32_t)
                }
            }
            return ts_node__null()
        };
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child_by_field_name(mut self_0: TSNode,
                                                     mut name:
                                                         *const libc::c_char,
                                                     mut name_length:
                                                         uint32_t) -> TSNode {
    let mut field_id: TSFieldId =
        ts_language_field_id_for_name((*self_0.tree).language, name,
                                      name_length);
    return ts_node_child_by_field_id(self_0, field_id);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child_count(mut self_0: TSNode) -> uint32_t {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint {
        return (*tree.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count
    } else { return 0 as libc::c_int as uint32_t };
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_child_count(mut self_0: TSNode)
 -> uint32_t {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint {
        return (*tree.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count
    } else { return 0 as libc::c_int as uint32_t };
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_next_sibling(mut self_0: TSNode) -> TSNode {
    return ts_node__next_sibling(self_0, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_prev_named_sibling(mut self_0: TSNode)
 -> TSNode {
    return ts_node__prev_sibling(self_0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_descendant_for_byte_range(mut self_0: TSNode,
                                                           mut start:
                                                               uint32_t,
                                                           mut end: uint32_t)
 -> TSNode {
    return ts_node__descendant_for_byte_range(self_0, start, end,
                                              1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_descendant_for_point_range(mut self_0:
                                                                TSNode,
                                                            mut start:
                                                                TSPoint,
                                                            mut end: TSPoint)
 -> TSNode {
    return ts_node__descendant_for_point_range(self_0, start, end,
                                               1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_descendant_for_point_range(mut self_0:
                                                                      TSNode,
                                                                  mut start:
                                                                      TSPoint,
                                                                  mut end:
                                                                      TSPoint)
 -> TSNode {
    return ts_node__descendant_for_point_range(self_0, start, end,
                                               0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_prev_sibling(mut self_0: TSNode) -> TSNode {
    return ts_node__prev_sibling(self_0, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_first_child_for_byte(mut self_0: TSNode,
                                                      mut byte: uint32_t)
 -> TSNode {
    return ts_node__first_child_for_byte(self_0, byte, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_descendant_for_byte_range(mut self_0:
                                                                     TSNode,
                                                                 mut start:
                                                                     uint32_t,
                                                                 mut end:
                                                                     uint32_t)
 -> TSNode {
    return ts_node__descendant_for_byte_range(self_0, start, end,
                                              0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_edit(mut self_0: *mut TSNode,
                                      mut edit: *const TSInputEdit) {
    let mut start_byte: uint32_t = ts_node_start_byte(*self_0);
    let mut start_point: TSPoint = ts_node_start_point(*self_0);
    if start_byte >= (*edit).old_end_byte {
        start_byte =
            (*edit).new_end_byte.wrapping_add(start_byte.wrapping_sub((*edit).old_end_byte));
        start_point =
            point_add((*edit).new_end_point,
                      point_sub(start_point, (*edit).old_end_point))
    } else if start_byte > (*edit).start_byte {
        start_byte = (*edit).new_end_byte;
        start_point = (*edit).new_end_point
    }
    (*self_0).context[0 as libc::c_int as usize] = start_byte;
    (*self_0).context[1 as libc::c_int as usize] = start_point.row;
    (*self_0).context[2 as libc::c_int as usize] = start_point.column;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_first_named_child_for_byte(mut self_0:
                                                                TSNode,
                                                            mut byte:
                                                                uint32_t)
 -> TSNode {
    return ts_node__first_child_for_byte(self_0, byte, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_next_named_sibling(mut self_0: TSNode)
 -> TSNode {
    return ts_node__next_sibling(self_0, 0 as libc::c_int != 0);
}
// POSIX with monotonic clock support (Linux)
// * Represent a time as a monotonic (seconds, nanoseconds) pair.
// * Represent a duration as a number of microseconds.
//
// On these platforms, parse timeouts will correspond accurately to
// real time, regardless of what other processes are running.
#[inline]
unsafe extern "C" fn clock_null() -> TSClock {
    return {
               let mut init =
                   timespec{tv_sec: 0 as libc::c_int as __time_t,
                            tv_nsec: 0 as libc::c_int as __syscall_slong_t,};
               init
           };
}
#[inline]
unsafe extern "C" fn duration_to_micros(mut self_0: TSDuration) -> uint64_t {
    return self_0;
}
// StringInput
// Parser - Private
// At the end of a non-terminal extra node, the lexer normally returns
  // NULL, which indicates that the parser should look for a reduce action
  // at symbol `0`. Avoid reusing tokens in this situation to ensure that
  // the same thing happens when incrementally reparsing.
// If the token was created in a state with the same set of lookaheads, it is reusable.
// Empty tokens are not reusable in states with different lookaheads.
// If the current state allows external tokens or other tokens that conflict with this
  // token, this token is not reusable.
// Zero-length external tokens are generally allowed, but they're not
      // allowed right after a syntax error. This is for two reasons:
      // 1. After a syntax error, the lexer is looking for any possible token,
      //    as opposed to the specific set of tokens that are valid in some
      //    parse state. In this situation, it's very easy for an external
      //    scanner to produce unwanted zero-length tokens.
      // 2. The parser sometimes inserts *missing* tokens to recover from
      //    errors. These tokens are also zero-length. If we allow more
      //    zero-length tokens to be created after missing tokens, it
      //    can lead to infinite loops. Forbidding zero-length tokens
      //    right at the point of error recovery is a conservative strategy
      //    for preventing this kind of infinite loop.
// Do not reuse an EOF node if the included ranges array has changes
    // later on in the file.
// Error recovery can sometimes cause lots of stack versions to merge,
    // such that a single pop operation can produce a lots of slices.
    // Avoid creating too many stack versions in that situation.
// Extra tokens on top of the stack should not be included in this new parent
    // node. They will be re-pushed onto the stack after the parent node is
    // created and pushed.
// This pop operation may have caused multiple stack versions to collapse
    // into one, because they all diverged from a common state. In that case,
    // choose one of the arrays of trees to be the parent node's children, and
    // delete the rest of the tree arrays.
// Push the parent node onto the stack, along with any extra tokens that
    // were previously on top of the stack.
// Return the first new stack version that was created.
// Perform any reductions that can happen in this state, regardless of the lookahead. After
  // skipping one or more invalid tokens, the parser might find a token that would have allowed
  // a reduction to take place.
// Push a discontinuity onto the stack. Merge all of the stack versions that
  // were created in the previous step.
// In case the parser is currently outside of any included range, the lexer will
          // snap to the beginning of the next included range. The missing token's padding
          // must be assigned to position it within the next included range.
// When the parser is in the error state, there are two strategies for recovering with a
  // given lookahead token:
  // 1. Find a previous state on the stack in which that lookahead token would be valid. Then,
  //    create a new stack version that is in that state again. This entails popping all of the
  //    subtrees that have been pushed onto the stack since that previous state, and wrapping
  //    them in an ERROR node.
  // 2. Wrap the lookahead token in an ERROR node, push that ERROR node onto the stack, and
  //    move on to the next lookahead token, remaining in the error state.
  //
  // First, try the strategy 1. Upon entering the error state, the parser recorded a summary
  // of the previous parse states and their depths. Look at each state in the summary, to see
  // if the current lookahead token would be valid in that state.
// Do not recover in ways that create redundant stack versions.
// Do not recover if the result would clearly be worse than some existing stack version.
// If the current lookahead token is valid in some previous state, recover to that state.
      // Then stop looking for further recoveries.
// In the process of attemping to recover, some stack versions may have been created
  // and subsequently halted. Remove those versions.
// If strategy 1 succeeded, a new stack version will have been created which is able to handle
  // the current lookahead token. Now, in addition, try strategy 2 described above: skip the
  // current lookahead token by wrapping it in an ERROR node.
// Don't pursue this additional strategy if there are already too many stack versions.
// If the parser is still in the error state at the end of the file, just wrap everything
  // in an ERROR node and terminate.
// Do not recover if the result would clearly be worse than some existing stack version.
// If the current lookahead token is an extra token, mark it as extra. This means it won't
  // be counted in error cost calculations.
// Wrap the lookahead token in an ERROR.
// If other tokens have already been skipped, so there is already an ERROR at the top of the
  // stack, then pop that ERROR off the stack and wrap the two ERRORs together into one larger
  // ERROR.
// TODO: Figure out how to make this condition occur.
    // See https://github.com/atom/atom/issues/18450#issuecomment-439579778
    // If multiple stack versions have merged at this point, just pick one of the errors
    // arbitrarily and discard the rest.
// Push the new ERROR onto the stack.
// If possible, reuse a node from the previous syntax tree.
// If no node from the previous syntax tree could be reused, then try to
  // reuse the token previously returned by the lexer.
// Otherwise, re-run the lexer.
// When parsing a non-terminal extra, a null lookahead indicates the
    // end of the rule. The reduction is stored in the EOF table entry.
    // After the reduction, the lexer needs to be run again.
// If a cancellation flag or a timeout was provided, then check every
    // time a fixed number of parse actions has been processed.
// Process each parse action for the current lookahead token in
    // the current state. If there are multiple actions, then this is
    // an ambiguous state. REDUCE actions always create a new stack
    // version, whereas SHIFT actions update the existing stack version
    // and terminate this loop.
// TODO: remove when TREE_SITTER_LANGUAGE_VERSION 9 is out.
// If a reduction was performed, then replace the current stack version
    // with one of the stack versions created by a reduction, and continue
    // processing this version of the stack with the same lookahead symbol.
// At the end of a non-terminal extra rule, the lexer will return a
      // null subtree, because the parser needs to perform a fixed reduction
      // regardless of the lookahead node. After performing that reduction,
      // (and completing the non-terminal extra rule) run the lexer again based
      // on the current parse state.
// If there were no parse actions for the current lookahead token, then
    // it is not valid in this state. If the current lookahead token is a
    // keyword, then switch to treating it as the normal word token if that
    // token is valid in this state.
// If the current lookahead token is not valid and the parser is
    // already in the error state, restart the error recovery process.
    // TODO - can this be unified with the other `RECOVER` case above?
// If the current lookahead token is not valid and the previous
    // subtree on the stack was reused from an old tree, it isn't actually
    // valid to reuse it. Remove it from the stack, and in its place,
    // push each of its children. Then try again to process the current
    // lookahead.
// At this point, the current lookahead token is definitely not valid
    // for this parse stack version. Mark this version as paused and continue
    // processing any other stack versions that might exist. If some other
    // version advances successfully, then this version can simply be removed.
    // But if all versions end up paused, then error recovery is needed.
// Prune any versions that have been marked for removal.
// Keep track of the minimum error cost of any stack version so
    // that it can be returned.
// Examine each pair of stack versions, removing any versions that
    // are clearly worse than another version. Ensure that the versions
    // are ordered from most promising to least promising.
// Enfore a hard upper bound on the number of stack versions by
  // discarding the least promising versions.
// If the best-performing stack version is currently paused, or all
  // versions are paused, then resume the best paused version and begin
  // the error recovery process. Otherwise, remove the paused versions.
// Parser - Public
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_cancellation_flag(mut self_0:
                                                             *mut TSParser,
                                                         mut flag:
                                                             *const size_t) {
    (*self_0).cancellation_flag = flag as *const size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_cancellation_flag(mut self_0:
                                                         *const TSParser)
 -> *const size_t {
    return (*self_0).cancellation_flag as *const size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_logger(mut self_0: *mut TSParser,
                                              mut logger: TSLogger) {
    (*self_0).lexer.logger = logger;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_logger(mut self_0: *const TSParser)
 -> TSLogger {
    return (*self_0).lexer.logger;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_print_dot_graphs(mut self_0: *mut TSParser,
                                                    mut fd: libc::c_int) {
    if !(*self_0).dot_graph_file.is_null() {
        fclose((*self_0).dot_graph_file);
    }
    if fd >= 0 as libc::c_int {
        (*self_0).dot_graph_file =
            fdopen(fd, b"a\x00" as *const u8 as *const libc::c_char)
    } else { (*self_0).dot_graph_file = 0 as *mut FILE };
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_delete(mut self_0: *mut TSParser) {
    if self_0.is_null() { return }
    ts_parser_set_language(self_0, 0 as *const TSLanguage);
    ts_stack_delete((*self_0).stack);
    if !(*self_0).reduce_actions.contents.is_null() {
        array__delete(&mut (*self_0).reduce_actions as *mut ReduceActionSet as
                          *mut VoidArray);
    }
    if !(*self_0).included_range_differences.contents.is_null() {
        array__delete(&mut (*self_0).included_range_differences as
                          *mut TSRangeArray as *mut VoidArray);
    }
    if !(*self_0).old_tree.ptr.is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*self_0).old_tree);
        (*self_0).old_tree = Subtree{ptr: 0 as *const SubtreeHeapData,}
    }
    ts_lexer_delete(&mut (*self_0).lexer);
    ts_parser__set_cached_token(self_0, 0 as libc::c_int as size_t,
                                Subtree{ptr: 0 as *const SubtreeHeapData,},
                                Subtree{ptr: 0 as *const SubtreeHeapData,});
    ts_subtree_pool_delete(&mut (*self_0).tree_pool);
    reusable_node_delete(&mut (*self_0).reusable_node);
    ts_free(self_0 as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn reusable_node_delete(mut self_0: *mut ReusableNode) {
    array__delete(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                      *mut VoidArray);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_language(mut self_0: *mut TSParser,
                                                mut language:
                                                    *const TSLanguage)
 -> bool {
    if !language.is_null() {
        if (*language).version > 11 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int != 0
        }
        if (*language).version < 9 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int != 0
        }
    }
    if !(*self_0).external_scanner_payload.is_null() &&
           (*(*self_0).language).external_scanner.destroy.is_some() {
        (*(*self_0).language).external_scanner.destroy.expect("non-null function pointer")((*self_0).external_scanner_payload);
    }
    if !language.is_null() && (*language).external_scanner.create.is_some() {
        (*self_0).external_scanner_payload =
            (*language).external_scanner.create.expect("non-null function pointer")()
    } else { (*self_0).external_scanner_payload = 0 as *mut libc::c_void }
    (*self_0).language = language;
    ts_parser_reset(self_0);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_reset(mut self_0: *mut TSParser) {
    if !(*self_0).language.is_null() &&
           (*(*self_0).language).external_scanner.deserialize.is_some() {
        (*(*self_0).language).external_scanner.deserialize.expect("non-null function pointer")((*self_0).external_scanner_payload,
                                                                                               0
                                                                                                   as
                                                                                                   *const libc::c_char,
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint);
    }
    if !(*self_0).old_tree.ptr.is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*self_0).old_tree);
        (*self_0).old_tree = Subtree{ptr: 0 as *const SubtreeHeapData,}
    }
    reusable_node_clear(&mut (*self_0).reusable_node);
    ts_lexer_reset(&mut (*self_0).lexer, length_zero());
    ts_stack_clear((*self_0).stack);
    ts_parser__set_cached_token(self_0, 0 as libc::c_int as size_t,
                                Subtree{ptr: 0 as *const SubtreeHeapData,},
                                Subtree{ptr: 0 as *const SubtreeHeapData,});
    if !(*self_0).finished_tree.ptr.is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*self_0).finished_tree);
        (*self_0).finished_tree = Subtree{ptr: 0 as *const SubtreeHeapData,}
    }
    (*self_0).accept_count = 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn reusable_node_clear(mut self_0: *mut ReusableNode) {
    (*self_0).stack.size = 0 as libc::c_int as uint32_t;
    (*self_0).last_external_token =
        Subtree{ptr: 0 as *const SubtreeHeapData,};
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_language(mut self_0: *const TSParser)
 -> *const TSLanguage {
    return (*self_0).language;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_included_ranges(mut self_0:
                                                           *mut TSParser,
                                                       mut ranges:
                                                           *const TSRange,
                                                       mut count: uint32_t)
 -> bool {
    return ts_lexer_set_included_ranges(&mut (*self_0).lexer, ranges, count);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_parse_string_encoding(mut self_0:
                                                             *mut TSParser,
                                                         mut old_tree:
                                                             *const TSTree,
                                                         mut string:
                                                             *const libc::c_char,
                                                         mut length: uint32_t,
                                                         mut encoding:
                                                             TSInputEncoding)
 -> *mut TSTree {
    let mut input: TSStringInput =
        {
            let mut init = TSStringInput{string: string, length: length,};
            init
        };
    return ts_parser_parse(self_0, old_tree,
                           {
                               let mut init =
                                   TSInput{payload:
                                               &mut input as
                                                   *mut TSStringInput as
                                                   *mut libc::c_void,
                                           read:
                                               Some(ts_string_input_read as
                                                        unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 uint32_t,
                                                                             _:
                                                                                 TSPoint,
                                                                             _:
                                                                                 *mut uint32_t)
                                                            ->
                                                                *const libc::c_char),
                                           encoding: encoding,};
                               init
                           });
}
#[inline]
unsafe extern "C" fn duration_from_micros(mut micros: uint64_t)
 -> TSDuration {
    return micros;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_timeout_micros(mut self_0:
                                                          *mut TSParser,
                                                      mut timeout_micros:
                                                          uint64_t) {
    (*self_0).timeout_duration = duration_from_micros(timeout_micros);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_included_ranges(mut self_0:
                                                       *const TSParser,
                                                   mut count: *mut uint32_t)
 -> *const TSRange {
    return ts_lexer_included_ranges(&(*self_0).lexer, count);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_parse(mut self_0: *mut TSParser,
                                         mut old_tree: *const TSTree,
                                         mut input: TSInput) -> *mut TSTree {
    if (*self_0).language.is_null() || input.read.is_none() {
        return 0 as *mut TSTree
    }
    ts_lexer_set_input(&mut (*self_0).lexer, input);
    (*self_0).included_range_differences.size = 0 as libc::c_int as uint32_t;
    (*self_0).included_range_difference_index =
        0 as libc::c_int as libc::c_uint;
    if ts_parser_has_outstanding_parse(self_0) {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"resume_parsing\x00" as *const u8 as
                         *const libc::c_char);
            ts_parser__log(self_0);
        }
    } else if !old_tree.is_null() {
        ts_subtree_retain((*old_tree).root);
        (*self_0).old_tree = (*old_tree).root;
        ts_range_array_get_changed_ranges((*old_tree).included_ranges,
                                          (*old_tree).included_range_count,
                                          (*self_0).lexer.included_ranges,
                                          (*self_0).lexer.included_range_count
                                              as libc::c_uint,
                                          &mut (*self_0).included_range_differences);
        reusable_node_reset(&mut (*self_0).reusable_node, (*old_tree).root);
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"parse_after_edit\x00" as *const u8 as
                         *const libc::c_char);
            ts_parser__log(self_0);
        }
        if !(*self_0).dot_graph_file.is_null() {
            ts_subtree_print_dot_graph((*self_0).old_tree, (*self_0).language,
                                       (*self_0).dot_graph_file);
            fputs(b"\n\x00" as *const u8 as *const libc::c_char,
                  (*self_0).dot_graph_file);
        }
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*self_0).included_range_differences.size {
            let mut range: *mut TSRange =
                &mut *(*self_0).included_range_differences.contents.offset(i
                                                                               as
                                                                               isize)
                    as *mut TSRange;
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"different_included_range %u - %u\x00" as *const u8
                             as *const libc::c_char, (*range).start_byte,
                         (*range).end_byte);
                ts_parser__log(self_0);
            }
            i = i.wrapping_add(1)
        }
    } else {
        reusable_node_clear(&mut (*self_0).reusable_node);
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"new_parse\x00" as *const u8 as *const libc::c_char);
            ts_parser__log(self_0);
        }
    }
    let mut position: uint32_t = 0 as libc::c_int as uint32_t;
    let mut last_position: uint32_t = 0 as libc::c_int as uint32_t;
    let mut version_count: uint32_t = 0 as libc::c_int as uint32_t;
    (*self_0).operation_count = 0 as libc::c_int as libc::c_uint;
    if (*self_0).timeout_duration != 0 {
        (*self_0).end_clock =
            clock_after(clock_now(), (*self_0).timeout_duration)
    } else { (*self_0).end_clock = clock_null() }
    loop  {
        let mut version: StackVersion = 0 as libc::c_int as StackVersion;
        loop  {
            version_count = ts_stack_version_count((*self_0).stack);
            if !(version < version_count) { break ; }
            let mut allow_node_reuse: bool =
                version_count == 1 as libc::c_int as libc::c_uint;
            while ts_stack_is_active((*self_0).stack, version) {
                if (*self_0).lexer.logger.log.is_some() ||
                       !(*self_0).dot_graph_file.is_null() {
                    snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                             1024 as libc::c_int as libc::c_ulong,
                             b"process version:%d, version_count:%u, state:%d, row:%u, col:%u\x00"
                                 as *const u8 as *const libc::c_char, version,
                             ts_stack_version_count((*self_0).stack),
                             ts_stack_state((*self_0).stack, version) as
                                 libc::c_int,
                             ts_stack_position((*self_0).stack,
                                               version).extent.row.wrapping_add(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint),
                             ts_stack_position((*self_0).stack,
                                               version).extent.column);
                    ts_parser__log(self_0);
                }
                if !ts_parser__advance(self_0, version, allow_node_reuse) {
                    return 0 as *mut TSTree
                }
                if !(*self_0).dot_graph_file.is_null() {
                    ts_stack_print_dot_graph((*self_0).stack,
                                             (*self_0).language,
                                             (*self_0).dot_graph_file);
                    fputs(b"\n\n\x00" as *const u8 as *const libc::c_char,
                          (*self_0).dot_graph_file);
                }
                position = ts_stack_position((*self_0).stack, version).bytes;
                if !(position > last_position ||
                         version > 0 as libc::c_int as libc::c_uint &&
                             position == last_position) {
                    continue ;
                }
                last_position = position;
                break ;
            }
            version = version.wrapping_add(1)
        }
        let mut min_error_cost: libc::c_uint =
            ts_parser__condense_stack(self_0);
        if !(*self_0).finished_tree.ptr.is_null() &&
               ts_subtree_error_cost((*self_0).finished_tree) < min_error_cost
           {
            break ;
        }
        while (*self_0).included_range_difference_index <
                  (*self_0).included_range_differences.size {
            let mut range_0: *mut TSRange =
                &mut *(*self_0).included_range_differences.contents.offset((*self_0).included_range_difference_index
                                                                               as
                                                                               isize)
                    as *mut TSRange;
            if !((*range_0).end_byte <= position) { break ; }
            (*self_0).included_range_difference_index =
                (*self_0).included_range_difference_index.wrapping_add(1)
        }
        if !(version_count != 0 as libc::c_int as libc::c_uint) { break ; }
    }
    ts_subtree_balance((*self_0).finished_tree, &mut (*self_0).tree_pool,
                       (*self_0).language);
    if (*self_0).lexer.logger.log.is_some() ||
           !(*self_0).dot_graph_file.is_null() {
        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                 1024 as libc::c_int as libc::c_ulong,
                 b"done\x00" as *const u8 as *const libc::c_char);
        ts_parser__log(self_0);
    }
    if !(*self_0).dot_graph_file.is_null() {
        ts_subtree_print_dot_graph((*self_0).finished_tree,
                                   (*self_0).language,
                                   (*self_0).dot_graph_file);
        fputs(b"\n\x00" as *const u8 as *const libc::c_char,
              (*self_0).dot_graph_file);
    }
    let mut result: *mut TSTree =
        ts_tree_new((*self_0).finished_tree, (*self_0).language,
                    (*self_0).lexer.included_ranges,
                    (*self_0).lexer.included_range_count as libc::c_uint);
    (*self_0).finished_tree = Subtree{ptr: 0 as *const SubtreeHeapData,};
    ts_parser_reset(self_0);
    return result;
}
unsafe extern "C" fn ts_parser__reduce(mut self_0: *mut TSParser,
                                       mut version: StackVersion,
                                       mut symbol: TSSymbol,
                                       mut count: uint32_t,
                                       mut dynamic_precedence: libc::c_int,
                                       mut production_id: uint16_t,
                                       mut is_fragile: bool,
                                       mut is_extra: bool) -> StackVersion {
    let mut initial_version_count: uint32_t =
        ts_stack_version_count((*self_0).stack);
    let mut removed_version_count: uint32_t = 0 as libc::c_int as uint32_t;
    let mut pop: StackSliceArray =
        ts_stack_pop_count((*self_0).stack, version, count);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < pop.size {
        let mut slice: StackSlice = *pop.contents.offset(i as isize);
        let mut slice_version: StackVersion =
            slice.version.wrapping_sub(removed_version_count);
        if i > 0 as libc::c_int as libc::c_uint &&
               slice_version >
                   MAX_VERSION_COUNT.wrapping_add(MAX_VERSION_COUNT_OVERFLOW)
           {
            ts_stack_remove_version((*self_0).stack, slice_version);
            ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                    &mut slice.subtrees);
            removed_version_count = removed_version_count.wrapping_add(1);
            while i.wrapping_add(1 as libc::c_int as libc::c_uint) < pop.size
                  {
                let mut next_slice: StackSlice =
                    *pop.contents.offset(i.wrapping_add(1 as libc::c_int as
                                                            libc::c_uint) as
                                             isize);
                if next_slice.version != slice.version { break ; }
                ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                        &mut next_slice.subtrees);
                i = i.wrapping_add(1)
            }
        } else {
            let mut children: SubtreeArray = slice.subtrees;
            while children.size > 0 as libc::c_int as libc::c_uint &&
                      ts_subtree_extra(*children.contents.offset(children.size.wrapping_sub(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                libc::c_uint)
                                                                     as
                                                                     isize))
                          as libc::c_int != 0 {
                children.size = children.size.wrapping_sub(1)
            }
            let mut parent: MutableSubtree =
                ts_subtree_new_node(&mut (*self_0).tree_pool, symbol,
                                    &mut children,
                                    production_id as libc::c_uint,
                                    (*self_0).language);
            while i.wrapping_add(1 as libc::c_int as libc::c_uint) < pop.size
                  {
                let mut next_slice_0: StackSlice =
                    *pop.contents.offset(i.wrapping_add(1 as libc::c_int as
                                                            libc::c_uint) as
                                             isize);
                if next_slice_0.version != slice.version { break ; }
                i = i.wrapping_add(1);
                let mut children_0: SubtreeArray = next_slice_0.subtrees;
                while children_0.size > 0 as libc::c_int as libc::c_uint &&
                          ts_subtree_extra(*children_0.contents.offset(children_0.size.wrapping_sub(1
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        libc::c_uint)
                                                                           as
                                                                           isize))
                              as libc::c_int != 0 {
                    children_0.size = children_0.size.wrapping_sub(1)
                }
                if ts_parser__replace_children(self_0, &mut parent,
                                               &mut children_0) {
                    ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                            &mut slice.subtrees);
                    slice = next_slice_0
                } else {
                    ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                            &mut next_slice_0.subtrees);
                }
            }
            (*parent.ptr).c2rust_unnamed.c2rust_unnamed.dynamic_precedence +=
                dynamic_precedence;
            (*parent.ptr).c2rust_unnamed.c2rust_unnamed.production_id =
                production_id;
            let mut state: TSStateId =
                ts_stack_state((*self_0).stack, slice_version);
            let mut next_state: TSStateId =
                ts_language_next_state((*self_0).language, state, symbol);
            if is_extra { (*parent.ptr).set_extra(1 as libc::c_int != 0) }
            if is_fragile as libc::c_int != 0 ||
                   pop.size > 1 as libc::c_int as libc::c_uint ||
                   initial_version_count > 1 as libc::c_int as libc::c_uint {
                (*parent.ptr).set_fragile_left(1 as libc::c_int != 0);
                (*parent.ptr).set_fragile_right(1 as libc::c_int != 0);
                (*parent.ptr).parse_state = TS_TREE_STATE_NONE
            } else { (*parent.ptr).parse_state = state }
            ts_stack_push((*self_0).stack, slice_version,
                          ts_subtree_from_mut(parent), 0 as libc::c_int != 0,
                          next_state);
            let mut j: uint32_t = (*parent.ptr).child_count;
            while j < slice.subtrees.size {
                ts_stack_push((*self_0).stack, slice_version,
                              *slice.subtrees.contents.offset(j as isize),
                              0 as libc::c_int != 0, next_state);
                j = j.wrapping_add(1)
            }
            let mut j_0: StackVersion = 0 as libc::c_int as StackVersion;
            while j_0 < slice_version {
                if !(j_0 == version) {
                    if ts_stack_merge((*self_0).stack, j_0, slice_version) {
                        removed_version_count =
                            removed_version_count.wrapping_add(1);
                        break ;
                    }
                }
                j_0 = j_0.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    return if ts_stack_version_count((*self_0).stack) > initial_version_count
              {
               initial_version_count
           } else { -(1 as libc::c_int) as StackVersion };
}
unsafe extern "C" fn ts_string_input_read(mut _self: *mut libc::c_void,
                                          mut byte: uint32_t, mut pt: TSPoint,
                                          mut length: *mut uint32_t)
 -> *const libc::c_char {
    let mut self_0: *mut TSStringInput = _self as *mut TSStringInput;
    if byte >= (*self_0).length {
        *length = 0 as libc::c_int as uint32_t;
        return b"\x00" as *const u8 as *const libc::c_char
    } else {
        *length = (*self_0).length.wrapping_sub(byte);
        return (*self_0).string.offset(byte as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_parse_string(mut self_0: *mut TSParser,
                                                mut old_tree: *const TSTree,
                                                mut string:
                                                    *const libc::c_char,
                                                mut length: uint32_t)
 -> *mut TSTree {
    return ts_parser_parse_string_encoding(self_0, old_tree, string, length,
                                           TSInputEncodingUTF8);
}
unsafe extern "C" fn ts_parser_has_outstanding_parse(mut self_0:
                                                         *mut TSParser)
 -> bool {
    return ts_stack_state((*self_0).stack, 0 as libc::c_int as StackVersion)
               as libc::c_int != 1 as libc::c_int ||
               ts_stack_node_count_since_error((*self_0).stack,
                                               0 as libc::c_int as
                                                   StackVersion) !=
                   0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn ts_parser__log(mut self_0: *mut TSParser) {
    if (*self_0).lexer.logger.log.is_some() {
        (*self_0).lexer.logger.log.expect("non-null function pointer")((*self_0).lexer.logger.payload,
                                                                       TSLogTypeParse,
                                                                       (*self_0).lexer.debug_buffer.as_mut_ptr());
    }
    if !(*self_0).dot_graph_file.is_null() {
        fprintf((*self_0).dot_graph_file,
                b"graph {\nlabel=\"\x00" as *const u8 as *const libc::c_char);
        let mut c: *mut libc::c_char =
            &mut *(*self_0).lexer.debug_buffer.as_mut_ptr().offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
                as *mut libc::c_char;
        while *c as libc::c_int != 0 as libc::c_int {
            if *c as libc::c_int == '\"' as i32 {
                fputc('\\' as i32, (*self_0).dot_graph_file);
            }
            fputc(*c as libc::c_int, (*self_0).dot_graph_file);
            c = c.offset(1)
        }
        fprintf((*self_0).dot_graph_file,
                b"\"\n}\n\n\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn ts_parser__condense_stack(mut self_0: *mut TSParser)
 -> libc::c_uint {
    let mut made_changes: bool = 0 as libc::c_int != 0;
    let mut min_error_cost: libc::c_uint =
        (2147483647 as libc::c_int as
             libc::c_uint).wrapping_mul(2 as
                                            libc::c_uint).wrapping_add(1 as
                                                                           libc::c_uint);
    let mut i: StackVersion = 0 as libc::c_int as StackVersion;
    while i < ts_stack_version_count((*self_0).stack) {
        if ts_stack_is_halted((*self_0).stack, i) {
            ts_stack_remove_version((*self_0).stack, i);
            i = i.wrapping_sub(1)
        } else {
            let mut status_i: ErrorStatus =
                ts_parser__version_status(self_0, i);
            if !status_i.is_in_error && status_i.cost < min_error_cost {
                min_error_cost = status_i.cost
            }
            let mut j: StackVersion = 0 as libc::c_int as StackVersion;
            while j < i {
                let mut status_j: ErrorStatus =
                    ts_parser__version_status(self_0, j);
                match ts_parser__compare_versions(self_0, status_j, status_i)
                          as libc::c_uint {
                    0 => {
                        made_changes = 1 as libc::c_int != 0;
                        ts_stack_remove_version((*self_0).stack, i);
                        i = i.wrapping_sub(1);
                        j = i
                    }
                    1 | 2 => {
                        if ts_stack_merge((*self_0).stack, j, i) {
                            made_changes = 1 as libc::c_int != 0;
                            i = i.wrapping_sub(1);
                            j = i
                        }
                    }
                    3 => {
                        made_changes = 1 as libc::c_int != 0;
                        if ts_stack_merge((*self_0).stack, j, i) {
                            i = i.wrapping_sub(1);
                            j = i
                        } else {
                            ts_stack_swap_versions((*self_0).stack, i, j);
                        }
                    }
                    4 => {
                        made_changes = 1 as libc::c_int != 0;
                        ts_stack_remove_version((*self_0).stack, j);
                        i = i.wrapping_sub(1);
                        j = j.wrapping_sub(1)
                    }
                    _ => { }
                }
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    }
    while ts_stack_version_count((*self_0).stack) > MAX_VERSION_COUNT {
        ts_stack_remove_version((*self_0).stack, MAX_VERSION_COUNT);
        made_changes = 1 as libc::c_int != 0
    }
    if ts_stack_version_count((*self_0).stack) >
           0 as libc::c_int as libc::c_uint {
        let mut has_unpaused_version: bool = 0 as libc::c_int != 0;
        let mut i_0: StackVersion = 0 as libc::c_int as StackVersion;
        let mut n: StackVersion = ts_stack_version_count((*self_0).stack);
        while i_0 < n {
            if ts_stack_is_paused((*self_0).stack, i_0) {
                if !has_unpaused_version &&
                       (*self_0).accept_count < MAX_VERSION_COUNT {
                    if (*self_0).lexer.logger.log.is_some() ||
                           !(*self_0).dot_graph_file.is_null() {
                        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                 1024 as libc::c_int as libc::c_ulong,
                                 b"resume version:%u\x00" as *const u8 as
                                     *const libc::c_char, i_0);
                        ts_parser__log(self_0);
                    }
                    min_error_cost =
                        ts_stack_error_cost((*self_0).stack, i_0);
                    let mut lookahead_symbol: TSSymbol =
                        ts_stack_resume((*self_0).stack, i_0);
                    ts_parser__handle_error(self_0, i_0, lookahead_symbol);
                    has_unpaused_version = 1 as libc::c_int != 0
                } else {
                    ts_stack_remove_version((*self_0).stack, i_0);
                    i_0 = i_0.wrapping_sub(1);
                    n = n.wrapping_sub(1)
                }
            } else { has_unpaused_version = 1 as libc::c_int != 0 }
            i_0 = i_0.wrapping_add(1)
        }
    }
    if made_changes {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"condense\x00" as *const u8 as *const libc::c_char);
            ts_parser__log(self_0);
        }
        if !(*self_0).dot_graph_file.is_null() {
            ts_stack_print_dot_graph((*self_0).stack, (*self_0).language,
                                     (*self_0).dot_graph_file);
            fputs(b"\n\n\x00" as *const u8 as *const libc::c_char,
                  (*self_0).dot_graph_file);
        }
    }
    return min_error_cost;
}
#[inline]
unsafe extern "C" fn atomic_inc(mut p: *mut uint32_t) -> uint32_t {
    let fresh10 = p;
    let fresh11 = 1 as libc::c_uint;
    return ::std::intrinsics::atomic_xadd(fresh10, fresh11) + fresh11;
}
unsafe extern "C" fn ts_parser__handle_error(mut self_0: *mut TSParser,
                                             mut version: StackVersion,
                                             mut lookahead_symbol: TSSymbol) {
    let mut previous_version_count: uint32_t =
        ts_stack_version_count((*self_0).stack);
    ts_parser__do_all_potential_reductions(self_0, version,
                                           0 as libc::c_int as TSSymbol);
    let mut version_count: uint32_t = ts_stack_version_count((*self_0).stack);
    let mut position: Length = ts_stack_position((*self_0).stack, version);
    let mut did_insert_missing_token: bool = 0 as libc::c_int != 0;
    let mut v: StackVersion = version;
    while v < version_count {
        if !did_insert_missing_token {
            let mut state: TSStateId = ts_stack_state((*self_0).stack, v);
            let mut missing_symbol: TSSymbol = 1 as libc::c_int as TSSymbol;
            while (missing_symbol as libc::c_uint) <
                      (*(*self_0).language).token_count {
                let mut state_after_missing_symbol: TSStateId =
                    ts_language_next_state((*self_0).language, state,
                                           missing_symbol);
                if !(state_after_missing_symbol as libc::c_int ==
                         0 as libc::c_int ||
                         state_after_missing_symbol as libc::c_int ==
                             state as libc::c_int) {
                    if ts_language_has_reduce_action((*self_0).language,
                                                     state_after_missing_symbol,
                                                     lookahead_symbol) {
                        ts_lexer_reset(&mut (*self_0).lexer, position);
                        ts_lexer_mark_end(&mut (*self_0).lexer);
                        let mut padding: Length =
                            length_sub((*self_0).lexer.token_end_position,
                                       position);
                        let mut version_with_missing_tree: StackVersion =
                            ts_stack_copy_version((*self_0).stack, v);
                        let mut missing_tree: Subtree =
                            ts_subtree_new_missing_leaf(&mut (*self_0).tree_pool,
                                                        missing_symbol,
                                                        padding,
                                                        (*self_0).language);
                        ts_stack_push((*self_0).stack,
                                      version_with_missing_tree, missing_tree,
                                      0 as libc::c_int != 0,
                                      state_after_missing_symbol);
                        if ts_parser__do_all_potential_reductions(self_0,
                                                                  version_with_missing_tree,
                                                                  lookahead_symbol)
                           {
                            if (*self_0).lexer.logger.log.is_some() ||
                                   !(*self_0).dot_graph_file.is_null() {
                                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                         1024 as libc::c_int as libc::c_ulong,
                                         b"recover_with_missing symbol:%s, state:%u\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         ts_language_symbol_name((*self_0).language,
                                                                 missing_symbol),
                                         ts_stack_state((*self_0).stack,
                                                        version_with_missing_tree)
                                             as libc::c_int);
                                ts_parser__log(self_0);
                            }
                            did_insert_missing_token = 1 as libc::c_int != 0;
                            break ;
                        }
                    }
                }
                missing_symbol = missing_symbol.wrapping_add(1)
            }
        }
        ts_stack_push((*self_0).stack, v,
                      Subtree{ptr: 0 as *const SubtreeHeapData,},
                      0 as libc::c_int != 0, 0 as libc::c_int as TSStateId);
        v =
            if v == version {
                previous_version_count
            } else { v.wrapping_add(1 as libc::c_int as libc::c_uint) }
    }
    let mut i: libc::c_uint = previous_version_count;
    while i < version_count {
        let mut did_merge: bool =
            ts_stack_merge((*self_0).stack, version, previous_version_count);
        if did_merge {
        } else {
            __assert_fail(b"did_merge\x00" as *const u8 as
                              *const libc::c_char,
                          b"lib/src/./parser.c\x00" as *const u8 as
                              *const libc::c_char,
                          1076 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 65],
                                                    &[libc::c_char; 65]>(b"void ts_parser__handle_error(TSParser *, StackVersion, TSSymbol)\x00")).as_ptr());
        }
        i = i.wrapping_add(1)
    }
    ts_stack_record_summary((*self_0).stack, version, MAX_SUMMARY_DEPTH);
    if !(*self_0).dot_graph_file.is_null() {
        ts_stack_print_dot_graph((*self_0).stack, (*self_0).language,
                                 (*self_0).dot_graph_file);
        fputs(b"\n\n\x00" as *const u8 as *const libc::c_char,
              (*self_0).dot_graph_file);
    };
}
static mut MAX_SUMMARY_DEPTH: libc::c_uint =
    16 as libc::c_int as libc::c_uint;
unsafe extern "C" fn ts_parser__replace_children(mut self_0: *mut TSParser,
                                                 mut tree:
                                                     *mut MutableSubtree,
                                                 mut children:
                                                     *mut SubtreeArray)
 -> bool {
    *(*self_0).scratch_tree.ptr = *(*tree).ptr;
    (*(*self_0).scratch_tree.ptr).child_count = 0 as libc::c_int as uint32_t;
    ts_subtree_set_children((*self_0).scratch_tree, (*children).contents,
                            (*children).size, (*self_0).language);
    if ts_parser__select_tree(self_0, ts_subtree_from_mut(*tree),
                              ts_subtree_from_mut((*self_0).scratch_tree)) {
        *(*tree).ptr = *(*self_0).scratch_tree.ptr;
        return 1 as libc::c_int != 0
    } else { return 0 as libc::c_int != 0 };
}
#[inline]
unsafe extern "C" fn reusable_node_new() -> ReusableNode {
    return {
               let mut init =
                   ReusableNode{stack:
                                    {
                                        let mut init =
                                            C2RustUnnamed_8{contents:
                                                                0 as
                                                                    *mut StackEntry,
                                                            size:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,
                                                            capacity:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,};
                                        init
                                    },
                                last_external_token:
                                    Subtree{ptr:
                                                0 as
                                                    *const SubtreeHeapData,},};
               init
           };
}
static mut MAX_VERSION_COUNT: libc::c_uint = 6 as libc::c_int as libc::c_uint;
#[inline]
unsafe extern "C" fn reusable_node_reset(mut self_0: *mut ReusableNode,
                                         mut tree: Subtree) {
    reusable_node_clear(self_0);
    array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackEntry>() as libc::c_ulong);
    let fresh12 = (*self_0).stack.size;
    (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
    *(*self_0).stack.contents.offset(fresh12 as isize) =
        {
            let mut init =
                StackEntry{tree: tree,
                           child_index: 0 as libc::c_int as uint32_t,
                           byte_offset: 0 as libc::c_int as uint32_t,};
            init
        };
}
#[inline]
unsafe extern "C" fn atomic_dec(mut p: *mut uint32_t) -> uint32_t {
    let fresh13 = p;
    let fresh14 = 1 as libc::c_uint;
    return ::std::intrinsics::atomic_xsub(fresh13, fresh14) - fresh14;
}
unsafe extern "C" fn ts_parser__set_cached_token(mut self_0: *mut TSParser,
                                                 mut byte_index: size_t,
                                                 mut last_external_token:
                                                     Subtree,
                                                 mut token: Subtree) {
    let mut cache: *mut TokenCache = &mut (*self_0).token_cache;
    if !token.ptr.is_null() { ts_subtree_retain(token); }
    if !last_external_token.ptr.is_null() {
        ts_subtree_retain(last_external_token);
    }
    if !(*cache).token.ptr.is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*cache).token);
    }
    if !(*cache).last_external_token.ptr.is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool,
                           (*cache).last_external_token);
    }
    (*cache).token = token;
    (*cache).byte_index = byte_index as uint32_t;
    (*cache).last_external_token = last_external_token;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_new() -> *mut TSParser {
    let mut self_0: *mut TSParser =
        ts_calloc(1 as libc::c_int as size_t,
                  ::std::mem::size_of::<TSParser>() as libc::c_ulong) as
            *mut TSParser;
    ts_lexer_init(&mut (*self_0).lexer);
    (*self_0).reduce_actions.size = 0 as libc::c_int as uint32_t;
    (*self_0).reduce_actions.capacity = 0 as libc::c_int as uint32_t;
    (*self_0).reduce_actions.contents = 0 as *mut ReduceAction;
    array__reserve(&mut (*self_0).reduce_actions as *mut ReduceActionSet as
                       *mut VoidArray,
                   ::std::mem::size_of::<ReduceAction>() as libc::c_ulong,
                   4 as libc::c_int as uint32_t);
    (*self_0).tree_pool = ts_subtree_pool_new(32 as libc::c_int as uint32_t);
    (*self_0).stack = ts_stack_new(&mut (*self_0).tree_pool);
    (*self_0).finished_tree = Subtree{ptr: 0 as *const SubtreeHeapData,};
    (*self_0).reusable_node = reusable_node_new();
    (*self_0).dot_graph_file = 0 as *mut FILE;
    (*self_0).cancellation_flag = 0 as *const size_t;
    (*self_0).timeout_duration = 0 as libc::c_int as TSDuration;
    (*self_0).end_clock = clock_null();
    (*self_0).operation_count = 0 as libc::c_int as libc::c_uint;
    (*self_0).old_tree = Subtree{ptr: 0 as *const SubtreeHeapData,};
    (*self_0).scratch_tree.ptr = &mut (*self_0).scratch_tree_data;
    (*self_0).included_range_differences =
        {
            let mut init =
                TSRangeArray{contents: 0 as *mut TSRange,
                             size: 0 as libc::c_int as uint32_t,
                             capacity: 0 as libc::c_int as uint32_t,};
            init
        };
    (*self_0).included_range_difference_index =
        0 as libc::c_int as libc::c_uint;
    ts_parser__set_cached_token(self_0, 0 as libc::c_int as size_t,
                                Subtree{ptr: 0 as *const SubtreeHeapData,},
                                Subtree{ptr: 0 as *const SubtreeHeapData,});
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_timeout_micros(mut self_0: *const TSParser)
 -> uint64_t {
    return duration_to_micros((*self_0).timeout_duration);
}
unsafe extern "C" fn ts_parser__select_tree(mut self_0: *mut TSParser,
                                            mut left: Subtree,
                                            mut right: Subtree) -> bool {
    if left.ptr.is_null() { return 1 as libc::c_int != 0 }
    if right.ptr.is_null() { return 0 as libc::c_int != 0 }
    if ts_subtree_error_cost(right) < ts_subtree_error_cost(left) {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"select_smaller_error symbol:%s, over_symbol:%s\x00" as
                         *const u8 as *const libc::c_char,
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(right)),
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(left)));
            ts_parser__log(self_0);
        }
        return 1 as libc::c_int != 0
    }
    if ts_subtree_error_cost(left) < ts_subtree_error_cost(right) {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"select_smaller_error symbol:%s, over_symbol:%s\x00" as
                         *const u8 as *const libc::c_char,
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(left)),
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(right)));
            ts_parser__log(self_0);
        }
        return 0 as libc::c_int != 0
    }
    if ts_subtree_dynamic_precedence(right) >
           ts_subtree_dynamic_precedence(left) {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"select_higher_precedence symbol:%s, prec:%u, over_symbol:%s, other_prec:%u\x00"
                         as *const u8 as *const libc::c_char,
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(right)),
                     ts_subtree_dynamic_precedence(right),
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(left)),
                     ts_subtree_dynamic_precedence(left));
            ts_parser__log(self_0);
        }
        return 1 as libc::c_int != 0
    }
    if ts_subtree_dynamic_precedence(left) >
           ts_subtree_dynamic_precedence(right) {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"select_higher_precedence symbol:%s, prec:%u, over_symbol:%s, other_prec:%u\x00"
                         as *const u8 as *const libc::c_char,
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(left)),
                     ts_subtree_dynamic_precedence(left),
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(right)),
                     ts_subtree_dynamic_precedence(right));
            ts_parser__log(self_0);
        }
        return 0 as libc::c_int != 0
    }
    if ts_subtree_error_cost(left) > 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0
    }
    let mut comparison: libc::c_int = ts_subtree_compare(left, right);
    match comparison {
        -1 => {
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"select_earlier symbol:%s, over_symbol:%s\x00" as
                             *const u8 as *const libc::c_char,
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(left)),
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(right)));
                ts_parser__log(self_0);
            }
            return 0 as libc::c_int != 0
        }
        1 => {
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"select_earlier symbol:%s, over_symbol:%s\x00" as
                             *const u8 as *const libc::c_char,
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(right)),
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(left)));
                ts_parser__log(self_0);
            }
            return 1 as libc::c_int != 0
        }
        _ => {
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"select_existing symbol:%s, over_symbol:%s\x00" as
                             *const u8 as *const libc::c_char,
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(left)),
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(right)));
                ts_parser__log(self_0);
            }
            return 0 as libc::c_int != 0
        }
    };
}
unsafe extern "C" fn ts_parser__do_all_potential_reductions(mut self_0:
                                                                *mut TSParser,
                                                            mut starting_version:
                                                                StackVersion,
                                                            mut lookahead_symbol:
                                                                TSSymbol)
 -> bool {
    let mut initial_version_count: uint32_t =
        ts_stack_version_count((*self_0).stack);
    let mut can_shift_lookahead_symbol: bool = 0 as libc::c_int != 0;
    let mut version: StackVersion = starting_version;
    let mut current_block_33: u64;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        let mut version_count: uint32_t =
            ts_stack_version_count((*self_0).stack);
        if version >= version_count { break ; }
        let mut merged: bool = 0 as libc::c_int != 0;
        let mut i_0: StackVersion = initial_version_count;
        while i_0 < version {
            if ts_stack_merge((*self_0).stack, i_0, version) {
                merged = 1 as libc::c_int != 0;
                break ;
            } else { i_0 = i_0.wrapping_add(1) }
        }
        if !merged {
            let mut state: TSStateId =
                ts_stack_state((*self_0).stack, version);
            let mut has_shift_action: bool = 0 as libc::c_int != 0;
            (*self_0).reduce_actions.size = 0 as libc::c_int as uint32_t;
            let mut first_symbol: TSSymbol = 0;
            let mut end_symbol: TSSymbol = 0;
            if lookahead_symbol as libc::c_int != 0 as libc::c_int {
                first_symbol = lookahead_symbol;
                end_symbol =
                    (lookahead_symbol as libc::c_int + 1 as libc::c_int) as
                        TSSymbol
            } else {
                first_symbol = 1 as libc::c_int as TSSymbol;
                end_symbol = (*(*self_0).language).token_count as TSSymbol
            }
            let mut symbol: TSSymbol = first_symbol;
            while (symbol as libc::c_int) < end_symbol as libc::c_int {
                let mut entry: TableEntry =
                    TableEntry{actions: 0 as *const TSParseAction,
                               action_count: 0,
                               is_reusable: false,};
                ts_language_table_entry((*self_0).language, state, symbol,
                                        &mut entry);
                let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
                while i_1 < entry.action_count {
                    let mut action: TSParseAction =
                        *entry.actions.offset(i_1 as isize);
                    match action.type_0() as libc::c_int {
                        0 | 3 => {
                            if !action.params.shift.extra() &&
                                   !action.params.shift.repetition() {
                                has_shift_action = 1 as libc::c_int != 0
                            }
                        }
                        1 => {
                            if action.params.reduce.child_count as libc::c_int
                                   > 0 as libc::c_int {
                                ts_reduce_action_set_add(&mut (*self_0).reduce_actions,
                                                         {
                                                             let mut init =
                                                                 ReduceAction{count:
                                                                                  action.params.reduce.child_count
                                                                                      as
                                                                                      uint32_t,
                                                                              symbol:
                                                                                  action.params.reduce.symbol,
                                                                              dynamic_precedence:
                                                                                  action.params.reduce.dynamic_precedence
                                                                                      as
                                                                                      libc::c_int,
                                                                              production_id:
                                                                                  action.params.reduce.production_id
                                                                                      as
                                                                                      libc::c_ushort,};
                                                             init
                                                         });
                            }
                        }
                        _ => { }
                    }
                    i_1 = i_1.wrapping_add(1)
                }
                symbol = symbol.wrapping_add(1)
            }
            let mut reduction_version: StackVersion =
                -(1 as libc::c_int) as StackVersion;
            let mut i_2: uint32_t = 0 as libc::c_int as uint32_t;
            while i_2 < (*self_0).reduce_actions.size {
                let mut action_0: ReduceAction =
                    *(*self_0).reduce_actions.contents.offset(i_2 as isize);
                reduction_version =
                    ts_parser__reduce(self_0, version, action_0.symbol,
                                      action_0.count,
                                      action_0.dynamic_precedence,
                                      action_0.production_id,
                                      1 as libc::c_int != 0,
                                      0 as libc::c_int != 0);
                i_2 = i_2.wrapping_add(1)
            }
            if has_shift_action {
                can_shift_lookahead_symbol = 1 as libc::c_int != 0;
                current_block_33 = 13619784596304402172;
            } else if reduction_version != -(1 as libc::c_int) as StackVersion
                          && i < MAX_VERSION_COUNT {
                ts_stack_renumber_version((*self_0).stack, reduction_version,
                                          version);
                current_block_33 = 14916268686031723178;
            } else {
                if lookahead_symbol as libc::c_int != 0 as libc::c_int {
                    ts_stack_remove_version((*self_0).stack, version);
                }
                current_block_33 = 13619784596304402172;
            }
            match current_block_33 {
                14916268686031723178 => { }
                _ => {
                    if version == starting_version {
                        version = version_count
                    } else { version = version.wrapping_add(1) }
                }
            }
        }
        i = i.wrapping_add(1)
    }
    return can_shift_lookahead_symbol;
}
static mut MAX_VERSION_COUNT_OVERFLOW: libc::c_uint =
    4 as libc::c_int as libc::c_uint;
unsafe extern "C" fn ts_parser__has_included_range_difference(mut self_0:
                                                                  *const TSParser,
                                                              mut start_position:
                                                                  uint32_t,
                                                              mut end_position:
                                                                  uint32_t)
 -> bool {
    return ts_range_array_intersects(&(*self_0).included_range_differences,
                                     (*self_0).included_range_difference_index,
                                     start_position, end_position);
}
#[inline]
unsafe extern "C" fn reusable_node_byte_offset(mut self_0: *mut ReusableNode)
 -> uint32_t {
    return if (*self_0).stack.size > 0 as libc::c_int as libc::c_uint {
               (*(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                                                     as isize)).byte_offset
           } else { 4294967295 as libc::c_uint };
}
#[inline]
unsafe extern "C" fn reusable_node_advance_past_leaf(mut self_0:
                                                         *mut ReusableNode) {
    while reusable_node_descend(self_0) { }
    reusable_node_advance(self_0);
}
unsafe extern "C" fn ts_parser__reuse_node(mut self_0: *mut TSParser,
                                           mut version: StackVersion,
                                           mut state: *mut TSStateId,
                                           mut position: uint32_t,
                                           mut last_external_token: Subtree,
                                           mut table_entry: *mut TableEntry)
 -> Subtree {
    let mut result: Subtree =
        Subtree{data:
                    SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                          [0; 1],
                                      symbol: 0,
                                      padding_bytes: 0,
                                      size_bytes: 0,
                                      padding_columns: 0,
                                      padding_rows_lookahead_bytes: [0; 1],
                                      parse_state: 0,},};
    loop  {
        result = reusable_node_tree(&mut (*self_0).reusable_node);
        if result.ptr.is_null() { break ; }
        let mut byte_offset: uint32_t =
            reusable_node_byte_offset(&mut (*self_0).reusable_node);
        let mut end_byte_offset: uint32_t =
            byte_offset.wrapping_add(ts_subtree_total_bytes(result));
        if ts_subtree_is_eof(result) {
            end_byte_offset = 4294967295 as libc::c_uint
        }
        if byte_offset > position {
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"before_reusable_node symbol:%s\x00" as *const u8 as
                             *const libc::c_char,
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(result)));
                ts_parser__log(self_0);
            }
            break ;
        } else if byte_offset < position {
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"past_reusable_node symbol:%s\x00" as *const u8 as
                             *const libc::c_char,
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(result)));
                ts_parser__log(self_0);
            }
            if end_byte_offset <= position ||
                   !reusable_node_descend(&mut (*self_0).reusable_node) {
                reusable_node_advance(&mut (*self_0).reusable_node);
            }
        } else if !ts_subtree_external_scanner_state_eq((*self_0).reusable_node.last_external_token,
                                                        last_external_token) {
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"reusable_node_has_different_external_scanner_state symbol:%s\x00"
                             as *const u8 as *const libc::c_char,
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(result)));
                ts_parser__log(self_0);
            }
            reusable_node_advance(&mut (*self_0).reusable_node);
        } else {
            let mut reason: *const libc::c_char = 0 as *const libc::c_char;
            if ts_subtree_has_changes(result) {
                reason =
                    b"has_changes\x00" as *const u8 as *const libc::c_char
            } else if ts_subtree_is_error(result) {
                reason = b"is_error\x00" as *const u8 as *const libc::c_char
            } else if ts_subtree_missing(result) {
                reason = b"is_missing\x00" as *const u8 as *const libc::c_char
            } else if ts_subtree_is_fragile(result) {
                reason = b"is_fragile\x00" as *const u8 as *const libc::c_char
            } else if ts_parser__has_included_range_difference(self_0,
                                                               byte_offset,
                                                               end_byte_offset)
             {
                reason =
                    b"contains_different_included_range\x00" as *const u8 as
                        *const libc::c_char
            }
            if !reason.is_null() {
                if (*self_0).lexer.logger.log.is_some() ||
                       !(*self_0).dot_graph_file.is_null() {
                    snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                             1024 as libc::c_int as libc::c_ulong,
                             b"cant_reuse_node_%s tree:%s\x00" as *const u8 as
                                 *const libc::c_char, reason,
                             ts_language_symbol_name((*self_0).language,
                                                     ts_subtree_symbol(result)));
                    ts_parser__log(self_0);
                }
                if !reusable_node_descend(&mut (*self_0).reusable_node) {
                    reusable_node_advance(&mut (*self_0).reusable_node);
                    ts_parser__breakdown_top_of_stack(self_0, version);
                    *state = ts_stack_state((*self_0).stack, version)
                }
            } else {
                let mut leaf_symbol: TSSymbol =
                    ts_subtree_leaf_symbol(result);
                ts_language_table_entry((*self_0).language, *state,
                                        leaf_symbol, table_entry);
                if !ts_parser__can_reuse_first_leaf(self_0, *state, result,
                                                    table_entry) {
                    if (*self_0).lexer.logger.log.is_some() ||
                           !(*self_0).dot_graph_file.is_null() {
                        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                 1024 as libc::c_int as libc::c_ulong,
                                 b"cant_reuse_node symbol:%s, first_leaf_symbol:%s\x00"
                                     as *const u8 as *const libc::c_char,
                                 ts_language_symbol_name((*self_0).language,
                                                         ts_subtree_symbol(result)),
                                 ts_language_symbol_name((*self_0).language,
                                                         leaf_symbol));
                        ts_parser__log(self_0);
                    }
                    reusable_node_advance_past_leaf(&mut (*self_0).reusable_node);
                    break ;
                } else {
                    if (*self_0).lexer.logger.log.is_some() ||
                           !(*self_0).dot_graph_file.is_null() {
                        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                 1024 as libc::c_int as libc::c_ulong,
                                 b"reuse_node symbol:%s\x00" as *const u8 as
                                     *const libc::c_char,
                                 ts_language_symbol_name((*self_0).language,
                                                         ts_subtree_symbol(result)));
                        ts_parser__log(self_0);
                    }
                    ts_subtree_retain(result);
                    return result
                }
            }
        }
    }
    return Subtree{ptr: 0 as *const SubtreeHeapData,};
}
unsafe extern "C" fn ts_parser__can_reuse_first_leaf(mut self_0:
                                                         *mut TSParser,
                                                     mut state: TSStateId,
                                                     mut tree: Subtree,
                                                     mut table_entry:
                                                         *mut TableEntry)
 -> bool {
    let mut current_lex_mode: TSLexMode =
        *(*(*self_0).language).lex_modes.offset(state as isize);
    let mut leaf_symbol: TSSymbol = ts_subtree_leaf_symbol(tree);
    let mut leaf_state: TSStateId = ts_subtree_leaf_parse_state(tree);
    let mut leaf_lex_mode: TSLexMode =
        *(*(*self_0).language).lex_modes.offset(leaf_state as isize);
    if current_lex_mode.lex_state as libc::c_int ==
           -(1 as libc::c_int) as uint16_t as libc::c_int {
        return 0 as libc::c_int != 0
    }
    if (*table_entry).action_count > 0 as libc::c_int as libc::c_uint &&
           memcmp(&mut leaf_lex_mode as *mut TSLexMode as *const libc::c_void,
                  &mut current_lex_mode as *mut TSLexMode as
                      *const libc::c_void,
                  ::std::mem::size_of::<TSLexMode>() as libc::c_ulong) ==
               0 as libc::c_int &&
           (leaf_symbol as libc::c_int !=
                (*(*self_0).language).keyword_capture_token as libc::c_int ||
                !ts_subtree_is_keyword(tree) &&
                    ts_subtree_parse_state(tree) as libc::c_int ==
                        state as libc::c_int) {
        return 1 as libc::c_int != 0
    }
    if ts_subtree_size(tree).bytes == 0 as libc::c_int as libc::c_uint &&
           leaf_symbol as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int != 0
    }
    return current_lex_mode.external_lex_state as libc::c_int ==
               0 as libc::c_int &&
               (*table_entry).is_reusable as libc::c_int != 0;
}
unsafe extern "C" fn ts_parser__get_cached_token(mut self_0: *mut TSParser,
                                                 mut state: TSStateId,
                                                 mut position: size_t,
                                                 mut last_external_token:
                                                     Subtree,
                                                 mut table_entry:
                                                     *mut TableEntry)
 -> Subtree {
    let mut cache: *mut TokenCache = &mut (*self_0).token_cache;
    if !(*cache).token.ptr.is_null() &&
           (*cache).byte_index as libc::c_ulong == position &&
           ts_subtree_external_scanner_state_eq((*cache).last_external_token,
                                                last_external_token) as
               libc::c_int != 0 {
        ts_language_table_entry((*self_0).language, state,
                                ts_subtree_symbol((*cache).token),
                                table_entry);
        if ts_parser__can_reuse_first_leaf(self_0, state, (*cache).token,
                                           table_entry) {
            ts_subtree_retain((*cache).token);
            return (*cache).token
        }
    }
    return Subtree{ptr: 0 as *const SubtreeHeapData,};
}
static mut OP_COUNT_PER_TIMEOUT_CHECK: libc::c_uint =
    100 as libc::c_int as libc::c_uint;
#[inline]
unsafe extern "C" fn atomic_load(mut p: *const size_t) -> size_t {
    return ::std::intrinsics::atomic_load_relaxed(p);
}
#[inline]
unsafe extern "C" fn ts_reduce_action_set_add(mut self_0:
                                                  *mut ReduceActionSet,
                                              mut new_action: ReduceAction) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).size {
        let mut action: ReduceAction = *(*self_0).contents.offset(i as isize);
        if action.symbol as libc::c_int == new_action.symbol as libc::c_int &&
               action.count == new_action.count {
            return
        }
        i = i.wrapping_add(1)
    }
    array__grow(self_0 as *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<ReduceAction>() as libc::c_ulong);
    let fresh15 = (*self_0).size;
    (*self_0).size = (*self_0).size.wrapping_add(1);
    *(*self_0).contents.offset(fresh15 as isize) = new_action;
}
#[inline]
unsafe extern "C" fn clock_now() -> TSClock {
    let mut result: TSClock = TSClock{tv_sec: 0, tv_nsec: 0,};
    clock_gettime(1 as libc::c_int, &mut result);
    return result;
}
unsafe extern "C" fn ts_parser__shift(mut self_0: *mut TSParser,
                                      mut version: StackVersion,
                                      mut state: TSStateId,
                                      mut lookahead: Subtree,
                                      mut extra: bool) {
    let mut subtree_to_push: Subtree =
        Subtree{data:
                    SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                          [0; 1],
                                      symbol: 0,
                                      padding_bytes: 0,
                                      size_bytes: 0,
                                      padding_columns: 0,
                                      padding_rows_lookahead_bytes: [0; 1],
                                      parse_state: 0,},};
    if extra as libc::c_int != ts_subtree_extra(lookahead) as libc::c_int {
        let mut result: MutableSubtree =
            ts_subtree_make_mut(&mut (*self_0).tree_pool, lookahead);
        ts_subtree_set_extra(&mut result);
        subtree_to_push = ts_subtree_from_mut(result)
    } else { subtree_to_push = lookahead }
    let mut is_pending: bool =
        ts_subtree_child_count(subtree_to_push) >
            0 as libc::c_int as libc::c_uint;
    ts_stack_push((*self_0).stack, version, subtree_to_push, is_pending,
                  state);
    if ts_subtree_has_external_tokens(subtree_to_push) {
        ts_stack_set_last_external_token((*self_0).stack, version,
                                         ts_subtree_last_external_token(subtree_to_push));
    };
}
#[inline]
unsafe extern "C" fn reusable_node_descend(mut self_0: *mut ReusableNode)
 -> bool {
    if (*self_0).stack.size.wrapping_sub(1 as libc::c_int as libc::c_uint) <
           (*self_0).stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->stack)->size - 1 < (&self->stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./reusable_node.h\x00" as *const u8 as
                          *const libc::c_char,
                      72 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 44],
                                                &[libc::c_char; 44]>(b"_Bool reusable_node_descend(ReusableNode *)\x00")).as_ptr());
    }
    let mut last_entry: StackEntry =
        *(&mut *(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint)
                                                    as isize) as
              *mut StackEntry);
    if ts_subtree_child_count(last_entry.tree) >
           0 as libc::c_int as libc::c_uint {
        array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                        *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<StackEntry>() as libc::c_ulong);
        let fresh16 = (*self_0).stack.size;
        (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
        *(*self_0).stack.contents.offset(fresh16 as isize) =
            {
                let mut init =
                    StackEntry{tree:
                                   *(*last_entry.tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0
                                                                                                             as
                                                                                                             libc::c_int
                                                                                                             as
                                                                                                             isize),
                               child_index: 0 as libc::c_int as uint32_t,
                               byte_offset: last_entry.byte_offset,};
                init
            };
        return 1 as libc::c_int != 0
    } else { return 0 as libc::c_int != 0 };
}
#[inline]
unsafe extern "C" fn reusable_node_tree(mut self_0: *mut ReusableNode)
 -> Subtree {
    return if (*self_0).stack.size > 0 as libc::c_int as libc::c_uint {
               (*(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(1
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_uint)
                                                     as isize)).tree
           } else { Subtree{ptr: 0 as *const SubtreeHeapData,} };
}
unsafe extern "C" fn ts_parser__breakdown_lookahead(mut self_0: *mut TSParser,
                                                    mut lookahead:
                                                        *mut Subtree,
                                                    mut state: TSStateId,
                                                    mut reusable_node:
                                                        *mut ReusableNode) {
    let mut did_descend: bool = 0 as libc::c_int != 0;
    let mut tree: Subtree = reusable_node_tree(reusable_node);
    while ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint &&
              ts_subtree_parse_state(tree) as libc::c_int !=
                  state as libc::c_int {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"state_mismatch sym:%s\x00" as *const u8 as
                         *const libc::c_char,
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(tree)));
            ts_parser__log(self_0);
        }
        reusable_node_descend(reusable_node);
        tree = reusable_node_tree(reusable_node);
        did_descend = 1 as libc::c_int != 0
    }
    if did_descend {
        ts_subtree_release(&mut (*self_0).tree_pool, *lookahead);
        *lookahead = tree;
        ts_subtree_retain(*lookahead);
    };
}
#[inline]
unsafe extern "C" fn reusable_node_advance(mut self_0: *mut ReusableNode) {
    if (*self_0).stack.size.wrapping_sub(1 as libc::c_int as libc::c_uint) <
           (*self_0).stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->stack)->size - 1 < (&self->stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./reusable_node.h\x00" as *const u8 as
                          *const libc::c_char,
                      49 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 43],
                                                &[libc::c_char; 43]>(b"void reusable_node_advance(ReusableNode *)\x00")).as_ptr());
    }
    let mut last_entry: StackEntry =
        *(&mut *(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_uint)
                                                    as isize) as
              *mut StackEntry);
    let mut byte_offset: uint32_t =
        last_entry.byte_offset.wrapping_add(ts_subtree_total_bytes(last_entry.tree));
    if ts_subtree_has_external_tokens(last_entry.tree) {
        (*self_0).last_external_token =
            ts_subtree_last_external_token(last_entry.tree)
    }
    let mut tree: Subtree =
        Subtree{data:
                    SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                          [0; 1],
                                      symbol: 0,
                                      padding_bytes: 0,
                                      size_bytes: 0,
                                      padding_columns: 0,
                                      padding_rows_lookahead_bytes: [0; 1],
                                      parse_state: 0,},};
    let mut next_index: uint32_t = 0;
    loop  {
        (*self_0).stack.size = (*self_0).stack.size.wrapping_sub(1);
        let mut popped_entry: StackEntry =
            *(*self_0).stack.contents.offset((*self_0).stack.size as isize);
        next_index =
            popped_entry.child_index.wrapping_add(1 as libc::c_int as
                                                      libc::c_uint);
        if (*self_0).stack.size == 0 as libc::c_int as libc::c_uint { return }
        if (*self_0).stack.size.wrapping_sub(1 as libc::c_int as libc::c_uint)
               < (*self_0).stack.size {
        } else {
            __assert_fail(b"(uint32_t)(&self->stack)->size - 1 < (&self->stack)->size\x00"
                              as *const u8 as *const libc::c_char,
                          b"lib/src/./reusable_node.h\x00" as *const u8 as
                              *const libc::c_char,
                          61 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 43],
                                                    &[libc::c_char; 43]>(b"void reusable_node_advance(ReusableNode *)\x00")).as_ptr());
        }
        tree =
            (*(&mut *(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint)
                                                         as isize) as
                   *mut StackEntry)).tree;
        if !(ts_subtree_child_count(tree) <= next_index) { break ; }
    }
    array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackEntry>() as libc::c_ulong);
    let fresh17 = (*self_0).stack.size;
    (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
    *(*self_0).stack.contents.offset(fresh17 as isize) =
        {
            let mut init =
                StackEntry{tree:
                               *(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(next_index
                                                                                              as
                                                                                              isize),
                           child_index: next_index,
                           byte_offset: byte_offset,};
            init
        };
}
unsafe extern "C" fn ts_parser__restore_external_scanner(mut self_0:
                                                             *mut TSParser,
                                                         mut external_token:
                                                             Subtree) {
    if !external_token.ptr.is_null() {
        (*(*self_0).language).external_scanner.deserialize.expect("non-null function pointer")((*self_0).external_scanner_payload,
                                                                                               ts_external_scanner_state_data(&(*external_token.ptr).c2rust_unnamed.external_scanner_state),
                                                                                               (*external_token.ptr).c2rust_unnamed.external_scanner_state.length);
    } else {
        (*(*self_0).language).external_scanner.deserialize.expect("non-null function pointer")((*self_0).external_scanner_payload,
                                                                                               0
                                                                                                   as
                                                                                                   *const libc::c_char,
                                                                                               0
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint);
    };
}
unsafe extern "C" fn ts_parser__lex(mut self_0: *mut TSParser,
                                    mut version: StackVersion,
                                    mut parse_state: TSStateId) -> Subtree {
    let mut start_position: Length =
        ts_stack_position((*self_0).stack, version);
    let mut external_token: Subtree =
        ts_stack_last_external_token((*self_0).stack, version);
    let mut lex_mode: TSLexMode =
        *(*(*self_0).language).lex_modes.offset(parse_state as isize);
    if lex_mode.lex_state as libc::c_int ==
           -(1 as libc::c_int) as uint16_t as libc::c_int {
        return Subtree{ptr: 0 as *const SubtreeHeapData,}
    }
    let mut valid_external_tokens: *const bool =
        ts_language_enabled_external_tokens((*self_0).language,
                                            lex_mode.external_lex_state as
                                                libc::c_uint);
    let mut found_external_token: bool = 0 as libc::c_int != 0;
    let mut error_mode: bool = parse_state as libc::c_int == 0 as libc::c_int;
    let mut skipped_error: bool = 0 as libc::c_int != 0;
    let mut first_error_character: int32_t = 0 as libc::c_int;
    let mut error_start_position: Length = length_zero();
    let mut error_end_position: Length = length_zero();
    let mut lookahead_end_byte: uint32_t = 0 as libc::c_int as uint32_t;
    ts_lexer_reset(&mut (*self_0).lexer, start_position);
    loop  {
        let mut current_position: Length = (*self_0).lexer.current_position;
        if !valid_external_tokens.is_null() {
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"lex_external state:%d, row:%u, column:%u\x00" as
                             *const u8 as *const libc::c_char,
                         lex_mode.external_lex_state as libc::c_int,
                         current_position.extent.row.wrapping_add(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint),
                         current_position.extent.column);
                ts_parser__log(self_0);
            }
            ts_lexer_start(&mut (*self_0).lexer);
            ts_parser__restore_external_scanner(self_0, external_token);
            let mut found_token: bool =
                (*(*self_0).language).external_scanner.scan.expect("non-null function pointer")((*self_0).external_scanner_payload,
                                                                                                &mut (*self_0).lexer.data,
                                                                                                valid_external_tokens);
            ts_lexer_finish(&mut (*self_0).lexer, &mut lookahead_end_byte);
            if found_token as libc::c_int != 0 &&
                   ((*self_0).lexer.token_end_position.bytes >
                        current_position.bytes ||
                        !error_mode &&
                            ts_stack_has_advanced_since_error((*self_0).stack,
                                                              version) as
                                libc::c_int != 0) {
                found_external_token = 1 as libc::c_int != 0;
                break ;
            } else { ts_lexer_reset(&mut (*self_0).lexer, current_position); }
        }
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"lex_internal state:%d, row:%u, column:%u\x00" as
                         *const u8 as *const libc::c_char,
                     lex_mode.lex_state as libc::c_int,
                     current_position.extent.row.wrapping_add(1 as libc::c_int
                                                                  as
                                                                  libc::c_uint),
                     current_position.extent.column);
            ts_parser__log(self_0);
        }
        ts_lexer_start(&mut (*self_0).lexer);
        let mut found_token_0: bool =
            (*(*self_0).language).lex_fn.expect("non-null function pointer")(&mut (*self_0).lexer.data,
                                                                             lex_mode.lex_state);
        ts_lexer_finish(&mut (*self_0).lexer, &mut lookahead_end_byte);
        if found_token_0 { break ; }
        if !error_mode {
            error_mode = 1 as libc::c_int != 0;
            lex_mode =
                *(*(*self_0).language).lex_modes.offset(0 as libc::c_int as
                                                            isize);
            valid_external_tokens =
                ts_language_enabled_external_tokens((*self_0).language,
                                                    lex_mode.external_lex_state
                                                        as libc::c_uint);
            ts_lexer_reset(&mut (*self_0).lexer, start_position);
        } else {
            if !skipped_error {
                if (*self_0).lexer.logger.log.is_some() ||
                       !(*self_0).dot_graph_file.is_null() {
                    snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                             1024 as libc::c_int as libc::c_ulong,
                             b"skip_unrecognized_character\x00" as *const u8
                                 as *const libc::c_char);
                    ts_parser__log(self_0);
                }
                skipped_error = 1 as libc::c_int != 0;
                error_start_position = (*self_0).lexer.token_start_position;
                error_end_position = (*self_0).lexer.token_start_position;
                first_error_character = (*self_0).lexer.data.lookahead
            }
            if (*self_0).lexer.current_position.bytes ==
                   error_end_position.bytes {
                if (*self_0).lexer.data.eof.expect("non-null function pointer")(&mut (*self_0).lexer.data)
                   {
                    (*self_0).lexer.data.result_symbol =
                        -(1 as libc::c_int) as TSSymbol;
                    break ;
                } else {
                    (*self_0).lexer.data.advance.expect("non-null function pointer")(&mut (*self_0).lexer.data,
                                                                                     0
                                                                                         as
                                                                                         libc::c_int
                                                                                         !=
                                                                                         0);
                }
            }
            error_end_position = (*self_0).lexer.current_position
        }
    }
    let mut result: Subtree =
        Subtree{data:
                    SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                          [0; 1],
                                      symbol: 0,
                                      padding_bytes: 0,
                                      size_bytes: 0,
                                      padding_columns: 0,
                                      padding_rows_lookahead_bytes: [0; 1],
                                      parse_state: 0,},};
    if skipped_error {
        let mut padding: Length =
            length_sub(error_start_position, start_position);
        let mut size: Length =
            length_sub(error_end_position, error_start_position);
        let mut lookahead_bytes: uint32_t =
            lookahead_end_byte.wrapping_sub(error_end_position.bytes);
        result =
            ts_subtree_new_error(&mut (*self_0).tree_pool,
                                 first_error_character, padding, size,
                                 lookahead_bytes, parse_state,
                                 (*self_0).language);
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"lexed_lookahead sym:%s, size:%u, character:\'%c\'\x00"
                         as *const u8 as *const libc::c_char,
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(result)),
                     ts_subtree_total_size(result).bytes,
                     first_error_character);
            ts_parser__log(self_0);
        }
    } else {
        if (*self_0).lexer.token_end_position.bytes <
               (*self_0).lexer.token_start_position.bytes {
            (*self_0).lexer.token_start_position =
                (*self_0).lexer.token_end_position
        }
        let mut is_keyword: bool = 0 as libc::c_int != 0;
        let mut symbol: TSSymbol = (*self_0).lexer.data.result_symbol;
        let mut padding_0: Length =
            length_sub((*self_0).lexer.token_start_position, start_position);
        let mut size_0: Length =
            length_sub((*self_0).lexer.token_end_position,
                       (*self_0).lexer.token_start_position);
        let mut lookahead_bytes_0: uint32_t =
            lookahead_end_byte.wrapping_sub((*self_0).lexer.token_end_position.bytes);
        if found_external_token {
            symbol =
                *(*(*self_0).language).external_scanner.symbol_map.offset(symbol
                                                                              as
                                                                              isize)
        } else if symbol as libc::c_int ==
                      (*(*self_0).language).keyword_capture_token as
                          libc::c_int &&
                      symbol as libc::c_int != 0 as libc::c_int {
            let mut end_byte: uint32_t =
                (*self_0).lexer.token_end_position.bytes;
            ts_lexer_reset(&mut (*self_0).lexer,
                           (*self_0).lexer.token_start_position);
            ts_lexer_start(&mut (*self_0).lexer);
            if (*(*self_0).language).keyword_lex_fn.expect("non-null function pointer")(&mut (*self_0).lexer.data,
                                                                                        0
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            TSStateId)
                   as libc::c_int != 0 &&
                   (*self_0).lexer.token_end_position.bytes == end_byte &&
                   ts_language_has_actions((*self_0).language, parse_state,
                                           (*self_0).lexer.data.result_symbol)
                       as libc::c_int != 0 {
                is_keyword = 1 as libc::c_int != 0;
                symbol = (*self_0).lexer.data.result_symbol
            }
        }
        result =
            ts_subtree_new_leaf(&mut (*self_0).tree_pool, symbol, padding_0,
                                size_0, lookahead_bytes_0, parse_state,
                                found_external_token, is_keyword,
                                (*self_0).language);
        if found_external_token {
            let mut length: libc::c_uint =
                (*(*self_0).language).external_scanner.serialize.expect("non-null function pointer")((*self_0).external_scanner_payload,
                                                                                                     (*self_0).lexer.debug_buffer.as_mut_ptr());
            ts_external_scanner_state_init(&mut (*(result.ptr as
                                                       *mut SubtreeHeapData)).c2rust_unnamed.external_scanner_state,
                                           (*self_0).lexer.debug_buffer.as_mut_ptr(),
                                           length);
        }
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"lexed_lookahead sym:%s, size:%u\x00" as *const u8 as
                         *const libc::c_char,
                     ts_language_symbol_name((*self_0).language,
                                             ts_subtree_symbol(result)),
                     ts_subtree_total_size(result).bytes);
            ts_parser__log(self_0);
        }
    }
    return result;
}
unsafe extern "C" fn ts_parser__recover_to_state(mut self_0: *mut TSParser,
                                                 mut version: StackVersion,
                                                 mut depth: libc::c_uint,
                                                 mut goal_state: TSStateId)
 -> bool {
    let mut pop: StackSliceArray =
        ts_stack_pop_count((*self_0).stack, version, depth);
    let mut previous_version: StackVersion =
        -(1 as libc::c_int) as StackVersion;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < pop.size {
        let mut slice: StackSlice = *pop.contents.offset(i as isize);
        if slice.version == previous_version {
            ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                    &mut slice.subtrees);
            let fresh18 = i;
            i = i.wrapping_sub(1);
            array__erase(&mut pop as *mut StackSliceArray as *mut VoidArray,
                         ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
                         fresh18);
        } else if ts_stack_state((*self_0).stack, slice.version) as
                      libc::c_int != goal_state as libc::c_int {
            ts_stack_halt((*self_0).stack, slice.version);
            ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                    &mut slice.subtrees);
            let fresh19 = i;
            i = i.wrapping_sub(1);
            array__erase(&mut pop as *mut StackSliceArray as *mut VoidArray,
                         ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
                         fresh19);
        } else {
            let mut error_trees: SubtreeArray =
                ts_stack_pop_error((*self_0).stack, slice.version);
            if error_trees.size > 0 as libc::c_int as libc::c_uint {
                if error_trees.size == 1 as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"error_trees.size == 1\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lib/src/./parser.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1110 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 85],
                                                            &[libc::c_char; 85]>(b"_Bool ts_parser__recover_to_state(TSParser *, StackVersion, unsigned int, TSStateId)\x00")).as_ptr());
                }
                let mut error_tree: Subtree =
                    *error_trees.contents.offset(0 as libc::c_int as isize);
                let mut error_child_count: uint32_t =
                    ts_subtree_child_count(error_tree);
                if error_child_count > 0 as libc::c_int as libc::c_uint {
                    array__splice(&mut slice.subtrees as *mut SubtreeArray as
                                      *mut VoidArray,
                                  ::std::mem::size_of::<Subtree>() as
                                      libc::c_ulong,
                                  0 as libc::c_int as uint32_t,
                                  0 as libc::c_int as uint32_t,
                                  error_child_count,
                                  (*error_tree.ptr).c2rust_unnamed.c2rust_unnamed.children
                                      as *const libc::c_void);
                    let mut j: libc::c_uint =
                        0 as libc::c_int as libc::c_uint;
                    while j < error_child_count {
                        ts_subtree_retain(*slice.subtrees.contents.offset(j as
                                                                              isize));
                        j = j.wrapping_add(1)
                    }
                }
                ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                        &mut error_trees);
            }
            let mut trailing_extras: SubtreeArray =
                ts_subtree_array_remove_trailing_extras(&mut slice.subtrees);
            if slice.subtrees.size > 0 as libc::c_int as libc::c_uint {
                let mut error: Subtree =
                    ts_subtree_new_error_node(&mut (*self_0).tree_pool,
                                              &mut slice.subtrees,
                                              1 as libc::c_int != 0,
                                              (*self_0).language);
                ts_stack_push((*self_0).stack, slice.version, error,
                              0 as libc::c_int != 0, goal_state);
            } else {
                array__delete(&mut slice.subtrees as *mut SubtreeArray as
                                  *mut VoidArray);
            }
            let mut j_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while j_0 < trailing_extras.size {
                let mut tree: Subtree =
                    *trailing_extras.contents.offset(j_0 as isize);
                ts_stack_push((*self_0).stack, slice.version, tree,
                              0 as libc::c_int != 0, goal_state);
                j_0 = j_0.wrapping_add(1)
            }
            previous_version = slice.version;
            array__delete(&mut trailing_extras as *mut SubtreeArray as
                              *mut VoidArray);
        }
        i = i.wrapping_add(1)
    }
    return previous_version != -(1 as libc::c_int) as StackVersion;
}
unsafe extern "C" fn ts_parser__version_status(mut self_0: *mut TSParser,
                                               mut version: StackVersion)
 -> ErrorStatus {
    let mut cost: libc::c_uint =
        ts_stack_error_cost((*self_0).stack, version);
    let mut is_paused: bool = ts_stack_is_paused((*self_0).stack, version);
    if is_paused {
        cost = cost.wrapping_add(100 as libc::c_int as libc::c_uint)
    }
    return {
               let mut init =
                   ErrorStatus{cost: cost,
                               node_count:
                                   ts_stack_node_count_since_error((*self_0).stack,
                                                                   version),
                               dynamic_precedence:
                                   ts_stack_dynamic_precedence((*self_0).stack,
                                                               version),
                               is_in_error:
                                   is_paused as libc::c_int != 0 ||
                                       ts_stack_state((*self_0).stack,
                                                      version) as libc::c_int
                                           == 0 as libc::c_int,};
               init
           };
}
unsafe extern "C" fn ts_parser__accept(mut self_0: *mut TSParser,
                                       mut version: StackVersion,
                                       mut lookahead: Subtree) {
    if ts_subtree_is_eof(lookahead) {
    } else {
        __assert_fail(b"ts_subtree_is_eof(lookahead)\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./parser.c\x00" as *const u8 as
                          *const libc::c_char,
                      865 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"void ts_parser__accept(TSParser *, StackVersion, Subtree)\x00")).as_ptr());
    }
    ts_stack_push((*self_0).stack, version, lookahead, 0 as libc::c_int != 0,
                  1 as libc::c_int as TSStateId);
    let mut pop: StackSliceArray = ts_stack_pop_all((*self_0).stack, version);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < pop.size {
        let mut trees: SubtreeArray =
            (*pop.contents.offset(i as isize)).subtrees;
        let mut root: Subtree = Subtree{ptr: 0 as *const SubtreeHeapData,};
        let mut j: uint32_t =
            trees.size.wrapping_sub(1 as libc::c_int as libc::c_uint);
        while j.wrapping_add(1 as libc::c_int as libc::c_uint) >
                  0 as libc::c_int as libc::c_uint {
            let mut child: Subtree = *trees.contents.offset(j as isize);
            if !ts_subtree_extra(child) {
                if !child.data.is_inline() {
                } else {
                    __assert_fail(b"!child.data.is_inline\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lib/src/./parser.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  876 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 58],
                                                            &[libc::c_char; 58]>(b"void ts_parser__accept(TSParser *, StackVersion, Subtree)\x00")).as_ptr());
                }
                let mut child_count: uint32_t = ts_subtree_child_count(child);
                let mut k: uint32_t = 0 as libc::c_int as uint32_t;
                while k < child_count {
                    ts_subtree_retain(*(*child.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(k
                                                                                                      as
                                                                                                      isize));
                    k = k.wrapping_add(1)
                }
                array__splice(&mut trees as *mut SubtreeArray as
                                  *mut VoidArray,
                              ::std::mem::size_of::<Subtree>() as
                                  libc::c_ulong, j,
                              1 as libc::c_int as uint32_t, child_count,
                              (*child.ptr).c2rust_unnamed.c2rust_unnamed.children
                                  as *const libc::c_void);
                root =
                    ts_subtree_from_mut(ts_subtree_new_node(&mut (*self_0).tree_pool,
                                                            ts_subtree_symbol(child),
                                                            &mut trees,
                                                            (*child.ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                                                as
                                                                libc::c_uint,
                                                            (*self_0).language));
                ts_subtree_release(&mut (*self_0).tree_pool, child);
                break ;
            } else { j = j.wrapping_sub(1) }
        }
        if !root.ptr.is_null() {
        } else {
            __assert_fail(b"root.ptr\x00" as *const u8 as *const libc::c_char,
                          b"lib/src/./parser.c\x00" as *const u8 as
                              *const libc::c_char,
                          894 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 58],
                                                    &[libc::c_char; 58]>(b"void ts_parser__accept(TSParser *, StackVersion, Subtree)\x00")).as_ptr());
        }
        (*self_0).accept_count = (*self_0).accept_count.wrapping_add(1);
        if !(*self_0).finished_tree.ptr.is_null() {
            if ts_parser__select_tree(self_0, (*self_0).finished_tree, root) {
                ts_subtree_release(&mut (*self_0).tree_pool,
                                   (*self_0).finished_tree);
                (*self_0).finished_tree = root
            } else { ts_subtree_release(&mut (*self_0).tree_pool, root); }
        } else { (*self_0).finished_tree = root }
        i = i.wrapping_add(1)
    }
    ts_stack_remove_version((*self_0).stack,
                            (*pop.contents.offset(0 as libc::c_int as
                                                      isize)).version);
    ts_stack_halt((*self_0).stack, version);
}
unsafe extern "C" fn ts_parser__compare_versions(mut self_0: *mut TSParser,
                                                 mut a: ErrorStatus,
                                                 mut b: ErrorStatus)
 -> ErrorComparison {
    if !a.is_in_error && b.is_in_error as libc::c_int != 0 {
        if a.cost < b.cost {
            return ErrorComparisonTakeLeft
        } else { return ErrorComparisonPreferLeft }
    }
    if a.is_in_error as libc::c_int != 0 && !b.is_in_error {
        if b.cost < a.cost {
            return ErrorComparisonTakeRight
        } else { return ErrorComparisonPreferRight }
    }
    if a.cost < b.cost {
        if b.cost.wrapping_sub(a.cost).wrapping_mul((1 as libc::c_int as
                                                         libc::c_uint).wrapping_add(a.node_count))
               > MAX_COST_DIFFERENCE {
            return ErrorComparisonTakeLeft
        } else { return ErrorComparisonPreferLeft }
    }
    if b.cost < a.cost {
        if a.cost.wrapping_sub(b.cost).wrapping_mul((1 as libc::c_int as
                                                         libc::c_uint).wrapping_add(b.node_count))
               > MAX_COST_DIFFERENCE {
            return ErrorComparisonTakeRight
        } else { return ErrorComparisonPreferRight }
    }
    if a.dynamic_precedence > b.dynamic_precedence {
        return ErrorComparisonPreferLeft
    }
    if b.dynamic_precedence > a.dynamic_precedence {
        return ErrorComparisonPreferRight
    }
    return ErrorComparisonNone;
}
unsafe extern "C" fn ts_parser__better_version_exists(mut self_0:
                                                          *mut TSParser,
                                                      mut version:
                                                          StackVersion,
                                                      mut is_in_error: bool,
                                                      mut cost: libc::c_uint)
 -> bool {
    if !(*self_0).finished_tree.ptr.is_null() &&
           ts_subtree_error_cost((*self_0).finished_tree) <= cost {
        return 1 as libc::c_int != 0
    }
    let mut position: Length = ts_stack_position((*self_0).stack, version);
    let mut status: ErrorStatus =
        {
            let mut init =
                ErrorStatus{cost: cost,
                            node_count:
                                ts_stack_node_count_since_error((*self_0).stack,
                                                                version),
                            dynamic_precedence:
                                ts_stack_dynamic_precedence((*self_0).stack,
                                                            version),
                            is_in_error: is_in_error,};
            init
        };
    let mut i: StackVersion = 0 as libc::c_int as StackVersion;
    let mut n: StackVersion = ts_stack_version_count((*self_0).stack);
    while i < n {
        if !(i == version || !ts_stack_is_active((*self_0).stack, i) ||
                 ts_stack_position((*self_0).stack, i).bytes < position.bytes)
           {
            let mut status_i: ErrorStatus =
                ts_parser__version_status(self_0, i);
            match ts_parser__compare_versions(self_0, status, status_i) as
                      libc::c_uint {
                4 => { return 1 as libc::c_int != 0 }
                3 => {
                    if ts_stack_can_merge((*self_0).stack, i, version) {
                        return 1 as libc::c_int != 0
                    }
                }
                _ => { }
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn ts_parser__recover(mut self_0: *mut TSParser,
                                        mut version: StackVersion,
                                        mut lookahead: Subtree) {
    let mut did_recover: bool = 0 as libc::c_int != 0;
    let mut previous_version_count: libc::c_uint =
        ts_stack_version_count((*self_0).stack);
    let mut position: Length = ts_stack_position((*self_0).stack, version);
    let mut summary: *mut StackSummary =
        ts_stack_get_summary((*self_0).stack, version);
    let mut node_count_since_error: libc::c_uint =
        ts_stack_node_count_since_error((*self_0).stack, version);
    let mut current_error_cost: libc::c_uint =
        ts_stack_error_cost((*self_0).stack, version);
    if !summary.is_null() && !ts_subtree_is_error(lookahead) {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*summary).size {
            let mut entry: StackSummaryEntry =
                *(*summary).contents.offset(i as isize);
            if !(entry.state as libc::c_int == 0 as libc::c_int) {
                if !(entry.position.bytes == position.bytes) {
                    let mut depth: libc::c_uint = entry.depth;
                    if node_count_since_error >
                           0 as libc::c_int as libc::c_uint {
                        depth = depth.wrapping_add(1)
                    }
                    let mut would_merge: bool = 0 as libc::c_int != 0;
                    let mut j: libc::c_uint =
                        0 as libc::c_int as libc::c_uint;
                    while j < previous_version_count {
                        if ts_stack_state((*self_0).stack, j) as libc::c_int
                               == entry.state as libc::c_int &&
                               ts_stack_position((*self_0).stack, j).bytes ==
                                   position.bytes {
                            would_merge = 1 as libc::c_int != 0;
                            break ;
                        } else { j = j.wrapping_add(1) }
                    }
                    if !would_merge {
                        let mut new_cost: libc::c_uint =
                            current_error_cost.wrapping_add(entry.depth.wrapping_mul(100
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_uint)).wrapping_add(position.bytes.wrapping_sub(entry.position.bytes).wrapping_mul(1
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_int
                                                                                                                                                                                        as
                                                                                                                                                                                        libc::c_uint)).wrapping_add(position.extent.row.wrapping_sub(entry.position.extent.row).wrapping_mul(30
                                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                                 libc::c_int
                                                                                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                                                                                 libc::c_uint));
                        if ts_parser__better_version_exists(self_0, version,
                                                            0 as libc::c_int
                                                                != 0,
                                                            new_cost) {
                            break ;
                        }
                        if ts_language_has_actions((*self_0).language,
                                                   entry.state,
                                                   ts_subtree_symbol(lookahead))
                           {
                            if ts_parser__recover_to_state(self_0, version,
                                                           depth, entry.state)
                               {
                                did_recover = 1 as libc::c_int != 0;
                                if (*self_0).lexer.logger.log.is_some() ||
                                       !(*self_0).dot_graph_file.is_null() {
                                    snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                             1024 as libc::c_int as
                                                 libc::c_ulong,
                                             b"recover_to_previous state:%u, depth:%u\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             entry.state as libc::c_int,
                                             depth);
                                    ts_parser__log(self_0);
                                }
                                if !(*self_0).dot_graph_file.is_null() {
                                    ts_stack_print_dot_graph((*self_0).stack,
                                                             (*self_0).language,
                                                             (*self_0).dot_graph_file);
                                    fputs(b"\n\n\x00" as *const u8 as
                                              *const libc::c_char,
                                          (*self_0).dot_graph_file);
                                }
                                break ;
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1)
        }
    }
    let mut i_0: libc::c_uint = previous_version_count;
    while i_0 < ts_stack_version_count((*self_0).stack) {
        if !ts_stack_is_active((*self_0).stack, i_0) {
            let fresh20 = i_0;
            i_0 = i_0.wrapping_sub(1);
            ts_stack_remove_version((*self_0).stack, fresh20);
        }
        i_0 = i_0.wrapping_add(1)
    }
    if did_recover as libc::c_int != 0 &&
           ts_stack_version_count((*self_0).stack) > MAX_VERSION_COUNT {
        ts_stack_halt((*self_0).stack, version);
        ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
        return
    }
    if ts_subtree_is_eof(lookahead) {
        if (*self_0).lexer.logger.log.is_some() ||
               !(*self_0).dot_graph_file.is_null() {
            snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                     1024 as libc::c_int as libc::c_ulong,
                     b"recover_eof\x00" as *const u8 as *const libc::c_char);
            ts_parser__log(self_0);
        }
        let mut children: SubtreeArray =
            {
                let mut init =
                    SubtreeArray{contents: 0 as *mut Subtree,
                                 size: 0 as libc::c_int as uint32_t,
                                 capacity: 0 as libc::c_int as uint32_t,};
                init
            };
        let mut parent: Subtree =
            ts_subtree_new_error_node(&mut (*self_0).tree_pool, &mut children,
                                      0 as libc::c_int != 0,
                                      (*self_0).language);
        ts_stack_push((*self_0).stack, version, parent, 0 as libc::c_int != 0,
                      1 as libc::c_int as TSStateId);
        ts_parser__accept(self_0, version, lookahead);
        return
    }
    let mut new_cost_0: libc::c_uint =
        current_error_cost.wrapping_add(100 as libc::c_int as
                                            libc::c_uint).wrapping_add(ts_subtree_total_bytes(lookahead).wrapping_mul(1
                                                                                                                          as
                                                                                                                          libc::c_int
                                                                                                                          as
                                                                                                                          libc::c_uint)).wrapping_add(ts_subtree_total_size(lookahead).extent.row.wrapping_mul(30
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   libc::c_int
                                                                                                                                                                                                                   as
                                                                                                                                                                                                                   libc::c_uint));
    if ts_parser__better_version_exists(self_0, version,
                                        0 as libc::c_int != 0, new_cost_0) {
        ts_stack_halt((*self_0).stack, version);
        ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
        return
    }
    let mut n: libc::c_uint = 0;
    let mut actions: *const TSParseAction =
        ts_language_actions((*self_0).language, 1 as libc::c_int as TSStateId,
                            ts_subtree_symbol(lookahead), &mut n);
    if n > 0 as libc::c_int as libc::c_uint &&
           (*actions.offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize)).type_0() as libc::c_int ==
               TSParseActionTypeShift as libc::c_int &&
           (*actions.offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize)).params.shift.extra() as libc::c_int
               != 0 {
        let mut mutable_lookahead: MutableSubtree =
            ts_subtree_make_mut(&mut (*self_0).tree_pool, lookahead);
        ts_subtree_set_extra(&mut mutable_lookahead);
        lookahead = ts_subtree_from_mut(mutable_lookahead)
    }
    if (*self_0).lexer.logger.log.is_some() ||
           !(*self_0).dot_graph_file.is_null() {
        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                 1024 as libc::c_int as libc::c_ulong,
                 b"skip_token symbol:%s\x00" as *const u8 as
                     *const libc::c_char,
                 ts_language_symbol_name((*self_0).language,
                                         ts_subtree_symbol(lookahead)));
        ts_parser__log(self_0);
    }
    let mut children_0: SubtreeArray =
        {
            let mut init =
                SubtreeArray{contents: 0 as *mut Subtree,
                             size: 0 as libc::c_int as uint32_t,
                             capacity: 0 as libc::c_int as uint32_t,};
            init
        };
    array__reserve(&mut children_0 as *mut SubtreeArray as *mut VoidArray,
                   ::std::mem::size_of::<Subtree>() as libc::c_ulong,
                   1 as libc::c_int as uint32_t);
    array__grow(&mut children_0 as *mut SubtreeArray as *mut VoidArray,
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<Subtree>() as libc::c_ulong);
    let fresh21 = children_0.size;
    children_0.size = children_0.size.wrapping_add(1);
    *children_0.contents.offset(fresh21 as isize) = lookahead;
    let mut error_repeat: MutableSubtree =
        ts_subtree_new_node(&mut (*self_0).tree_pool,
                            (-(1 as libc::c_int) as TSSymbol as libc::c_int -
                                 1 as libc::c_int) as TSSymbol,
                            &mut children_0, 0 as libc::c_int as libc::c_uint,
                            (*self_0).language);
    if node_count_since_error > 0 as libc::c_int as libc::c_uint {
        let mut pop: StackSliceArray =
            ts_stack_pop_count((*self_0).stack, version,
                               1 as libc::c_int as uint32_t);
        if pop.size > 1 as libc::c_int as libc::c_uint {
            let mut i_1: libc::c_uint = 1 as libc::c_int as libc::c_uint;
            while i_1 < pop.size {
                ts_subtree_array_delete(&mut (*self_0).tree_pool,
                                        &mut (*pop.contents.offset(i_1 as
                                                                       isize)).subtrees);
                i_1 = i_1.wrapping_add(1)
            }
            while ts_stack_version_count((*self_0).stack) >
                      (*pop.contents.offset(0 as libc::c_int as
                                                isize)).version.wrapping_add(1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint)
                  {
                ts_stack_remove_version((*self_0).stack,
                                        (*pop.contents.offset(0 as libc::c_int
                                                                  as
                                                                  isize)).version.wrapping_add(1
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_uint));
            }
        }
        ts_stack_renumber_version((*self_0).stack,
                                  (*pop.contents.offset(0 as libc::c_int as
                                                            isize)).version,
                                  version);
        array__grow(&mut (*pop.contents.offset(0 as libc::c_int as
                                                   isize)).subtrees as
                        *mut SubtreeArray as *mut VoidArray,
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<Subtree>() as libc::c_ulong);
        let ref mut fresh22 =
            (*pop.contents.offset(0 as libc::c_int as isize)).subtrees.size;
        let fresh23 = *fresh22;
        *fresh22 = (*fresh22).wrapping_add(1);
        *(*pop.contents.offset(0 as libc::c_int as
                                   isize)).subtrees.contents.offset(fresh23 as
                                                                        isize)
            = ts_subtree_from_mut(error_repeat);
        error_repeat =
            ts_subtree_new_node(&mut (*self_0).tree_pool,
                                (-(1 as libc::c_int) as TSSymbol as
                                     libc::c_int - 1 as libc::c_int) as
                                    TSSymbol,
                                &mut (*pop.contents.offset(0 as libc::c_int as
                                                               isize)).subtrees,
                                0 as libc::c_int as libc::c_uint,
                                (*self_0).language)
    }
    ts_stack_push((*self_0).stack, version, ts_subtree_from_mut(error_repeat),
                  0 as libc::c_int != 0, 0 as libc::c_int as TSStateId);
    if ts_subtree_has_external_tokens(lookahead) {
        ts_stack_set_last_external_token((*self_0).stack, version,
                                         ts_subtree_last_external_token(lookahead));
    };
}
unsafe extern "C" fn ts_parser__breakdown_top_of_stack(mut self_0:
                                                           *mut TSParser,
                                                       mut version:
                                                           StackVersion)
 -> bool {
    let mut did_break_down: bool = 0 as libc::c_int != 0;
    let mut pending: bool = 0 as libc::c_int != 0;
    loop  {
        let mut pop: StackSliceArray =
            ts_stack_pop_pending((*self_0).stack, version);
        if pop.size == 0 { break ; }
        did_break_down = 1 as libc::c_int != 0;
        pending = 0 as libc::c_int != 0;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < pop.size {
            let mut slice: StackSlice = *pop.contents.offset(i as isize);
            let mut state: TSStateId =
                ts_stack_state((*self_0).stack, slice.version);
            if (0 as libc::c_int as uint32_t) < slice.subtrees.size {
            } else {
                __assert_fail(b"(uint32_t)0 < (&slice.subtrees)->size\x00" as
                                  *const u8 as *const libc::c_char,
                              b"lib/src/./parser.c\x00" as *const u8 as
                                  *const libc::c_char,
                              155 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 66],
                                                        &[libc::c_char; 66]>(b"_Bool ts_parser__breakdown_top_of_stack(TSParser *, StackVersion)\x00")).as_ptr());
            }
            let mut parent: Subtree =
                *(&mut *slice.subtrees.contents.offset(0 as libc::c_int as
                                                           isize) as
                      *mut Subtree);
            let mut j: uint32_t = 0 as libc::c_int as uint32_t;
            let mut n: uint32_t = ts_subtree_child_count(parent);
            while j < n {
                let mut child: Subtree =
                    *(*parent.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(j
                                                                                     as
                                                                                     isize);
                pending =
                    ts_subtree_child_count(child) >
                        0 as libc::c_int as libc::c_uint;
                if ts_subtree_is_error(child) {
                    state = 0 as libc::c_int as TSStateId
                } else if !ts_subtree_extra(child) {
                    state =
                        ts_language_next_state((*self_0).language, state,
                                               ts_subtree_symbol(child))
                }
                ts_subtree_retain(child);
                ts_stack_push((*self_0).stack, slice.version, child, pending,
                              state);
                j = j.wrapping_add(1)
            }
            let mut j_0: uint32_t = 1 as libc::c_int as uint32_t;
            while j_0 < slice.subtrees.size {
                let mut tree: Subtree =
                    *slice.subtrees.contents.offset(j_0 as isize);
                ts_stack_push((*self_0).stack, slice.version, tree,
                              0 as libc::c_int != 0, state);
                j_0 = j_0.wrapping_add(1)
            }
            ts_subtree_release(&mut (*self_0).tree_pool, parent);
            array__delete(&mut slice.subtrees as *mut SubtreeArray as
                              *mut VoidArray);
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"breakdown_top_of_stack tree:%s\x00" as *const u8 as
                             *const libc::c_char,
                         ts_language_symbol_name((*self_0).language,
                                                 ts_subtree_symbol(parent)));
                ts_parser__log(self_0);
            }
            if !(*self_0).dot_graph_file.is_null() {
                ts_stack_print_dot_graph((*self_0).stack, (*self_0).language,
                                         (*self_0).dot_graph_file);
                fputs(b"\n\n\x00" as *const u8 as *const libc::c_char,
                      (*self_0).dot_graph_file);
            }
            i = i.wrapping_add(1)
        }
        if !pending { break ; }
    }
    return did_break_down;
}
unsafe extern "C" fn ts_parser__advance(mut self_0: *mut TSParser,
                                        mut version: StackVersion,
                                        mut allow_node_reuse: bool) -> bool {
    let mut state: TSStateId = ts_stack_state((*self_0).stack, version);
    let mut position: uint32_t =
        ts_stack_position((*self_0).stack, version).bytes;
    let mut last_external_token: Subtree =
        ts_stack_last_external_token((*self_0).stack, version);
    let mut did_reuse: bool = 1 as libc::c_int != 0;
    let mut lookahead: Subtree = Subtree{ptr: 0 as *const SubtreeHeapData,};
    let mut table_entry: TableEntry =
        {
            let mut init =
                TableEntry{actions: 0 as *const TSParseAction,
                           action_count: 0 as libc::c_int as uint32_t,
                           is_reusable: false,};
            init
        };
    if allow_node_reuse {
        lookahead =
            ts_parser__reuse_node(self_0, version, &mut state, position,
                                  last_external_token, &mut table_entry)
    }
    if lookahead.ptr.is_null() {
        did_reuse = 0 as libc::c_int != 0;
        lookahead =
            ts_parser__get_cached_token(self_0, state, position as size_t,
                                        last_external_token, &mut table_entry)
    }
    if lookahead.ptr.is_null() {
        lookahead = ts_parser__lex(self_0, version, state);
        if !lookahead.ptr.is_null() {
            ts_parser__set_cached_token(self_0, position as size_t,
                                        last_external_token, lookahead);
            ts_language_table_entry((*self_0).language, state,
                                    ts_subtree_symbol(lookahead),
                                    &mut table_entry);
        } else {
            ts_language_table_entry((*self_0).language, state,
                                    0 as libc::c_int as TSSymbol,
                                    &mut table_entry);
        }
    }
    loop  {
        (*self_0).operation_count = (*self_0).operation_count.wrapping_add(1);
        if (*self_0).operation_count == OP_COUNT_PER_TIMEOUT_CHECK {
            (*self_0).operation_count = 0 as libc::c_int as libc::c_uint
        }
        if (*self_0).operation_count == 0 as libc::c_int as libc::c_uint &&
               (!(*self_0).cancellation_flag.is_null() &&
                    atomic_load((*self_0).cancellation_flag) != 0 ||
                    !clock_is_null((*self_0).end_clock) &&
                        clock_is_gt(clock_now(), (*self_0).end_clock) as
                            libc::c_int != 0) {
            ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
            return 0 as libc::c_int != 0
        }
        let mut last_reduction_version: StackVersion =
            -(1 as libc::c_int) as StackVersion;
        let mut current_block_67: u64;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < table_entry.action_count {
            let mut action: TSParseAction =
                *table_entry.actions.offset(i as isize);
            match action.type_0() as libc::c_int {
                0 => {
                    if !action.params.shift.repetition() {
                        let mut next_state: TSStateId = 0;
                        if action.params.shift.extra() {
                            if state as libc::c_int == 0 as libc::c_int {
                                current_block_67 = 15125582407903384992;
                            } else {
                                next_state = state;
                                if (*self_0).lexer.logger.log.is_some() ||
                                       !(*self_0).dot_graph_file.is_null() {
                                    snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                             1024 as libc::c_int as
                                                 libc::c_ulong,
                                             b"shift_extra\x00" as *const u8
                                                 as *const libc::c_char);
                                    ts_parser__log(self_0);
                                }
                                current_block_67 = 6717214610478484138;
                            }
                        } else {
                            next_state = action.params.shift.state;
                            if (*self_0).lexer.logger.log.is_some() ||
                                   !(*self_0).dot_graph_file.is_null() {
                                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                         1024 as libc::c_int as libc::c_ulong,
                                         b"shift state:%u\x00" as *const u8 as
                                             *const libc::c_char,
                                         next_state as libc::c_int);
                                ts_parser__log(self_0);
                            }
                            current_block_67 = 6717214610478484138;
                        }
                        match current_block_67 {
                            15125582407903384992 => { }
                            _ => {
                                if ts_subtree_child_count(lookahead) >
                                       0 as libc::c_int as libc::c_uint {
                                    ts_parser__breakdown_lookahead(self_0,
                                                                   &mut lookahead,
                                                                   state,
                                                                   &mut (*self_0).reusable_node);
                                    next_state =
                                        ts_language_next_state((*self_0).language,
                                                               state,
                                                               ts_subtree_symbol(lookahead))
                                }
                                ts_parser__shift(self_0, version, next_state,
                                                 lookahead,
                                                 action.params.shift.extra());
                                if did_reuse {
                                    reusable_node_advance(&mut (*self_0).reusable_node);
                                }
                                return 1 as libc::c_int != 0
                            }
                        }
                    }
                }
                1 => {
                    let mut is_fragile: bool =
                        table_entry.action_count >
                            1 as libc::c_int as libc::c_uint;
                    let mut is_extra: bool = lookahead.ptr.is_null();
                    if (*self_0).lexer.logger.log.is_some() ||
                           !(*self_0).dot_graph_file.is_null() {
                        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                 1024 as libc::c_int as libc::c_ulong,
                                 b"reduce sym:%s, child_count:%u\x00" as
                                     *const u8 as *const libc::c_char,
                                 ts_language_symbol_name((*self_0).language,
                                                         action.params.reduce.symbol),
                                 action.params.reduce.child_count as
                                     libc::c_int);
                        ts_parser__log(self_0);
                    }
                    let mut reduction_version: StackVersion =
                        ts_parser__reduce(self_0, version,
                                          action.params.reduce.symbol,
                                          action.params.reduce.child_count as
                                              uint32_t,
                                          action.params.reduce.dynamic_precedence
                                              as libc::c_int,
                                          action.params.reduce.production_id
                                              as uint16_t, is_fragile,
                                          is_extra);
                    if reduction_version !=
                           -(1 as libc::c_int) as StackVersion {
                        last_reduction_version = reduction_version
                    }
                }
                2 => {
                    if (*self_0).lexer.logger.log.is_some() ||
                           !(*self_0).dot_graph_file.is_null() {
                        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                 1024 as libc::c_int as libc::c_ulong,
                                 b"accept\x00" as *const u8 as
                                     *const libc::c_char);
                        ts_parser__log(self_0);
                    }
                    ts_parser__accept(self_0, version, lookahead);
                    return 1 as libc::c_int != 0
                }
                3 => {
                    if ts_subtree_child_count(lookahead) >
                           0 as libc::c_int as libc::c_uint {
                        ts_parser__breakdown_lookahead(self_0, &mut lookahead,
                                                       0 as libc::c_int as
                                                           TSStateId,
                                                       &mut (*self_0).reusable_node);
                    }
                    ts_parser__recover(self_0, version, lookahead);
                    if did_reuse {
                        reusable_node_advance(&mut (*self_0).reusable_node);
                    }
                    return 1 as libc::c_int != 0
                }
                _ => { }
            }
            i = i.wrapping_add(1)
        }
        if last_reduction_version != -(1 as libc::c_int) as StackVersion {
            ts_stack_renumber_version((*self_0).stack, last_reduction_version,
                                      version);
            if !(*self_0).dot_graph_file.is_null() {
                ts_stack_print_dot_graph((*self_0).stack, (*self_0).language,
                                         (*self_0).dot_graph_file);
                fputs(b"\n\n\x00" as *const u8 as *const libc::c_char,
                      (*self_0).dot_graph_file);
            }
            state = ts_stack_state((*self_0).stack, version);
            if lookahead.ptr.is_null() {
                lookahead = ts_parser__lex(self_0, version, state)
            }
            ts_language_table_entry((*self_0).language, state,
                                    ts_subtree_leaf_symbol(lookahead),
                                    &mut table_entry);
        } else {
            if ts_subtree_is_keyword(lookahead) as libc::c_int != 0 &&
                   ts_subtree_symbol(lookahead) as libc::c_int !=
                       (*(*self_0).language).keyword_capture_token as
                           libc::c_int {
                ts_language_table_entry((*self_0).language, state,
                                        (*(*self_0).language).keyword_capture_token,
                                        &mut table_entry);
                if table_entry.action_count > 0 as libc::c_int as libc::c_uint
                   {
                    if (*self_0).lexer.logger.log.is_some() ||
                           !(*self_0).dot_graph_file.is_null() {
                        snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                                 1024 as libc::c_int as libc::c_ulong,
                                 b"switch from_keyword:%s, to_word_token:%s\x00"
                                     as *const u8 as *const libc::c_char,
                                 ts_language_symbol_name((*self_0).language,
                                                         ts_subtree_symbol(lookahead)),
                                 ts_language_symbol_name((*self_0).language,
                                                         (*(*self_0).language).keyword_capture_token));
                        ts_parser__log(self_0);
                    }
                    let mut mutable_lookahead: MutableSubtree =
                        ts_subtree_make_mut(&mut (*self_0).tree_pool,
                                            lookahead);
                    ts_subtree_set_symbol(&mut mutable_lookahead,
                                          (*(*self_0).language).keyword_capture_token,
                                          (*self_0).language);
                    lookahead = ts_subtree_from_mut(mutable_lookahead);
                    continue ;
                }
            }
            if state as libc::c_int == 0 as libc::c_int {
                ts_parser__recover(self_0, version, lookahead);
                return 1 as libc::c_int != 0
            }
            if ts_parser__breakdown_top_of_stack(self_0, version) {
                continue ;
            }
            if (*self_0).lexer.logger.log.is_some() ||
                   !(*self_0).dot_graph_file.is_null() {
                snprintf((*self_0).lexer.debug_buffer.as_mut_ptr(),
                         1024 as libc::c_int as libc::c_ulong,
                         b"detect_error\x00" as *const u8 as
                             *const libc::c_char);
                ts_parser__log(self_0);
            }
            ts_stack_pause((*self_0).stack, version,
                           ts_subtree_leaf_symbol(lookahead));
            ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
            return 1 as libc::c_int != 0
        }
    };
}
static mut MAX_COST_DIFFERENCE: libc::c_uint =
    (16 as libc::c_int * 100 as libc::c_int) as libc::c_uint;
#[inline]
unsafe extern "C" fn clock_after(mut base: TSClock, mut duration: TSDuration)
 -> TSClock {
    let mut result: TSClock = base;
    result.tv_sec =
        (result.tv_sec as
             libc::c_ulong).wrapping_add(duration.wrapping_div(1000000 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong))
            as __time_t as __time_t;
    result.tv_nsec =
        (result.tv_nsec as
             libc::c_ulong).wrapping_add(duration.wrapping_rem(1000000 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(1000
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong))
            as __syscall_slong_t as __syscall_slong_t;
    return result;
}
#[inline]
unsafe extern "C" fn clock_is_null(mut self_0: TSClock) -> bool {
    return self_0.tv_sec == 0;
}
#[inline]
unsafe extern "C" fn clock_is_gt(mut self_0: TSClock, mut other: TSClock)
 -> bool {
    if self_0.tv_sec > other.tv_sec { return 1 as libc::c_int != 0 }
    if self_0.tv_sec < other.tv_sec { return 0 as libc::c_int != 0 }
    return self_0.tv_nsec > other.tv_nsec;
}
static mut PARENT_DONE: TSQueryError = 4294967295 as TSQueryError;
static mut PATTERN_DONE_MARKER: uint16_t = 65535 as libc::c_int as uint16_t;
static mut NONE: uint16_t = 65535 as libc::c_int as uint16_t;
static mut WILDCARD_SYMBOL: TSSymbol = 0 as libc::c_int as TSSymbol;
static mut NAMED_WILDCARD_SYMBOL: TSSymbol =
    (65535 as libc::c_int - 1 as libc::c_int) as TSSymbol;
/* *********
 * Stream
 **********/
// Advance to the next unicode code point in the stream.
unsafe extern "C" fn stream_advance(mut self_0: *mut Stream) -> bool {
    (*self_0).input =
        (*self_0).input.offset((*self_0).next_size as libc::c_int as isize);
    if (*self_0).input < (*self_0).end {
        let mut size: uint32_t =
            ts_decode_utf8((*self_0).input as *const uint8_t,
                           (*self_0).end.wrapping_offset_from((*self_0).input)
                               as libc::c_long as uint32_t,
                           &mut (*self_0).next);
        if size > 0 as libc::c_int as libc::c_uint {
            (*self_0).next_size = size as uint8_t;
            return 1 as libc::c_int != 0
        }
    } else {
        (*self_0).next_size = 0 as libc::c_int as uint8_t;
        (*self_0).next = '\u{0}' as i32
    }
    return 0 as libc::c_int != 0;
}
// Reset the stream to the given input position, represented as a pointer
// into the input string.
unsafe extern "C" fn stream_reset(mut self_0: *mut Stream,
                                  mut input: *const libc::c_char) {
    (*self_0).input = input;
    (*self_0).next_size = 0 as libc::c_int as uint8_t;
    stream_advance(self_0);
}
unsafe extern "C" fn stream_new(mut string: *const libc::c_char,
                                mut length: uint32_t) -> Stream {
    let mut self_0: Stream =
        {
            let mut init =
                Stream{input: string,
                       end: string.offset(length as isize),
                       next: 0 as libc::c_int,
                       next_size: 0,};
            init
        };
    stream_advance(&mut self_0);
    return self_0;
}
unsafe extern "C" fn stream_skip_whitespace(mut stream: *mut Stream) {
    loop  {
        if iswspace((*stream).next as wint_t) != 0 {
            stream_advance(stream);
        } else {
            if !((*stream).next == ';' as i32) { break ; }
            // skip over comments
            stream_advance(stream);
            while (*stream).next != 0 && (*stream).next != '\n' as i32 {
                if !stream_advance(stream) { break ; }
            }
        }
    };
}
unsafe extern "C" fn stream_is_ident_start(mut stream: *mut Stream) -> bool {
    return iswalnum((*stream).next as wint_t) != 0 ||
               (*stream).next == '_' as i32 || (*stream).next == '-' as i32;
}
unsafe extern "C" fn stream_scan_identifier(mut stream: *mut Stream) {
    loop  {
        stream_advance(stream);
        if !(iswalnum((*stream).next as wint_t) != 0 ||
                 (*stream).next == '_' as i32 || (*stream).next == '-' as i32
                 || (*stream).next == '.' as i32 ||
                 (*stream).next == '?' as i32 || (*stream).next == '!' as i32)
           {
            break ;
        }
    };
}
/* *****************
 * CaptureListPool
 ******************/
unsafe extern "C" fn capture_list_pool_new() -> CaptureListPool {
    return {
               let mut init =
                   CaptureListPool{list:
                                       [CaptureList{contents:
                                                        0 as
                                                            *mut TSQueryCapture,
                                                    size: 0,
                                                    capacity: 0,}; 32],
                                   empty_list:
                                       {
                                           let mut init =
                                               CaptureList{contents:
                                                               0 as
                                                                   *mut TSQueryCapture,
                                                           size:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   uint32_t,
                                                           capacity:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   uint32_t,};
                                           init
                                       },
                                   usage_map: 4294967295 as libc::c_uint,};
               init
           };
}
unsafe extern "C" fn capture_list_pool_reset(mut self_0:
                                                 *mut CaptureListPool) {
    (*self_0).usage_map = 4294967295 as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 32 as libc::c_int as libc::c_uint {
        (*self_0).list[i as usize].size = 0 as libc::c_int as uint32_t;
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn capture_list_pool_delete(mut self_0:
                                                  *mut CaptureListPool) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 32 as libc::c_int as libc::c_uint {
        array__delete(&mut *(*self_0).list.as_mut_ptr().offset(i as isize) as
                          *mut CaptureList as *mut VoidArray);
        i = i.wrapping_add(1)
    };
}
unsafe extern "C" fn capture_list_pool_get(mut self_0: *const CaptureListPool,
                                           mut id: uint16_t)
 -> *const CaptureList {
    if id as libc::c_int >= 32 as libc::c_int { return &(*self_0).empty_list }
    return &*(*self_0).list.as_ptr().offset(id as isize) as
               *const CaptureList;
}
unsafe extern "C" fn capture_list_pool_get_mut(mut self_0:
                                                   *mut CaptureListPool,
                                               mut id: uint16_t)
 -> *mut CaptureList {
    if (id as libc::c_int) < 32 as libc::c_int {
    } else {
        __assert_fail(b"id < MAX_CAPTURE_LIST_COUNT\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./query.c\x00" as *const u8 as
                          *const libc::c_char,
                      288 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"CaptureList *capture_list_pool_get_mut(CaptureListPool *, uint16_t)\x00")).as_ptr());
    }
    return &mut *(*self_0).list.as_mut_ptr().offset(id as isize) as
               *mut CaptureList;
}
unsafe extern "C" fn capture_list_pool_is_empty(mut self_0:
                                                    *const CaptureListPool)
 -> bool {
    return (*self_0).usage_map == 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn capture_list_pool_acquire(mut self_0:
                                                   *mut CaptureListPool)
 -> uint16_t {
    // In the usage_map bitmask, ones represent free lists, and zeros represent
  // lists that are in use. A free list id can quickly be found by counting
  // the leading zeros in the usage map. An id of zero corresponds to the
  // highest-order bit in the bitmask.
    let mut id: uint16_t =
        count_leading_zeros((*self_0).usage_map) as uint16_t;
    if id as libc::c_int >= 32 as libc::c_int { return NONE }
    (*self_0).usage_map &= !bitmask_for_index(id);
    (*self_0).list[id as usize].size = 0 as libc::c_int as uint32_t;
    return id;
}
unsafe extern "C" fn capture_list_pool_release(mut self_0:
                                                   *mut CaptureListPool,
                                               mut id: uint16_t) {
    if id as libc::c_int >= 32 as libc::c_int { return }
    (*self_0).list[id as usize].size = 0 as libc::c_int as uint32_t;
    (*self_0).usage_map |= bitmask_for_index(id);
}
/* *************
 * SymbolTable
 **************/
unsafe extern "C" fn symbol_table_new() -> SymbolTable {
    return {
               let mut init =
                   SymbolTable{characters:
                                   {
                                       let mut init =
                                           C2RustUnnamed_17{contents:
                                                                0 as
                                                                    *mut libc::c_char,
                                                            size:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,
                                                            capacity:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,};
                                       init
                                   },
                               slices:
                                   {
                                       let mut init =
                                           C2RustUnnamed_16{contents:
                                                                0 as
                                                                    *mut Slice,
                                                            size:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,
                                                            capacity:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,};
                                       init
                                   },};
               init
           };
}
unsafe extern "C" fn symbol_table_delete(mut self_0: *mut SymbolTable) {
    array__delete(&mut (*self_0).characters as *mut C2RustUnnamed_17 as
                      *mut VoidArray);
    array__delete(&mut (*self_0).slices as *mut C2RustUnnamed_16 as
                      *mut VoidArray);
}
unsafe extern "C" fn symbol_table_id_for_name(mut self_0: *const SymbolTable,
                                              mut name: *const libc::c_char,
                                              mut length: uint32_t)
 -> libc::c_int {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).slices.size {
        let mut slice: Slice = *(*self_0).slices.contents.offset(i as isize);
        if slice.length == length &&
               strncmp(&mut *(*self_0).characters.contents.offset(slice.offset
                                                                      as
                                                                      isize),
                       name, length as libc::c_ulong) == 0 {
            return i as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn symbol_table_name_for_id(mut self_0: *const SymbolTable,
                                              mut id: uint16_t,
                                              mut length: *mut uint32_t)
 -> *const libc::c_char {
    let mut slice: Slice = *(*self_0).slices.contents.offset(id as isize);
    *length = slice.length;
    return &mut *(*self_0).characters.contents.offset(slice.offset as isize)
               as *mut libc::c_char;
}
unsafe extern "C" fn symbol_table_insert_name(mut self_0: *mut SymbolTable,
                                              mut name: *const libc::c_char,
                                              mut length: uint32_t)
 -> uint16_t {
    let mut id: libc::c_int = symbol_table_id_for_name(self_0, name, length);
    if id >= 0 as libc::c_int { return id as uint16_t }
    let mut slice: Slice =
        {
            let mut init =
                Slice{offset: (*self_0).characters.size, length: length,};
            init
        };
    array__grow(&mut (*self_0).characters as *mut C2RustUnnamed_17 as
                    *mut VoidArray,
                length.wrapping_add(1 as libc::c_int as libc::c_uint) as
                    size_t,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong);
    memset((*self_0).characters.contents.offset((*self_0).characters.size as
                                                    isize) as
               *mut libc::c_void, 0 as libc::c_int,
           (length.wrapping_add(1 as libc::c_int as libc::c_uint) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    (*self_0).characters.size =
        ((*self_0).characters.size as
             libc::c_uint).wrapping_add(length.wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint))
            as uint32_t as uint32_t;
    memcpy(&mut *(*self_0).characters.contents.offset(slice.offset as isize)
               as *mut libc::c_char as *mut libc::c_void,
           name as *const libc::c_void, length as libc::c_ulong);
    *(*self_0).characters.contents.offset((*self_0).characters.size.wrapping_sub(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_uint)
                                              as isize) =
        0 as libc::c_int as libc::c_char;
    array__grow(&mut (*self_0).slices as *mut C2RustUnnamed_16 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<Slice>() as libc::c_ulong);
    let fresh24 = (*self_0).slices.size;
    (*self_0).slices.size = (*self_0).slices.size.wrapping_add(1);
    *(*self_0).slices.contents.offset(fresh24 as isize) = slice;
    return (*self_0).slices.size.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) as uint16_t;
}
unsafe extern "C" fn symbol_table_insert_name_with_escapes(mut self_0:
                                                               *mut SymbolTable,
                                                           mut escaped_name:
                                                               *const libc::c_char,
                                                           mut escaped_length:
                                                               uint32_t)
 -> uint16_t {
    let mut slice: Slice =
        {
            let mut init =
                Slice{offset: (*self_0).characters.size,
                      length: 0 as libc::c_int as uint32_t,};
            init
        };
    array__grow(&mut (*self_0).characters as *mut C2RustUnnamed_17 as
                    *mut VoidArray,
                escaped_length.wrapping_add(1 as libc::c_int as libc::c_uint)
                    as size_t,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong);
    memset((*self_0).characters.contents.offset((*self_0).characters.size as
                                                    isize) as
               *mut libc::c_void, 0 as libc::c_int,
           (escaped_length.wrapping_add(1 as libc::c_int as libc::c_uint) as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_char>()
                                                as libc::c_ulong));
    (*self_0).characters.size =
        ((*self_0).characters.size as
             libc::c_uint).wrapping_add(escaped_length.wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint))
            as uint32_t as uint32_t;
    // Copy the contents of the literal into the characters buffer, processing escape
  // sequences like \n and \". This needs to be done before checking if the literal
  // is already present, in order to do the string comparison.
    let mut is_escaped: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < escaped_length {
        let mut src: *const libc::c_char =
            &*escaped_name.offset(i as isize) as *const libc::c_char;
        let mut dest: *mut libc::c_char =
            &mut *(*self_0).characters.contents.offset(slice.offset.wrapping_add(slice.length)
                                                           as isize) as
                *mut libc::c_char;
        if is_escaped {
            match *src as libc::c_int {
                110 => { *dest = '\n' as i32 as libc::c_char }
                114 => { *dest = '\r' as i32 as libc::c_char }
                116 => { *dest = '\t' as i32 as libc::c_char }
                48 => { *dest = '\u{0}' as i32 as libc::c_char }
                _ => { *dest = *src }
            }
            is_escaped = 0 as libc::c_int != 0;
            slice.length = slice.length.wrapping_add(1)
        } else if *src as libc::c_int == '\\' as i32 {
            is_escaped = 1 as libc::c_int != 0
        } else { *dest = *src; slice.length = slice.length.wrapping_add(1) }
        i = i.wrapping_add(1)
    }
    // If the string is already present, remove the redundant content from the characters
  // buffer and return the existing id.
    let mut id: libc::c_int =
        symbol_table_id_for_name(self_0,
                                 &mut *(*self_0).characters.contents.offset(slice.offset
                                                                                as
                                                                                isize),
                                 slice.length);
    if id >= 0 as libc::c_int {
        (*self_0).characters.size =
            ((*self_0).characters.size as
                 libc::c_uint).wrapping_sub(escaped_length.wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint))
                as uint32_t as uint32_t;
        return id as uint16_t
    }
    *(*self_0).characters.contents.offset(slice.offset.wrapping_add(slice.length)
                                              as isize) =
        0 as libc::c_int as libc::c_char;
    array__grow(&mut (*self_0).slices as *mut C2RustUnnamed_16 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<Slice>() as libc::c_ulong);
    let fresh25 = (*self_0).slices.size;
    (*self_0).slices.size = (*self_0).slices.size.wrapping_add(1);
    *(*self_0).slices.contents.offset(fresh25 as isize) = slice;
    return (*self_0).slices.size.wrapping_sub(1 as libc::c_int as
                                                  libc::c_uint) as uint16_t;
}
/* ***********
 * QueryStep
 ************/
unsafe extern "C" fn query_step__new(mut symbol: TSSymbol,
                                     mut depth: uint16_t,
                                     mut is_immediate: bool) -> QueryStep {
    return {
               let mut init =
                   QueryStep{contains_captures_is_pattern_start_is_immediate_is_last_child_is_pass_through_is_dead_end_alternative_is_immediate:
                                 [0; 1],
                             c2rust_padding: [0; 1],
                             symbol: symbol,
                             field: 0 as libc::c_int as TSFieldId,
                             capture_ids: [NONE, NONE, NONE],
                             alternative_index: NONE,
                             depth: depth,};
               init.set_contains_captures(0 as libc::c_int != 0);
               init.set_is_pattern_start(0 as libc::c_int != 0);
               init.set_is_immediate(is_immediate);
               init.set_is_last_child(0 as libc::c_int != 0);
               init.set_is_pass_through(0 as libc::c_int != 0);
               init.set_is_dead_end(0 as libc::c_int != 0);
               init.set_alternative_is_immediate(0 as libc::c_int != 0);
               init
           };
}
unsafe extern "C" fn query_step__add_capture(mut self_0: *mut QueryStep,
                                             mut capture_id: uint16_t) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        if (*self_0).capture_ids[i as usize] as libc::c_int ==
               NONE as libc::c_int {
            (*self_0).capture_ids[i as usize] = capture_id;
            break ;
        } else { i = i.wrapping_add(1) }
    };
}
unsafe extern "C" fn query_step__remove_capture(mut self_0: *mut QueryStep,
                                                mut capture_id: uint16_t) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        if (*self_0).capture_ids[i as usize] as libc::c_int ==
               capture_id as libc::c_int {
            (*self_0).capture_ids[i as usize] = NONE;
            while i.wrapping_add(1 as libc::c_int as libc::c_uint) <
                      3 as libc::c_int as libc::c_uint {
                if (*self_0).capture_ids[i.wrapping_add(1 as libc::c_int as
                                                            libc::c_uint) as
                                             usize] as libc::c_int ==
                       NONE as libc::c_int {
                    break ;
                }
                (*self_0).capture_ids[i as usize] =
                    (*self_0).capture_ids[i.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) as
                                              usize];
                (*self_0).capture_ids[i.wrapping_add(1 as libc::c_int as
                                                         libc::c_uint) as
                                          usize] = NONE;
                i = i.wrapping_add(1)
            }
            break ;
        } else { i = i.wrapping_add(1) }
    };
}
/* ********
 * Query
 *********/
// The `pattern_map` contains a mapping from TSSymbol values to indices in the
// `steps` array. For a given syntax node, the `pattern_map` makes it possible
// to quickly find the starting steps of all of the patterns whose root matches
// that node. Each entry has two fields: a `pattern_index`, which identifies one
// of the patterns in the query, and a `step_index`, which indicates the start
// offset of that pattern's steps within the `steps` array.
//
// The entries are sorted by the patterns' root symbols, and lookups use a
// binary search. This ensures that the cost of this initial lookup step
// scales logarithmically with the number of patterns in the query.
//
// This returns `true` if the symbol is present and `false` otherwise.
// If the symbol is not present `*result` is set to the index where the
// symbol should be inserted.
#[inline]
unsafe extern "C" fn ts_query__pattern_map_search(mut self_0: *const TSQuery,
                                                  mut needle: TSSymbol,
                                                  mut result: *mut uint32_t)
 -> bool {
    let mut base_index: uint32_t =
        (*self_0).wildcard_root_pattern_count as uint32_t;
    let mut size: uint32_t =
        (*self_0).pattern_map.size.wrapping_sub(base_index);
    if size == 0 as libc::c_int as libc::c_uint {
        *result = base_index;
        return 0 as libc::c_int != 0
    }
    while size > 1 as libc::c_int as libc::c_uint {
        let mut half_size: uint32_t =
            size.wrapping_div(2 as libc::c_int as libc::c_uint);
        let mut mid_index: uint32_t = base_index.wrapping_add(half_size);
        let mut mid_symbol: TSSymbol =
            (*(*self_0).steps.contents.offset((*(*self_0).pattern_map.contents.offset(mid_index
                                                                                          as
                                                                                          isize)).step_index
                                                  as isize)).symbol;
        if needle as libc::c_int > mid_symbol as libc::c_int {
            base_index = mid_index
        }
        size =
            (size as libc::c_uint).wrapping_sub(half_size) as uint32_t as
                uint32_t
    }
    let mut symbol: TSSymbol =
        (*(*self_0).steps.contents.offset((*(*self_0).pattern_map.contents.offset(base_index
                                                                                      as
                                                                                      isize)).step_index
                                              as isize)).symbol;
    if needle as libc::c_int > symbol as libc::c_int {
        base_index = base_index.wrapping_add(1);
        if base_index < (*self_0).pattern_map.size {
            symbol =
                (*(*self_0).steps.contents.offset((*(*self_0).pattern_map.contents.offset(base_index
                                                                                              as
                                                                                              isize)).step_index
                                                      as isize)).symbol
        }
    }
    *result = base_index;
    return needle as libc::c_int == symbol as libc::c_int;
}
// Insert a new pattern's start index into the pattern map, maintaining
// the pattern map's ordering invariant.
#[inline]
unsafe extern "C" fn ts_query__pattern_map_insert(mut self_0: *mut TSQuery,
                                                  mut symbol: TSSymbol,
                                                  mut start_step_index:
                                                      uint32_t,
                                                  mut pattern_index:
                                                      uint32_t) {
    let mut index: uint32_t = 0;
    ts_query__pattern_map_search(self_0, symbol, &mut index);
    array__splice(&mut (*self_0).pattern_map as *mut C2RustUnnamed_14 as
                      *mut VoidArray,
                  ::std::mem::size_of::<PatternEntry>() as libc::c_ulong,
                  index, 0 as libc::c_int as uint32_t,
                  1 as libc::c_int as uint32_t,
                  &mut {
                           let mut init =
                               PatternEntry{step_index:
                                                start_step_index as uint16_t,
                                            pattern_index:
                                                pattern_index as uint16_t,};
                           init
                       } as *mut PatternEntry as *const libc::c_void);
}
unsafe extern "C" fn ts_query__finalize_steps(mut self_0: *mut TSQuery) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).steps.size {
        let mut step: *mut QueryStep =
            &mut *(*self_0).steps.contents.offset(i as isize) as
                *mut QueryStep;
        let mut depth: uint32_t = (*step).depth as uint32_t;
        if (*step).capture_ids[0 as libc::c_int as usize] as libc::c_int !=
               NONE as libc::c_int {
            (*step).set_contains_captures(1 as libc::c_int != 0)
        } else {
            (*step).set_contains_captures(0 as libc::c_int != 0);
            let mut j: libc::c_uint =
                i.wrapping_add(1 as libc::c_int as libc::c_uint);
            while j < (*self_0).steps.size {
                let mut s: *mut QueryStep =
                    &mut *(*self_0).steps.contents.offset(j as isize) as
                        *mut QueryStep;
                if (*s).depth as libc::c_int ==
                       PATTERN_DONE_MARKER as libc::c_int ||
                       (*s).depth as libc::c_uint <= depth {
                    break ;
                }
                if (*s).capture_ids[0 as libc::c_int as usize] as libc::c_int
                       != NONE as libc::c_int {
                    (*step).set_contains_captures(1 as libc::c_int != 0)
                }
                j = j.wrapping_add(1)
            }
        }
        i = i.wrapping_add(1)
    };
}
// Parse a single predicate associated with a pattern, adding it to the
// query's internal `predicate_steps` array. Predicates are arbitrary
// S-expressions associated with a pattern which are meant to be handled at
// a higher level of abstraction, such as the Rust/JavaScript bindings. They
// can contain '@'-prefixed capture names, double-quoted strings, and bare
// symbols, which also represent strings.
unsafe extern "C" fn ts_query__parse_predicate(mut self_0: *mut TSQuery,
                                               mut stream: *mut Stream)
 -> TSQueryError {
    if !stream_is_ident_start(stream) { return TSQueryErrorSyntax }
    let mut predicate_name: *const libc::c_char = (*stream).input;
    stream_scan_identifier(stream);
    let mut length: uint32_t =
        (*stream).input.wrapping_offset_from(predicate_name) as libc::c_long
            as uint32_t;
    let mut id: uint16_t =
        symbol_table_insert_name(&mut (*self_0).predicate_values,
                                 predicate_name, length);
    if (*self_0).predicates_by_pattern.size.wrapping_sub(1 as libc::c_int as
                                                             libc::c_uint) <
           (*self_0).predicates_by_pattern.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->predicates_by_pattern)->size - 1 < (&self->predicates_by_pattern)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./query.c\x00" as *const u8 as
                          *const libc::c_char,
                      591 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 60],
                                                &[libc::c_char; 60]>(b"TSQueryError ts_query__parse_predicate(TSQuery *, Stream *)\x00")).as_ptr());
    }
    let ref mut fresh26 =
        (*(&mut *(*self_0).predicates_by_pattern.contents.offset((*self_0).predicates_by_pattern.size.wrapping_sub(1
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       libc::c_uint)
                                                                     as isize)
               as *mut Slice)).length;
    *fresh26 = (*fresh26).wrapping_add(1);
    array__grow(&mut (*self_0).predicate_steps as *mut C2RustUnnamed_13 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<TSQueryPredicateStep>() as
                    libc::c_ulong);
    let fresh27 = (*self_0).predicate_steps.size;
    (*self_0).predicate_steps.size =
        (*self_0).predicate_steps.size.wrapping_add(1);
    *(*self_0).predicate_steps.contents.offset(fresh27 as isize) =
        {
            let mut init =
                TSQueryPredicateStep{type_0: TSQueryPredicateStepTypeString,
                                     value_id: id as uint32_t,};
            init
        };
    stream_skip_whitespace(stream);
    loop  {
        if (*stream).next == ')' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            if (*self_0).predicates_by_pattern.size.wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                   < (*self_0).predicates_by_pattern.size {
            } else {
                __assert_fail(b"(uint32_t)(&self->predicates_by_pattern)->size - 1 < (&self->predicates_by_pattern)->size\x00"
                                  as *const u8 as *const libc::c_char,
                              b"lib/src/./query.c\x00" as *const u8 as
                                  *const libc::c_char,
                              602 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 60],
                                                        &[libc::c_char; 60]>(b"TSQueryError ts_query__parse_predicate(TSQuery *, Stream *)\x00")).as_ptr());
            }
            let ref mut fresh28 =
                (*(&mut *(*self_0).predicates_by_pattern.contents.offset((*self_0).predicates_by_pattern.size.wrapping_sub(1
                                                                                                                               as
                                                                                                                               libc::c_int
                                                                                                                               as
                                                                                                                               libc::c_uint)
                                                                             as
                                                                             isize)
                       as *mut Slice)).length;
            *fresh28 = (*fresh28).wrapping_add(1);
            array__grow(&mut (*self_0).predicate_steps as
                            *mut C2RustUnnamed_13 as *mut VoidArray,
                        1 as libc::c_int as size_t,
                        ::std::mem::size_of::<TSQueryPredicateStep>() as
                            libc::c_ulong);
            let fresh29 = (*self_0).predicate_steps.size;
            (*self_0).predicate_steps.size =
                (*self_0).predicate_steps.size.wrapping_add(1);
            *(*self_0).predicate_steps.contents.offset(fresh29 as isize) =
                {
                    let mut init =
                        TSQueryPredicateStep{type_0:
                                                 TSQueryPredicateStepTypeDone,
                                             value_id:
                                                 0 as libc::c_int as
                                                     uint32_t,};
                    init
                };
            break ;
        } else {
            // Parse an '@'-prefixed capture name
            if (*stream).next == '@' as i32 {
                stream_advance(stream);
                // Parse the capture name
                if !stream_is_ident_start(stream) {
                    return TSQueryErrorSyntax
                }
                let mut capture_name: *const libc::c_char = (*stream).input;
                stream_scan_identifier(stream);
                let mut length_0: uint32_t =
                    (*stream).input.wrapping_offset_from(capture_name) as
                        libc::c_long as uint32_t;
                // Add the capture id to the first step of the pattern
                let mut capture_id: libc::c_int =
                    symbol_table_id_for_name(&mut (*self_0).captures,
                                             capture_name, length_0);
                if capture_id == -(1 as libc::c_int) {
                    stream_reset(stream, capture_name);
                    return TSQueryErrorCapture
                }
                if (*self_0).predicates_by_pattern.size.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                       < (*self_0).predicates_by_pattern.size {
                } else {
                    __assert_fail(b"(uint32_t)(&self->predicates_by_pattern)->size - 1 < (&self->predicates_by_pattern)->size\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lib/src/./query.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  631 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 60],
                                                            &[libc::c_char; 60]>(b"TSQueryError ts_query__parse_predicate(TSQuery *, Stream *)\x00")).as_ptr());
                }
                let ref mut fresh30 =
                    (*(&mut *(*self_0).predicates_by_pattern.contents.offset((*self_0).predicates_by_pattern.size.wrapping_sub(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
                                                                                 as
                                                                                 isize)
                           as *mut Slice)).length;
                *fresh30 = (*fresh30).wrapping_add(1);
                array__grow(&mut (*self_0).predicate_steps as
                                *mut C2RustUnnamed_13 as *mut VoidArray,
                            1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TSQueryPredicateStep>() as
                                libc::c_ulong);
                let fresh31 = (*self_0).predicate_steps.size;
                (*self_0).predicate_steps.size =
                    (*self_0).predicate_steps.size.wrapping_add(1);
                *(*self_0).predicate_steps.contents.offset(fresh31 as isize) =
                    {
                        let mut init =
                            TSQueryPredicateStep{type_0:
                                                     TSQueryPredicateStepTypeCapture,
                                                 value_id:
                                                     capture_id as uint32_t,};
                        init
                    }
            } else if (*stream).next == '\"' as i32 {
                stream_advance(stream);
                // Parse a string literal
                // Parse the string content
                let mut is_escaped: bool = 0 as libc::c_int != 0;
                let mut string_content: *const libc::c_char = (*stream).input;
                loop  {
                    if is_escaped {
                        is_escaped = 0 as libc::c_int != 0
                    } else if (*stream).next == '\\' as i32 {
                        is_escaped = 1 as libc::c_int != 0
                    } else {
                        if (*stream).next == '\"' as i32 { break ; }
                        if (*stream).next == '\n' as i32 {
                            stream_reset(stream,
                                         string_content.offset(-(1 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)));
                            return TSQueryErrorSyntax
                        }
                    }
                    if !stream_advance(stream) {
                        stream_reset(stream,
                                     string_content.offset(-(1 as libc::c_int
                                                                 as isize)));
                        return TSQueryErrorSyntax
                    }
                }
                let mut length_1: uint32_t =
                    (*stream).input.wrapping_offset_from(string_content) as
                        libc::c_long as uint32_t;
                // Add a step for the node
                let mut id_0: uint16_t =
                    symbol_table_insert_name_with_escapes(&mut (*self_0).predicate_values,
                                                          string_content,
                                                          length_1);
                if (*self_0).predicates_by_pattern.size.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                       < (*self_0).predicates_by_pattern.size {
                } else {
                    __assert_fail(b"(uint32_t)(&self->predicates_by_pattern)->size - 1 < (&self->predicates_by_pattern)->size\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lib/src/./query.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  671 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 60],
                                                            &[libc::c_char; 60]>(b"TSQueryError ts_query__parse_predicate(TSQuery *, Stream *)\x00")).as_ptr());
                }
                let ref mut fresh32 =
                    (*(&mut *(*self_0).predicates_by_pattern.contents.offset((*self_0).predicates_by_pattern.size.wrapping_sub(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
                                                                                 as
                                                                                 isize)
                           as *mut Slice)).length;
                *fresh32 = (*fresh32).wrapping_add(1);
                array__grow(&mut (*self_0).predicate_steps as
                                *mut C2RustUnnamed_13 as *mut VoidArray,
                            1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TSQueryPredicateStep>() as
                                libc::c_ulong);
                let fresh33 = (*self_0).predicate_steps.size;
                (*self_0).predicate_steps.size =
                    (*self_0).predicate_steps.size.wrapping_add(1);
                *(*self_0).predicate_steps.contents.offset(fresh33 as isize) =
                    {
                        let mut init =
                            TSQueryPredicateStep{type_0:
                                                     TSQueryPredicateStepTypeString,
                                                 value_id: id_0 as uint32_t,};
                        init
                    };
                if (*stream).next != '\"' as i32 { return TSQueryErrorSyntax }
                stream_advance(stream);
            } else if stream_is_ident_start(stream) {
                let mut symbol_start: *const libc::c_char = (*stream).input;
                stream_scan_identifier(stream);
                let mut length_2: uint32_t =
                    (*stream).input.wrapping_offset_from(symbol_start) as
                        libc::c_long as uint32_t;
                let mut id_1: uint16_t =
                    symbol_table_insert_name(&mut (*self_0).predicate_values,
                                             symbol_start, length_2);
                if (*self_0).predicates_by_pattern.size.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                       < (*self_0).predicates_by_pattern.size {
                } else {
                    __assert_fail(b"(uint32_t)(&self->predicates_by_pattern)->size - 1 < (&self->predicates_by_pattern)->size\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"lib/src/./query.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  691 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 60],
                                                            &[libc::c_char; 60]>(b"TSQueryError ts_query__parse_predicate(TSQuery *, Stream *)\x00")).as_ptr());
                }
                let ref mut fresh34 =
                    (*(&mut *(*self_0).predicates_by_pattern.contents.offset((*self_0).predicates_by_pattern.size.wrapping_sub(1
                                                                                                                                   as
                                                                                                                                   libc::c_int
                                                                                                                                   as
                                                                                                                                   libc::c_uint)
                                                                                 as
                                                                                 isize)
                           as *mut Slice)).length;
                *fresh34 = (*fresh34).wrapping_add(1);
                array__grow(&mut (*self_0).predicate_steps as
                                *mut C2RustUnnamed_13 as *mut VoidArray,
                            1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TSQueryPredicateStep>() as
                                libc::c_ulong);
                let fresh35 = (*self_0).predicate_steps.size;
                (*self_0).predicate_steps.size =
                    (*self_0).predicate_steps.size.wrapping_add(1);
                *(*self_0).predicate_steps.contents.offset(fresh35 as isize) =
                    {
                        let mut init =
                            TSQueryPredicateStep{type_0:
                                                     TSQueryPredicateStepTypeString,
                                                 value_id: id_1 as uint32_t,};
                        init
                    }
            } else { return TSQueryErrorSyntax }
            stream_skip_whitespace(stream);
        }
    }
    return TSQueryErrorNone;
}
// Parse a bare symbol
// Read one S-expression pattern from the stream, and incorporate it into
// the query's internal state machine representation. For nested patterns,
// this function calls itself recursively.
unsafe extern "C" fn ts_query__parse_pattern(mut self_0: *mut TSQuery,
                                             mut stream: *mut Stream,
                                             mut depth: uint32_t,
                                             mut capture_count: *mut uint32_t,
                                             mut is_immediate: bool)
 -> TSQueryError {
    let mut starting_step_index: uint32_t = (*self_0).steps.size;
    if (*stream).next == 0 as libc::c_int { return TSQueryErrorSyntax }
    // Finish the parent S-expression.
    if (*stream).next == ')' as i32 || (*stream).next == ']' as i32 {
        return PARENT_DONE
    } else {
        // An open bracket is the start of an alternation.
        if (*stream).next == '[' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            // Parse each branch, and add a placeholder step in between the branches.
            let mut branch_step_indices: C2RustUnnamed_23 =
                {
                    let mut init =
                        C2RustUnnamed_23{contents: 0 as *mut uint32_t,
                                         size: 0 as libc::c_int as uint32_t,
                                         capacity:
                                             0 as libc::c_int as uint32_t,};
                    init
                };
            loop  {
                let mut start_index: uint32_t = (*self_0).steps.size;
                let mut e: TSQueryError =
                    ts_query__parse_pattern(self_0, stream, depth,
                                            capture_count, is_immediate);
                if e as libc::c_uint == PARENT_DONE as libc::c_uint &&
                       (*stream).next == ']' as i32 &&
                       branch_step_indices.size >
                           0 as libc::c_int as libc::c_uint {
                    stream_advance(stream);
                    break ;
                } else {
                    if e as u64 != 0 {
                        array__delete(&mut branch_step_indices as
                                          *mut C2RustUnnamed_23 as
                                          *mut VoidArray);
                        return e
                    }
                    array__grow(&mut branch_step_indices as
                                    *mut C2RustUnnamed_23 as *mut VoidArray,
                                1 as libc::c_int as size_t,
                                ::std::mem::size_of::<uint32_t>() as
                                    libc::c_ulong);
                    let fresh36 = branch_step_indices.size;
                    branch_step_indices.size =
                        branch_step_indices.size.wrapping_add(1);
                    *branch_step_indices.contents.offset(fresh36 as isize) =
                        start_index;
                    array__grow(&mut (*self_0).steps as *mut C2RustUnnamed_15
                                    as *mut VoidArray,
                                1 as libc::c_int as size_t,
                                ::std::mem::size_of::<QueryStep>() as
                                    libc::c_ulong);
                    let fresh37 = (*self_0).steps.size;
                    (*self_0).steps.size =
                        (*self_0).steps.size.wrapping_add(1);
                    *(*self_0).steps.contents.offset(fresh37 as isize) =
                        query_step__new(0 as libc::c_int as TSSymbol,
                                        depth as uint16_t,
                                        0 as libc::c_int != 0)
                }
            }
            (*self_0).steps.size = (*self_0).steps.size.wrapping_sub(1);
            // For all of the branches except for the last one, add the subsequent branch as an
    // alternative, and link the end of the branch to the current end of the steps.
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while i <
                      branch_step_indices.size.wrapping_sub(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                  {
                let mut step_index: uint32_t =
                    *branch_step_indices.contents.offset(i as isize);
                let mut next_step_index: uint32_t =
                    *branch_step_indices.contents.offset(i.wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                             as isize);
                let mut start_step: *mut QueryStep =
                    &mut *(*self_0).steps.contents.offset(step_index as isize)
                        as *mut QueryStep;
                let mut end_step: *mut QueryStep =
                    &mut *(*self_0).steps.contents.offset(next_step_index.wrapping_sub(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_uint)
                                                              as isize) as
                        *mut QueryStep;
                (*start_step).alternative_index = next_step_index as uint16_t;
                (*end_step).alternative_index =
                    (*self_0).steps.size as uint16_t;
                (*end_step).set_is_dead_end(1 as libc::c_int != 0);
                i = i.wrapping_add(1)
            }
            array__delete(&mut branch_step_indices as *mut C2RustUnnamed_23 as
                              *mut VoidArray);
        } else if (*stream).next == '(' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            // An open parenthesis can be the start of three possible constructs:
  // * A grouped sequence
  // * A predicate
  // * A named node
            // If this parenthesis is followed by a node, then it represents a grouped sequence.
            if (*stream).next == '(' as i32 || (*stream).next == '\"' as i32
                   || (*stream).next == '[' as i32 {
                let mut child_is_immediate: bool = 0 as libc::c_int != 0;
                loop  {
                    if (*stream).next == '.' as i32 {
                        child_is_immediate = 1 as libc::c_int != 0;
                        stream_advance(stream);
                        stream_skip_whitespace(stream);
                    }
                    let mut e_0: TSQueryError =
                        ts_query__parse_pattern(self_0, stream, depth,
                                                capture_count,
                                                child_is_immediate);
                    if e_0 as libc::c_uint == PARENT_DONE as libc::c_uint &&
                           (*stream).next == ')' as i32 {
                        stream_advance(stream);
                        break ;
                    } else {
                        if e_0 as u64 != 0 { return e_0 }
                        child_is_immediate = 0 as libc::c_int != 0
                    }
                }
            } else if (*stream).next == '#' as i32 {
                stream_advance(stream);
                return ts_query__parse_predicate(self_0, stream)
            } else {
                // A pound character indicates the start of a predicate.
                // Otherwise, this parenthesis is the start of a named node.
                let mut symbol: TSSymbol = 0;
                // Parse the wildcard symbol
                if (*stream).next == '_' as i32 ||
                       (*stream).next == '*' as i32 {
                    symbol =
                        if depth > 0 as libc::c_int as libc::c_uint {
                            NAMED_WILDCARD_SYMBOL as libc::c_int
                        } else { WILDCARD_SYMBOL as libc::c_int } as TSSymbol;
                    stream_advance(stream);
                } else if stream_is_ident_start(stream) {
                    let mut node_name: *const libc::c_char = (*stream).input;
                    stream_scan_identifier(stream);
                    let mut length: uint32_t =
                        (*stream).input.wrapping_offset_from(node_name) as
                            libc::c_long as uint32_t;
                    // Parse a normal node name
                    // TODO - remove.
        // For temporary backward compatibility, handle predicates without the leading '#' sign.
                    if length > 0 as libc::c_int as libc::c_uint &&
                           (*node_name.offset(length.wrapping_sub(1 as
                                                                      libc::c_int
                                                                      as
                                                                      libc::c_uint)
                                                  as isize) as libc::c_int ==
                                '!' as i32 ||
                                *node_name.offset(length.wrapping_sub(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
                                                      as isize) as libc::c_int
                                    == '?' as i32) {
                        stream_reset(stream, node_name);
                        return ts_query__parse_predicate(self_0, stream)
                    }
                    symbol =
                        ts_language_symbol_for_name((*self_0).language,
                                                    node_name, length,
                                                    1 as libc::c_int != 0);
                    if symbol == 0 {
                        stream_reset(stream, node_name);
                        return TSQueryErrorNodeType
                    }
                } else { return TSQueryErrorSyntax }
                // Add a step for the node.
                array__grow(&mut (*self_0).steps as *mut C2RustUnnamed_15 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<QueryStep>() as
                                libc::c_ulong);
                let fresh38 = (*self_0).steps.size;
                (*self_0).steps.size = (*self_0).steps.size.wrapping_add(1);
                *(*self_0).steps.contents.offset(fresh38 as isize) =
                    query_step__new(symbol, depth as uint16_t, is_immediate);
                // Parse the child patterns
                stream_skip_whitespace(stream);
                let mut child_is_immediate_0: bool = 0 as libc::c_int != 0;
                let mut child_start_step_index: uint16_t =
                    (*self_0).steps.size as uint16_t;
                loop  {
                    if (*stream).next == '.' as i32 {
                        child_is_immediate_0 = 1 as libc::c_int != 0;
                        stream_advance(stream);
                        stream_skip_whitespace(stream);
                    }
                    let mut e_1: TSQueryError =
                        ts_query__parse_pattern(self_0, stream,
                                                depth.wrapping_add(1 as
                                                                       libc::c_int
                                                                       as
                                                                       libc::c_uint),
                                                capture_count,
                                                child_is_immediate_0);
                    if e_1 as libc::c_uint == PARENT_DONE as libc::c_uint &&
                           (*stream).next == ')' as i32 {
                        if child_is_immediate_0 {
                            let ref mut fresh39 =
                                *(*self_0).steps.contents.offset(child_start_step_index
                                                                     as
                                                                     isize);
                            (*fresh39).set_is_last_child(1 as libc::c_int !=
                                                             0)
                        }
                        stream_advance(stream);
                        break ;
                    } else {
                        if e_1 as u64 != 0 { return e_1 }
                        child_is_immediate_0 = 0 as libc::c_int != 0
                    }
                }
            }
        } else if (*stream).next == '_' as i32 || (*stream).next == '*' as i32
         {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            // Parse a wildcard pattern
            // Add a step that matches any kind of node
            array__grow(&mut (*self_0).steps as *mut C2RustUnnamed_15 as
                            *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<QueryStep>() as libc::c_ulong);
            let fresh40 = (*self_0).steps.size;
            (*self_0).steps.size = (*self_0).steps.size.wrapping_add(1);
            *(*self_0).steps.contents.offset(fresh40 as isize) =
                query_step__new(WILDCARD_SYMBOL, depth as uint16_t,
                                is_immediate)
        } else if (*stream).next == '\"' as i32 {
            stream_advance(stream);
            // Parse a double-quoted anonymous leaf node expression
            // Parse the string content
            let mut string_content: *const libc::c_char = (*stream).input;
            while (*stream).next != '\"' as i32 {
                if !stream_advance(stream) {
                    stream_reset(stream,
                                 string_content.offset(-(1 as libc::c_int as
                                                             isize)));
                    return TSQueryErrorSyntax
                }
            }
            let mut length_0: uint32_t =
                (*stream).input.wrapping_offset_from(string_content) as
                    libc::c_long as uint32_t;
            // Add a step for the node
            let mut symbol_0: TSSymbol =
                ts_language_symbol_for_name((*self_0).language,
                                            string_content, length_0,
                                            0 as libc::c_int != 0);
            if symbol_0 == 0 {
                stream_reset(stream, string_content);
                return TSQueryErrorNodeType
            }
            array__grow(&mut (*self_0).steps as *mut C2RustUnnamed_15 as
                            *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<QueryStep>() as libc::c_ulong);
            let fresh41 = (*self_0).steps.size;
            (*self_0).steps.size = (*self_0).steps.size.wrapping_add(1);
            *(*self_0).steps.contents.offset(fresh41 as isize) =
                query_step__new(symbol_0, depth as uint16_t, is_immediate);
            if (*stream).next != '\"' as i32 { return TSQueryErrorSyntax }
            stream_advance(stream);
        } else if stream_is_ident_start(stream) {
            // Parse a field-prefixed pattern
            // Parse the field name
            let mut field_name: *const libc::c_char = (*stream).input;
            stream_scan_identifier(stream);
            let mut length_1: uint32_t =
                (*stream).input.wrapping_offset_from(field_name) as
                    libc::c_long as uint32_t;
            stream_skip_whitespace(stream);
            if (*stream).next != ':' as i32 {
                stream_reset(stream, field_name);
                return TSQueryErrorSyntax
            }
            stream_advance(stream);
            stream_skip_whitespace(stream);
            // Parse the pattern
            let mut step_index_0: uint32_t = (*self_0).steps.size;
            let mut e_2: TSQueryError =
                ts_query__parse_pattern(self_0, stream, depth, capture_count,
                                        is_immediate);
            if e_2 as libc::c_uint == PARENT_DONE as libc::c_uint {
                return TSQueryErrorSyntax
            }
            if e_2 as u64 != 0 { return e_2 }
            // Add the field name to the first step of the pattern
            let mut field_id: TSFieldId =
                ts_language_field_id_for_name((*self_0).language, field_name,
                                              length_1);
            if field_id == 0 {
                (*stream).input = field_name;
                return TSQueryErrorField
            }
            (*(*self_0).steps.contents.offset(step_index_0 as isize)).field =
                field_id
        } else { return TSQueryErrorSyntax }
    }
    stream_skip_whitespace(stream);
    loop 
         // Parse suffixes modifiers for this pattern
         {
        let mut step: *mut QueryStep =
            &mut *(*self_0).steps.contents.offset(starting_step_index as
                                                      isize) as
                *mut QueryStep;
        // Parse the one-or-more operator.
        if (*stream).next == '+' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut repeat_step: QueryStep =
                query_step__new(WILDCARD_SYMBOL, depth as uint16_t,
                                0 as libc::c_int != 0);
            repeat_step.alternative_index = starting_step_index as uint16_t;
            repeat_step.set_is_pass_through(1 as libc::c_int != 0);
            repeat_step.set_alternative_is_immediate(1 as libc::c_int != 0);
            array__grow(&mut (*self_0).steps as *mut C2RustUnnamed_15 as
                            *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<QueryStep>() as libc::c_ulong);
            let fresh42 = (*self_0).steps.size;
            (*self_0).steps.size = (*self_0).steps.size.wrapping_add(1);
            *(*self_0).steps.contents.offset(fresh42 as isize) = repeat_step
        } else if (*stream).next == '*' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut repeat_step_0: QueryStep =
                query_step__new(WILDCARD_SYMBOL, depth as uint16_t,
                                0 as libc::c_int != 0);
            repeat_step_0.alternative_index = starting_step_index as uint16_t;
            repeat_step_0.set_is_pass_through(1 as libc::c_int != 0);
            repeat_step_0.set_alternative_is_immediate(1 as libc::c_int != 0);
            array__grow(&mut (*self_0).steps as *mut C2RustUnnamed_15 as
                            *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<QueryStep>() as libc::c_ulong);
            let fresh43 = (*self_0).steps.size;
            (*self_0).steps.size = (*self_0).steps.size.wrapping_add(1);
            *(*self_0).steps.contents.offset(fresh43 as isize) =
                repeat_step_0;
            while (*step).alternative_index as libc::c_int !=
                      NONE as libc::c_int {
                step =
                    &mut *(*self_0).steps.contents.offset((*step).alternative_index
                                                              as isize) as
                        *mut QueryStep
            }
            (*step).alternative_index = (*self_0).steps.size as uint16_t
        } else if (*stream).next == '?' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            while (*step).alternative_index as libc::c_int !=
                      NONE as libc::c_int {
                step =
                    &mut *(*self_0).steps.contents.offset((*step).alternative_index
                                                              as isize) as
                        *mut QueryStep
            }
            (*step).alternative_index = (*self_0).steps.size as uint16_t
        } else {
            // Parse the zero-or-more repetition operator.
            // Parse the optional operator.
            // Parse an '@'-prefixed capture pattern
            if !((*stream).next == '@' as i32) { break ; }
            stream_advance(stream);
            if !stream_is_ident_start(stream) { return TSQueryErrorSyntax }
            let mut capture_name: *const libc::c_char = (*stream).input;
            stream_scan_identifier(stream);
            let mut length_2: uint32_t =
                (*stream).input.wrapping_offset_from(capture_name) as
                    libc::c_long as uint32_t;
            stream_skip_whitespace(stream);
            // Add the capture id to the first step of the pattern
            let mut capture_id: uint16_t =
                symbol_table_insert_name(&mut (*self_0).captures,
                                         capture_name, length_2);
            loop  {
                query_step__add_capture(step, capture_id);
                if !((*step).alternative_index as libc::c_int !=
                         NONE as libc::c_int &&
                         (*step).alternative_index as libc::c_uint >
                             starting_step_index &&
                         ((*step).alternative_index as libc::c_uint) <
                             (*self_0).steps.size) {
                    break ;
                }
                starting_step_index = (*step).alternative_index as uint32_t;
                step =
                    &mut *(*self_0).steps.contents.offset(starting_step_index
                                                              as isize) as
                        *mut QueryStep
            }
            *capture_count = (*capture_count).wrapping_add(1)
        }
    }
    return TSQueryErrorNone;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_new(mut language: *const TSLanguage,
                                      mut source: *const libc::c_char,
                                      mut source_len: uint32_t,
                                      mut error_offset: *mut uint32_t,
                                      mut error_type: *mut TSQueryError)
 -> *mut TSQuery {
    let mut symbol_map: *mut TSSymbol = 0 as *mut TSSymbol;
    if ts_language_version(language) >= 11 as libc::c_int as libc::c_uint {
        symbol_map = 0 as *mut TSSymbol
    } else {
        // Work around the fact that multiple symbols can currently be
    // associated with the same name, due to "simple aliases".
    // In the next language ABI version, this map will be contained
    // in the language's `public_symbol_map` field.
        let mut symbol_count: uint32_t = ts_language_symbol_count(language);
        symbol_map =
            ts_malloc((::std::mem::size_of::<TSSymbol>() as
                           libc::c_ulong).wrapping_mul(symbol_count as
                                                           libc::c_ulong)) as
                *mut TSSymbol;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < symbol_count {
            let mut name: *const libc::c_char =
                ts_language_symbol_name(language, i as TSSymbol);
            let symbol_type: TSSymbolType =
                ts_language_symbol_type(language, i as TSSymbol);
            *symbol_map.offset(i as isize) = i as TSSymbol;
            let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while j < i {
                if ts_language_symbol_type(language, j as TSSymbol) as
                       libc::c_uint == symbol_type as libc::c_uint {
                    if strcmp(name,
                              ts_language_symbol_name(language,
                                                      j as TSSymbol)) == 0 {
                        *symbol_map.offset(i as isize) = j as TSSymbol;
                        break ;
                    }
                }
                j = j.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
    }
    let mut self_0: *mut TSQuery =
        ts_malloc(::std::mem::size_of::<TSQuery>() as libc::c_ulong) as
            *mut TSQuery;
    *self_0 =
        {
            let mut init =
                TSQuery{captures: symbol_table_new(),
                        predicate_values: symbol_table_new(),
                        steps:
                            {
                                let mut init =
                                    C2RustUnnamed_15{contents:
                                                         0 as *mut QueryStep,
                                                     size:
                                                         0 as libc::c_int as
                                                             uint32_t,
                                                     capacity:
                                                         0 as libc::c_int as
                                                             uint32_t,};
                                init
                            },
                        pattern_map:
                            {
                                let mut init =
                                    C2RustUnnamed_14{contents:
                                                         0 as
                                                             *mut PatternEntry,
                                                     size:
                                                         0 as libc::c_int as
                                                             uint32_t,
                                                     capacity:
                                                         0 as libc::c_int as
                                                             uint32_t,};
                                init
                            },
                        predicate_steps:
                            {
                                let mut init =
                                    C2RustUnnamed_13{contents:
                                                         0 as
                                                             *mut TSQueryPredicateStep,
                                                     size:
                                                         0 as libc::c_int as
                                                             uint32_t,
                                                     capacity:
                                                         0 as libc::c_int as
                                                             uint32_t,};
                                init
                            },
                        predicates_by_pattern:
                            {
                                let mut init =
                                    C2RustUnnamed_12{contents:
                                                         0 as *mut Slice,
                                                     size:
                                                         0 as libc::c_int as
                                                             uint32_t,
                                                     capacity:
                                                         0 as libc::c_int as
                                                             uint32_t,};
                                init
                            },
                        start_bytes_by_pattern:
                            C2RustUnnamed_11{contents: 0 as *mut uint32_t,
                                             size: 0,
                                             capacity: 0,},
                        language: language,
                        wildcard_root_pattern_count:
                            0 as libc::c_int as uint16_t,
                        symbol_map: symbol_map,};
            init
        };
    // Parse all of the S-expressions in the given string.
    let mut stream: Stream = stream_new(source, source_len);
    stream_skip_whitespace(&mut stream);
    while stream.input < stream.end {
        let mut pattern_index: uint32_t =
            (*self_0).predicates_by_pattern.size;
        let mut start_step_index: uint32_t = (*self_0).steps.size;
        let mut capture_count: uint32_t = 0 as libc::c_int as uint32_t;
        array__grow(&mut (*self_0).start_bytes_by_pattern as
                        *mut C2RustUnnamed_11 as *mut VoidArray,
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong);
        let fresh44 = (*self_0).start_bytes_by_pattern.size;
        (*self_0).start_bytes_by_pattern.size =
            (*self_0).start_bytes_by_pattern.size.wrapping_add(1);
        *(*self_0).start_bytes_by_pattern.contents.offset(fresh44 as isize) =
            stream.input.wrapping_offset_from(source) as libc::c_long as
                uint32_t;
        array__grow(&mut (*self_0).predicates_by_pattern as
                        *mut C2RustUnnamed_12 as *mut VoidArray,
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<Slice>() as libc::c_ulong);
        let fresh45 = (*self_0).predicates_by_pattern.size;
        (*self_0).predicates_by_pattern.size =
            (*self_0).predicates_by_pattern.size.wrapping_add(1);
        *(*self_0).predicates_by_pattern.contents.offset(fresh45 as isize) =
            {
                let mut init =
                    Slice{offset: (*self_0).predicate_steps.size,
                          length: 0 as libc::c_int as uint32_t,};
                init
            };
        *error_type =
            ts_query__parse_pattern(self_0, &mut stream,
                                    0 as libc::c_int as uint32_t,
                                    &mut capture_count,
                                    0 as libc::c_int != 0);
        array__grow(&mut (*self_0).steps as *mut C2RustUnnamed_15 as
                        *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<QueryStep>() as libc::c_ulong);
        let fresh46 = (*self_0).steps.size;
        (*self_0).steps.size = (*self_0).steps.size.wrapping_add(1);
        *(*self_0).steps.contents.offset(fresh46 as isize) =
            query_step__new(0 as libc::c_int as TSSymbol, PATTERN_DONE_MARKER,
                            0 as libc::c_int != 0);
        // If any pattern could not be parsed, then report the error information
    // and terminate.
        if *error_type as u64 != 0 {
            if *error_type as libc::c_uint == PARENT_DONE as libc::c_uint {
                *error_type = TSQueryErrorSyntax
            }
            *error_offset =
                stream.input.wrapping_offset_from(source) as libc::c_long as
                    uint32_t;
            ts_query_delete(self_0);
            return 0 as *mut TSQuery
        }
        // If a pattern has a wildcard at its root, optimize the matching process
    // by skipping matching the wildcard.
        if (*(*self_0).steps.contents.offset(start_step_index as
                                                 isize)).symbol as libc::c_int
               == WILDCARD_SYMBOL as libc::c_int {
            let mut second_step: *mut QueryStep =
                &mut *(*self_0).steps.contents.offset(start_step_index.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                                                          as isize) as
                    *mut QueryStep;
            if (*second_step).symbol as libc::c_int !=
                   WILDCARD_SYMBOL as libc::c_int &&
                   (*second_step).depth as libc::c_int !=
                       PATTERN_DONE_MARKER as libc::c_int {
                start_step_index =
                    (start_step_index as
                         libc::c_uint).wrapping_add(1 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t
            }
        }
        loop 
             // Maintain a map that can look up patterns for a given root symbol.
             {
            let mut step: *mut QueryStep =
                &mut *(*self_0).steps.contents.offset(start_step_index as
                                                          isize) as
                    *mut QueryStep;
            (*step).set_is_pattern_start(1 as libc::c_int != 0);
            ts_query__pattern_map_insert(self_0, (*step).symbol,
                                         start_step_index, pattern_index);
            if (*step).symbol as libc::c_int == WILDCARD_SYMBOL as libc::c_int
               {
                (*self_0).wildcard_root_pattern_count =
                    (*self_0).wildcard_root_pattern_count.wrapping_add(1)
            }
            // If there are alternatives or options at the root of the pattern,
      // then add multiple entries to the pattern map.
            if !((*step).alternative_index as libc::c_int !=
                     NONE as libc::c_int) {
                break ;
            }
            start_step_index = (*step).alternative_index as uint32_t
        }
    }
    ts_query__finalize_steps(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_delete(mut self_0: *mut TSQuery) {
    if !self_0.is_null() {
        array__delete(&mut (*self_0).steps as *mut C2RustUnnamed_15 as
                          *mut VoidArray);
        array__delete(&mut (*self_0).pattern_map as *mut C2RustUnnamed_14 as
                          *mut VoidArray);
        array__delete(&mut (*self_0).predicate_steps as *mut C2RustUnnamed_13
                          as *mut VoidArray);
        array__delete(&mut (*self_0).predicates_by_pattern as
                          *mut C2RustUnnamed_12 as *mut VoidArray);
        array__delete(&mut (*self_0).start_bytes_by_pattern as
                          *mut C2RustUnnamed_11 as *mut VoidArray);
        symbol_table_delete(&mut (*self_0).captures);
        symbol_table_delete(&mut (*self_0).predicate_values);
        ts_free((*self_0).symbol_map as *mut libc::c_void);
        ts_free(self_0 as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_pattern_count(mut self_0: *const TSQuery)
 -> uint32_t {
    return (*self_0).predicates_by_pattern.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_capture_count(mut self_0: *const TSQuery)
 -> uint32_t {
    return (*self_0).captures.slices.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_string_count(mut self_0: *const TSQuery)
 -> uint32_t {
    return (*self_0).predicate_values.slices.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_capture_name_for_id(mut self_0:
                                                          *const TSQuery,
                                                      mut index: uint32_t,
                                                      mut length:
                                                          *mut uint32_t)
 -> *const libc::c_char {
    return symbol_table_name_for_id(&(*self_0).captures, index as uint16_t,
                                    length);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_string_value_for_id(mut self_0:
                                                          *const TSQuery,
                                                      mut index: uint32_t,
                                                      mut length:
                                                          *mut uint32_t)
 -> *const libc::c_char {
    return symbol_table_name_for_id(&(*self_0).predicate_values,
                                    index as uint16_t, length);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_predicates_for_pattern(mut self_0:
                                                             *const TSQuery,
                                                         mut pattern_index:
                                                             uint32_t,
                                                         mut step_count:
                                                             *mut uint32_t)
 -> *const TSQueryPredicateStep {
    let mut slice: Slice =
        *(*self_0).predicates_by_pattern.contents.offset(pattern_index as
                                                             isize);
    *step_count = slice.length;
    return &mut *(*self_0).predicate_steps.contents.offset(slice.offset as
                                                               isize) as
               *mut TSQueryPredicateStep;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_start_byte_for_pattern(mut self_0:
                                                             *const TSQuery,
                                                         mut pattern_index:
                                                             uint32_t)
 -> uint32_t {
    return *(*self_0).start_bytes_by_pattern.contents.offset(pattern_index as
                                                                 isize);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_disable_capture(mut self_0: *mut TSQuery,
                                                  mut name:
                                                      *const libc::c_char,
                                                  mut length: uint32_t) {
    // Remove capture information for any pattern step that previously
  // captured with the given name.
    let mut id: libc::c_int =
        symbol_table_id_for_name(&mut (*self_0).captures, name, length);
    if id != -(1 as libc::c_int) {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*self_0).steps.size {
            let mut step: *mut QueryStep =
                &mut *(*self_0).steps.contents.offset(i as isize) as
                    *mut QueryStep;
            query_step__remove_capture(step, id as uint16_t);
            i = i.wrapping_add(1)
        }
        ts_query__finalize_steps(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_disable_pattern(mut self_0: *mut TSQuery,
                                                  mut pattern_index:
                                                      uint32_t) {
    // Remove the given pattern from the pattern map. Its steps will still
  // be in the `steps` array, but they will never be read.
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).pattern_map.size {
        let mut pattern: *mut PatternEntry =
            &mut *(*self_0).pattern_map.contents.offset(i as isize) as
                *mut PatternEntry;
        if (*pattern).pattern_index as libc::c_uint == pattern_index {
            array__erase(&mut (*self_0).pattern_map as *mut C2RustUnnamed_14
                             as *mut VoidArray,
                         ::std::mem::size_of::<PatternEntry>() as
                             libc::c_ulong, i);
            i = i.wrapping_sub(1)
        }
        i = i.wrapping_add(1)
    };
}
/* **************
 * QueryCursor
 ***************/
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_new() -> *mut TSQueryCursor {
    let mut self_0: *mut TSQueryCursor =
        ts_malloc(::std::mem::size_of::<TSQueryCursor>() as libc::c_ulong) as
            *mut TSQueryCursor;
    *self_0 =
        {
            let mut init =
                TSQueryCursor{query: 0 as *const TSQuery,
                              cursor:
                                  TSTreeCursor{tree: 0 as *const libc::c_void,
                                               id: 0 as *const libc::c_void,
                                               context: [0; 2],},
                              states:
                                  {
                                      let mut init =
                                          C2RustUnnamed_19{contents:
                                                               0 as
                                                                   *mut QueryState,
                                                           size:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   uint32_t,
                                                           capacity:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   uint32_t,};
                                      init
                                  },
                              finished_states:
                                  {
                                      let mut init =
                                          C2RustUnnamed_18{contents:
                                                               0 as
                                                                   *mut QueryState,
                                                           size:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   uint32_t,
                                                           capacity:
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   uint32_t,};
                                      init
                                  },
                              capture_list_pool: capture_list_pool_new(),
                              depth: 0,
                              start_byte: 0 as libc::c_int as uint32_t,
                              end_byte: 4294967295 as libc::c_uint,
                              next_state_id: 0,
                              start_point:
                                  {
                                      let mut init =
                                          TSPoint{row:
                                                      0 as libc::c_int as
                                                          uint32_t,
                                                  column:
                                                      0 as libc::c_int as
                                                          uint32_t,};
                                      init
                                  },
                              end_point:
                                  {
                                      let mut init =
                                          TSPoint{row:
                                                      4294967295 as
                                                          libc::c_uint,
                                                  column:
                                                      4294967295 as
                                                          libc::c_uint,};
                                      init
                                  },
                              ascending: 0 as libc::c_int != 0,};
            init
        };
    array__reserve(&mut (*self_0).states as *mut C2RustUnnamed_19 as
                       *mut VoidArray,
                   ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                   256 as libc::c_int as uint32_t);
    array__reserve(&mut (*self_0).finished_states as *mut C2RustUnnamed_18 as
                       *mut VoidArray,
                   ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                   32 as libc::c_int as uint32_t);
    return self_0;
}
#[inline]
unsafe extern "C" fn bitmask_for_index(mut id: uint16_t) -> uint32_t {
    return (1 as libc::c_uint) << 31 as libc::c_int - id as libc::c_int;
}
// Search through all of the in-progress states, and find the captured
// node that occurs earliest in the document.
// Determine which node is first in a depth-first traversal
// Determine if either state contains a superset of the other state's captures.
// Duplicate the given state and insert the newly-created state immediately after
// the given state in the `states` array.
// If the state has captures, copy its capture list.
// Walk the tree, processing patterns until at least one pattern finishes,
// If one or more patterns finish, return `true` and store their states in the
// `finished_states` array. Multiple patterns can finish on the same node. If
// there are no more matches, return `false`.
#[inline]
unsafe extern "C" fn ts_query_cursor__advance(mut self_0: *mut TSQueryCursor)
 -> bool {
    loop  {
        if (*self_0).ascending {
            // Leave this node by stepping to its next sibling or to its parent.
            let mut did_move: bool = 1 as libc::c_int != 0;
            if ts_tree_cursor_goto_next_sibling(&mut (*self_0).cursor) {
                (*self_0).ascending = 0 as libc::c_int != 0
            } else if ts_tree_cursor_goto_parent(&mut (*self_0).cursor) {
                (*self_0).depth = (*self_0).depth.wrapping_sub(1)
            } else { did_move = 0 as libc::c_int != 0 }
            // After leaving a node, remove any states that cannot make further progress.
            let mut deleted_count: uint32_t = 0 as libc::c_int as uint32_t;
            let mut current_block_18: u64;
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            let mut n: libc::c_uint = (*self_0).states.size;
            while i < n {
                let mut state: *mut QueryState =
                    &mut *(*self_0).states.contents.offset(i as isize) as
                        *mut QueryState;
                let mut step: *mut QueryStep =
                    &mut *(*(*self_0).query).steps.contents.offset((*state).step_index
                                                                       as
                                                                       isize)
                        as *mut QueryStep;
                // If a state completed its pattern inside of this node, but was deferred from finishing
        // in order to search for longer matches, mark it as finished.
                if (*step).depth as libc::c_int ==
                       PATTERN_DONE_MARKER as libc::c_int {
                    if (*state).start_depth as libc::c_uint > (*self_0).depth
                           || !did_move {
                        let fresh47 = (*self_0).next_state_id;
                        (*self_0).next_state_id =
                            (*self_0).next_state_id.wrapping_add(1);
                        (*state).id = fresh47;
                        array__grow(&mut (*self_0).finished_states as
                                        *mut C2RustUnnamed_18 as
                                        *mut VoidArray,
                                    1 as libc::c_int as size_t,
                                    ::std::mem::size_of::<QueryState>() as
                                        libc::c_ulong);
                        let fresh48 = (*self_0).finished_states.size;
                        (*self_0).finished_states.size =
                            (*self_0).finished_states.size.wrapping_add(1);
                        *(*self_0).finished_states.contents.offset(fresh48 as
                                                                       isize)
                            = *state;
                        deleted_count = deleted_count.wrapping_add(1);
                        current_block_18 = 5399440093318478209;
                    } else { current_block_18 = 15768484401365413375; }
                } else if ((*state).start_depth as
                               uint32_t).wrapping_add((*step).depth as
                                                          uint32_t) >
                              (*self_0).depth {
                    capture_list_pool_release(&mut (*self_0).capture_list_pool,
                                              (*state).capture_list_id);
                    deleted_count = deleted_count.wrapping_add(1);
                    current_block_18 = 5399440093318478209;
                } else { current_block_18 = 15768484401365413375; }
                match current_block_18 {
                    15768484401365413375 => {
                        if deleted_count > 0 as libc::c_int as libc::c_uint {
                            *(*self_0).states.contents.offset(i.wrapping_sub(deleted_count)
                                                                  as isize) =
                                *state
                        }
                    }
                    _ => { }
                }
                i = i.wrapping_add(1)
            }
            (*self_0).states.size =
                ((*self_0).states.size as
                     libc::c_uint).wrapping_sub(deleted_count) as uint32_t as
                    uint32_t;
            if !did_move {
                return (*self_0).finished_states.size >
                           0 as libc::c_int as libc::c_uint
            }
        } else {
            // If a state needed to match something within this node, then remove that state
        // as it has failed to match.
            // If this node is before the selected range, then avoid descending into it.
            let mut node: TSNode =
                ts_tree_cursor_current_node(&mut (*self_0).cursor);
            if ts_node_end_byte(node) <= (*self_0).start_byte ||
                   point_lte(ts_node_end_point(node), (*self_0).start_point)
                       as libc::c_int != 0 {
                if !ts_tree_cursor_goto_next_sibling(&mut (*self_0).cursor) {
                    (*self_0).ascending = 1 as libc::c_int != 0
                }
            } else {
                // If this node is after the selected range, then stop walking.
                if (*self_0).end_byte <= ts_node_start_byte(node) ||
                       point_lte((*self_0).end_point,
                                 ts_node_start_point(node)) as libc::c_int !=
                           0 {
                    return 0 as libc::c_int != 0
                }
                // Get the properties of the current node.
                let mut symbol: TSSymbol = ts_node_symbol(node);
                let mut is_named: bool = ts_node_is_named(node);
                if symbol as libc::c_int !=
                       -(1 as libc::c_int) as TSSymbol as libc::c_int &&
                       !(*(*self_0).query).symbol_map.is_null() {
                    symbol =
                        *(*(*self_0).query).symbol_map.offset(symbol as isize)
                }
                let mut can_have_later_siblings: bool = false;
                let mut can_have_later_siblings_with_this_field: bool = false;
                let mut field_id: TSFieldId =
                    ts_tree_cursor_current_status(&mut (*self_0).cursor,
                                                  &mut can_have_later_siblings,
                                                  &mut can_have_later_siblings_with_this_field);
                // Add new states for any patterns whose root node is a wildcard.
                let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while i_0 <
                          (*(*self_0).query).wildcard_root_pattern_count as
                              libc::c_uint {
                    let mut pattern: *mut PatternEntry =
                        &mut *(*(*self_0).query).pattern_map.contents.offset(i_0
                                                                                 as
                                                                                 isize)
                            as *mut PatternEntry;
                    let mut step_0: *mut QueryStep =
                        &mut *(*(*self_0).query).steps.contents.offset((*pattern).step_index
                                                                           as
                                                                           isize)
                            as *mut QueryStep;
                    // If this node matches the first step of the pattern, then add a new
        // state at the start of this pattern.
                    if !((*step_0).field as libc::c_int != 0 &&
                             field_id as libc::c_int !=
                                 (*step_0).field as libc::c_int) {
                        if !ts_query_cursor__add_state(self_0, pattern) {
                            break ;
                        }
                    }
                    i_0 = i_0.wrapping_add(1)
                }
                // Add new states for any patterns whose root node matches this node.
                let mut i_1: libc::c_uint = 0;
                if ts_query__pattern_map_search((*self_0).query, symbol,
                                                &mut i_1) {
                    let mut pattern_0: *mut PatternEntry =
                        &mut *(*(*self_0).query).pattern_map.contents.offset(i_1
                                                                                 as
                                                                                 isize)
                            as *mut PatternEntry;
                    let mut step_1: *mut QueryStep =
                        &mut *(*(*self_0).query).steps.contents.offset((*pattern_0).step_index
                                                                           as
                                                                           isize)
                            as *mut QueryStep;
                    loop 
                         // If this node matches the first step of the pattern, then add a new
          // state at the start of this pattern.
                         {
                        if !((*step_1).field as libc::c_int != 0 &&
                                 field_id as libc::c_int !=
                                     (*step_1).field as libc::c_int) {
                            if !ts_query_cursor__add_state(self_0, pattern_0)
                               {
                                break ;
                            }
                            // Advance to the next pattern whose root node matches this node.
                            i_1 = i_1.wrapping_add(1);
                            if i_1 == (*(*self_0).query).pattern_map.size {
                                break ;
                            }
                            pattern_0 =
                                &mut *(*(*self_0).query).pattern_map.contents.offset(i_1
                                                                                         as
                                                                                         isize)
                                    as *mut PatternEntry;
                            step_1 =
                                &mut *(*(*self_0).query).steps.contents.offset((*pattern_0).step_index
                                                                                   as
                                                                                   isize)
                                    as *mut QueryStep
                        }
                        if !((*step_1).symbol as libc::c_int ==
                                 symbol as libc::c_int) {
                            break ;
                        }
                    }
                }
                let mut current_block_103: u64;
                // Update all of the in-progress states with current node.
                let mut i_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut copy_count: libc::c_uint =
                    0 as libc::c_int as libc::c_uint;
                while i_2 < (*self_0).states.size {
                    let mut state_0: *mut QueryState =
                        &mut *(*self_0).states.contents.offset(i_2 as isize)
                            as *mut QueryState;
                    let mut step_2: *mut QueryStep =
                        &mut *(*(*self_0).query).steps.contents.offset((*state_0).step_index
                                                                           as
                                                                           isize)
                            as *mut QueryStep;
                    (*state_0).set_has_in_progress_alternatives(0 as
                                                                    libc::c_int
                                                                    != 0);
                    copy_count = 0 as libc::c_int as libc::c_uint;
                    // Check that the node matches all of the criteria for the next
        // step of the pattern.
                    if !(((*state_0).start_depth as
                              uint32_t).wrapping_add((*step_2).depth as
                                                         uint32_t) !=
                             (*self_0).depth) {
                        // Determine if this node matches this step of the pattern, and also
        // if this node can have later siblings that match this step of the
        // pattern.
                        let mut node_does_match: bool =
                            (*step_2).symbol as libc::c_int ==
                                symbol as libc::c_int ||
                                (*step_2).symbol as libc::c_int ==
                                    WILDCARD_SYMBOL as libc::c_int ||
                                (*step_2).symbol as libc::c_int ==
                                    NAMED_WILDCARD_SYMBOL as libc::c_int &&
                                    is_named as libc::c_int != 0;
                        let mut later_sibling_can_match: bool =
                            can_have_later_siblings;
                        if (*step_2).is_immediate() as libc::c_int != 0 &&
                               is_named as libc::c_int != 0 ||
                               (*state_0).seeking_immediate_match() as
                                   libc::c_int != 0 {
                            later_sibling_can_match = 0 as libc::c_int != 0
                        }
                        if (*step_2).is_last_child() as libc::c_int != 0 &&
                               can_have_later_siblings as libc::c_int != 0 {
                            node_does_match = 0 as libc::c_int != 0
                        }
                        if (*step_2).field != 0 {
                            if (*step_2).field as libc::c_int ==
                                   field_id as libc::c_int {
                                if !can_have_later_siblings_with_this_field {
                                    later_sibling_can_match =
                                        0 as libc::c_int != 0
                                }
                            } else { node_does_match = 0 as libc::c_int != 0 }
                        }
                        // Remove states immediately if it is ever clear that they cannot match.
                        if !node_does_match {
                            if !later_sibling_can_match {
                                capture_list_pool_release(&mut (*self_0).capture_list_pool,
                                                          (*state_0).capture_list_id);
                                array__erase(&mut (*self_0).states as
                                                 *mut C2RustUnnamed_19 as
                                                 *mut VoidArray,
                                             ::std::mem::size_of::<QueryState>()
                                                 as libc::c_ulong, i_2);
                                i_2 = i_2.wrapping_sub(1)
                            }
                        } else {
                            // Some patterns can match their root node in multiple ways, capturing different
        // children. If this pattern step could match later children within the same
        // parent, then this query state cannot simply be updated in place. It must be
        // split into two states: one that matches this node, and one which skips over
        // this node, to preserve the possibility of matching later siblings.
                            if later_sibling_can_match as libc::c_int != 0 &&
                                   !(*step_2).is_pattern_start() &&
                                   (*step_2).contains_captures() as
                                       libc::c_int != 0 {
                                if !ts_query__cursor_copy_state(self_0,
                                                                state_0).is_null()
                                   {
                                    copy_count = copy_count.wrapping_add(1)
                                }
                            }
                            // If the current node is captured in this pattern, add it to the capture list.
        // For the first capture in a pattern, lazily acquire a capture list.
                            if (*step_2).capture_ids[0 as libc::c_int as
                                                         usize] as libc::c_int
                                   != NONE as libc::c_int {
                                if (*state_0).capture_list_id as libc::c_int
                                       == NONE as libc::c_int {
                                    (*state_0).capture_list_id =
                                        capture_list_pool_acquire(&mut (*self_0).capture_list_pool);
                                    // If there are no capture lists left in the pool, then terminate whichever
            // state has captured the earliest node in the document, and steal its
            // capture list.
                                    if (*state_0).capture_list_id as
                                           libc::c_int == NONE as libc::c_int
                                       {
                                        let mut state_index: uint32_t = 0;
                                        let mut byte_offset: uint32_t = 0;
                                        let mut pattern_index: uint32_t = 0;
                                        if ts_query_cursor__first_in_progress_capture(self_0,
                                                                                      &mut state_index,
                                                                                      &mut byte_offset,
                                                                                      &mut pattern_index)
                                           {
                                            (*state_0).capture_list_id =
                                                (*(*self_0).states.contents.offset(state_index
                                                                                       as
                                                                                       isize)).capture_list_id;
                                            array__erase(&mut (*self_0).states
                                                             as
                                                             *mut C2RustUnnamed_19
                                                             as
                                                             *mut VoidArray,
                                                         ::std::mem::size_of::<QueryState>()
                                                             as libc::c_ulong,
                                                         state_index);
                                            if state_index < i_2 {
                                                i_2 = i_2.wrapping_sub(1);
                                                state_0 = state_0.offset(-1)
                                            }
                                            current_block_103 =
                                                8304106758420804164;
                                        } else {
                                            array__erase(&mut (*self_0).states
                                                             as
                                                             *mut C2RustUnnamed_19
                                                             as
                                                             *mut VoidArray,
                                                         ::std::mem::size_of::<QueryState>()
                                                             as libc::c_ulong,
                                                         i_2);
                                            i_2 = i_2.wrapping_sub(1);
                                            current_block_103 =
                                                317151059986244064;
                                        }
                                    } else {
                                        current_block_103 =
                                            8304106758420804164;
                                    }
                                } else {
                                    current_block_103 = 8304106758420804164;
                                }
                                match current_block_103 {
                                    317151059986244064 => { }
                                    _ => {
                                        let mut capture_list:
                                                *mut CaptureList =
                                            capture_list_pool_get_mut(&mut (*self_0).capture_list_pool,
                                                                      (*state_0).capture_list_id);
                                        let mut j: libc::c_uint =
                                            0 as libc::c_int as libc::c_uint;
                                        while j <
                                                  3 as libc::c_int as
                                                      libc::c_uint {
                                            let mut capture_id: uint16_t =
                                                (*step_2).capture_ids[j as
                                                                          usize];
                                            if (*step_2).capture_ids[j as
                                                                         usize]
                                                   as libc::c_int ==
                                                   NONE as libc::c_int {
                                                break ;
                                            }
                                            array__grow(capture_list as
                                                            *mut VoidArray,
                                                        1 as libc::c_int as
                                                            size_t,
                                                        ::std::mem::size_of::<TSQueryCapture>()
                                                            as libc::c_ulong);
                                            let fresh49 =
                                                (*capture_list).size;
                                            (*capture_list).size =
                                                (*capture_list).size.wrapping_add(1);
                                            *(*capture_list).contents.offset(fresh49
                                                                                 as
                                                                                 isize)
                                                =
                                                {
                                                    let mut init =
                                                        TSQueryCapture{node:
                                                                           node,
                                                                       index:
                                                                           capture_id
                                                                               as
                                                                               uint32_t,};
                                                    init
                                                };
                                            j = j.wrapping_add(1)
                                        }
                                        current_block_103 =
                                            1069630499025798221;
                                    }
                                }
                            } else {
                                current_block_103 = 1069630499025798221;
                            }
                            match current_block_103 {
                                317151059986244064 => { }
                                _ => {
                                    // Advance this state to the next step of its pattern.
                                    (*state_0).step_index =
                                        (*state_0).step_index.wrapping_add(1);
                                    (*state_0).set_seeking_immediate_match(0
                                                                               as
                                                                               libc::c_int
                                                                               !=
                                                                               0);
                                    // If this state's next step has an 'alternative' step (the step is either optional,
        // or is the end of a repetition), then copy the state in order to pursue both
        // alternatives. The alternative step itself may have an alternative, so this is
        // an interative process.
                                    let mut end_index: libc::c_uint =
                                        i_2.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint);
                                    let mut j_0: libc::c_uint = i_2;
                                    while j_0 < end_index {
                                        let mut state_1: *mut QueryState =
                                            &mut *(*self_0).states.contents.offset(j_0
                                                                                       as
                                                                                       isize)
                                                as *mut QueryState;
                                        let mut next_step: *mut QueryStep =
                                            &mut *(*(*self_0).query).steps.contents.offset((*state_1).step_index
                                                                                               as
                                                                                               isize)
                                                as *mut QueryStep;
                                        if (*next_step).alternative_index as
                                               libc::c_int !=
                                               NONE as libc::c_int {
                                            if (*next_step).is_dead_end() {
                                                (*state_1).step_index =
                                                    (*next_step).alternative_index;
                                                j_0 = j_0.wrapping_sub(1)
                                            } else {
                                                let mut copy:
                                                        *mut QueryState =
                                                    ts_query__cursor_copy_state(self_0,
                                                                                state_1);
                                                if (*next_step).is_pass_through()
                                                   {
                                                    (*state_1).step_index =
                                                        (*state_1).step_index.wrapping_add(1);
                                                    j_0 = j_0.wrapping_sub(1)
                                                }
                                                if !copy.is_null() {
                                                    copy_count =
                                                        copy_count.wrapping_add(1);
                                                    end_index =
                                                        end_index.wrapping_add(1);
                                                    (*copy).step_index =
                                                        (*next_step).alternative_index;
                                                    if (*next_step).alternative_is_immediate()
                                                       {
                                                        (*copy).set_seeking_immediate_match(1
                                                                                                as
                                                                                                libc::c_int
                                                                                                !=
                                                                                                0)
                                                    }
                                                }
                                            }
                                        }
                                        j_0 = j_0.wrapping_add(1)
                                    }
                                }
                            }
                        }
                    }
                    i_2 =
                        i_2.wrapping_add((1 as libc::c_int as
                                              libc::c_uint).wrapping_add(copy_count))
                }
                let mut i_3: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while i_3 < (*self_0).states.size {
                    let mut state_2: *mut QueryState =
                        &mut *(*self_0).states.contents.offset(i_3 as isize)
                            as *mut QueryState;
                    let mut did_remove: bool = 0 as libc::c_int != 0;
                    let mut current_block_115: u64;
                    // Enfore the longest-match criteria. When a query pattern contains optional or
        // repeated nodes, this is necesssary to avoid multiple redundant states, where
        // one state has a strict subset of another state's captures.
                    let mut j_1: libc::c_uint =
                        i_3.wrapping_add(1 as libc::c_int as libc::c_uint);
                    while j_1 < (*self_0).states.size {
                        let mut other_state: *mut QueryState =
                            &mut *(*self_0).states.contents.offset(j_1 as
                                                                       isize)
                                as *mut QueryState;
                        if (*state_2).pattern_index as libc::c_int ==
                               (*other_state).pattern_index as libc::c_int &&
                               (*state_2).start_depth as libc::c_int ==
                                   (*other_state).start_depth as libc::c_int {
                            let mut left_contains_right: bool = false;
                            let mut right_contains_left: bool = false;
                            ts_query_cursor__compare_captures(self_0, state_2,
                                                              other_state,
                                                              &mut left_contains_right,
                                                              &mut right_contains_left);
                            if left_contains_right {
                                if (*state_2).step_index as libc::c_int ==
                                       (*other_state).step_index as
                                           libc::c_int {
                                    capture_list_pool_release(&mut (*self_0).capture_list_pool,
                                                              (*other_state).capture_list_id);
                                    array__erase(&mut (*self_0).states as
                                                     *mut C2RustUnnamed_19 as
                                                     *mut VoidArray,
                                                 ::std::mem::size_of::<QueryState>()
                                                     as libc::c_ulong, j_1);
                                    j_1 = j_1.wrapping_sub(1);
                                    current_block_115 = 10783567741412653655;
                                } else {
                                    (*other_state).set_has_in_progress_alternatives(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        !=
                                                                                        0);
                                    current_block_115 = 11359721434352816539;
                                }
                            } else {
                                current_block_115 = 11359721434352816539;
                            }
                            match current_block_115 {
                                10783567741412653655 => { }
                                _ => {
                                    if right_contains_left {
                                        if (*state_2).step_index as
                                               libc::c_int ==
                                               (*other_state).step_index as
                                                   libc::c_int {
                                            capture_list_pool_release(&mut (*self_0).capture_list_pool,
                                                                      (*state_2).capture_list_id);
                                            array__erase(&mut (*self_0).states
                                                             as
                                                             *mut C2RustUnnamed_19
                                                             as
                                                             *mut VoidArray,
                                                         ::std::mem::size_of::<QueryState>()
                                                             as libc::c_ulong,
                                                         i_3);
                                            did_remove =
                                                1 as libc::c_int != 0;
                                            break ;
                                        } else {
                                            (*state_2).set_has_in_progress_alternatives(1
                                                                                            as
                                                                                            libc::c_int
                                                                                            !=
                                                                                            0)
                                        }
                                    }
                                }
                            }
                        }
                        j_1 = j_1.wrapping_add(1)
                    }
                    // If there the state is at the end of its pattern, remove it from the list
        // of in-progress states and add it to the list of finished states.
                    if !did_remove {
                        let mut next_step_0: *mut QueryStep =
                            &mut *(*(*self_0).query).steps.contents.offset((*state_2).step_index
                                                                               as
                                                                               isize)
                                as *mut QueryStep;
                        if (*next_step_0).depth as libc::c_int ==
                               PATTERN_DONE_MARKER as libc::c_int {
                            if !(*state_2).has_in_progress_alternatives() {
                                let fresh50 = (*self_0).next_state_id;
                                (*self_0).next_state_id =
                                    (*self_0).next_state_id.wrapping_add(1);
                                (*state_2).id = fresh50;
                                array__grow(&mut (*self_0).finished_states as
                                                *mut C2RustUnnamed_18 as
                                                *mut VoidArray,
                                            1 as libc::c_int as size_t,
                                            ::std::mem::size_of::<QueryState>()
                                                as libc::c_ulong);
                                let fresh51 = (*self_0).finished_states.size;
                                (*self_0).finished_states.size =
                                    (*self_0).finished_states.size.wrapping_add(1);
                                *(*self_0).finished_states.contents.offset(fresh51
                                                                               as
                                                                               isize)
                                    = *state_2;
                                array__erase(&mut (*self_0).states as
                                                 *mut C2RustUnnamed_19 as
                                                 *mut VoidArray,
                                             ::std::mem::size_of::<QueryState>()
                                                 as libc::c_ulong,
                                             state_2.wrapping_offset_from((*self_0).states.contents)
                                                 as libc::c_long as uint32_t);
                                i_3 = i_3.wrapping_sub(1)
                            }
                        }
                    }
                    i_3 = i_3.wrapping_add(1)
                }
                // Continue descending if possible.
                if ts_tree_cursor_goto_first_child(&mut (*self_0).cursor) {
                    (*self_0).depth = (*self_0).depth.wrapping_add(1)
                } else { (*self_0).ascending = 1 as libc::c_int != 0 }
            }
        }
        if !((*self_0).finished_states.size ==
                 0 as libc::c_int as libc::c_uint) {
            break ;
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor__compare_captures(mut self_0:
                                                               *mut TSQueryCursor,
                                                           mut left_state:
                                                               *mut QueryState,
                                                           mut right_state:
                                                               *mut QueryState,
                                                           mut left_contains_right:
                                                               *mut bool,
                                                           mut right_contains_left:
                                                               *mut bool) {
    let mut left_captures: *const CaptureList =
        capture_list_pool_get(&mut (*self_0).capture_list_pool,
                              (*left_state).capture_list_id);
    let mut right_captures: *const CaptureList =
        capture_list_pool_get(&mut (*self_0).capture_list_pool,
                              (*right_state).capture_list_id);
    *left_contains_right = 1 as libc::c_int != 0;
    *right_contains_left = 1 as libc::c_int != 0;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        if i < (*left_captures).size {
            if j < (*right_captures).size {
                let mut left: *mut TSQueryCapture =
                    &mut *(*left_captures).contents.offset(i as isize) as
                        *mut TSQueryCapture;
                let mut right: *mut TSQueryCapture =
                    &mut *(*right_captures).contents.offset(j as isize) as
                        *mut TSQueryCapture;
                if (*left).node.id == (*right).node.id &&
                       (*left).index == (*right).index {
                    i = i.wrapping_add(1);
                    j = j.wrapping_add(1)
                } else {
                    match ts_query_cursor__compare_nodes((*left).node,
                                                         (*right).node) {
                        -1 => {
                            *right_contains_left = 0 as libc::c_int != 0;
                            i = i.wrapping_add(1)
                        }
                        1 => {
                            *left_contains_right = 0 as libc::c_int != 0;
                            j = j.wrapping_add(1)
                        }
                        _ => {
                            *right_contains_left = 0 as libc::c_int != 0;
                            *left_contains_right = 0 as libc::c_int != 0;
                            i = i.wrapping_add(1);
                            j = j.wrapping_add(1)
                        }
                    }
                }
            } else { *right_contains_left = 0 as libc::c_int != 0; break ; }
        } else {
            if j < (*right_captures).size {
                *left_contains_right = 0 as libc::c_int != 0
            }
            break ;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor__compare_nodes(mut left: TSNode,
                                                        mut right: TSNode)
 -> libc::c_int {
    if left.id != right.id {
        let mut left_start: uint32_t = ts_node_start_byte(left);
        let mut right_start: uint32_t = ts_node_start_byte(right);
        if left_start < right_start { return -(1 as libc::c_int) }
        if left_start > right_start { return 1 as libc::c_int }
        let mut left_node_count: uint32_t = ts_node_end_byte(left);
        let mut right_node_count: uint32_t = ts_node_end_byte(right);
        if left_node_count > right_node_count { return -(1 as libc::c_int) }
        if left_node_count < right_node_count { return 1 as libc::c_int }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ts_query__cursor_copy_state(mut self_0:
                                                     *mut TSQueryCursor,
                                                 mut state: *const QueryState)
 -> *mut QueryState {
    if (*self_0).states.size >= 256 as libc::c_int as libc::c_uint {
        return 0 as *mut QueryState
    }
    let mut copy: QueryState = *state;
    copy.capture_list_id = (*state).capture_list_id;
    if (*state).capture_list_id as libc::c_int != NONE as libc::c_int {
        copy.capture_list_id =
            capture_list_pool_acquire(&mut (*self_0).capture_list_pool);
        if copy.capture_list_id as libc::c_int == NONE as libc::c_int {
            return 0 as *mut QueryState
        }
        let mut old_captures: *const CaptureList =
            capture_list_pool_get(&mut (*self_0).capture_list_pool,
                                  (*state).capture_list_id);
        let mut new_captures: *mut CaptureList =
            capture_list_pool_get_mut(&mut (*self_0).capture_list_pool,
                                      copy.capture_list_id);
        array__splice(new_captures as *mut VoidArray,
                      ::std::mem::size_of::<TSQueryCapture>() as
                          libc::c_ulong, (*new_captures).size,
                      0 as libc::c_int as uint32_t, (*old_captures).size,
                      (*old_captures).contents as *const libc::c_void);
    }
    let mut index: uint32_t =
        (state.wrapping_offset_from((*self_0).states.contents) as libc::c_long
             + 1 as libc::c_int as libc::c_long) as uint32_t;
    array__splice(&mut (*self_0).states as *mut C2RustUnnamed_19 as
                      *mut VoidArray,
                  ::std::mem::size_of::<QueryState>() as libc::c_ulong, index,
                  0 as libc::c_int as uint32_t, 1 as libc::c_int as uint32_t,
                  &mut copy as *mut QueryState as *const libc::c_void);
    return &mut *(*self_0).states.contents.offset(index as isize) as
               *mut QueryState;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_delete(mut self_0:
                                                    *mut TSQueryCursor) {
    array__delete(&mut (*self_0).states as *mut C2RustUnnamed_19 as
                      *mut VoidArray);
    array__delete(&mut (*self_0).finished_states as *mut C2RustUnnamed_18 as
                      *mut VoidArray);
    ts_tree_cursor_delete(&mut (*self_0).cursor);
    capture_list_pool_delete(&mut (*self_0).capture_list_pool);
    ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_exec(mut self_0: *mut TSQueryCursor,
                                              mut query: *const TSQuery,
                                              mut node: TSNode) {
    (*self_0).states.size = 0 as libc::c_int as uint32_t;
    (*self_0).finished_states.size = 0 as libc::c_int as uint32_t;
    ts_tree_cursor_reset(&mut (*self_0).cursor, node);
    capture_list_pool_reset(&mut (*self_0).capture_list_pool);
    (*self_0).next_state_id = 0 as libc::c_int as uint32_t;
    (*self_0).depth = 0 as libc::c_int as uint32_t;
    (*self_0).ascending = 0 as libc::c_int != 0;
    (*self_0).query = query;
}
#[inline]
unsafe extern "C" fn count_leading_zeros(mut x: uint32_t) -> uint32_t {
    if x == 0 as libc::c_int as libc::c_uint {
        return 32 as libc::c_int as uint32_t
    }
    return x.leading_zeros() as i32 as uint32_t;
}
unsafe extern "C" fn ts_query_cursor__first_in_progress_capture(mut self_0:
                                                                    *mut TSQueryCursor,
                                                                mut state_index:
                                                                    *mut uint32_t,
                                                                mut byte_offset:
                                                                    *mut uint32_t,
                                                                mut pattern_index:
                                                                    *mut uint32_t)
 -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    *state_index = 4294967295 as libc::c_uint;
    *byte_offset = 4294967295 as libc::c_uint;
    *pattern_index = 4294967295 as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).states.size {
        let mut state: *const QueryState =
            &mut *(*self_0).states.contents.offset(i as isize) as
                *mut QueryState;
        let mut captures: *const CaptureList =
            capture_list_pool_get(&mut (*self_0).capture_list_pool,
                                  (*state).capture_list_id);
        if (*captures).size > 0 as libc::c_int as libc::c_uint {
            let mut capture_byte: uint32_t =
                ts_node_start_byte((*(*captures).contents.offset(0 as
                                                                     libc::c_int
                                                                     as
                                                                     isize)).node);
            if !result || capture_byte < *byte_offset ||
                   capture_byte == *byte_offset &&
                       ((*state).pattern_index as libc::c_uint) <
                           *pattern_index {
                result = 1 as libc::c_int != 0;
                *state_index = i;
                *byte_offset = capture_byte;
                *pattern_index = (*state).pattern_index as uint32_t
            }
        }
        i = i.wrapping_add(1)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_set_byte_range(mut self_0:
                                                            *mut TSQueryCursor,
                                                        mut start_byte:
                                                            uint32_t,
                                                        mut end_byte:
                                                            uint32_t) {
    if end_byte == 0 as libc::c_int as libc::c_uint {
        start_byte = 0 as libc::c_int as uint32_t;
        end_byte = 4294967295 as libc::c_uint
    }
    (*self_0).start_byte = start_byte;
    (*self_0).end_byte = end_byte;
}
unsafe extern "C" fn ts_query_cursor__add_state(mut self_0:
                                                    *mut TSQueryCursor,
                                                mut pattern:
                                                    *const PatternEntry)
 -> bool {
    if (*self_0).states.size >= 256 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int != 0
    }
    let mut step: *mut QueryStep =
        &mut *(*(*self_0).query).steps.contents.offset((*pattern).step_index
                                                           as isize) as
            *mut QueryStep;
    array__grow(&mut (*self_0).states as *mut C2RustUnnamed_19 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<QueryState>() as libc::c_ulong);
    let fresh52 = (*self_0).states.size;
    (*self_0).states.size = (*self_0).states.size.wrapping_add(1);
    *(*self_0).states.contents.offset(fresh52 as isize) =
        {
            let mut init =
                QueryState{consumed_capture_count_seeking_immediate_match_has_in_progress_alternatives:
                               [0; 2],
                           c2rust_padding: [0; 2],
                           id: 0,
                           start_depth:
                               (*self_0).depth.wrapping_sub((*step).depth as
                                                                libc::c_uint)
                                   as uint16_t,
                           step_index: (*pattern).step_index,
                           pattern_index: (*pattern).pattern_index,
                           capture_list_id: NONE,};
            init.set_consumed_capture_count(0 as libc::c_int as uint16_t);
            init.set_seeking_immediate_match(0 as libc::c_int != 0);
            init.set_has_in_progress_alternatives(false);
            init
        };
    return 1 as libc::c_int != 0;
}
/* *******************/
/* Section - Parser */
/* *******************/
/* *
 * Create a new parser.
 */
/* *
 * Delete the parser, freeing all of the memory that it used.
 */
/* *
 * Set the language that the parser should use for parsing.
 *
 * Returns a boolean indicating whether or not the language was successfully
 * assigned. True means assignment succeeded. False means there was a version
 * mismatch: the language was generated with an incompatible version of the
 * Tree-sitter CLI. Check the language's version using `ts_language_version`
 * and compare it to this library's `TREE_SITTER_LANGUAGE_VERSION` and
 * `TREE_SITTER_MIN_COMPATIBLE_LANGUAGE_VERSION` constants.
 */
/* *
 * Get the parser's current language.
 */
/* *
 * Set the ranges of text that the parser should include when parsing.
 *
 * By default, the parser will always include entire documents. This function
 * allows you to parse only a *portion* of a document but still return a syntax
 * tree whose ranges match up with the document as a whole. You can also pass
 * multiple disjoint ranges.
 *
 * The second and third parameters specify the location and length of an array
 * of ranges. The parser does *not* take ownership of these ranges; it copies
 * the data, so it doesn't matter how these ranges are allocated.
 *
 * If `length` is zero, then the entire document will be parsed. Otherwise,
 * the given ranges must be ordered from earliest to latest in the document,
 * and they must not overlap. That is, the following must hold for all
 * `i` < `length - 1`:
 *
 *     ranges[i].end_byte <= ranges[i + 1].start_byte
 *
 * If this requirement is not satisfied, the operation will fail, the ranges
 * will not be assigned, and this function will return `false`. On success,
 * this function returns `true`
 */
/* *
 * Get the ranges of text that the parser will include when parsing.
 *
 * The returned pointer is owned by the parser. The caller should not free it
 * or write to it. The length of the array will be written to the given
 * `length` pointer.
 */
/* *
 * Use the parser to parse some source code and create a syntax tree.
 *
 * If you are parsing this document for the first time, pass `NULL` for the
 * `old_tree` parameter. Otherwise, if you have already parsed an earlier
 * version of this document and the document has since been edited, pass the
 * previous syntax tree so that the unchanged parts of it can be reused.
 * This will save time and memory. For this to work correctly, you must have
 * already edited the old syntax tree using the `ts_tree_edit` function in a
 * way that exactly matches the source code changes.
 *
 * The `TSInput` parameter lets you specify how to read the text. It has the
 * following three fields:
 * 1. `read`: A function to retrieve a chunk of text at a given byte offset
 *    and (row, column) position. The function should return a pointer to the
 *    text and write its length to the the `bytes_read` pointer. The parser
 *    does not take ownership of this buffer; it just borrows it until it has
 *    finished reading it. The function should write a zero value to the
 *    `bytes_read` pointer to indicate the end of the document.
 * 2. `payload`: An arbitrary pointer that will be passed to each invocation
 *    of the `read` function.
 * 3. `encoding`: An indication of how the text is encoded. Either
 *    `TSInputEncodingUTF8` or `TSInputEncodingUTF16`.
 *
 * This function returns a syntax tree on success, and `NULL` on failure. There
 * are three possible reasons for failure:
 * 1. The parser does not have a language assigned. Check for this using the
      `ts_parser_language` function.
 * 2. Parsing was cancelled due to a timeout that was set by an earlier call to
 *    the `ts_parser_set_timeout_micros` function. You can resume parsing from
 *    where the parser left out by calling `ts_parser_parse` again with the
 *    same arguments. Or you can start parsing from scratch by first calling
 *    `ts_parser_reset`.
 * 3. Parsing was cancelled using a cancellation flag that was set by an
 *    earlier call to `ts_parser_set_cancellation_flag`. You can resume parsing
 *    from where the parser left out by calling `ts_parser_parse` again with
 *    the same arguments.
 */
/* *
 * Use the parser to parse some source code stored in one contiguous buffer.
 * The first two parameters are the same as in the `ts_parser_parse` function
 * above. The second two parameters indicate the location of the buffer and its
 * length in bytes.
 */
/* *
 * Use the parser to parse some source code stored in one contiguous buffer with
 * a given encoding. The first four parameters work the same as in the
 * `ts_parser_parse_string` method above. The final parameter indicates whether
 * the text is encoded as UTF8 or UTF16.
 */
/* *
 * Instruct the parser to start the next parse from the beginning.
 *
 * If the parser previously failed because of a timeout or a cancellation, then
 * by default, it will resume where it left off on the next call to
 * `ts_parser_parse` or other parsing functions. If you don't want to resume,
 * and instead intend to use this parser to parse some other document, you must
 * call `ts_parser_reset` first.
 */
/* *
 * Set the maximum duration in microseconds that parsing should be allowed to
 * take before halting.
 *
 * If parsing takes longer than this, it will halt early, returning NULL.
 * See `ts_parser_parse` for more information.
 */
/* *
 * Get the duration in microseconds that parsing is allowed to take.
 */
/* *
 * Set the parser's current cancellation flag pointer.
 *
 * If a non-null pointer is assigned, then the parser will periodically read
 * from this pointer during parsing. If it reads a non-zero value, it will
 * halt early, returning NULL. See `ts_parser_parse` for more information.
 */
/* *
 * Get the parser's current cancellation flag pointer.
 */
/* *
 * Set the logger that a parser should use during parsing.
 *
 * The parser does not take ownership over the logger payload. If a logger was
 * previously assigned, the caller is responsible for releasing any memory
 * owned by the previous logger.
 */
/* *
 * Get the parser's current logger.
 */
/* *
 * Set the file descriptor to which the parser should write debugging graphs
 * during parsing. The graphs are formatted in the DOT language. You may want
 * to pipe these graphs directly to a `dot(1)` process in order to generate
 * SVG output. You can turn off this logging by passing a negative number.
 */
/* *****************/
/* Section - Tree */
/* *****************/
/* *
 * Create a shallow copy of the syntax tree. This is very fast.
 *
 * You need to copy a syntax tree in order to use it on more than one thread at
 * a time, as syntax trees are not thread safe.
 */
/* *
 * Delete the syntax tree, freeing all of the memory that it used.
 */
/* *
 * Get the root node of the syntax tree.
 */
/* *
 * Get the language that was used to parse the syntax tree.
 */
/* *
 * Edit the syntax tree to keep it in sync with source code that has been
 * edited.
 *
 * You must describe the edit both in terms of byte offsets and in terms of
 * (row, column) coordinates.
 */
/* *
 * Compare an old edited syntax tree to a new syntax tree representing the same
 * document, returning an array of ranges whose syntactic structure has changed.
 *
 * For this to work correctly, the old syntax tree must have been edited such
 * that its ranges match up to the new tree. Generally, you'll want to call
 * this function right after calling one of the `ts_parser_parse` functions.
 * You need to pass the old tree that was passed to parse, as well as the new
 * tree that was returned from that function.
 *
 * The returned array is allocated using `malloc` and the caller is responsible
 * for freeing it using `free`. The length of the array will be written to the
 * given `length` pointer.
 */
/* *
 * Write a DOT graph describing the syntax tree to the given file.
 */
/* *****************/
/* Section - Node */
/* *****************/
/* *
 * Get the node's type as a null-terminated string.
 */
/* *
 * Get the node's type as a numerical id.
 */
/* *
 * Get the node's start byte.
 */
/* *
 * Get the node's start position in terms of rows and columns.
 */
/* *
 * Get the node's end byte.
 */
/* *
 * Get the node's end position in terms of rows and columns.
 */
/* *
 * Get an S-expression representing the node as a string.
 *
 * This string is allocated with `malloc` and the caller is responsible for
 * freeing it using `free`.
 */
/* *
 * Check if the node is null. Functions like `ts_node_child` and
 * `ts_node_next_sibling` will return a null node to indicate that no such node
 * was found.
 */
/* *
 * Check if the node is *named*. Named nodes correspond to named rules in the
 * grammar, whereas *anonymous* nodes correspond to string literals in the
 * grammar.
 */
/* *
 * Check if the node is *missing*. Missing nodes are inserted by the parser in
 * order to recover from certain kinds of syntax errors.
 */
/* *
 * Check if the node is *extra*. Extra nodes represent things like comments,
 * which are not required the grammar, but can appear anywhere.
 */
/* *
 * Check if a syntax node has been edited.
 */
/* *
 * Check if the node is a syntax error or contains any syntax errors.
 */
/* *
 * Get the node's immediate parent.
 */
/* *
 * Get the node's child at the given index, where zero represents the first
 * child.
 */
/* *
 * Get the node's number of children.
 */
/* *
 * Get the node's *named* child at the given index.
 *
 * See also `ts_node_is_named`.
 */
/* *
 * Get the node's number of *named* children.
 *
 * See also `ts_node_is_named`.
 */
/* *
 * Get the node's child with the given field name.
 */
/* *
 * Get the node's child with the given numerical field id.
 *
 * You can convert a field name to an id using the
 * `ts_language_field_id_for_name` function.
 */
/* *
 * Get the node's next / previous sibling.
 */
/* *
 * Get the node's next / previous *named* sibling.
 */
/* *
 * Get the node's first child that extends beyond the given byte offset.
 */
/* *
 * Get the node's first named child that extends beyond the given byte offset.
 */
/* *
 * Get the smallest node within this node that spans the given range of bytes
 * or (row, column) positions.
 */
/* *
 * Get the smallest named node within this node that spans the given range of
 * bytes or (row, column) positions.
 */
/* *
 * Edit the node to keep it in-sync with source code that has been edited.
 *
 * This function is only rarely needed. When you edit a syntax tree with the
 * `ts_tree_edit` function, all of the nodes that you retrieve from the tree
 * afterward will already reflect the edit. You only need to use `ts_node_edit`
 * when you have a `TSNode` instance that you want to keep and continue to use
 * after an edit.
 */
/* *
 * Check if two nodes are identical.
 */
/* ***********************/
/* Section - TreeCursor */
/* ***********************/
/* *
 * Create a new tree cursor starting from the given node.
 *
 * A tree cursor allows you to walk a syntax tree more efficiently than is
 * possible using the `TSNode` functions. It is a mutable object that is always
 * on a certain syntax node, and can be moved imperatively to different nodes.
 */
/* *
 * Delete a tree cursor, freeing all of the memory that it used.
 */
/* *
 * Re-initialize a tree cursor to start at a different node.
 */
/* *
 * Get the tree cursor's current node.
 */
/* *
 * Get the field name of the tree cursor's current node.
 *
 * This returns `NULL` if the current node doesn't have a field.
 * See also `ts_node_child_by_field_name`.
 */
/* *
 * Get the field name of the tree cursor's current node.
 *
 * This returns zero if the current node doesn't have a field.
 * See also `ts_node_child_by_field_id`, `ts_language_field_id_for_name`.
 */
/* *
 * Move the cursor to the parent of its current node.
 *
 * This returns `true` if the cursor successfully moved, and returns `false`
 * if there was no parent node (the cursor was already on the root node).
 */
/* *
 * Move the cursor to the next sibling of its current node.
 *
 * This returns `true` if the cursor successfully moved, and returns `false`
 * if there was no next sibling node.
 */
/* *
 * Move the cursor to the first child of its current node.
 *
 * This returns `true` if the cursor successfully moved, and returns `false`
 * if there were no children.
 */
/* *
 * Move the cursor to the first child of its current node that extends beyond
 * the given byte offset.
 *
 * This returns the index of the child node if one was found, and returns -1
 * if no such child was found.
 */
/* ******************/
/* Section - Query */
/* ******************/
/* *
 * Create a new query from a string containing one or more S-expression
 * patterns. The query is associated with a particular language, and can
 * only be run on syntax nodes parsed with that language.
 *
 * If all of the given patterns are valid, this returns a `TSQuery`.
 * If a pattern is invalid, this returns `NULL`, and provides two pieces
 * of information about the problem:
 * 1. The byte offset of the error is written to the `error_offset` parameter.
 * 2. The type of error is written to the `error_type` parameter.
 */
/* *
 * Delete a query, freeing all of the memory that it used.
 */
/* *
 * Get the number of patterns, captures, or string literals in the query.
 */
/* *
 * Get the byte offset where the given pattern starts in the query's source.
 *
 * This can be useful when combining queries by concatenating their source
 * code strings.
 */
/* *
 * Get all of the predicates for the given pattern in the query.
 *
 * The predicates are represented as a single array of steps. There are three
 * types of steps in this array, which correspond to the three legal values for
 * the `type` field:
 * - `TSQueryPredicateStepTypeCapture` - Steps with this type represent names
 *    of captures. Their `value_id` can be used with the
 *   `ts_query_capture_name_for_id` function to obtain the name of the capture.
 * - `TSQueryPredicateStepTypeString` - Steps with this type represent literal
 *    strings. Their `value_id` can be used with the
 *    `ts_query_string_value_for_id` function to obtain their string value.
 * - `TSQueryPredicateStepTypeDone` - Steps with this type are *sentinels*
 *    that represent the end of an individual predicate. If a pattern has two
 *    predicates, then there will be two steps with this `type` in the array.
 */
/* *
 * Get the name and length of one of the query's captures, or one of the
 * query's string literals. Each capture and string is associated with a
 * numeric id based on the order that it appeared in the query's source.
 */
/* *
 * Disable a certain capture within a query.
 *
 * This prevents the capture from being returned in matches, and also avoids
 * any resource usage associated with recording the capture. Currently, there
 * is no way to undo this.
 */
/* *
 * Disable a certain pattern within a query.
 *
 * This prevents the pattern from matching and removes most of the overhead
 * associated with the pattern. Currently, there is no way to undo this.
 */
/* *
 * Create a new cursor for executing a given query.
 *
 * The cursor stores the state that is needed to iteratively search
 * for matches. To use the query cursor, first call `ts_query_cursor_exec`
 * to start running a given query on a given syntax node. Then, there are
 * two options for consuming the results of the query:
 * 1. Repeatedly call `ts_query_cursor_next_match` to iterate over all of the
 *    the *matches* in the order that they were found. Each match contains the
 *    index of the pattern that matched, and an array of captures. Because
 *    multiple patterns can match the same set of nodes, one match may contain
 *    captures that appear *before* some of the captures from a previous match.
 * 2. Repeatedly call `ts_query_cursor_next_capture` to iterate over all of the
 *    individual *captures* in the order that they appear. This is useful if
 *    don't care about which pattern matched, and just want a single ordered
 *    sequence of captures.
 *
 * If you don't care about consuming all of the results, you can stop calling
 * `ts_query_cursor_next_match` or `ts_query_cursor_next_capture` at any point.
 *  You can then start executing another query on another node by calling
 *  `ts_query_cursor_exec` again.
 */
/* *
 * Delete a query cursor, freeing all of the memory that it used.
 */
/* *
 * Start running a given query on a given node.
 */
/* *
 * Set the range of bytes or (row, column) positions in which the query
 * will be executed.
 */
/* *
 * Advance to the next match of the currently running query.
 *
 * If there is a match, write it to `*match` and return `true`.
 * Otherwise, return `false`.
 */
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_remove_match(mut self_0:
                                                          *mut TSQueryCursor,
                                                      mut match_id:
                                                          uint32_t) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).finished_states.size {
        let mut state: *const QueryState =
            &mut *(*self_0).finished_states.contents.offset(i as isize) as
                *mut QueryState;
        if (*state).id == match_id {
            capture_list_pool_release(&mut (*self_0).capture_list_pool,
                                      (*state).capture_list_id);
            array__erase(&mut (*self_0).finished_states as
                             *mut C2RustUnnamed_18 as *mut VoidArray,
                         ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                         i);
            return
        }
        i = i.wrapping_add(1)
    };
}
/* *
 * Advance to the next capture of the currently running query.
 *
 * If there is a capture, write its match to `*match` and its index within
 * the matche's capture list to `*capture_index`. Otherwise, return `false`.
 */
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_next_capture(mut self_0:
                                                          *mut TSQueryCursor,
                                                      mut match_0:
                                                          *mut TSQueryMatch,
                                                      mut capture_index:
                                                          *mut uint32_t)
 -> bool {
    loop  {
        // The goal here is to return captures in order, even though they may not
    // be discovered in order, because patterns can overlap. If there are any
    // finished patterns, then try to find one that contains a capture that
    // is *definitely* before any capture in an *unfinished* pattern.
        if (*self_0).finished_states.size > 0 as libc::c_int as libc::c_uint {
            // First, identify the position of the earliest capture in an unfinished
      // match. For a finished capture to be returned, it must be *before*
      // this position.
            let mut first_unfinished_capture_byte: uint32_t = 0;
            let mut first_unfinished_pattern_index: uint32_t = 0;
            let mut first_unfinished_state_index: uint32_t = 0;
            ts_query_cursor__first_in_progress_capture(self_0,
                                                       &mut first_unfinished_state_index,
                                                       &mut first_unfinished_capture_byte,
                                                       &mut first_unfinished_pattern_index);
            // Find the earliest capture in a finished match.
            let mut first_finished_state_index: libc::c_int =
                -(1 as libc::c_int);
            let mut first_finished_capture_byte: uint32_t =
                first_unfinished_capture_byte;
            let mut first_finished_pattern_index: uint32_t =
                first_unfinished_pattern_index;
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while i < (*self_0).finished_states.size {
                let mut state: *const QueryState =
                    &mut *(*self_0).finished_states.contents.offset(i as
                                                                        isize)
                        as *mut QueryState;
                let mut captures: *const CaptureList =
                    capture_list_pool_get(&mut (*self_0).capture_list_pool,
                                          (*state).capture_list_id);
                if (*captures).size >
                       (*state).consumed_capture_count() as libc::c_uint {
                    let mut capture_byte: uint32_t =
                        ts_node_start_byte((*(*captures).contents.offset((*state).consumed_capture_count()
                                                                             as
                                                                             isize)).node);
                    if capture_byte < first_finished_capture_byte ||
                           capture_byte == first_finished_capture_byte &&
                               ((*state).pattern_index as libc::c_uint) <
                                   first_finished_pattern_index {
                        first_finished_state_index = i as libc::c_int;
                        first_finished_capture_byte = capture_byte;
                        first_finished_pattern_index =
                            (*state).pattern_index as uint32_t
                    }
                } else {
                    capture_list_pool_release(&mut (*self_0).capture_list_pool,
                                              (*state).capture_list_id);
                    array__erase(&mut (*self_0).finished_states as
                                     *mut C2RustUnnamed_18 as *mut VoidArray,
                                 ::std::mem::size_of::<QueryState>() as
                                     libc::c_ulong, i);
                    i = i.wrapping_sub(1)
                }
                i = i.wrapping_add(1)
            }
            // If there is finished capture that is clearly before any unfinished
      // capture, then return its match, and its capture index. Internally
      // record the fact that the capture has been 'consumed'.
            if first_finished_state_index != -(1 as libc::c_int) {
                let mut state_0: *mut QueryState =
                    &mut *(*self_0).finished_states.contents.offset(first_finished_state_index
                                                                        as
                                                                        isize)
                        as *mut QueryState;
                (*match_0).id = (*state_0).id;
                (*match_0).pattern_index = (*state_0).pattern_index;
                let mut captures_0: *const CaptureList =
                    capture_list_pool_get(&mut (*self_0).capture_list_pool,
                                          (*state_0).capture_list_id);
                (*match_0).captures = (*captures_0).contents;
                (*match_0).capture_count = (*captures_0).size as uint16_t;
                *capture_index =
                    (*state_0).consumed_capture_count() as uint32_t;
                (*state_0).set_consumed_capture_count((*state_0).consumed_capture_count()
                                                          + 1);
                return 1 as libc::c_int != 0
            }
            if capture_list_pool_is_empty(&mut (*self_0).capture_list_pool) {
                capture_list_pool_release(&mut (*self_0).capture_list_pool,
                                          (*(*self_0).states.contents.offset(first_unfinished_state_index
                                                                                 as
                                                                                 isize)).capture_list_id);
                array__erase(&mut (*self_0).states as *mut C2RustUnnamed_19 as
                                 *mut VoidArray,
                             ::std::mem::size_of::<QueryState>() as
                                 libc::c_ulong, first_unfinished_state_index);
            }
        }
        // If there are no finished matches that are ready to be returned, then
    // continue finding more matches.
        if !ts_query_cursor__advance(self_0) { return 0 as libc::c_int != 0 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_set_point_range(mut self_0:
                                                             *mut TSQueryCursor,
                                                         mut start_point:
                                                             TSPoint,
                                                         mut end_point:
                                                             TSPoint) {
    if end_point.row == 0 as libc::c_int as libc::c_uint &&
           end_point.column == 0 as libc::c_int as libc::c_uint {
        start_point =
            {
                let mut init =
                    TSPoint{row: 0 as libc::c_int as uint32_t,
                            column: 0 as libc::c_int as uint32_t,};
                init
            };
        end_point =
            {
                let mut init =
                    TSPoint{row: 4294967295 as libc::c_uint,
                            column: 4294967295 as libc::c_uint,};
                init
            }
    }
    (*self_0).start_point = start_point;
    (*self_0).end_point = end_point;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_next_match(mut self_0:
                                                        *mut TSQueryCursor,
                                                    mut match_0:
                                                        *mut TSQueryMatch)
 -> bool {
    if (*self_0).finished_states.size == 0 as libc::c_int as libc::c_uint {
        if !ts_query_cursor__advance(self_0) { return 0 as libc::c_int != 0 }
    }
    let mut state: *mut QueryState =
        &mut *(*self_0).finished_states.contents.offset(0 as libc::c_int as
                                                            isize) as
            *mut QueryState;
    (*match_0).id = (*state).id;
    (*match_0).pattern_index = (*state).pattern_index;
    let mut captures: *const CaptureList =
        capture_list_pool_get(&mut (*self_0).capture_list_pool,
                              (*state).capture_list_id);
    (*match_0).captures = (*captures).contents;
    (*match_0).capture_count = (*captures).size as uint16_t;
    capture_list_pool_release(&mut (*self_0).capture_list_pool,
                              (*state).capture_list_id);
    array__erase(&mut (*self_0).finished_states as *mut C2RustUnnamed_18 as
                     *mut VoidArray,
                 ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                 0 as libc::c_int as uint32_t);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn stack_node_retain(mut self_0: *mut StackNode) {
    if self_0.is_null() { return }
    if (*self_0).ref_count > 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"self->ref_count > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      89 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void stack_node_retain(StackNode *)\x00")).as_ptr());
    }
    (*self_0).ref_count = (*self_0).ref_count.wrapping_add(1);
    if (*self_0).ref_count != 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"self->ref_count != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      91 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 36],
                                                &[libc::c_char; 36]>(b"void stack_node_retain(StackNode *)\x00")).as_ptr());
    };
}
unsafe extern "C" fn stack_node_release(mut self_0: *mut StackNode,
                                        mut pool: *mut StackNodeArray,
                                        mut subtree_pool: *mut SubtreePool) {
    loop  {
        if (*self_0).ref_count != 0 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(b"self->ref_count != 0\x00" as *const u8 as
                              *const libc::c_char,
                          b"lib/src/./stack.c\x00" as *const u8 as
                              *const libc::c_char,
                          96 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 70],
                                                    &[libc::c_char; 70]>(b"void stack_node_release(StackNode *, StackNodeArray *, SubtreePool *)\x00")).as_ptr());
        }
        (*self_0).ref_count = (*self_0).ref_count.wrapping_sub(1);
        if (*self_0).ref_count > 0 as libc::c_int as libc::c_uint { return }
        let mut first_predecessor: *mut StackNode = 0 as *mut StackNode;
        if (*self_0).link_count as libc::c_int > 0 as libc::c_int {
            let mut i: libc::c_uint =
                ((*self_0).link_count as libc::c_int - 1 as libc::c_int) as
                    libc::c_uint;
            while i > 0 as libc::c_int as libc::c_uint {
                let mut link: StackLink = (*self_0).links[i as usize];
                if !link.subtree.ptr.is_null() {
                    ts_subtree_release(subtree_pool, link.subtree);
                }
                stack_node_release(link.node, pool, subtree_pool);
                i = i.wrapping_sub(1)
            }
            let mut link_0: StackLink =
                (*self_0).links[0 as libc::c_int as usize];
            if !link_0.subtree.ptr.is_null() {
                ts_subtree_release(subtree_pool, link_0.subtree);
            }
            first_predecessor =
                (*self_0).links[0 as libc::c_int as usize].node
        }
        if (*pool).size < 50 as libc::c_int as libc::c_uint {
            array__grow(pool as *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<*mut StackNode>() as
                            libc::c_ulong);
            let fresh53 = (*pool).size;
            (*pool).size = (*pool).size.wrapping_add(1);
            let ref mut fresh54 = *(*pool).contents.offset(fresh53 as isize);
            *fresh54 = self_0
        } else { ts_free(self_0 as *mut libc::c_void); }
        if first_predecessor.is_null() { break ; }
        self_0 = first_predecessor
    };
}
unsafe extern "C" fn stack_node_new(mut previous_node: *mut StackNode,
                                    mut subtree: Subtree,
                                    mut is_pending: bool,
                                    mut state: TSStateId,
                                    mut pool: *mut StackNodeArray)
 -> *mut StackNode {
    let mut node: *mut StackNode =
        if (*pool).size > 0 as libc::c_int as libc::c_uint {
            (*pool).size = (*pool).size.wrapping_sub(1);
            *(*pool).contents.offset((*pool).size as isize) as
                *mut libc::c_void
        } else {
            ts_malloc(::std::mem::size_of::<StackNode>() as libc::c_ulong)
        } as *mut StackNode;
    *node =
        {
            let mut init =
                StackNode{state: state,
                          position:
                              Length{bytes: 0,
                                     extent: TSPoint{row: 0, column: 0,},},
                          links:
                              [StackLink{node: 0 as *mut StackNode,
                                         subtree:
                                             Subtree{data:
                                                         SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                                                               [0;
                                                                                   1],
                                                                           symbol:
                                                                               0,
                                                                           padding_bytes:
                                                                               0,
                                                                           size_bytes:
                                                                               0,
                                                                           padding_columns:
                                                                               0,
                                                                           padding_rows_lookahead_bytes:
                                                                               [0;
                                                                                   1],
                                                                           parse_state:
                                                                               0,},},
                                         is_pending: false,}; 8],
                          link_count: 0 as libc::c_int as libc::c_ushort,
                          ref_count: 1 as libc::c_int as uint32_t,
                          error_cost: 0,
                          node_count: 0,
                          dynamic_precedence: 0,};
            init
        };
    if !previous_node.is_null() {
        (*node).link_count = 1 as libc::c_int as libc::c_ushort;
        (*node).links[0 as libc::c_int as usize] =
            {
                let mut init =
                    StackLink{node: previous_node,
                              subtree: subtree,
                              is_pending: is_pending,};
                init
            };
        (*node).position = (*previous_node).position;
        (*node).error_cost = (*previous_node).error_cost;
        (*node).dynamic_precedence = (*previous_node).dynamic_precedence;
        (*node).node_count = (*previous_node).node_count;
        if !subtree.ptr.is_null() {
            (*node).error_cost =
                (*node).error_cost.wrapping_add(ts_subtree_error_cost(subtree));
            (*node).position =
                length_add((*node).position, ts_subtree_total_size(subtree));
            (*node).node_count =
                (*node).node_count.wrapping_add(ts_subtree_node_count(subtree));
            (*node).dynamic_precedence +=
                ts_subtree_dynamic_precedence(subtree)
        }
    } else {
        (*node).position = length_zero();
        (*node).error_cost = 0 as libc::c_int as libc::c_uint
    }
    return node;
}
unsafe extern "C" fn stack__subtree_is_equivalent(mut left: Subtree,
                                                  mut right: Subtree)
 -> bool {
    return left.ptr == right.ptr ||
               !left.ptr.is_null() && !right.ptr.is_null() &&
                   ts_subtree_symbol(left) as libc::c_int ==
                       ts_subtree_symbol(right) as libc::c_int &&
                   (ts_subtree_error_cost(left) >
                        0 as libc::c_int as libc::c_uint &&
                        ts_subtree_error_cost(right) >
                            0 as libc::c_int as libc::c_uint ||
                        ts_subtree_padding(left).bytes ==
                            ts_subtree_padding(right).bytes &&
                            ts_subtree_size(left).bytes ==
                                ts_subtree_size(right).bytes &&
                            ts_subtree_child_count(left) ==
                                ts_subtree_child_count(right) &&
                            ts_subtree_extra(left) as libc::c_int ==
                                ts_subtree_extra(right) as libc::c_int &&
                            ts_subtree_external_scanner_state_eq(left, right)
                                as libc::c_int != 0);
}
unsafe extern "C" fn stack_node_add_link(mut self_0: *mut StackNode,
                                         mut link: StackLink,
                                         mut subtree_pool: *mut SubtreePool) {
    if link.node == self_0 { return }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*self_0).link_count as libc::c_int {
        let mut existing_link: *mut StackLink =
            &mut *(*self_0).links.as_mut_ptr().offset(i as isize) as
                *mut StackLink;
        if stack__subtree_is_equivalent((*existing_link).subtree,
                                        link.subtree) {
            // In general, we preserve ambiguities until they are removed from the stack
      // during a pop operation where multiple paths lead to the same node. But in
      // the special case where two links directly connect the same pair of nodes,
      // we can safely remove the ambiguity ahead of time without changing behavior.
            if (*existing_link).node == link.node {
                if ts_subtree_dynamic_precedence(link.subtree) >
                       ts_subtree_dynamic_precedence((*existing_link).subtree)
                   {
                    ts_subtree_retain(link.subtree);
                    ts_subtree_release(subtree_pool,
                                       (*existing_link).subtree);
                    (*existing_link).subtree = link.subtree;
                    (*self_0).dynamic_precedence =
                        (*link.node).dynamic_precedence +
                            ts_subtree_dynamic_precedence(link.subtree)
                }
                return
            }
            // If the previous nodes are mergeable, merge them recursively.
            if (*(*existing_link).node).state as libc::c_int ==
                   (*link.node).state as libc::c_int &&
                   (*(*existing_link).node).position.bytes ==
                       (*link.node).position.bytes {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < (*link.node).link_count as libc::c_int {
                    stack_node_add_link((*existing_link).node,
                                        (*link.node).links[j as usize],
                                        subtree_pool);
                    j += 1
                }
                let mut dynamic_precedence: int32_t =
                    (*link.node).dynamic_precedence;
                if !link.subtree.ptr.is_null() {
                    dynamic_precedence +=
                        ts_subtree_dynamic_precedence(link.subtree)
                }
                if dynamic_precedence > (*self_0).dynamic_precedence {
                    (*self_0).dynamic_precedence = dynamic_precedence
                }
                return
            }
        }
        i += 1
    }
    if (*self_0).link_count as libc::c_int == 8 as libc::c_int { return }
    stack_node_retain(link.node);
    let mut node_count: libc::c_uint = (*link.node).node_count;
    let mut dynamic_precedence_0: libc::c_int =
        (*link.node).dynamic_precedence;
    let fresh55 = (*self_0).link_count;
    (*self_0).link_count = (*self_0).link_count.wrapping_add(1);
    (*self_0).links[fresh55 as usize] = link;
    if !link.subtree.ptr.is_null() {
        ts_subtree_retain(link.subtree);
        node_count =
            node_count.wrapping_add(ts_subtree_node_count(link.subtree));
        dynamic_precedence_0 += ts_subtree_dynamic_precedence(link.subtree)
    }
    if node_count > (*self_0).node_count { (*self_0).node_count = node_count }
    if dynamic_precedence_0 > (*self_0).dynamic_precedence {
        (*self_0).dynamic_precedence = dynamic_precedence_0
    };
}
unsafe extern "C" fn stack_head_delete(mut self_0: *mut StackHead,
                                       mut pool: *mut StackNodeArray,
                                       mut subtree_pool: *mut SubtreePool) {
    if !(*self_0).node.is_null() {
        if !(*self_0).last_external_token.ptr.is_null() {
            ts_subtree_release(subtree_pool, (*self_0).last_external_token);
        }
        if !(*self_0).summary.is_null() {
            array__delete((*self_0).summary as *mut VoidArray);
            ts_free((*self_0).summary as *mut libc::c_void);
        }
        stack_node_release((*self_0).node, pool, subtree_pool);
    };
}
unsafe extern "C" fn ts_stack__add_version(mut self_0: *mut Stack,
                                           mut original_version: StackVersion,
                                           mut node: *mut StackNode)
 -> StackVersion {
    let mut head: StackHead =
        {
            let mut init =
                StackHead{node: node,
                          last_external_token:
                              (*(*self_0).heads.contents.offset(original_version
                                                                    as
                                                                    isize)).last_external_token,
                          summary: 0 as *mut StackSummary,
                          node_count_at_last_error:
                              (*(*self_0).heads.contents.offset(original_version
                                                                    as
                                                                    isize)).node_count_at_last_error,
                          lookahead_when_paused: 0 as libc::c_int as TSSymbol,
                          status: StackStatusActive,};
            init
        };
    array__grow(&mut (*self_0).heads as *mut C2RustUnnamed_10 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackHead>() as libc::c_ulong);
    let fresh56 = (*self_0).heads.size;
    (*self_0).heads.size = (*self_0).heads.size.wrapping_add(1);
    *(*self_0).heads.contents.offset(fresh56 as isize) = head;
    stack_node_retain(node);
    if !head.last_external_token.ptr.is_null() {
        ts_subtree_retain(head.last_external_token);
    }
    return (*self_0).heads.size.wrapping_sub(1 as libc::c_int as
                                                 libc::c_uint);
}
unsafe extern "C" fn ts_stack__add_slice(mut self_0: *mut Stack,
                                         mut original_version: StackVersion,
                                         mut node: *mut StackNode,
                                         mut subtrees: *mut SubtreeArray) {
    let mut i: uint32_t =
        (*self_0).slices.size.wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) >
              0 as libc::c_int as libc::c_uint {
        let mut version: StackVersion =
            (*(*self_0).slices.contents.offset(i as isize)).version;
        if (*(*self_0).heads.contents.offset(version as isize)).node == node {
            let mut slice: StackSlice =
                {
                    let mut init =
                        StackSlice{subtrees: *subtrees, version: version,};
                    init
                };
            array__splice(&mut (*self_0).slices as *mut StackSliceArray as
                              *mut VoidArray,
                          ::std::mem::size_of::<StackSlice>() as
                              libc::c_ulong,
                          i.wrapping_add(1 as libc::c_int as libc::c_uint),
                          0 as libc::c_int as uint32_t,
                          1 as libc::c_int as uint32_t,
                          &mut slice as *mut StackSlice as
                              *const libc::c_void);
            return
        }
        i = i.wrapping_sub(1)
    }
    let mut version_0: StackVersion =
        ts_stack__add_version(self_0, original_version, node);
    let mut slice_0: StackSlice =
        {
            let mut init =
                StackSlice{subtrees: *subtrees, version: version_0,};
            init
        };
    array__grow(&mut (*self_0).slices as *mut StackSliceArray as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackSlice>() as libc::c_ulong);
    let fresh57 = (*self_0).slices.size;
    (*self_0).slices.size = (*self_0).slices.size.wrapping_add(1);
    *(*self_0).slices.contents.offset(fresh57 as isize) = slice_0;
}
#[inline(always)]
unsafe extern "C" fn stack__iter(mut self_0: *mut Stack,
                                 mut version: StackVersion,
                                 mut callback: StackCallback,
                                 mut payload: *mut libc::c_void,
                                 mut goal_subtree_count: libc::c_int)
 -> StackSliceArray {
    (*self_0).slices.size = 0 as libc::c_int as uint32_t;
    (*self_0).iterators.size = 0 as libc::c_int as uint32_t;
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      280 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 79],
                                                &[libc::c_char; 79]>(b"StackSliceArray stack__iter(Stack *, StackVersion, StackCallback, void *, int)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    let mut iterator: StackIterator =
        {
            let mut init =
                StackIterator{node: (*head).node,
                              subtrees:
                                  {
                                      let mut init =
                                          SubtreeArray{contents:
                                                           0 as *mut Subtree,
                                                       size:
                                                           0 as libc::c_int as
                                                               uint32_t,
                                                       capacity:
                                                           0 as libc::c_int as
                                                               uint32_t,};
                                      init
                                  },
                              subtree_count: 0 as libc::c_int as uint32_t,
                              is_pending: 1 as libc::c_int != 0,};
            init
        };
    let mut include_subtrees: bool = 0 as libc::c_int != 0;
    if goal_subtree_count >= 0 as libc::c_int {
        include_subtrees = 1 as libc::c_int != 0;
        array__reserve(&mut iterator.subtrees as *mut SubtreeArray as
                           *mut VoidArray,
                       ::std::mem::size_of::<Subtree>() as libc::c_ulong,
                       goal_subtree_count as uint32_t);
    }
    array__grow(&mut (*self_0).iterators as *mut C2RustUnnamed_9 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackIterator>() as libc::c_ulong);
    let fresh58 = (*self_0).iterators.size;
    (*self_0).iterators.size = (*self_0).iterators.size.wrapping_add(1);
    *(*self_0).iterators.contents.offset(fresh58 as isize) = iterator;
    while (*self_0).iterators.size > 0 as libc::c_int as libc::c_uint {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut size: uint32_t = (*self_0).iterators.size;
        while i < size {
            let mut iterator_0: *mut StackIterator =
                &mut *(*self_0).iterators.contents.offset(i as isize) as
                    *mut StackIterator;
            let mut node: *mut StackNode = (*iterator_0).node;
            let mut action: StackAction =
                callback.expect("non-null function pointer")(payload,
                                                             iterator_0);
            let mut should_pop: bool =
                action & StackActionPop as libc::c_int as libc::c_uint != 0;
            let mut should_stop: bool =
                action & StackActionStop as libc::c_int as libc::c_uint != 0
                    || (*node).link_count as libc::c_int == 0 as libc::c_int;
            if should_pop {
                let mut subtrees: SubtreeArray = (*iterator_0).subtrees;
                if !should_stop {
                    ts_subtree_array_copy(subtrees, &mut subtrees);
                }
                ts_subtree_array_reverse(&mut subtrees);
                ts_stack__add_slice(self_0, version, node, &mut subtrees);
            }
            if should_stop {
                if !should_pop {
                    ts_subtree_array_delete((*self_0).subtree_pool,
                                            &mut (*iterator_0).subtrees);
                }
                array__erase(&mut (*self_0).iterators as *mut C2RustUnnamed_9
                                 as *mut VoidArray,
                             ::std::mem::size_of::<StackIterator>() as
                                 libc::c_ulong, i);
                i = i.wrapping_sub(1);
                size = size.wrapping_sub(1)
            } else {
                let mut current_block_39: u64;
                let mut j: uint32_t = 1 as libc::c_int as uint32_t;
                while j <= (*node).link_count as libc::c_uint {
                    let mut next_iterator: *mut StackIterator =
                        0 as *mut StackIterator;
                    let mut link: StackLink =
                        StackLink{node: 0 as *mut StackNode,
                                  subtree:
                                      Subtree{data:
                                                  SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                                                        [0;
                                                                            1],
                                                                    symbol: 0,
                                                                    padding_bytes:
                                                                        0,
                                                                    size_bytes:
                                                                        0,
                                                                    padding_columns:
                                                                        0,
                                                                    padding_rows_lookahead_bytes:
                                                                        [0;
                                                                            1],
                                                                    parse_state:
                                                                        0,},},
                                  is_pending: false,};
                    if j == (*node).link_count as libc::c_uint {
                        link = (*node).links[0 as libc::c_int as usize];
                        next_iterator =
                            &mut *(*self_0).iterators.contents.offset(i as
                                                                          isize)
                                as *mut StackIterator;
                        current_block_39 = 11048769245176032998;
                    } else if (*self_0).iterators.size >=
                                  64 as libc::c_int as libc::c_uint {
                        current_block_39 = 7172762164747879670;
                    } else {
                        link = (*node).links[j as usize];
                        let mut current_iterator: StackIterator =
                            *(*self_0).iterators.contents.offset(i as isize);
                        array__grow(&mut (*self_0).iterators as
                                        *mut C2RustUnnamed_9 as
                                        *mut VoidArray,
                                    1 as libc::c_int as size_t,
                                    ::std::mem::size_of::<StackIterator>() as
                                        libc::c_ulong);
                        let fresh59 = (*self_0).iterators.size;
                        (*self_0).iterators.size =
                            (*self_0).iterators.size.wrapping_add(1);
                        *(*self_0).iterators.contents.offset(fresh59 as isize)
                            = current_iterator;
                        if (*self_0).iterators.size.wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                               < (*self_0).iterators.size {
                        } else {
                            __assert_fail(b"(uint32_t)(&self->iterators)->size - 1 < (&self->iterators)->size\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lib/src/./stack.c\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          337 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 79],
                                                                    &[libc::c_char; 79]>(b"StackSliceArray stack__iter(Stack *, StackVersion, StackCallback, void *, int)\x00")).as_ptr());
                        }
                        next_iterator =
                            &mut *(*self_0).iterators.contents.offset((*self_0).iterators.size.wrapping_sub(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                          as
                                                                          isize)
                                as *mut StackIterator;
                        ts_subtree_array_copy((*next_iterator).subtrees,
                                              &mut (*next_iterator).subtrees);
                        current_block_39 = 11048769245176032998;
                    }
                    match current_block_39 {
                        11048769245176032998 => {
                            (*next_iterator).node = link.node;
                            if !link.subtree.ptr.is_null() {
                                if include_subtrees {
                                    array__grow(&mut (*next_iterator).subtrees
                                                    as *mut SubtreeArray as
                                                    *mut VoidArray,
                                                1 as libc::c_int as size_t,
                                                ::std::mem::size_of::<Subtree>()
                                                    as libc::c_ulong);
                                    let fresh60 =
                                        (*next_iterator).subtrees.size;
                                    (*next_iterator).subtrees.size =
                                        (*next_iterator).subtrees.size.wrapping_add(1);
                                    *(*next_iterator).subtrees.contents.offset(fresh60
                                                                                   as
                                                                                   isize)
                                        = link.subtree;
                                    ts_subtree_retain(link.subtree);
                                }
                                if !ts_subtree_extra(link.subtree) {
                                    (*next_iterator).subtree_count =
                                        (*next_iterator).subtree_count.wrapping_add(1);
                                    if !link.is_pending {
                                        (*next_iterator).is_pending =
                                            0 as libc::c_int != 0
                                    }
                                }
                            } else {
                                (*next_iterator).subtree_count =
                                    (*next_iterator).subtree_count.wrapping_add(1);
                                (*next_iterator).is_pending =
                                    0 as libc::c_int != 0
                            }
                        }
                        _ => { }
                    }
                    j = j.wrapping_add(1)
                }
            }
            i = i.wrapping_add(1)
        }
    }
    return (*self_0).slices;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_new(mut subtree_pool: *mut SubtreePool)
 -> *mut Stack {
    let mut self_0: *mut Stack =
        ts_calloc(1 as libc::c_int as size_t,
                  ::std::mem::size_of::<Stack>() as libc::c_ulong) as
            *mut Stack;
    (*self_0).heads.size = 0 as libc::c_int as uint32_t;
    (*self_0).heads.capacity = 0 as libc::c_int as uint32_t;
    (*self_0).heads.contents = 0 as *mut StackHead;
    (*self_0).slices.size = 0 as libc::c_int as uint32_t;
    (*self_0).slices.capacity = 0 as libc::c_int as uint32_t;
    (*self_0).slices.contents = 0 as *mut StackSlice;
    (*self_0).iterators.size = 0 as libc::c_int as uint32_t;
    (*self_0).iterators.capacity = 0 as libc::c_int as uint32_t;
    (*self_0).iterators.contents = 0 as *mut StackIterator;
    (*self_0).node_pool.size = 0 as libc::c_int as uint32_t;
    (*self_0).node_pool.capacity = 0 as libc::c_int as uint32_t;
    (*self_0).node_pool.contents = 0 as *mut *mut StackNode;
    array__reserve(&mut (*self_0).heads as *mut C2RustUnnamed_10 as
                       *mut VoidArray,
                   ::std::mem::size_of::<StackHead>() as libc::c_ulong,
                   4 as libc::c_int as uint32_t);
    array__reserve(&mut (*self_0).slices as *mut StackSliceArray as
                       *mut VoidArray,
                   ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
                   4 as libc::c_int as uint32_t);
    array__reserve(&mut (*self_0).iterators as *mut C2RustUnnamed_9 as
                       *mut VoidArray,
                   ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
                   4 as libc::c_int as uint32_t);
    array__reserve(&mut (*self_0).node_pool as *mut StackNodeArray as
                       *mut VoidArray,
                   ::std::mem::size_of::<*mut StackNode>() as libc::c_ulong,
                   50 as libc::c_int as uint32_t);
    (*self_0).subtree_pool = subtree_pool;
    (*self_0).base_node =
        stack_node_new(0 as *mut StackNode,
                       Subtree{ptr: 0 as *const SubtreeHeapData,},
                       0 as libc::c_int != 0, 1 as libc::c_int as TSStateId,
                       &mut (*self_0).node_pool);
    ts_stack_clear(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_delete(mut self_0: *mut Stack) {
    if !(*self_0).slices.contents.is_null() {
        array__delete(&mut (*self_0).slices as *mut StackSliceArray as
                          *mut VoidArray);
    }
    if !(*self_0).iterators.contents.is_null() {
        array__delete(&mut (*self_0).iterators as *mut C2RustUnnamed_9 as
                          *mut VoidArray);
    }
    stack_node_release((*self_0).base_node, &mut (*self_0).node_pool,
                       (*self_0).subtree_pool);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).heads.size {
        stack_head_delete(&mut *(*self_0).heads.contents.offset(i as isize),
                          &mut (*self_0).node_pool, (*self_0).subtree_pool);
        i = i.wrapping_add(1)
    }
    (*self_0).heads.size = 0 as libc::c_int as uint32_t;
    if !(*self_0).node_pool.contents.is_null() {
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*self_0).node_pool.size {
            ts_free(*(*self_0).node_pool.contents.offset(i_0 as isize) as
                        *mut libc::c_void);
            i_0 = i_0.wrapping_add(1)
        }
        array__delete(&mut (*self_0).node_pool as *mut StackNodeArray as
                          *mut VoidArray);
    }
    array__delete(&mut (*self_0).heads as *mut C2RustUnnamed_10 as
                      *mut VoidArray);
    ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_version_count(mut self_0: *const Stack)
 -> uint32_t {
    return (*self_0).heads.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_state(mut self_0: *const Stack,
                                        mut version: StackVersion)
 -> TSStateId {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      408 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"TSStateId ts_stack_state(const Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(*(&mut *(*self_0).heads.contents.offset(version as isize) as
                    *mut StackHead)).node).state;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_position(mut self_0: *const Stack,
                                           mut version: StackVersion)
 -> Length {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      412 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"Length ts_stack_position(const Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(*(&mut *(*self_0).heads.contents.offset(version as isize) as
                    *mut StackHead)).node).position;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_last_external_token(mut self_0:
                                                          *const Stack,
                                                      mut version:
                                                          StackVersion)
 -> Subtree {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      416 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[libc::c_char; 66]>(b"Subtree ts_stack_last_external_token(const Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(&mut *(*self_0).heads.contents.offset(version as isize) as
                  *mut StackHead)).last_external_token;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_set_last_external_token(mut self_0:
                                                              *mut Stack,
                                                          mut version:
                                                              StackVersion,
                                                          mut token:
                                                              Subtree) {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      420 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 70],
                                                &[libc::c_char; 70]>(b"void ts_stack_set_last_external_token(Stack *, StackVersion, Subtree)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    if !token.ptr.is_null() { ts_subtree_retain(token); }
    if !(*head).last_external_token.ptr.is_null() {
        ts_subtree_release((*self_0).subtree_pool,
                           (*head).last_external_token);
    }
    (*head).last_external_token = token;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_error_cost(mut self_0: *const Stack,
                                             mut version: StackVersion)
 -> libc::c_uint {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      427 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 62],
                                                &[libc::c_char; 62]>(b"unsigned int ts_stack_error_cost(const Stack *, StackVersion)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    let mut result: libc::c_uint = (*(*head).node).error_cost;
    if (*head).status as libc::c_uint ==
           StackStatusPaused as libc::c_int as libc::c_uint ||
           (*(*head).node).state as libc::c_int == 0 as libc::c_int &&
               (*(*head).node).links[0 as libc::c_int as
                                         usize].subtree.ptr.is_null() {
        result = result.wrapping_add(500 as libc::c_int as libc::c_uint)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_node_count_since_error(mut self_0:
                                                             *const Stack,
                                                         mut version:
                                                             StackVersion)
 -> libc::c_uint {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      438 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 74],
                                                &[libc::c_char; 74]>(b"unsigned int ts_stack_node_count_since_error(const Stack *, StackVersion)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    if (*(*head).node).node_count < (*head).node_count_at_last_error {
        (*head).node_count_at_last_error = (*(*head).node).node_count
    }
    return (*(*head).node).node_count.wrapping_sub((*head).node_count_at_last_error);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_push(mut self_0: *mut Stack,
                                       mut version: StackVersion,
                                       mut subtree: Subtree,
                                       mut pending: bool,
                                       mut state: TSStateId) {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      447 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[libc::c_char; 69]>(b"void ts_stack_push(Stack *, StackVersion, Subtree, _Bool, TSStateId)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    let mut new_node: *mut StackNode =
        stack_node_new((*head).node, subtree, pending, state,
                       &mut (*self_0).node_pool);
    if subtree.ptr.is_null() {
        (*head).node_count_at_last_error = (*new_node).node_count
    }
    (*head).node = new_node;
}
#[inline(always)]
unsafe extern "C" fn iterate_callback(mut payload: *mut libc::c_void,
                                      mut iterator: *const StackIterator)
 -> StackAction {
    let mut session: *mut StackIterateSession =
        payload as *mut StackIterateSession;
    (*session).callback.expect("non-null function pointer")((*session).payload,
                                                            (*(*iterator).node).state,
                                                            (*iterator).subtree_count);
    return StackActionNone as libc::c_int as StackAction;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_iterate(mut self_0: *mut Stack,
                                          mut version: StackVersion,
                                          mut callback: StackIterateCallback,
                                          mut payload: *mut libc::c_void) {
    let mut session: StackIterateSession =
        {
            let mut init =
                StackIterateSession{payload: payload, callback: callback,};
            init
        };
    stack__iter(self_0, version,
                Some(iterate_callback as
                         unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *const StackIterator)
                             -> StackAction),
                &mut session as *mut StackIterateSession as *mut libc::c_void,
                -(1 as libc::c_int));
}
#[inline(always)]
unsafe extern "C" fn pop_count_callback(mut payload: *mut libc::c_void,
                                        mut iterator: *const StackIterator)
 -> StackAction {
    let mut goal_subtree_count: *mut libc::c_uint =
        payload as *mut libc::c_uint;
    if (*iterator).subtree_count == *goal_subtree_count {
        return (StackActionPop as libc::c_int |
                    StackActionStop as libc::c_int) as StackAction
    } else { return StackActionNone as libc::c_int as StackAction };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_count(mut self_0: *mut Stack,
                                            mut version: StackVersion,
                                            mut count: uint32_t)
 -> StackSliceArray {
    return stack__iter(self_0, version,
                       Some(pop_count_callback as
                                unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: *const StackIterator)
                                    -> StackAction),
                       &mut count as *mut uint32_t as *mut libc::c_void,
                       count as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn pop_pending_callback(mut payload: *mut libc::c_void,
                                          mut iterator: *const StackIterator)
 -> StackAction {
    if (*iterator).subtree_count >= 1 as libc::c_int as libc::c_uint {
        if (*iterator).is_pending {
            return (StackActionPop as libc::c_int |
                        StackActionStop as libc::c_int) as StackAction
        } else { return StackActionStop as libc::c_int as StackAction }
    } else { return StackActionNone as libc::c_int as StackAction };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_pending(mut self_0: *mut Stack,
                                              mut version: StackVersion)
 -> StackSliceArray {
    let mut pop: StackSliceArray =
        stack__iter(self_0, version,
                    Some(pop_pending_callback as
                             unsafe extern "C" fn(_: *mut libc::c_void,
                                                  _: *const StackIterator)
                                 -> StackAction), 0 as *mut libc::c_void,
                    0 as libc::c_int);
    if pop.size > 0 as libc::c_int as libc::c_uint {
        ts_stack_renumber_version(self_0,
                                  (*pop.contents.offset(0 as libc::c_int as
                                                            isize)).version,
                                  version);
        (*pop.contents.offset(0 as libc::c_int as isize)).version = version
    }
    return pop;
}
#[inline(always)]
unsafe extern "C" fn pop_error_callback(mut payload: *mut libc::c_void,
                                        mut iterator: *const StackIterator)
 -> StackAction {
    if (*iterator).subtrees.size > 0 as libc::c_int as libc::c_uint {
        let mut found_error: *mut bool = payload as *mut bool;
        if !*found_error &&
               ts_subtree_is_error(*(*iterator).subtrees.contents.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize))
                   as libc::c_int != 0 {
            *found_error = 1 as libc::c_int != 0;
            return (StackActionPop as libc::c_int |
                        StackActionStop as libc::c_int) as StackAction
        } else { return StackActionStop as libc::c_int as StackAction }
    } else { return StackActionNone as libc::c_int as StackAction };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_error(mut self_0: *mut Stack,
                                            mut version: StackVersion)
 -> SubtreeArray {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      519 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"SubtreeArray ts_stack_pop_error(Stack *, StackVersion)\x00")).as_ptr());
    }
    let mut node: *mut StackNode =
        (*(&mut *(*self_0).heads.contents.offset(version as isize) as
               *mut StackHead)).node;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*node).link_count as libc::c_uint {
        if !(*node).links[i as usize].subtree.ptr.is_null() &&
               ts_subtree_is_error((*node).links[i as usize].subtree) as
                   libc::c_int != 0 {
            let mut found_error: bool = 0 as libc::c_int != 0;
            let mut pop: StackSliceArray =
                stack__iter(self_0, version,
                            Some(pop_error_callback as
                                     unsafe extern "C" fn(_:
                                                              *mut libc::c_void,
                                                          _:
                                                              *const StackIterator)
                                         -> StackAction),
                            &mut found_error as *mut bool as
                                *mut libc::c_void, 1 as libc::c_int);
            if pop.size > 0 as libc::c_int as libc::c_uint {
                if pop.size == 1 as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(b"pop.size == 1\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"lib/src/./stack.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  525 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 55],
                                                            &[libc::c_char; 55]>(b"SubtreeArray ts_stack_pop_error(Stack *, StackVersion)\x00")).as_ptr());
                }
                ts_stack_renumber_version(self_0,
                                          (*pop.contents.offset(0 as
                                                                    libc::c_int
                                                                    as
                                                                    isize)).version,
                                          version);
                return (*pop.contents.offset(0 as libc::c_int as
                                                 isize)).subtrees
            }
            break ;
        } else { i = i.wrapping_add(1) }
    }
    return {
               let mut init =
                   SubtreeArray{contents: 0 as *mut Subtree,
                                size: 0 as libc::c_int as uint32_t,
                                capacity: 0,};
               init
           };
}
#[inline(always)]
unsafe extern "C" fn pop_all_callback(mut payload: *mut libc::c_void,
                                      mut iterator: *const StackIterator)
 -> StackAction {
    return if (*(*iterator).node).link_count as libc::c_int ==
                  0 as libc::c_int {
               StackActionPop as libc::c_int
           } else { StackActionNone as libc::c_int } as StackAction;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_all(mut self_0: *mut Stack,
                                          mut version: StackVersion)
 -> StackSliceArray {
    return stack__iter(self_0, version,
                       Some(pop_all_callback as
                                unsafe extern "C" fn(_: *mut libc::c_void,
                                                     _: *const StackIterator)
                                    -> StackAction), 0 as *mut libc::c_void,
                       0 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn summarize_stack_callback(mut payload: *mut libc::c_void,
                                              mut iterator:
                                                  *const StackIterator)
 -> StackAction {
    let mut session: *mut SummarizeStackSession =
        payload as *mut SummarizeStackSession;
    let mut state: TSStateId = (*(*iterator).node).state;
    let mut depth: libc::c_uint = (*iterator).subtree_count;
    if depth > (*session).max_depth {
        return StackActionStop as libc::c_int as StackAction
    }
    let mut i: libc::c_uint =
        (*(*session).summary).size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) >
              0 as libc::c_int as libc::c_uint {
        let mut entry: StackSummaryEntry =
            *(*(*session).summary).contents.offset(i as isize);
        if entry.depth < depth { break ; }
        if entry.depth == depth &&
               entry.state as libc::c_int == state as libc::c_int {
            return StackActionNone as libc::c_int as StackAction
        }
        i = i.wrapping_sub(1)
    }
    array__grow((*session).summary as *mut VoidArray,
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackSummaryEntry>() as libc::c_ulong);
    let fresh61 = (*(*session).summary).size;
    (*(*session).summary).size = (*(*session).summary).size.wrapping_add(1);
    *(*(*session).summary).contents.offset(fresh61 as isize) =
        {
            let mut init =
                StackSummaryEntry{position: (*(*iterator).node).position,
                                  depth: depth,
                                  state: state,};
            init
        };
    return StackActionNone as libc::c_int as StackAction;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_record_summary(mut self_0: *mut Stack,
                                                 mut version: StackVersion,
                                                 mut max_depth:
                                                     libc::c_uint) {
    let mut session: SummarizeStackSession =
        {
            let mut init =
                SummarizeStackSession{summary:
                                          ts_malloc(::std::mem::size_of::<StackSummary>()
                                                        as libc::c_ulong) as
                                              *mut StackSummary,
                                      max_depth: max_depth,};
            init
        };
    (*session.summary).size = 0 as libc::c_int as uint32_t;
    (*session.summary).capacity = 0 as libc::c_int as uint32_t;
    (*session.summary).contents = 0 as *mut StackSummaryEntry;
    stack__iter(self_0, version,
                Some(summarize_stack_callback as
                         unsafe extern "C" fn(_: *mut libc::c_void,
                                              _: *const StackIterator)
                             -> StackAction),
                &mut session as *mut SummarizeStackSession as
                    *mut libc::c_void, -(1 as libc::c_int));
    let ref mut fresh62 =
        (*(*self_0).heads.contents.offset(version as isize)).summary;
    *fresh62 = session.summary;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_get_summary(mut self_0: *mut Stack,
                                              mut version: StackVersion)
 -> *mut StackSummary {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      578 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"StackSummary *ts_stack_get_summary(Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(&mut *(*self_0).heads.contents.offset(version as isize) as
                  *mut StackHead)).summary;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_dynamic_precedence(mut self_0: *mut Stack,
                                                     mut version:
                                                         StackVersion)
 -> libc::c_int {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      582 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 55],
                                                &[libc::c_char; 55]>(b"int ts_stack_dynamic_precedence(Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(*(&mut *(*self_0).heads.contents.offset(version as isize) as
                    *mut StackHead)).node).dynamic_precedence;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_has_advanced_since_error(mut self_0:
                                                               *const Stack,
                                                           mut version:
                                                               StackVersion)
 -> bool {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      586 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[libc::c_char; 69]>(b"_Bool ts_stack_has_advanced_since_error(const Stack *, StackVersion)\x00")).as_ptr());
    }
    let mut head: *const StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    let mut node: *const StackNode = (*head).node;
    if (*node).error_cost == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0
    }
    while !node.is_null() {
        if !((*node).link_count as libc::c_int > 0 as libc::c_int) { break ; }
        let mut subtree: Subtree =
            (*node).links[0 as libc::c_int as usize].subtree;
        if subtree.ptr.is_null() { break ; }
        if ts_subtree_total_bytes(subtree) > 0 as libc::c_int as libc::c_uint
           {
            return 1 as libc::c_int != 0
        } else {
            if !((*node).node_count > (*head).node_count_at_last_error &&
                     ts_subtree_error_cost(subtree) ==
                         0 as libc::c_int as libc::c_uint) {
                break ;
            }
            node = (*node).links[0 as libc::c_int as usize].node
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_remove_version(mut self_0: *mut Stack,
                                                 mut version: StackVersion) {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      610 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 52],
                                                &[libc::c_char; 52]>(b"void ts_stack_remove_version(Stack *, StackVersion)\x00")).as_ptr());
    }
    stack_head_delete(&mut *(*self_0).heads.contents.offset(version as isize),
                      &mut (*self_0).node_pool, (*self_0).subtree_pool);
    array__erase(&mut (*self_0).heads as *mut C2RustUnnamed_10 as
                     *mut VoidArray,
                 ::std::mem::size_of::<StackHead>() as libc::c_ulong,
                 version);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_renumber_version(mut self_0: *mut Stack,
                                                   mut v1: StackVersion,
                                                   mut v2: StackVersion) {
    if v1 == v2 { return }
    if v2 < v1 {
    } else {
        __assert_fail(b"v2 < v1\x00" as *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      616 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"void ts_stack_renumber_version(Stack *, StackVersion, StackVersion)\x00")).as_ptr());
    }
    if v1 < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)v1 < self->heads.size\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      617 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 68],
                                                &[libc::c_char; 68]>(b"void ts_stack_renumber_version(Stack *, StackVersion, StackVersion)\x00")).as_ptr());
    }
    let mut source_head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(v1 as isize) as *mut StackHead;
    let mut target_head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(v2 as isize) as *mut StackHead;
    if !(*target_head).summary.is_null() && (*source_head).summary.is_null() {
        (*source_head).summary = (*target_head).summary;
        (*target_head).summary = 0 as *mut StackSummary
    }
    stack_head_delete(target_head, &mut (*self_0).node_pool,
                      (*self_0).subtree_pool);
    *target_head = *source_head;
    array__erase(&mut (*self_0).heads as *mut C2RustUnnamed_10 as
                     *mut VoidArray,
                 ::std::mem::size_of::<StackHead>() as libc::c_ulong, v1);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_swap_versions(mut self_0: *mut Stack,
                                                mut v1: StackVersion,
                                                mut v2: StackVersion) {
    let mut temporary_head: StackHead =
        *(*self_0).heads.contents.offset(v1 as isize);
    *(*self_0).heads.contents.offset(v1 as isize) =
        *(*self_0).heads.contents.offset(v2 as isize);
    *(*self_0).heads.contents.offset(v2 as isize) = temporary_head;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_copy_version(mut self_0: *mut Stack,
                                               mut version: StackVersion)
 -> StackVersion {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"version < self->heads.size\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      636 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"StackVersion ts_stack_copy_version(Stack *, StackVersion)\x00")).as_ptr());
    }
    array__grow(&mut (*self_0).heads as *mut C2RustUnnamed_10 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackHead>() as libc::c_ulong);
    let fresh63 = (*self_0).heads.size;
    (*self_0).heads.size = (*self_0).heads.size.wrapping_add(1);
    *(*self_0).heads.contents.offset(fresh63 as isize) =
        *(*self_0).heads.contents.offset(version as isize);
    if (*self_0).heads.size.wrapping_sub(1 as libc::c_int as libc::c_uint) <
           (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->heads)->size - 1 < (&self->heads)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      638 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"StackVersion ts_stack_copy_version(Stack *, StackVersion)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset((*self_0).heads.size.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                  as isize) as *mut StackHead;
    stack_node_retain((*head).node);
    if !(*head).last_external_token.ptr.is_null() {
        ts_subtree_retain((*head).last_external_token);
    }
    (*head).summary = 0 as *mut StackSummary;
    return (*self_0).heads.size.wrapping_sub(1 as libc::c_int as
                                                 libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_merge(mut self_0: *mut Stack,
                                        mut version1: StackVersion,
                                        mut version2: StackVersion) -> bool {
    if !ts_stack_can_merge(self_0, version1, version2) {
        return 0 as libc::c_int != 0
    }
    let mut head1: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version1 as isize) as
            *mut StackHead;
    let mut head2: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version2 as isize) as
            *mut StackHead;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*head2).node).link_count as libc::c_uint {
        stack_node_add_link((*head1).node, (*(*head2).node).links[i as usize],
                            (*self_0).subtree_pool);
        i = i.wrapping_add(1)
    }
    if (*(*head1).node).state as libc::c_int == 0 as libc::c_int {
        (*head1).node_count_at_last_error = (*(*head1).node).node_count
    }
    ts_stack_remove_version(self_0, version2);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_can_merge(mut self_0: *mut Stack,
                                            mut version1: StackVersion,
                                            mut version2: StackVersion)
 -> bool {
    let mut head1: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version1 as isize) as
            *mut StackHead;
    let mut head2: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version2 as isize) as
            *mut StackHead;
    return (*head1).status as libc::c_uint ==
               StackStatusActive as libc::c_int as libc::c_uint &&
               (*head2).status as libc::c_uint ==
                   StackStatusActive as libc::c_int as libc::c_uint &&
               (*(*head1).node).state as libc::c_int ==
                   (*(*head2).node).state as libc::c_int &&
               (*(*head1).node).position.bytes ==
                   (*(*head2).node).position.bytes &&
               (*(*head1).node).error_cost == (*(*head2).node).error_cost &&
               ts_subtree_external_scanner_state_eq((*head1).last_external_token,
                                                    (*head2).last_external_token)
                   as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_halt(mut self_0: *mut Stack,
                                       mut version: StackVersion) {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      672 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 42],
                                                &[libc::c_char; 42]>(b"void ts_stack_halt(Stack *, StackVersion)\x00")).as_ptr());
    }
    (*(&mut *(*self_0).heads.contents.offset(version as isize) as
           *mut StackHead)).status = StackStatusHalted;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pause(mut self_0: *mut Stack,
                                        mut version: StackVersion,
                                        mut lookahead: TSSymbol) {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      676 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 53],
                                                &[libc::c_char; 53]>(b"void ts_stack_pause(Stack *, StackVersion, TSSymbol)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    (*head).status = StackStatusPaused;
    (*head).lookahead_when_paused = lookahead;
    (*head).node_count_at_last_error = (*(*head).node).node_count;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_is_active(mut self_0: *const Stack,
                                            mut version: StackVersion)
 -> bool {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      683 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"_Bool ts_stack_is_active(const Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(&mut *(*self_0).heads.contents.offset(version as isize) as
                  *mut StackHead)).status as libc::c_uint ==
               StackStatusActive as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_is_halted(mut self_0: *const Stack,
                                            mut version: StackVersion)
 -> bool {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      687 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"_Bool ts_stack_is_halted(const Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(&mut *(*self_0).heads.contents.offset(version as isize) as
                  *mut StackHead)).status as libc::c_uint ==
               StackStatusHalted as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_is_paused(mut self_0: *const Stack,
                                            mut version: StackVersion)
 -> bool {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      691 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"_Bool ts_stack_is_paused(const Stack *, StackVersion)\x00")).as_ptr());
    }
    return (*(&mut *(*self_0).heads.contents.offset(version as isize) as
                  *mut StackHead)).status as libc::c_uint ==
               StackStatusPaused as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_resume(mut self_0: *mut Stack,
                                         mut version: StackVersion)
 -> TSSymbol {
    if version < (*self_0).heads.size {
    } else {
        __assert_fail(b"(uint32_t)version < (&self->heads)->size\x00" as
                          *const u8 as *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      695 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"TSSymbol ts_stack_resume(Stack *, StackVersion)\x00")).as_ptr());
    }
    let mut head: *mut StackHead =
        &mut *(*self_0).heads.contents.offset(version as isize) as
            *mut StackHead;
    if (*head).status as libc::c_uint ==
           StackStatusPaused as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"head->status == StackStatusPaused\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./stack.c\x00" as *const u8 as
                          *const libc::c_char,
                      696 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"TSSymbol ts_stack_resume(Stack *, StackVersion)\x00")).as_ptr());
    }
    let mut result: TSSymbol = (*head).lookahead_when_paused;
    (*head).status = StackStatusActive;
    (*head).lookahead_when_paused = 0 as libc::c_int as TSSymbol;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_clear(mut self_0: *mut Stack) {
    stack_node_retain((*self_0).base_node);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).heads.size {
        stack_head_delete(&mut *(*self_0).heads.contents.offset(i as isize),
                          &mut (*self_0).node_pool, (*self_0).subtree_pool);
        i = i.wrapping_add(1)
    }
    (*self_0).heads.size = 0 as libc::c_int as uint32_t;
    array__grow(&mut (*self_0).heads as *mut C2RustUnnamed_10 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackHead>() as libc::c_ulong);
    let fresh64 = (*self_0).heads.size;
    (*self_0).heads.size = (*self_0).heads.size.wrapping_add(1);
    *(*self_0).heads.contents.offset(fresh64 as isize) =
        {
            let mut init =
                StackHead{node: (*self_0).base_node,
                          last_external_token:
                              Subtree{ptr: 0 as *const SubtreeHeapData,},
                          summary: 0 as *mut StackSummary,
                          node_count_at_last_error: 0,
                          lookahead_when_paused: 0 as libc::c_int as TSSymbol,
                          status: StackStatusActive,};
            init
        };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_print_dot_graph(mut self_0: *mut Stack,
                                                  mut language:
                                                      *const TSLanguage,
                                                  mut f: *mut FILE) -> bool {
    array__reserve(&mut (*self_0).iterators as *mut C2RustUnnamed_9 as
                       *mut VoidArray,
                   ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
                   32 as libc::c_int as uint32_t);
    let mut was_recording_allocations: bool =
        ts_toggle_allocation_recording(0 as libc::c_int != 0);
    if f.is_null() { f = stderr }
    fprintf(f, b"digraph stack {\n\x00" as *const u8 as *const libc::c_char);
    fprintf(f, b"rankdir=\"RL\";\n\x00" as *const u8 as *const libc::c_char);
    fprintf(f,
            b"edge [arrowhead=none]\n\x00" as *const u8 as
                *const libc::c_char);
    let mut visited_nodes: C2RustUnnamed_20 =
        {
            let mut init =
                C2RustUnnamed_20{contents: 0 as *mut *mut StackNode,
                                 size: 0 as libc::c_int as uint32_t,
                                 capacity: 0 as libc::c_int as uint32_t,};
            init
        };
    (*self_0).iterators.size = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).heads.size {
        let mut head: *mut StackHead =
            &mut *(*self_0).heads.contents.offset(i as isize) as
                *mut StackHead;
        if !((*head).status as libc::c_uint ==
                 StackStatusHalted as libc::c_int as libc::c_uint) {
            fprintf(f,
                    b"node_head_%u [shape=none, label=\"\"]\n\x00" as
                        *const u8 as *const libc::c_char, i);
            fprintf(f,
                    b"node_head_%u -> node_%p [\x00" as *const u8 as
                        *const libc::c_char, i, (*head).node);
            if (*head).status as libc::c_uint ==
                   StackStatusPaused as libc::c_int as libc::c_uint {
                fprintf(f,
                        b"color=red \x00" as *const u8 as
                            *const libc::c_char);
            }
            fprintf(f,
                    b"label=%u, fontcolor=blue, weight=10000, labeltooltip=\"node_count: %u\nerror_cost: %u\x00"
                        as *const u8 as *const libc::c_char, i,
                    ts_stack_node_count_since_error(self_0, i),
                    ts_stack_error_cost(self_0, i));
            if !(*head).last_external_token.ptr.is_null() {
                let mut state: *const ExternalScannerState =
                    &(*(*head).last_external_token.ptr).c2rust_unnamed.external_scanner_state;
                let mut data: *const libc::c_char =
                    ts_external_scanner_state_data(state);
                fprintf(f,
                        b"\nexternal_scanner_state:\x00" as *const u8 as
                            *const libc::c_char);
                let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                while j < (*state).length {
                    fprintf(f,
                            b" %2X\x00" as *const u8 as *const libc::c_char,
                            *data.offset(j as isize) as libc::c_int);
                    j = j.wrapping_add(1)
                }
            }
            fprintf(f, b"\"]\n\x00" as *const u8 as *const libc::c_char);
            array__grow(&mut (*self_0).iterators as *mut C2RustUnnamed_9 as
                            *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<StackIterator>() as
                            libc::c_ulong);
            let fresh65 = (*self_0).iterators.size;
            (*self_0).iterators.size =
                (*self_0).iterators.size.wrapping_add(1);
            *(*self_0).iterators.contents.offset(fresh65 as isize) =
                {
                    let mut init =
                        StackIterator{node: (*head).node,
                                      subtrees:
                                          SubtreeArray{contents:
                                                           0 as *mut Subtree,
                                                       size: 0,
                                                       capacity: 0,},
                                      subtree_count: 0,
                                      is_pending: false,};
                    init
                }
        }
        i = i.wrapping_add(1)
    }
    let mut all_iterators_done: bool = 0 as libc::c_int != 0;
    while !all_iterators_done {
        all_iterators_done = 1 as libc::c_int != 0;
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*self_0).iterators.size {
            let mut iterator: StackIterator =
                *(*self_0).iterators.contents.offset(i_0 as isize);
            let mut node: *mut StackNode = iterator.node;
            let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
            while j_0 < visited_nodes.size {
                if *visited_nodes.contents.offset(j_0 as isize) == node {
                    node = 0 as *mut StackNode;
                    break ;
                } else { j_0 = j_0.wrapping_add(1) }
            }
            if !node.is_null() {
                all_iterators_done = 0 as libc::c_int != 0;
                fprintf(f,
                        b"node_%p [\x00" as *const u8 as *const libc::c_char,
                        node);
                if (*node).state as libc::c_int == 0 as libc::c_int {
                    fprintf(f,
                            b"label=\"?\"\x00" as *const u8 as
                                *const libc::c_char);
                } else if (*node).link_count as libc::c_int ==
                              1 as libc::c_int &&
                              !(*node).links[0 as libc::c_int as
                                                 usize].subtree.ptr.is_null()
                              &&
                              ts_subtree_extra((*node).links[0 as libc::c_int
                                                                 as
                                                                 usize].subtree)
                                  as libc::c_int != 0 {
                    fprintf(f,
                            b"shape=point margin=0 label=\"\"\x00" as
                                *const u8 as *const libc::c_char);
                } else {
                    fprintf(f,
                            b"label=\"%d\"\x00" as *const u8 as
                                *const libc::c_char,
                            (*node).state as libc::c_int);
                }
                fprintf(f,
                        b" tooltip=\"position: %u,%u\nnode_count:%u\nerror_cost: %u\ndynamic_precedence: %d\"];\n\x00"
                            as *const u8 as *const libc::c_char,
                        (*node).position.extent.row.wrapping_add(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint),
                        (*node).position.extent.column, (*node).node_count,
                        (*node).error_cost, (*node).dynamic_precedence);
                let mut j_1: libc::c_int = 0 as libc::c_int;
                while j_1 < (*node).link_count as libc::c_int {
                    let mut link: StackLink = (*node).links[j_1 as usize];
                    fprintf(f,
                            b"node_%p -> node_%p [\x00" as *const u8 as
                                *const libc::c_char, node, link.node);
                    if link.is_pending {
                        fprintf(f,
                                b"style=dashed \x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    if !link.subtree.ptr.is_null() &&
                           ts_subtree_extra(link.subtree) as libc::c_int != 0
                       {
                        fprintf(f,
                                b"fontcolor=gray \x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    if link.subtree.ptr.is_null() {
                        fprintf(f,
                                b"color=red\x00" as *const u8 as
                                    *const libc::c_char);
                    } else {
                        fprintf(f,
                                b"label=\"\x00" as *const u8 as
                                    *const libc::c_char);
                        let mut quoted: bool =
                            ts_subtree_visible(link.subtree) as libc::c_int !=
                                0 && !ts_subtree_named(link.subtree);
                        if quoted {
                            fprintf(f,
                                    b"\'\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        let mut name: *const libc::c_char =
                            ts_language_symbol_name(language,
                                                    ts_subtree_symbol(link.subtree));
                        let mut c: *const libc::c_char = name;
                        while *c != 0 {
                            if *c as libc::c_int == '\"' as i32 ||
                                   *c as libc::c_int == '\\' as i32 {
                                fprintf(f,
                                        b"\\\x00" as *const u8 as
                                            *const libc::c_char);
                            }
                            fprintf(f,
                                    b"%c\x00" as *const u8 as
                                        *const libc::c_char,
                                    *c as libc::c_int);
                            c = c.offset(1)
                        }
                        if quoted {
                            fprintf(f,
                                    b"\'\x00" as *const u8 as
                                        *const libc::c_char);
                        }
                        fprintf(f,
                                b"\"\x00" as *const u8 as
                                    *const libc::c_char);
                        fprintf(f,
                                b"labeltooltip=\"error_cost: %u\ndynamic_precedence: %u\"\x00"
                                    as *const u8 as *const libc::c_char,
                                ts_subtree_error_cost(link.subtree),
                                ts_subtree_dynamic_precedence(link.subtree));
                    }
                    fprintf(f,
                            b"];\n\x00" as *const u8 as *const libc::c_char);
                    let mut next_iterator: *mut StackIterator =
                        0 as *mut StackIterator;
                    if j_1 == 0 as libc::c_int {
                        next_iterator =
                            &mut *(*self_0).iterators.contents.offset(i_0 as
                                                                          isize)
                                as *mut StackIterator
                    } else {
                        array__grow(&mut (*self_0).iterators as
                                        *mut C2RustUnnamed_9 as
                                        *mut VoidArray,
                                    1 as libc::c_int as size_t,
                                    ::std::mem::size_of::<StackIterator>() as
                                        libc::c_ulong);
                        let fresh66 = (*self_0).iterators.size;
                        (*self_0).iterators.size =
                            (*self_0).iterators.size.wrapping_add(1);
                        *(*self_0).iterators.contents.offset(fresh66 as isize)
                            = iterator;
                        if (*self_0).iterators.size.wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                               < (*self_0).iterators.size {
                        } else {
                            __assert_fail(b"(uint32_t)(&self->iterators)->size - 1 < (&self->iterators)->size\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"lib/src/./stack.c\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          832 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 68],
                                                                    &[libc::c_char; 68]>(b"_Bool ts_stack_print_dot_graph(Stack *, const TSLanguage *, FILE *)\x00")).as_ptr());
                        }
                        next_iterator =
                            &mut *(*self_0).iterators.contents.offset((*self_0).iterators.size.wrapping_sub(1
                                                                                                                as
                                                                                                                libc::c_int
                                                                                                                as
                                                                                                                libc::c_uint)
                                                                          as
                                                                          isize)
                                as *mut StackIterator
                    }
                    (*next_iterator).node = link.node;
                    j_1 += 1
                }
                array__grow(&mut visited_nodes as *mut C2RustUnnamed_20 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<*mut StackNode>() as
                                libc::c_ulong);
                let fresh67 = visited_nodes.size;
                visited_nodes.size = visited_nodes.size.wrapping_add(1);
                let ref mut fresh68 =
                    *visited_nodes.contents.offset(fresh67 as isize);
                *fresh68 = node
            }
            i_0 = i_0.wrapping_add(1)
        }
    }
    fprintf(f, b"}\n\x00" as *const u8 as *const libc::c_char);
    array__delete(&mut visited_nodes as *mut C2RustUnnamed_20 as
                      *mut VoidArray);
    ts_toggle_allocation_recording(was_recording_allocations);
    return 1 as libc::c_int != 0;
}
// ExternalScannerState
// SubtreeArray
// SubtreePool
// Subtree
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_set_symbol(mut self_0:
                                                   *mut MutableSubtree,
                                               mut symbol: TSSymbol,
                                               mut language:
                                                   *const TSLanguage) {
    let mut metadata: TSSymbolMetadata =
        ts_language_symbol_metadata(language, symbol);
    if (*self_0).data.is_inline() {
        if (symbol as libc::c_int) < 255 as libc::c_int {
        } else {
            __assert_fail(b"symbol < UINT8_MAX\x00" as *const u8 as
                              *const libc::c_char,
                          b"lib/src/./subtree.c\x00" as *const u8 as
                              *const libc::c_char,
                          224 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 75],
                                                    &[libc::c_char; 75]>(b"void ts_subtree_set_symbol(MutableSubtree *, TSSymbol, const TSLanguage *)\x00")).as_ptr());
        }
        (*self_0).data.symbol = symbol as uint8_t;
        (*self_0).data.set_named(metadata.named());
        (*self_0).data.set_visible(metadata.visible())
    } else {
        (*(*self_0).ptr).symbol = symbol;
        (*(*self_0).ptr).set_named(metadata.named());
        (*(*self_0).ptr).set_visible(metadata.visible())
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_reverse(mut self_0:
                                                      *mut SubtreeArray) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut limit: uint32_t =
        (*self_0).size.wrapping_div(2 as libc::c_int as libc::c_uint);
    while i < limit {
        let mut reverse_index: size_t =
            (*self_0).size.wrapping_sub(1 as libc::c_int as
                                            libc::c_uint).wrapping_sub(i) as
                size_t;
        let mut swap: Subtree = *(*self_0).contents.offset(i as isize);
        *(*self_0).contents.offset(i as isize) =
            *(*self_0).contents.offset(reverse_index as isize);
        *(*self_0).contents.offset(reverse_index as isize) = swap;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_pool_delete(mut self_0:
                                                    *mut SubtreePool) {
    if !(*self_0).free_trees.contents.is_null() {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*self_0).free_trees.size {
            ts_free((*(*self_0).free_trees.contents.offset(i as isize)).ptr as
                        *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        array__delete(&mut (*self_0).free_trees as *mut MutableSubtreeArray as
                          *mut VoidArray);
    }
    if !(*self_0).tree_stack.contents.is_null() {
        array__delete(&mut (*self_0).tree_stack as *mut MutableSubtreeArray as
                          *mut VoidArray);
    };
}
static mut empty_state: ExternalScannerState =
    {
        let mut init =
            ExternalScannerState{c2rust_unnamed:
                                     C2RustUnnamed_5{short_data:
                                                         [0 as libc::c_int as
                                                              libc::c_char, 0,
                                                          0, 0, 0, 0, 0, 0, 0,
                                                          0, 0, 0, 0, 0, 0, 0,
                                                          0, 0, 0, 0, 0, 0, 0,
                                                          0],},
                                 length: 0 as libc::c_int as uint32_t,};
        init
    };
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_eq(mut a:
                                                          *const ExternalScannerState,
                                                      mut b:
                                                          *const ExternalScannerState)
 -> bool {
    return a == b ||
               (*a).length == (*b).length &&
                   memcmp(ts_external_scanner_state_data(a) as
                              *const libc::c_void,
                          ts_external_scanner_state_data(b) as
                              *const libc::c_void,
                          (*a).length as libc::c_ulong) == 0;
}
unsafe extern "C" fn ts_subtree_pool_allocate(mut self_0: *mut SubtreePool)
 -> *mut SubtreeHeapData {
    if (*self_0).free_trees.size > 0 as libc::c_int as libc::c_uint {
        (*self_0).free_trees.size = (*self_0).free_trees.size.wrapping_sub(1);
        return (*(*self_0).free_trees.contents.offset((*self_0).free_trees.size
                                                          as isize)).ptr
    } else {
        return ts_malloc(::std::mem::size_of::<SubtreeHeapData>() as
                             libc::c_ulong) as *mut SubtreeHeapData
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_init(mut self_0:
                                                            *mut ExternalScannerState,
                                                        mut data:
                                                            *const libc::c_char,
                                                        mut length:
                                                            libc::c_uint) {
    (*self_0).length = length;
    if length as libc::c_ulong >
           ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong {
        (*self_0).c2rust_unnamed.long_data =
            ts_malloc(length as size_t) as *mut libc::c_char;
        memcpy((*self_0).c2rust_unnamed.long_data as *mut libc::c_void,
               data as *const libc::c_void, length as libc::c_ulong);
    } else {
        memcpy((*self_0).c2rust_unnamed.short_data.as_mut_ptr() as
                   *mut libc::c_void, data as *const libc::c_void,
               length as libc::c_ulong);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_leaf(mut pool: *mut SubtreePool,
                                             mut symbol: TSSymbol,
                                             mut padding: Length,
                                             mut size: Length,
                                             mut lookahead_bytes: uint32_t,
                                             mut parse_state: TSStateId,
                                             mut has_external_tokens: bool,
                                             mut is_keyword: bool,
                                             mut language: *const TSLanguage)
 -> Subtree {
    let mut metadata: TSSymbolMetadata =
        ts_language_symbol_metadata(language, symbol);
    let mut extra: bool = symbol as libc::c_int == 0 as libc::c_int;
    let mut is_inline: bool =
        symbol as libc::c_int <= 255 as libc::c_int && !has_external_tokens &&
            ts_subtree_can_inline(padding, size, lookahead_bytes) as
                libc::c_int != 0;
    if is_inline {
        return Subtree{data:
                           {
                               let mut init =
                                   SubtreeInlineData{is_inline_visible_named_extra_has_changes_is_missing_is_keyword:
                                                         [0; 1],
                                                     padding_rows_lookahead_bytes:
                                                         [0; 1],
                                                     symbol:
                                                         symbol as uint8_t,
                                                     padding_bytes:
                                                         padding.bytes as
                                                             uint8_t,
                                                     size_bytes:
                                                         size.bytes as
                                                             uint8_t,
                                                     padding_columns:
                                                         padding.extent.column
                                                             as uint8_t,
                                                     parse_state:
                                                         parse_state,};
                               init.set_is_inline(1 as libc::c_int != 0);
                               init.set_visible(metadata.visible());
                               init.set_named(metadata.named());
                               init.set_extra(extra);
                               init.set_has_changes(0 as libc::c_int != 0);
                               init.set_is_missing(0 as libc::c_int != 0);
                               init.set_is_keyword(is_keyword);
                               init.set_padding_rows(padding.extent.row as
                                                         uint8_t);
                               init.set_lookahead_bytes(lookahead_bytes as
                                                            uint8_t);
                               init
                           },}
    } else {
        let mut data: *mut SubtreeHeapData = ts_subtree_pool_allocate(pool);
        *data =
            {
                let mut init =
                    SubtreeHeapData{visible_named_extra_fragile_left_fragile_right_has_changes_has_external_tokens_is_missing_is_keyword:
                                        [0; 2],
                                    c2rust_padding: [0; 2],
                                    ref_count: 1 as libc::c_int as uint32_t,
                                    padding: padding,
                                    size: size,
                                    lookahead_bytes: lookahead_bytes,
                                    error_cost: 0 as libc::c_int as uint32_t,
                                    child_count: 0 as libc::c_int as uint32_t,
                                    symbol: symbol,
                                    parse_state: parse_state,
                                    c2rust_unnamed:
                                        C2RustUnnamed_4{c2rust_unnamed:
                                                            {
                                                                let mut init =
                                                                    C2RustUnnamed_6{children:
                                                                                        0
                                                                                            as
                                                                                            *mut Subtree,
                                                                                    visible_child_count:
                                                                                        0,
                                                                                    named_child_count:
                                                                                        0,
                                                                                    node_count:
                                                                                        0,
                                                                                    repeat_depth:
                                                                                        0,
                                                                                    dynamic_precedence:
                                                                                        0,
                                                                                    production_id:
                                                                                        0,
                                                                                    first_leaf:
                                                                                        {
                                                                                            let mut init =
                                                                                                C2RustUnnamed_7{symbol:
                                                                                                                    0
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        TSSymbol,
                                                                                                                parse_state:
                                                                                                                    0
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        TSStateId,};
                                                                                            init
                                                                                        },};
                                                                init
                                                            },},};
                init.set_visible(metadata.visible());
                init.set_named(metadata.named());
                init.set_extra(extra);
                init.set_fragile_left(0 as libc::c_int != 0);
                init.set_fragile_right(0 as libc::c_int != 0);
                init.set_has_changes(0 as libc::c_int != 0);
                init.set_has_external_tokens(has_external_tokens);
                init.set_is_missing(0 as libc::c_int != 0);
                init.set_is_keyword(is_keyword);
                init
            };
        return Subtree{ptr: data,}
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_can_inline(mut padding: Length,
                                           mut size: Length,
                                           mut lookahead_bytes: uint32_t)
 -> bool {
    return padding.bytes < 255 as libc::c_int as libc::c_uint &&
               padding.extent.row < 16 as libc::c_int as libc::c_uint &&
               padding.extent.column < 255 as libc::c_int as libc::c_uint &&
               size.extent.row == 0 as libc::c_int as libc::c_uint &&
               size.extent.column < 255 as libc::c_int as libc::c_uint &&
               lookahead_bytes < 16 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_remove_trailing_extras(mut self_0:
                                                                     *mut SubtreeArray)
 -> SubtreeArray {
    let mut result: SubtreeArray =
        {
            let mut init =
                SubtreeArray{contents: 0 as *mut Subtree,
                             size: 0 as libc::c_int as uint32_t,
                             capacity: 0 as libc::c_int as uint32_t,};
            init
        };
    let mut i: uint32_t =
        (*self_0).size.wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) >
              0 as libc::c_int as libc::c_uint {
        let mut child: Subtree = *(*self_0).contents.offset(i as isize);
        if !ts_subtree_extra(child) { break ; }
        array__grow(&mut result as *mut SubtreeArray as *mut VoidArray,
                    1 as libc::c_int as size_t,
                    ::std::mem::size_of::<Subtree>() as libc::c_ulong);
        let fresh69 = result.size;
        result.size = result.size.wrapping_add(1);
        *result.contents.offset(fresh69 as isize) = child;
        i = i.wrapping_sub(1)
    }
    (*self_0).size = i.wrapping_add(1 as libc::c_int as libc::c_uint);
    ts_subtree_array_reverse(&mut result);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_data(mut self_0:
                                                            *const ExternalScannerState)
 -> *const libc::c_char {
    if (*self_0).length as libc::c_ulong >
           ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong {
        return (*self_0).c2rust_unnamed.long_data
    } else { return (*self_0).c2rust_unnamed.short_data.as_ptr() };
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_copy(mut self_0:
                                                            *const ExternalScannerState)
 -> ExternalScannerState {
    let mut result: ExternalScannerState = *self_0;
    if (*self_0).length as libc::c_ulong >
           ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong {
        result.c2rust_unnamed.long_data =
            ts_malloc((*self_0).length as size_t) as *mut libc::c_char;
        memcpy(result.c2rust_unnamed.long_data as *mut libc::c_void,
               (*self_0).c2rust_unnamed.long_data as *const libc::c_void,
               (*self_0).length as libc::c_ulong);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_pool_new(mut capacity: uint32_t)
 -> SubtreePool {
    let mut self_0: SubtreePool =
        {
            let mut init =
                SubtreePool{free_trees:
                                {
                                    let mut init =
                                        MutableSubtreeArray{contents:
                                                                0 as
                                                                    *mut MutableSubtree,
                                                            size:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,
                                                            capacity:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,};
                                    init
                                },
                            tree_stack:
                                {
                                    let mut init =
                                        MutableSubtreeArray{contents:
                                                                0 as
                                                                    *mut MutableSubtree,
                                                            size:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,
                                                            capacity:
                                                                0 as
                                                                    libc::c_int
                                                                    as
                                                                    uint32_t,};
                                    init
                                },};
            init
        };
    array__reserve(&mut self_0.free_trees as *mut MutableSubtreeArray as
                       *mut VoidArray,
                   ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
                   capacity);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_delete(mut pool: *mut SubtreePool,
                                                 mut self_0:
                                                     *mut SubtreeArray) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).size {
        ts_subtree_release(pool, *(*self_0).contents.offset(i as isize));
        i = i.wrapping_add(1)
    }
    array__delete(self_0 as *mut VoidArray);
}
unsafe extern "C" fn ts_subtree_pool_free(mut self_0: *mut SubtreePool,
                                          mut tree: *mut SubtreeHeapData) {
    if (*self_0).free_trees.capacity > 0 as libc::c_int as libc::c_uint &&
           (*self_0).free_trees.size.wrapping_add(1 as libc::c_int as
                                                      libc::c_uint) <=
               32 as libc::c_int as libc::c_uint {
        array__grow(&mut (*self_0).free_trees as *mut MutableSubtreeArray as
                        *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong);
        let fresh70 = (*self_0).free_trees.size;
        (*self_0).free_trees.size = (*self_0).free_trees.size.wrapping_add(1);
        *(*self_0).free_trees.contents.offset(fresh70 as isize) =
            MutableSubtree{ptr: tree,}
    } else { ts_free(tree as *mut libc::c_void); };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_copy(mut self_0: SubtreeArray,
                                               mut dest: *mut SubtreeArray) {
    (*dest).size = self_0.size;
    (*dest).capacity = self_0.capacity;
    (*dest).contents = self_0.contents;
    if self_0.capacity > 0 as libc::c_int as libc::c_uint {
        (*dest).contents =
            ts_calloc(self_0.capacity as size_t,
                      ::std::mem::size_of::<Subtree>() as libc::c_ulong) as
                *mut Subtree;
        memcpy((*dest).contents as *mut libc::c_void,
               self_0.contents as *const libc::c_void,
               (self_0.size as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<Subtree>()
                                                    as libc::c_ulong));
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < self_0.size {
            ts_subtree_retain(*(*dest).contents.offset(i as isize));
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_delete(mut self_0:
                                                              *mut ExternalScannerState) {
    if (*self_0).length as libc::c_ulong >
           ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong {
        ts_free((*self_0).c2rust_unnamed.long_data as *mut libc::c_void);
    };
}
// If the edit is entirely within the space before this subtree, then shift this
    // subtree over according to the edit without changing its size.
// If the edit starts in the space before this subtree and extends into this subtree,
    // shrink the subtree's content to compensate for the change in the space before it.
// If the edit is a pure insertion right at the start of the subtree,
    // shift the subtree over according to the insertion.
// If the edit is within this subtree, resize the subtree to reflect the edit.
// If this child ends before the edit, it is not affected.
// If this child starts after the edit, then we're done processing children.
// Transform edit into the child's coordinate space.
// Clamp child_edit to the child's bounds.
// Interpret all inserted text as applying to the *first* child that touches the edit.
      // Subsequent children are only never have any text inserted into them; they are only
      // shrunk to compensate for the edit.
// Children that occur before the edit are not reshaped by the edit.
// Queue processing of this child's subtree.
unsafe extern "C" fn ts_subtree__write_char_to_string(mut s:
                                                          *mut libc::c_char,
                                                      mut n: size_t,
                                                      mut c: int32_t)
 -> size_t {
    if c == -(1 as libc::c_int) {
        return snprintf(s, n,
                        b"INVALID\x00" as *const u8 as *const libc::c_char) as
                   size_t
    } else if c == '\u{0}' as i32 {
        return snprintf(s, n,
                        b"\'\\0\'\x00" as *const u8 as *const libc::c_char) as
                   size_t
    } else if c == '\n' as i32 {
        return snprintf(s, n,
                        b"\'\\n\'\x00" as *const u8 as *const libc::c_char) as
                   size_t
    } else if c == '\t' as i32 {
        return snprintf(s, n,
                        b"\'\\t\'\x00" as *const u8 as *const libc::c_char) as
                   size_t
    } else if c == '\r' as i32 {
        return snprintf(s, n,
                        b"\'\\r\'\x00" as *const u8 as *const libc::c_char) as
                   size_t
    } else if (0 as libc::c_int) < c && c < 128 as libc::c_int &&
                  *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                      _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
        return snprintf(s, n,
                        b"\'%c\'\x00" as *const u8 as *const libc::c_char, c)
                   as size_t
    } else {
        return snprintf(s, n, b"%d\x00" as *const u8 as *const libc::c_char,
                        c) as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_make_mut(mut pool: *mut SubtreePool,
                                             mut self_0: Subtree)
 -> MutableSubtree {
    if self_0.data.is_inline() { return MutableSubtree{data: self_0.data,} }
    if (*self_0.ptr).ref_count == 1 as libc::c_int as libc::c_uint {
        return ts_subtree_to_mut_unsafe(self_0)
    }
    let mut result: *mut SubtreeHeapData = ts_subtree_pool_allocate(pool);
    memcpy(result as *mut libc::c_void, self_0.ptr as *const libc::c_void,
           ::std::mem::size_of::<SubtreeHeapData>() as libc::c_ulong);
    if (*result).child_count > 0 as libc::c_int as libc::c_uint {
        (*result).c2rust_unnamed.c2rust_unnamed.children =
            ts_calloc((*self_0.ptr).child_count as size_t,
                      ::std::mem::size_of::<Subtree>() as libc::c_ulong) as
                *mut Subtree;
        memcpy((*result).c2rust_unnamed.c2rust_unnamed.children as
                   *mut libc::c_void,
               (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children as
                   *const libc::c_void,
               ((*result).child_count as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<Subtree>()
                                                    as libc::c_ulong));
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*result).child_count {
            ts_subtree_retain(*(*result).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                           as
                                                                                           isize));
            i = i.wrapping_add(1)
        }
    } else if (*result).has_external_tokens() {
        (*result).c2rust_unnamed.external_scanner_state =
            ts_external_scanner_state_copy(&(*self_0.ptr).c2rust_unnamed.external_scanner_state)
    }
    ::std::ptr::write_volatile(&mut (*result).ref_count as *mut uint32_t,
                               1 as libc::c_int as uint32_t);
    ts_subtree_release(pool, self_0);
    return MutableSubtree{ptr: result,};
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_edit(mut self_0: Subtree,
                                         mut edit: *const TSInputEdit,
                                         mut pool: *mut SubtreePool)
 -> Subtree {
    let mut stack: C2RustUnnamed_21 =
        {
            let mut init =
                C2RustUnnamed_21{contents: 0 as *mut StackEntry_0,
                                 size: 0 as libc::c_int as uint32_t,
                                 capacity: 0 as libc::c_int as uint32_t,};
            init
        };
    array__grow(&mut stack as *mut C2RustUnnamed_21 as *mut VoidArray,
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<StackEntry_0>() as libc::c_ulong);
    let fresh71 = stack.size;
    stack.size = stack.size.wrapping_add(1);
    *stack.contents.offset(fresh71 as isize) =
        {
            let mut init =
                StackEntry_0{tree: &mut self_0,
                             edit:
                                 {
                                     let mut init =
                                         Edit{start:
                                                  {
                                                      let mut init =
                                                          Length{bytes:
                                                                     (*edit).start_byte,
                                                                 extent:
                                                                     (*edit).start_point,};
                                                      init
                                                  },
                                              old_end:
                                                  {
                                                      let mut init =
                                                          Length{bytes:
                                                                     (*edit).old_end_byte,
                                                                 extent:
                                                                     (*edit).old_end_point,};
                                                      init
                                                  },
                                              new_end:
                                                  {
                                                      let mut init =
                                                          Length{bytes:
                                                                     (*edit).new_end_byte,
                                                                 extent:
                                                                     (*edit).new_end_point,};
                                                      init
                                                  },};
                                     init
                                 },};
            init
        };
    while stack.size != 0 {
        stack.size = stack.size.wrapping_sub(1);
        let mut entry: StackEntry_0 =
            *stack.contents.offset(stack.size as isize);
        let mut edit_0: Edit = entry.edit;
        let mut is_noop: bool =
            edit_0.old_end.bytes == edit_0.start.bytes &&
                edit_0.new_end.bytes == edit_0.start.bytes;
        let mut is_pure_insertion: bool =
            edit_0.old_end.bytes == edit_0.start.bytes;
        let mut size: Length = ts_subtree_size(*entry.tree);
        let mut padding: Length = ts_subtree_padding(*entry.tree);
        let mut lookahead_bytes: uint32_t =
            ts_subtree_lookahead_bytes(*entry.tree);
        let mut end_byte: uint32_t =
            padding.bytes.wrapping_add(size.bytes).wrapping_add(lookahead_bytes);
        if edit_0.start.bytes > end_byte ||
               is_noop as libc::c_int != 0 && edit_0.start.bytes == end_byte {
            continue ;
        }
        if edit_0.old_end.bytes <= padding.bytes {
            padding =
                length_add(edit_0.new_end,
                           length_sub(padding, edit_0.old_end))
        } else if edit_0.start.bytes < padding.bytes {
            size = length_sub(size, length_sub(edit_0.old_end, padding));
            padding = edit_0.new_end
        } else if edit_0.start.bytes == padding.bytes &&
                      is_pure_insertion as libc::c_int != 0 {
            padding = edit_0.new_end
        } else {
            let mut total_bytes: uint32_t =
                padding.bytes.wrapping_add(size.bytes);
            if edit_0.start.bytes < total_bytes ||
                   edit_0.start.bytes == total_bytes &&
                       is_pure_insertion as libc::c_int != 0 {
                size =
                    length_add(length_sub(edit_0.new_end, padding),
                               length_sub(size,
                                          length_sub(edit_0.old_end,
                                                     padding)))
            }
        }
        let mut result: MutableSubtree =
            ts_subtree_make_mut(pool, *entry.tree);
        if result.data.is_inline() {
            if ts_subtree_can_inline(padding, size, lookahead_bytes) {
                result.data.padding_bytes = padding.bytes as uint8_t;
                result.data.set_padding_rows(padding.extent.row as uint8_t);
                result.data.padding_columns =
                    padding.extent.column as uint8_t;
                result.data.size_bytes = size.bytes as uint8_t
            } else {
                let mut data: *mut SubtreeHeapData =
                    ts_subtree_pool_allocate(pool);
                ::std::ptr::write_volatile(&mut (*data).ref_count as
                                               *mut uint32_t,
                                           1 as libc::c_int as uint32_t);
                (*data).padding = padding;
                (*data).size = size;
                (*data).lookahead_bytes = lookahead_bytes;
                (*data).error_cost = 0 as libc::c_int as uint32_t;
                (*data).child_count = 0 as libc::c_int as uint32_t;
                (*data).symbol = result.data.symbol as TSSymbol;
                (*data).parse_state = result.data.parse_state;
                (*data).set_visible(result.data.visible());
                (*data).set_named(result.data.named());
                (*data).set_extra(result.data.extra());
                (*data).set_fragile_left(0 as libc::c_int != 0);
                (*data).set_fragile_right(0 as libc::c_int != 0);
                (*data).set_has_changes(0 as libc::c_int != 0);
                (*data).set_has_external_tokens(0 as libc::c_int != 0);
                (*data).set_is_missing(result.data.is_missing());
                (*data).set_is_keyword(result.data.is_keyword());
                result.ptr = data
            }
        } else { (*result.ptr).padding = padding; (*result.ptr).size = size }
        ts_subtree_set_has_changes(&mut result);
        *entry.tree = ts_subtree_from_mut(result);
        let mut child_left: Length =
            Length{bytes: 0, extent: TSPoint{row: 0, column: 0,},};
        let mut child_right: Length = length_zero();
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut n: uint32_t = ts_subtree_child_count(*entry.tree);
        while i < n {
            let mut child: *mut Subtree =
                &mut *(*result.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                      as
                                                                                      isize)
                    as *mut Subtree;
            let mut child_size: Length = ts_subtree_total_size(*child);
            child_left = child_right;
            child_right = length_add(child_left, child_size);
            if !(child_right.bytes.wrapping_add(ts_subtree_lookahead_bytes(*child))
                     < edit_0.start.bytes) {
                if child_left.bytes > edit_0.old_end.bytes ||
                       child_left.bytes == edit_0.old_end.bytes &&
                           child_size.bytes > 0 as libc::c_int as libc::c_uint
                           && i > 0 as libc::c_int as libc::c_uint {
                    break ;
                }
                let mut child_edit: Edit =
                    {
                        let mut init =
                            Edit{start: length_sub(edit_0.start, child_left),
                                 old_end:
                                     length_sub(edit_0.old_end, child_left),
                                 new_end:
                                     length_sub(edit_0.new_end, child_left),};
                        init
                    };
                if edit_0.start.bytes < child_left.bytes {
                    child_edit.start = length_zero()
                }
                if edit_0.old_end.bytes < child_left.bytes {
                    child_edit.old_end = length_zero()
                }
                if edit_0.new_end.bytes < child_left.bytes {
                    child_edit.new_end = length_zero()
                }
                if edit_0.old_end.bytes > child_right.bytes {
                    child_edit.old_end = child_size
                }
                if child_right.bytes > edit_0.start.bytes ||
                       child_right.bytes == edit_0.start.bytes &&
                           is_pure_insertion as libc::c_int != 0 {
                    edit_0.new_end = edit_0.start
                } else {
                    child_edit.old_end = child_edit.start;
                    child_edit.new_end = child_edit.start
                }
                array__grow(&mut stack as *mut C2RustUnnamed_21 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<StackEntry_0>() as
                                libc::c_ulong);
                let fresh72 = stack.size;
                stack.size = stack.size.wrapping_add(1);
                *stack.contents.offset(fresh72 as isize) =
                    {
                        let mut init =
                            StackEntry_0{tree: child, edit: child_edit,};
                        init
                    }
            }
            i = i.wrapping_add(1)
        }
    }
    array__delete(&mut stack as *mut C2RustUnnamed_21 as *mut VoidArray);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_last_external_token(mut tree: Subtree)
 -> Subtree {
    if !ts_subtree_has_external_tokens(tree) {
        return Subtree{ptr: 0 as *const SubtreeHeapData,}
    }
    while (*tree.ptr).child_count > 0 as libc::c_int as libc::c_uint {
        let mut i: uint32_t =
            (*tree.ptr).child_count.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint);
        while i.wrapping_add(1 as libc::c_int as libc::c_uint) >
                  0 as libc::c_int as libc::c_uint {
            let mut child: Subtree =
                *(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                               as
                                                                               isize);
            if ts_subtree_has_external_tokens(child) {
                tree = child;
                break ;
            } else { i = i.wrapping_sub(1) }
        }
    }
    return tree;
}
#[inline]
unsafe extern "C" fn ts_subtree_set_has_changes(mut self_0:
                                                    *mut MutableSubtree) {
    if (*self_0).data.is_inline() {
        (*self_0).data.set_has_changes(1 as libc::c_int != 0)
    } else { (*(*self_0).ptr).set_has_changes(1 as libc::c_int != 0) };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_release(mut pool: *mut SubtreePool,
                                            mut self_0: Subtree) {
    if self_0.data.is_inline() { return }
    (*pool).tree_stack.size = 0 as libc::c_int as uint32_t;
    if (*self_0.ptr).ref_count > 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"self.ptr->ref_count > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./subtree.c\x00" as *const u8 as
                          *const libc::c_char,
                      520 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 48],
                                                &[libc::c_char; 48]>(b"void ts_subtree_release(SubtreePool *, Subtree)\x00")).as_ptr());
    }
    if atomic_dec(&(*self_0.ptr).ref_count as *const uint32_t as
                      *mut uint32_t) == 0 as libc::c_int as libc::c_uint {
        array__grow(&mut (*pool).tree_stack as *mut MutableSubtreeArray as
                        *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong);
        let fresh73 = (*pool).tree_stack.size;
        (*pool).tree_stack.size = (*pool).tree_stack.size.wrapping_add(1);
        *(*pool).tree_stack.contents.offset(fresh73 as isize) =
            ts_subtree_to_mut_unsafe(self_0)
    }
    while (*pool).tree_stack.size > 0 as libc::c_int as libc::c_uint {
        (*pool).tree_stack.size = (*pool).tree_stack.size.wrapping_sub(1);
        let mut tree: MutableSubtree =
            *(*pool).tree_stack.contents.offset((*pool).tree_stack.size as
                                                    isize);
        if (*tree.ptr).child_count > 0 as libc::c_int as libc::c_uint {
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*tree.ptr).child_count {
                let mut child: Subtree =
                    *(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                   as
                                                                                   isize);
                if !child.data.is_inline() {
                    if (*child.ptr).ref_count >
                           0 as libc::c_int as libc::c_uint {
                    } else {
                        __assert_fail(b"child.ptr->ref_count > 0\x00" as
                                          *const u8 as *const libc::c_char,
                                      b"lib/src/./subtree.c\x00" as *const u8
                                          as *const libc::c_char,
                                      531 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 48],
                                                                &[libc::c_char; 48]>(b"void ts_subtree_release(SubtreePool *, Subtree)\x00")).as_ptr());
                    }
                    if atomic_dec(&(*child.ptr).ref_count as *const uint32_t
                                      as *mut uint32_t) ==
                           0 as libc::c_int as libc::c_uint {
                        array__grow(&mut (*pool).tree_stack as
                                        *mut MutableSubtreeArray as
                                        *mut VoidArray,
                                    1 as libc::c_int as size_t,
                                    ::std::mem::size_of::<MutableSubtree>() as
                                        libc::c_ulong);
                        let fresh74 = (*pool).tree_stack.size;
                        (*pool).tree_stack.size =
                            (*pool).tree_stack.size.wrapping_add(1);
                        *(*pool).tree_stack.contents.offset(fresh74 as isize)
                            = ts_subtree_to_mut_unsafe(child)
                    }
                }
                i = i.wrapping_add(1)
            }
            ts_free((*tree.ptr).c2rust_unnamed.c2rust_unnamed.children as
                        *mut libc::c_void);
        } else if (*tree.ptr).has_external_tokens() {
            ts_external_scanner_state_delete(&mut (*tree.ptr).c2rust_unnamed.external_scanner_state);
        }
        ts_subtree_pool_free(pool, tree.ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_set_children(mut self_0: MutableSubtree,
                                                 mut children: *mut Subtree,
                                                 mut child_count: uint32_t,
                                                 mut language:
                                                     *const TSLanguage) {
    if !self_0.data.is_inline() {
    } else {
        __assert_fail(b"!self.data.is_inline\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./subtree.c\x00" as *const u8 as
                          *const libc::c_char,
                      347 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 86],
                                                &[libc::c_char; 86]>(b"void ts_subtree_set_children(MutableSubtree, Subtree *, uint32_t, const TSLanguage *)\x00")).as_ptr());
    }
    if (*self_0.ptr).child_count > 0 as libc::c_int as libc::c_uint &&
           children != (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children {
        ts_free((*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children as
                    *mut libc::c_void);
    }
    (*self_0.ptr).child_count = child_count;
    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children = children;
    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count =
        0 as libc::c_int as uint32_t;
    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count =
        0 as libc::c_int as uint32_t;
    (*self_0.ptr).error_cost = 0 as libc::c_int as uint32_t;
    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth =
        0 as libc::c_int as uint32_t;
    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.node_count =
        1 as libc::c_int as uint32_t;
    (*self_0.ptr).set_has_external_tokens(0 as libc::c_int != 0);
    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.dynamic_precedence =
        0 as libc::c_int;
    let mut non_extra_index: uint32_t = 0 as libc::c_int as uint32_t;
    let mut alias_sequence: *const TSSymbol =
        ts_language_alias_sequence(language,
                                   (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                       as uint32_t);
    let mut lookahead_end_byte: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0.ptr).child_count {
        let mut child: Subtree =
            *(*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i as
                                                                             isize);
        if i == 0 as libc::c_int as libc::c_uint {
            (*self_0.ptr).padding = ts_subtree_padding(child);
            (*self_0.ptr).size = ts_subtree_size(child)
        } else {
            (*self_0.ptr).size =
                length_add((*self_0.ptr).size, ts_subtree_total_size(child))
        }
        let mut child_lookahead_end_byte: uint32_t =
            (*self_0.ptr).padding.bytes.wrapping_add((*self_0.ptr).size.bytes).wrapping_add(ts_subtree_lookahead_bytes(child));
        if child_lookahead_end_byte > lookahead_end_byte {
            lookahead_end_byte = child_lookahead_end_byte
        }
        if ts_subtree_symbol(child) as libc::c_int !=
               -(1 as libc::c_int) as TSSymbol as libc::c_int -
                   1 as libc::c_int {
            (*self_0.ptr).error_cost =
                ((*self_0.ptr).error_cost as
                     libc::c_uint).wrapping_add(ts_subtree_error_cost(child))
                    as uint32_t as uint32_t
        }
        (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.dynamic_precedence +=
            ts_subtree_dynamic_precedence(child);
        (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.node_count =
            ((*self_0.ptr).c2rust_unnamed.c2rust_unnamed.node_count as
                 libc::c_uint).wrapping_add(ts_subtree_node_count(child)) as
                uint32_t as uint32_t;
        if !alias_sequence.is_null() &&
               *alias_sequence.offset(non_extra_index as isize) as libc::c_int
                   != 0 as libc::c_int && !ts_subtree_extra(child) {
            (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count =
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count.wrapping_add(1);
            if ts_language_symbol_metadata(language,
                                           *alias_sequence.offset(non_extra_index
                                                                      as
                                                                      isize)).named()
               {
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count
                    =
                    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count.wrapping_add(1)
            }
        } else if ts_subtree_visible(child) {
            (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count =
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count.wrapping_add(1);
            if ts_subtree_named(child) {
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count
                    =
                    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count.wrapping_add(1)
            }
        } else if ts_subtree_child_count(child) >
                      0 as libc::c_int as libc::c_uint {
            (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count =
                ((*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count
                     as
                     libc::c_uint).wrapping_add((*child.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count)
                    as uint32_t as uint32_t;
            (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count =
                ((*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count
                     as
                     libc::c_uint).wrapping_add((*child.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count)
                    as uint32_t as uint32_t
        }
        if ts_subtree_has_external_tokens(child) {
            (*self_0.ptr).set_has_external_tokens(1 as libc::c_int != 0)
        }
        if ts_subtree_is_error(child) {
            (*self_0.ptr).set_fragile_right(1 as libc::c_int != 0);
            (*self_0.ptr).set_fragile_left((*self_0.ptr).fragile_right());
            (*self_0.ptr).parse_state = TS_TREE_STATE_NONE
        }
        if !ts_subtree_extra(child) {
            non_extra_index = non_extra_index.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    (*self_0.ptr).lookahead_bytes =
        lookahead_end_byte.wrapping_sub((*self_0.ptr).size.bytes).wrapping_sub((*self_0.ptr).padding.bytes);
    if (*self_0.ptr).symbol as libc::c_int ==
           -(1 as libc::c_int) as TSSymbol as libc::c_int ||
           (*self_0.ptr).symbol as libc::c_int ==
               -(1 as libc::c_int) as TSSymbol as libc::c_int -
                   1 as libc::c_int {
        (*self_0.ptr).error_cost =
            ((*self_0.ptr).error_cost as
                 libc::c_uint).wrapping_add((500 as libc::c_int as
                                                 libc::c_uint).wrapping_add((1
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_uint).wrapping_mul((*self_0.ptr).size.bytes)).wrapping_add((30
                                                                                                                                                         as
                                                                                                                                                         libc::c_int
                                                                                                                                                         as
                                                                                                                                                         libc::c_uint).wrapping_mul((*self_0.ptr).size.extent.row)))
                as uint32_t as uint32_t;
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*self_0.ptr).child_count {
            let mut child_0: Subtree =
                *(*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i_0
                                                                                 as
                                                                                 isize);
            let mut grandchild_count: uint32_t =
                ts_subtree_child_count(child_0);
            if !ts_subtree_extra(child_0) {
                if !(ts_subtree_is_error(child_0) as libc::c_int != 0 &&
                         grandchild_count == 0 as libc::c_int as libc::c_uint)
                   {
                    if ts_subtree_visible(child_0) {
                        (*self_0.ptr).error_cost =
                            ((*self_0.ptr).error_cost as
                                 libc::c_uint).wrapping_add(100 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                as uint32_t as uint32_t
                    } else if grandchild_count >
                                  0 as libc::c_int as libc::c_uint {
                        (*self_0.ptr).error_cost =
                            ((*self_0.ptr).error_cost as
                                 libc::c_uint).wrapping_add((100 as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint).wrapping_mul((*child_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count))
                                as uint32_t as uint32_t
                    }
                }
            }
            i_0 = i_0.wrapping_add(1)
        }
    }
    if (*self_0.ptr).child_count > 0 as libc::c_int as libc::c_uint {
        let mut first_child: Subtree =
            *(*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0 as
                                                                             libc::c_int
                                                                             as
                                                                             isize);
        let mut last_child: Subtree =
            *(*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*self_0.ptr).child_count.wrapping_sub(1
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint)
                                                                             as
                                                                             isize);
        (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.first_leaf.symbol =
            ts_subtree_leaf_symbol(first_child);
        (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.first_leaf.parse_state =
            ts_subtree_leaf_parse_state(first_child);
        if ts_subtree_fragile_left(first_child) {
            (*self_0.ptr).set_fragile_left(1 as libc::c_int != 0)
        }
        if ts_subtree_fragile_right(last_child) {
            (*self_0.ptr).set_fragile_right(1 as libc::c_int != 0)
        }
        if (*self_0.ptr).child_count >= 2 as libc::c_int as libc::c_uint &&
               !(*self_0.ptr).visible() && !(*self_0.ptr).named() &&
               ts_subtree_symbol(first_child) as libc::c_int ==
                   (*self_0.ptr).symbol as libc::c_int {
            if ts_subtree_repeat_depth(first_child) >
                   ts_subtree_repeat_depth(last_child) {
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth =
                    ts_subtree_repeat_depth(first_child).wrapping_add(1 as
                                                                          libc::c_int
                                                                          as
                                                                          libc::c_uint)
            } else {
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth =
                    ts_subtree_repeat_depth(last_child).wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
            }
        }
    };
}
unsafe extern "C" fn ts_subtree__compress(mut self_0: MutableSubtree,
                                          mut count: libc::c_uint,
                                          mut language: *const TSLanguage,
                                          mut stack:
                                              *mut MutableSubtreeArray) {
    let mut initial_stack_size: libc::c_uint = (*stack).size;
    let mut tree: MutableSubtree = self_0;
    let mut symbol: TSSymbol = (*tree.ptr).symbol;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < count {
        if (*tree.ptr).ref_count > 1 as libc::c_int as libc::c_uint ||
               (*tree.ptr).child_count < 2 as libc::c_int as libc::c_uint {
            break ;
        }
        let mut child: MutableSubtree =
            ts_subtree_to_mut_unsafe(*(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize));
        if child.data.is_inline() as libc::c_int != 0 ||
               (*child.ptr).child_count < 2 as libc::c_int as libc::c_uint ||
               (*child.ptr).ref_count > 1 as libc::c_int as libc::c_uint ||
               (*child.ptr).symbol as libc::c_int != symbol as libc::c_int {
            break ;
        }
        let mut grandchild: MutableSubtree =
            ts_subtree_to_mut_unsafe(*(*child.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     isize));
        if grandchild.data.is_inline() as libc::c_int != 0 ||
               (*grandchild.ptr).child_count <
                   2 as libc::c_int as libc::c_uint ||
               (*grandchild.ptr).ref_count > 1 as libc::c_int as libc::c_uint
               ||
               (*grandchild.ptr).symbol as libc::c_int !=
                   symbol as libc::c_int {
            break ;
        }
        *(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0 as
                                                                       libc::c_int
                                                                       as
                                                                       isize)
            = ts_subtree_from_mut(grandchild);
        *(*child.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0 as
                                                                        libc::c_int
                                                                        as
                                                                        isize)
            =
            *(*grandchild.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*grandchild.ptr).child_count.wrapping_sub(1
                                                                                                                            as
                                                                                                                            libc::c_int
                                                                                                                            as
                                                                                                                            libc::c_uint)
                                                                                 as
                                                                                 isize);
        *(*grandchild.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*grandchild.ptr).child_count.wrapping_sub(1
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        libc::c_uint)
                                                                             as
                                                                             isize)
            = ts_subtree_from_mut(child);
        array__grow(stack as *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong);
        let fresh75 = (*stack).size;
        (*stack).size = (*stack).size.wrapping_add(1);
        *(*stack).contents.offset(fresh75 as isize) = tree;
        tree = grandchild;
        i = i.wrapping_add(1)
    }
    while (*stack).size > initial_stack_size {
        (*stack).size = (*stack).size.wrapping_sub(1);
        tree = *(*stack).contents.offset((*stack).size as isize);
        let mut child_0: MutableSubtree =
            ts_subtree_to_mut_unsafe(*(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize));
        let mut grandchild_0: MutableSubtree =
            ts_subtree_to_mut_unsafe(*(*child_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*child_0.ptr).child_count.wrapping_sub(1
                                                                                                                                               as
                                                                                                                                               libc::c_int
                                                                                                                                               as
                                                                                                                                               libc::c_uint)
                                                                                                       as
                                                                                                       isize));
        ts_subtree_set_children(grandchild_0,
                                (*grandchild_0.ptr).c2rust_unnamed.c2rust_unnamed.children,
                                (*grandchild_0.ptr).child_count, language);
        ts_subtree_set_children(child_0,
                                (*child_0.ptr).c2rust_unnamed.c2rust_unnamed.children,
                                (*child_0.ptr).child_count, language);
        ts_subtree_set_children(tree,
                                (*tree.ptr).c2rust_unnamed.c2rust_unnamed.children,
                                (*tree.ptr).child_count, language);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_error_node(mut pool: *mut SubtreePool,
                                                   mut children:
                                                       *mut SubtreeArray,
                                                   mut extra: bool,
                                                   mut language:
                                                       *const TSLanguage)
 -> Subtree {
    let mut result: MutableSubtree =
        ts_subtree_new_node(pool, -(1 as libc::c_int) as TSSymbol, children,
                            0 as libc::c_int as libc::c_uint, language);
    (*result.ptr).set_extra(extra);
    return ts_subtree_from_mut(result);
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_balance(mut self_0: Subtree,
                                            mut pool: *mut SubtreePool,
                                            mut language: *const TSLanguage) {
    (*pool).tree_stack.size = 0 as libc::c_int as uint32_t;
    if ts_subtree_child_count(self_0) > 0 as libc::c_int as libc::c_uint &&
           (*self_0.ptr).ref_count == 1 as libc::c_int as libc::c_uint {
        array__grow(&mut (*pool).tree_stack as *mut MutableSubtreeArray as
                        *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong);
        let fresh76 = (*pool).tree_stack.size;
        (*pool).tree_stack.size = (*pool).tree_stack.size.wrapping_add(1);
        *(*pool).tree_stack.contents.offset(fresh76 as isize) =
            ts_subtree_to_mut_unsafe(self_0)
    }
    while (*pool).tree_stack.size > 0 as libc::c_int as libc::c_uint {
        (*pool).tree_stack.size = (*pool).tree_stack.size.wrapping_sub(1);
        let mut tree: MutableSubtree =
            *(*pool).tree_stack.contents.offset((*pool).tree_stack.size as
                                                    isize);
        if (*tree.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth >
               0 as libc::c_int as libc::c_uint {
            let mut child1: Subtree =
                *(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(0
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize);
            let mut child2: Subtree =
                *(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*tree.ptr).child_count.wrapping_sub(1
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    libc::c_uint)
                                                                               as
                                                                               isize);
            let mut repeat_delta: libc::c_long =
                ts_subtree_repeat_depth(child1) as libc::c_long -
                    ts_subtree_repeat_depth(child2) as libc::c_long;
            if repeat_delta > 0 as libc::c_int as libc::c_long {
                let mut n: libc::c_uint = repeat_delta as libc::c_uint;
                let mut i: libc::c_uint =
                    n.wrapping_div(2 as libc::c_int as libc::c_uint);
                while i > 0 as libc::c_int as libc::c_uint {
                    ts_subtree__compress(tree, i, language,
                                         &mut (*pool).tree_stack);
                    n = n.wrapping_sub(i);
                    i = i.wrapping_div(2 as libc::c_int as libc::c_uint)
                }
            }
        }
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*tree.ptr).child_count {
            let mut child: Subtree =
                *(*tree.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i_0
                                                                               as
                                                                               isize);
            if ts_subtree_child_count(child) >
                   0 as libc::c_int as libc::c_uint &&
                   (*child.ptr).ref_count == 1 as libc::c_int as libc::c_uint
               {
                array__grow(&mut (*pool).tree_stack as
                                *mut MutableSubtreeArray as *mut VoidArray,
                            1 as libc::c_int as size_t,
                            ::std::mem::size_of::<MutableSubtree>() as
                                libc::c_ulong);
                let fresh77 = (*pool).tree_stack.size;
                (*pool).tree_stack.size =
                    (*pool).tree_stack.size.wrapping_add(1);
                *(*pool).tree_stack.contents.offset(fresh77 as isize) =
                    ts_subtree_to_mut_unsafe(child)
            }
            i_0 = i_0.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_eq(mut self_0: Subtree,
                                       mut other: Subtree) -> bool {
    if self_0.data.is_inline() as libc::c_int != 0 ||
           other.data.is_inline() as libc::c_int != 0 {
        return memcmp(&mut self_0 as *mut Subtree as *const libc::c_void,
                      &mut other as *mut Subtree as *const libc::c_void,
                      ::std::mem::size_of::<SubtreeInlineData>() as
                          libc::c_ulong) == 0 as libc::c_int
    }
    if !self_0.ptr.is_null() {
        if other.ptr.is_null() { return 0 as libc::c_int != 0 }
    } else { return other.ptr.is_null() }
    if (*self_0.ptr).symbol as libc::c_int !=
           (*other.ptr).symbol as libc::c_int {
        return 0 as libc::c_int != 0
    }
    if (*self_0.ptr).visible() as libc::c_int !=
           (*other.ptr).visible() as libc::c_int {
        return 0 as libc::c_int != 0
    }
    if (*self_0.ptr).named() as libc::c_int !=
           (*other.ptr).named() as libc::c_int {
        return 0 as libc::c_int != 0
    }
    if (*self_0.ptr).padding.bytes != (*other.ptr).padding.bytes {
        return 0 as libc::c_int != 0
    }
    if (*self_0.ptr).size.bytes != (*other.ptr).size.bytes {
        return 0 as libc::c_int != 0
    }
    if (*self_0.ptr).symbol as libc::c_int ==
           -(1 as libc::c_int) as TSSymbol as libc::c_int {
        return (*self_0.ptr).c2rust_unnamed.lookahead_char ==
                   (*other.ptr).c2rust_unnamed.lookahead_char
    }
    if (*self_0.ptr).child_count != (*other.ptr).child_count {
        return 0 as libc::c_int != 0
    }
    if (*self_0.ptr).child_count > 0 as libc::c_int as libc::c_uint {
        if (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count !=
               (*other.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count
           {
            return 0 as libc::c_int != 0
        }
        if (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count !=
               (*other.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count {
            return 0 as libc::c_int != 0
        }
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*self_0.ptr).child_count {
            if !ts_subtree_eq(*(*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                               as
                                                                                               isize),
                              *(*other.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                              as
                                                                                              isize))
               {
                return 0 as libc::c_int != 0
            }
            i = i.wrapping_add(1)
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_retain(mut self_0: Subtree) {
    if self_0.data.is_inline() { return }
    if (*self_0.ptr).ref_count > 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"self.ptr->ref_count > 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./subtree.c\x00" as *const u8 as
                          *const libc::c_char,
                      511 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void ts_subtree_retain(Subtree)\x00")).as_ptr());
    }
    atomic_inc(&(*self_0.ptr).ref_count as *const uint32_t as *mut uint32_t);
    if (*self_0.ptr).ref_count != 0 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(b"self.ptr->ref_count != 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"lib/src/./subtree.c\x00" as *const u8 as
                          *const libc::c_char,
                      513 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 32],
                                                &[libc::c_char; 32]>(b"void ts_subtree_retain(Subtree)\x00")).as_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_node(mut pool: *mut SubtreePool,
                                             mut symbol: TSSymbol,
                                             mut children: *mut SubtreeArray,
                                             mut production_id: libc::c_uint,
                                             mut language: *const TSLanguage)
 -> MutableSubtree {
    let mut metadata: TSSymbolMetadata =
        ts_language_symbol_metadata(language, symbol);
    let mut fragile: bool =
        symbol as libc::c_int ==
            -(1 as libc::c_int) as TSSymbol as libc::c_int ||
            symbol as libc::c_int ==
                -(1 as libc::c_int) as TSSymbol as libc::c_int -
                    1 as libc::c_int;
    let mut data: *mut SubtreeHeapData = ts_subtree_pool_allocate(pool);
    *data =
        {
            let mut init =
                SubtreeHeapData{visible_named_extra_fragile_left_fragile_right_has_changes_has_external_tokens_is_missing_is_keyword:
                                    [0; 2],
                                c2rust_padding: [0; 2],
                                ref_count: 1 as libc::c_int as uint32_t,
                                padding:
                                    Length{bytes: 0,
                                           extent:
                                               TSPoint{row: 0, column: 0,},},
                                size:
                                    Length{bytes: 0,
                                           extent:
                                               TSPoint{row: 0, column: 0,},},
                                lookahead_bytes: 0,
                                error_cost: 0,
                                child_count: 0,
                                symbol: symbol,
                                parse_state: 0,
                                c2rust_unnamed:
                                    C2RustUnnamed_4{c2rust_unnamed:
                                                        {
                                                            let mut init =
                                                                C2RustUnnamed_6{children:
                                                                                    0
                                                                                        as
                                                                                        *mut Subtree,
                                                                                visible_child_count:
                                                                                    0,
                                                                                named_child_count:
                                                                                    0,
                                                                                node_count:
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        uint32_t,
                                                                                repeat_depth:
                                                                                    0,
                                                                                dynamic_precedence:
                                                                                    0,
                                                                                production_id:
                                                                                    production_id
                                                                                        as
                                                                                        uint16_t,
                                                                                first_leaf:
                                                                                    {
                                                                                        let mut init =
                                                                                            C2RustUnnamed_7{symbol:
                                                                                                                0
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    TSSymbol,
                                                                                                            parse_state:
                                                                                                                0
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    TSStateId,};
                                                                                        init
                                                                                    },};
                                                            init
                                                        },},};
            init.set_visible(metadata.visible());
            init.set_named(metadata.named());
            init.set_extra(false);
            init.set_fragile_left(fragile);
            init.set_fragile_right(fragile);
            init.set_has_changes(0 as libc::c_int != 0);
            init.set_has_external_tokens(false);
            init.set_is_missing(false);
            init.set_is_keyword(0 as libc::c_int != 0);
            init
        };
    let mut result: MutableSubtree = MutableSubtree{ptr: data,};
    ts_subtree_set_children(result, (*children).contents, (*children).size,
                            language);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_missing_leaf(mut pool:
                                                         *mut SubtreePool,
                                                     mut symbol: TSSymbol,
                                                     mut padding: Length,
                                                     mut language:
                                                         *const TSLanguage)
 -> Subtree {
    let mut result: Subtree =
        ts_subtree_new_leaf(pool, symbol, padding, length_zero(),
                            0 as libc::c_int as uint32_t,
                            0 as libc::c_int as TSStateId,
                            0 as libc::c_int != 0, 0 as libc::c_int != 0,
                            language);
    if result.data.is_inline() {
        result.data.set_is_missing(1 as libc::c_int != 0)
    } else {
        let ref mut fresh78 = *(result.ptr as *mut SubtreeHeapData);
        (*fresh78).set_is_missing(1 as libc::c_int != 0)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_compare(mut left: Subtree,
                                            mut right: Subtree)
 -> libc::c_int {
    if (ts_subtree_symbol(left) as libc::c_int) <
           ts_subtree_symbol(right) as libc::c_int {
        return -(1 as libc::c_int)
    }
    if (ts_subtree_symbol(right) as libc::c_int) <
           ts_subtree_symbol(left) as libc::c_int {
        return 1 as libc::c_int
    }
    if ts_subtree_child_count(left) < ts_subtree_child_count(right) {
        return -(1 as libc::c_int)
    }
    if ts_subtree_child_count(right) < ts_subtree_child_count(left) {
        return 1 as libc::c_int
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut n: uint32_t = ts_subtree_child_count(left);
    while i < n {
        let mut left_child: Subtree =
            *(*left.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i as
                                                                           isize);
        let mut right_child: Subtree =
            *(*right.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i as
                                                                            isize);
        match ts_subtree_compare(left_child, right_child) {
            -1 => { return -(1 as libc::c_int) }
            1 => { return 1 as libc::c_int }
            _ => { }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_error(mut pool: *mut SubtreePool,
                                              mut lookahead_char: int32_t,
                                              mut padding: Length,
                                              mut size: Length,
                                              mut bytes_scanned: uint32_t,
                                              mut parse_state: TSStateId,
                                              mut language: *const TSLanguage)
 -> Subtree {
    let mut result: Subtree =
        ts_subtree_new_leaf(pool, -(1 as libc::c_int) as TSSymbol, padding,
                            size, bytes_scanned, parse_state,
                            0 as libc::c_int != 0, 0 as libc::c_int != 0,
                            language);
    let mut data: *mut SubtreeHeapData = result.ptr as *mut SubtreeHeapData;
    (*data).set_fragile_left(1 as libc::c_int != 0);
    (*data).set_fragile_right(1 as libc::c_int != 0);
    (*data).c2rust_unnamed.lookahead_char = lookahead_char;
    return result;
}
unsafe extern "C" fn ts_subtree__write_dot_string(mut f: *mut FILE,
                                                  mut string:
                                                      *const libc::c_char) {
    let mut c: *const libc::c_char = string;
    while *c != 0 {
        if *c as libc::c_int == '\"' as i32 {
            fputs(b"\\\"\x00" as *const u8 as *const libc::c_char, f);
        } else if *c as libc::c_int == '\n' as i32 {
            fputs(b"\\n\x00" as *const u8 as *const libc::c_char, f);
        } else { fputc(*c as libc::c_int, f); }
        c = c.offset(1)
    };
}
static mut ROOT_FIELD: *const libc::c_char =
    b"__ROOT__\x00" as *const u8 as *const libc::c_char;
unsafe extern "C" fn ts_subtree__write_to_string(mut self_0: Subtree,
                                                 mut string:
                                                     *mut libc::c_char,
                                                 mut limit: size_t,
                                                 mut language:
                                                     *const TSLanguage,
                                                 mut include_all: bool,
                                                 mut alias_symbol: TSSymbol,
                                                 mut alias_is_named: bool,
                                                 mut field_name:
                                                     *const libc::c_char)
 -> size_t {
    if self_0.ptr.is_null() {
        return snprintf(string, limit,
                        b"(NULL)\x00" as *const u8 as *const libc::c_char) as
                   size_t
    }
    let mut cursor: *mut libc::c_char = string;
    let mut writer: *mut *mut libc::c_char =
        if limit > 0 as libc::c_int as libc::c_ulong {
            &mut cursor
        } else { &mut string };
    let mut is_root: bool = field_name == ROOT_FIELD;
    let mut is_visible: bool =
        include_all as libc::c_int != 0 ||
            ts_subtree_missing(self_0) as libc::c_int != 0 ||
            (if alias_symbol as libc::c_int != 0 {
                 alias_is_named as libc::c_int
             } else {
                 (ts_subtree_visible(self_0) as libc::c_int != 0 &&
                      ts_subtree_named(self_0) as libc::c_int != 0) as
                     libc::c_int
             }) != 0;
    if is_visible {
        if !is_root {
            cursor =
                cursor.offset(snprintf(*writer, limit,
                                       b" \x00" as *const u8 as
                                           *const libc::c_char) as isize);
            if !field_name.is_null() {
                cursor =
                    cursor.offset(snprintf(*writer, limit,
                                           b"%s: \x00" as *const u8 as
                                               *const libc::c_char,
                                           field_name) as isize)
            }
        }
        if ts_subtree_is_error(self_0) as libc::c_int != 0 &&
               ts_subtree_child_count(self_0) ==
                   0 as libc::c_int as libc::c_uint &&
               (*self_0.ptr).size.bytes > 0 as libc::c_int as libc::c_uint {
            cursor =
                cursor.offset(snprintf(*writer, limit,
                                       b"(UNEXPECTED \x00" as *const u8 as
                                           *const libc::c_char) as isize);
            cursor =
                cursor.offset(ts_subtree__write_char_to_string(*writer, limit,
                                                               (*self_0.ptr).c2rust_unnamed.lookahead_char)
                                  as isize)
        } else {
            let mut symbol: TSSymbol =
                if alias_symbol as libc::c_int != 0 {
                    alias_symbol as libc::c_int
                } else { ts_subtree_symbol(self_0) as libc::c_int } as
                    TSSymbol;
            let mut symbol_name: *const libc::c_char =
                ts_language_symbol_name(language, symbol);
            if ts_subtree_missing(self_0) {
                cursor =
                    cursor.offset(snprintf(*writer, limit,
                                           b"(MISSING \x00" as *const u8 as
                                               *const libc::c_char) as isize);
                if alias_is_named as libc::c_int != 0 ||
                       ts_subtree_named(self_0) as libc::c_int != 0 {
                    cursor =
                        cursor.offset(snprintf(*writer, limit,
                                               b"%s\x00" as *const u8 as
                                                   *const libc::c_char,
                                               symbol_name) as isize)
                } else {
                    cursor =
                        cursor.offset(snprintf(*writer, limit,
                                               b"\"%s\"\x00" as *const u8 as
                                                   *const libc::c_char,
                                               symbol_name) as isize)
                }
            } else {
                cursor =
                    cursor.offset(snprintf(*writer, limit,
                                           b"(%s\x00" as *const u8 as
                                               *const libc::c_char,
                                           symbol_name) as isize)
            }
        }
    } else if is_root {
        let mut symbol_0: TSSymbol = ts_subtree_symbol(self_0);
        let mut symbol_name_0: *const libc::c_char =
            ts_language_symbol_name(language, symbol_0);
        cursor =
            cursor.offset(snprintf(*writer, limit,
                                   b"(\"%s\")\x00" as *const u8 as
                                       *const libc::c_char, symbol_name_0) as
                              isize)
    }
    if ts_subtree_child_count(self_0) != 0 {
        let mut alias_sequence: *const TSSymbol =
            ts_language_alias_sequence(language,
                                       (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                           as uint32_t);
        let mut field_map: *const TSFieldMapEntry =
            0 as *const TSFieldMapEntry;
        let mut field_map_end: *const TSFieldMapEntry =
            0 as *const TSFieldMapEntry;
        ts_language_field_map(language,
                              (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                  as uint32_t, &mut field_map,
                              &mut field_map_end);
        let mut structural_child_index: uint32_t =
            0 as libc::c_int as uint32_t;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*self_0.ptr).child_count {
            let mut child: Subtree =
                *(*self_0.ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                 as
                                                                                 isize);
            if ts_subtree_extra(child) {
                cursor =
                    cursor.offset(ts_subtree__write_to_string(child, *writer,
                                                              limit, language,
                                                              include_all,
                                                              0 as libc::c_int
                                                                  as TSSymbol,
                                                              0 as libc::c_int
                                                                  != 0,
                                                              0 as
                                                                  *const libc::c_char)
                                      as isize)
            } else {
                let mut alias_symbol_0: TSSymbol =
                    if !alias_sequence.is_null() {
                        *alias_sequence.offset(structural_child_index as
                                                   isize) as libc::c_int
                    } else { 0 as libc::c_int } as TSSymbol;
                let mut alias_is_named_0: bool =
                    if alias_symbol_0 as libc::c_int != 0 {
                        ts_language_symbol_metadata(language,
                                                    alias_symbol_0).named() as
                            libc::c_int
                    } else { 0 as libc::c_int } != 0;
                let mut child_field_name: *const libc::c_char =
                    if is_visible as libc::c_int != 0 {
                        0 as *const libc::c_char
                    } else { field_name };
                let mut i_0: *const TSFieldMapEntry = field_map;
                while i_0 < field_map_end {
                    if !(*i_0).inherited &&
                           (*i_0).child_index as libc::c_uint ==
                               structural_child_index {
                        child_field_name =
                            *(*language).field_names.offset((*i_0).field_id as
                                                                isize);
                        break ;
                    } else { i_0 = i_0.offset(1) }
                }
                cursor =
                    cursor.offset(ts_subtree__write_to_string(child, *writer,
                                                              limit, language,
                                                              include_all,
                                                              alias_symbol_0,
                                                              alias_is_named_0,
                                                              child_field_name)
                                      as isize);
                structural_child_index =
                    structural_child_index.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
    }
    if is_visible {
        cursor =
            cursor.offset(snprintf(*writer, limit,
                                   b")\x00" as *const u8 as
                                       *const libc::c_char) as isize)
    }
    return cursor.wrapping_offset_from(string) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_string(mut self_0: Subtree,
                                           mut language: *const TSLanguage,
                                           mut include_all: bool)
 -> *mut libc::c_char {
    let mut scratch_string: [libc::c_char; 1] = [0; 1];
    let mut size: size_t =
        ts_subtree__write_to_string(self_0, scratch_string.as_mut_ptr(),
                                    0 as libc::c_int as size_t, language,
                                    include_all, 0 as libc::c_int as TSSymbol,
                                    0 as libc::c_int != 0,
                                    ROOT_FIELD).wrapping_add(1 as libc::c_int
                                                                 as
                                                                 libc::c_ulong);
    let mut result: *mut libc::c_char =
        malloc(size.wrapping_mul(::std::mem::size_of::<libc::c_char>() as
                                     libc::c_ulong)) as *mut libc::c_char;
    ts_subtree__write_to_string(self_0, result, size, language, include_all,
                                0 as libc::c_int as TSSymbol,
                                0 as libc::c_int != 0, ROOT_FIELD);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree__print_dot_graph(mut self_0:
                                                         *const Subtree,
                                                     mut start_offset:
                                                         uint32_t,
                                                     mut language:
                                                         *const TSLanguage,
                                                     mut alias_symbol:
                                                         TSSymbol,
                                                     mut f: *mut FILE) {
    let mut subtree_symbol: TSSymbol = ts_subtree_symbol(*self_0);
    let mut symbol: TSSymbol =
        if alias_symbol as libc::c_int != 0 {
            alias_symbol as libc::c_int
        } else { subtree_symbol as libc::c_int } as TSSymbol;
    let mut end_offset: uint32_t =
        start_offset.wrapping_add(ts_subtree_total_bytes(*self_0));
    fprintf(f, b"tree_%p [label=\"\x00" as *const u8 as *const libc::c_char,
            self_0);
    ts_subtree__write_dot_string(f,
                                 ts_language_symbol_name(language, symbol));
    fprintf(f, b"\"\x00" as *const u8 as *const libc::c_char);
    if ts_subtree_child_count(*self_0) == 0 as libc::c_int as libc::c_uint {
        fprintf(f,
                b", shape=plaintext\x00" as *const u8 as *const libc::c_char);
    }
    if ts_subtree_extra(*self_0) {
        fprintf(f,
                b", fontcolor=gray\x00" as *const u8 as *const libc::c_char);
    }
    fprintf(f,
            b", tooltip=\"range: %u - %u\nstate: %d\nerror-cost: %u\nhas-changes: %u\nrepeat-depth: %u\nlookahead-bytes: %u\x00"
                as *const u8 as *const libc::c_char, start_offset, end_offset,
            ts_subtree_parse_state(*self_0) as libc::c_int,
            ts_subtree_error_cost(*self_0),
            ts_subtree_has_changes(*self_0) as libc::c_int,
            ts_subtree_repeat_depth(*self_0),
            ts_subtree_lookahead_bytes(*self_0));
    if ts_subtree_is_error(*self_0) as libc::c_int != 0 &&
           ts_subtree_child_count(*self_0) == 0 as libc::c_int as libc::c_uint
       {
        fprintf(f,
                b"\ncharacter: \'%c\'\x00" as *const u8 as
                    *const libc::c_char,
                (*(*self_0).ptr).c2rust_unnamed.lookahead_char);
    }
    fprintf(f, b"\"]\n\x00" as *const u8 as *const libc::c_char);
    let mut child_start_offset: uint32_t = start_offset;
    let mut child_info_offset: uint32_t =
        ((*language).max_alias_sequence_length as libc::c_int *
             ts_subtree_production_id(*self_0) as libc::c_int) as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut n: uint32_t = ts_subtree_child_count(*self_0);
    while i < n {
        let mut child: *const Subtree =
            &mut *(*(*self_0).ptr).c2rust_unnamed.c2rust_unnamed.children.offset(i
                                                                                     as
                                                                                     isize)
                as *mut Subtree;
        let mut alias_symbol_0: TSSymbol = 0 as libc::c_int as TSSymbol;
        if !ts_subtree_extra(*child) && child_info_offset != 0 {
            alias_symbol_0 =
                *(*language).alias_sequences.offset(child_info_offset as
                                                        isize);
            child_info_offset = child_info_offset.wrapping_add(1)
        }
        ts_subtree__print_dot_graph(child, child_start_offset, language,
                                    alias_symbol_0, f);
        fprintf(f,
                b"tree_%p -> tree_%p [tooltip=%u]\n\x00" as *const u8 as
                    *const libc::c_char, self_0, child, i);
        child_start_offset =
            (child_start_offset as
                 libc::c_uint).wrapping_add(ts_subtree_total_bytes(*child)) as
                uint32_t as uint32_t;
        i = i.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_print_dot_graph(mut self_0: Subtree,
                                                    mut language:
                                                        *const TSLanguage,
                                                    mut f: *mut FILE) {
    fprintf(f, b"digraph tree {\n\x00" as *const u8 as *const libc::c_char);
    fprintf(f,
            b"edge [arrowhead=none]\n\x00" as *const u8 as
                *const libc::c_char);
    ts_subtree__print_dot_graph(&mut self_0, 0 as libc::c_int as uint32_t,
                                language, 0 as libc::c_int as TSSymbol, f);
    fprintf(f, b"}\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_external_scanner_state_eq(mut self_0:
                                                                  Subtree,
                                                              mut other:
                                                                  Subtree)
 -> bool {
    let mut state1: *const ExternalScannerState = &empty_state;
    let mut state2: *const ExternalScannerState = &empty_state;
    if !self_0.ptr.is_null() &&
           ts_subtree_has_external_tokens(self_0) as libc::c_int != 0 &&
           (*self_0.ptr).child_count == 0 {
        state1 = &(*self_0.ptr).c2rust_unnamed.external_scanner_state
    }
    if !other.ptr.is_null() &&
           ts_subtree_has_external_tokens(other) as libc::c_int != 0 &&
           (*other.ptr).child_count == 0 {
        state2 = &(*other.ptr).c2rust_unnamed.external_scanner_state
    }
    return ts_external_scanner_state_eq(state1, state2);
}
// CursorChildIterator
#[inline]
unsafe extern "C" fn ts_tree_cursor_iterate_children(mut self_0:
                                                         *const TreeCursor)
 -> CursorChildIterator {
    if (*self_0).stack.size.wrapping_sub(1 as libc::c_int as libc::c_uint) <
           (*self_0).stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->stack)->size - 1 < (&self->stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./tree_cursor.c\x00" as *const u8 as
                          *const libc::c_char,
                      19 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"CursorChildIterator ts_tree_cursor_iterate_children(const TreeCursor *)\x00")).as_ptr());
    }
    let mut last_entry: *mut TreeCursorEntry =
        &mut *(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                  as isize) as
            *mut TreeCursorEntry;
    if ts_subtree_child_count(*(*last_entry).subtree) ==
           0 as libc::c_int as libc::c_uint {
        return {
                   let mut init =
                       CursorChildIterator{parent:
                                               Subtree{ptr:
                                                           0 as
                                                               *const SubtreeHeapData,},
                                           tree: (*self_0).tree,
                                           position: length_zero(),
                                           child_index:
                                               0 as libc::c_int as uint32_t,
                                           structural_child_index:
                                               0 as libc::c_int as uint32_t,
                                           alias_sequence:
                                               0 as *const TSSymbol,};
                   init
               }
    }
    let mut alias_sequence: *const TSSymbol =
        ts_language_alias_sequence((*(*self_0).tree).language,
                                   (*(*(*last_entry).subtree).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                       as uint32_t);
    return {
               let mut init =
                   CursorChildIterator{parent: *(*last_entry).subtree,
                                       tree: (*self_0).tree,
                                       position: (*last_entry).position,
                                       child_index:
                                           0 as libc::c_int as uint32_t,
                                       structural_child_index:
                                           0 as libc::c_int as uint32_t,
                                       alias_sequence: alias_sequence,};
               init
           };
}
#[inline]
unsafe extern "C" fn ts_tree_cursor_child_iterator_next(mut self_0:
                                                            *mut CursorChildIterator,
                                                        mut result:
                                                            *mut TreeCursorEntry,
                                                        mut visible:
                                                            *mut bool)
 -> bool {
    if (*self_0).parent.ptr.is_null() ||
           (*self_0).child_index == (*(*self_0).parent.ptr).child_count {
        return 0 as libc::c_int != 0
    }
    let mut child: *const Subtree =
        &mut *(*(*self_0).parent.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*self_0).child_index
                                                                                        as
                                                                                        isize)
            as *mut Subtree;
    *result =
        {
            let mut init =
                TreeCursorEntry{subtree: child,
                                position: (*self_0).position,
                                child_index: (*self_0).child_index,
                                structural_child_index:
                                    (*self_0).structural_child_index,};
            init
        };
    *visible = ts_subtree_visible(*child);
    let mut extra: bool = ts_subtree_extra(*child);
    if !extra && !(*self_0).alias_sequence.is_null() {
        *visible =
            (*visible as libc::c_int |
                 *(*self_0).alias_sequence.offset((*self_0).structural_child_index
                                                      as isize) as
                     libc::c_int) as bool;
        (*self_0).structural_child_index =
            (*self_0).structural_child_index.wrapping_add(1)
    }
    (*self_0).position =
        length_add((*self_0).position, ts_subtree_size(*child));
    (*self_0).child_index = (*self_0).child_index.wrapping_add(1);
    if (*self_0).child_index < (*(*self_0).parent.ptr).child_count {
        let mut next_child: Subtree =
            *(*(*self_0).parent.ptr).c2rust_unnamed.c2rust_unnamed.children.offset((*self_0).child_index
                                                                                       as
                                                                                       isize);
        (*self_0).position =
            length_add((*self_0).position, ts_subtree_padding(next_child))
    }
    return 1 as libc::c_int != 0;
}
// TSTreeCursor - lifecycle
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_new(mut node: TSNode)
 -> TSTreeCursor {
    let mut self_0: TSTreeCursor =
        {
            let mut init =
                TSTreeCursor{tree: 0 as *const libc::c_void,
                             id: 0 as *const libc::c_void,
                             context:
                                 [0 as libc::c_int as uint32_t,
                                  0 as libc::c_int as uint32_t],};
            init
        };
    ts_tree_cursor_init(&mut self_0 as *mut TSTreeCursor as *mut TreeCursor,
                        node);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_reset(mut _self: *mut TSTreeCursor,
                                              mut node: TSNode) {
    ts_tree_cursor_init(_self as *mut TreeCursor, node);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_init(mut self_0: *mut TreeCursor,
                                             mut node: TSNode) {
    (*self_0).tree = node.tree;
    (*self_0).stack.size = 0 as libc::c_int as uint32_t;
    array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_22 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<TreeCursorEntry>() as libc::c_ulong);
    let fresh79 = (*self_0).stack.size;
    (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
    *(*self_0).stack.contents.offset(fresh79 as isize) =
        {
            let mut init =
                TreeCursorEntry{subtree: node.id as *const Subtree,
                                position:
                                    {
                                        let mut init =
                                            Length{bytes:
                                                       ts_node_start_byte(node),
                                                   extent:
                                                       ts_node_start_point(node),};
                                        init
                                    },
                                child_index: 0 as libc::c_int as uint32_t,
                                structural_child_index:
                                    0 as libc::c_int as uint32_t,};
            init
        };
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_delete(mut _self: *mut TSTreeCursor) {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    array__delete(&mut (*self_0).stack as *mut C2RustUnnamed_22 as
                      *mut VoidArray);
}
// TSTreeCursor - walking the tree
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_first_child(mut _self:
                                                             *mut TSTreeCursor)
 -> bool {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut did_descend: bool = false;
    loop  {
        did_descend = 0 as libc::c_int != 0;
        let mut visible: bool = false;
        let mut entry: TreeCursorEntry =
            TreeCursorEntry{subtree: 0 as *const Subtree,
                            position:
                                Length{bytes: 0,
                                       extent: TSPoint{row: 0, column: 0,},},
                            child_index: 0,
                            structural_child_index: 0,};
        let mut iterator: CursorChildIterator =
            ts_tree_cursor_iterate_children(self_0);
        while ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry,
                                                 &mut visible) {
            if visible {
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_22 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh80 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh80 as isize) = entry;
                return 1 as libc::c_int != 0
            }
            if !(ts_subtree_visible_child_count(*entry.subtree) >
                     0 as libc::c_int as libc::c_uint) {
                continue ;
            }
            array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_22 as
                            *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<TreeCursorEntry>() as
                            libc::c_ulong);
            let fresh81 = (*self_0).stack.size;
            (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
            *(*self_0).stack.contents.offset(fresh81 as isize) = entry;
            did_descend = 1 as libc::c_int != 0;
            break ;
        }
        if !did_descend { break ; }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_first_child_for_byte(mut _self:
                                                                      *mut TSTreeCursor,
                                                                  mut goal_byte:
                                                                      uint32_t)
 -> int64_t {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut initial_size: uint32_t = (*self_0).stack.size;
    let mut visible_child_index: uint32_t = 0 as libc::c_int as uint32_t;
    let mut did_descend: bool = false;
    loop  {
        did_descend = 0 as libc::c_int != 0;
        let mut visible: bool = false;
        let mut entry: TreeCursorEntry =
            TreeCursorEntry{subtree: 0 as *const Subtree,
                            position:
                                Length{bytes: 0,
                                       extent: TSPoint{row: 0, column: 0,},},
                            child_index: 0,
                            structural_child_index: 0,};
        let mut iterator: CursorChildIterator =
            ts_tree_cursor_iterate_children(self_0);
        while ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry,
                                                 &mut visible) {
            let mut end_byte: uint32_t =
                entry.position.bytes.wrapping_add(ts_subtree_size(*entry.subtree).bytes);
            let mut at_goal: bool = end_byte > goal_byte;
            let mut visible_child_count: uint32_t =
                ts_subtree_visible_child_count(*entry.subtree);
            if at_goal {
                if visible {
                    array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_22
                                    as *mut VoidArray,
                                1 as libc::c_int as size_t,
                                ::std::mem::size_of::<TreeCursorEntry>() as
                                    libc::c_ulong);
                    let fresh82 = (*self_0).stack.size;
                    (*self_0).stack.size =
                        (*self_0).stack.size.wrapping_add(1);
                    *(*self_0).stack.contents.offset(fresh82 as isize) =
                        entry;
                    return visible_child_index as int64_t
                }
                if !(visible_child_count > 0 as libc::c_int as libc::c_uint) {
                    continue ;
                }
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_22 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh83 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh83 as isize) = entry;
                did_descend = 1 as libc::c_int != 0;
                break ;
            } else if visible {
                visible_child_index = visible_child_index.wrapping_add(1)
            } else {
                visible_child_index =
                    (visible_child_index as
                         libc::c_uint).wrapping_add(visible_child_count) as
                        uint32_t as uint32_t
            }
        }
        if !did_descend { break ; }
    }
    if (*self_0).stack.size > initial_size &&
           ts_tree_cursor_goto_next_sibling(self_0 as *mut TSTreeCursor) as
               libc::c_int != 0 {
        return visible_child_index as int64_t
    }
    (*self_0).stack.size = initial_size;
    return -(1 as libc::c_int) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_next_sibling(mut _self:
                                                              *mut TSTreeCursor)
 -> bool {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut initial_size: uint32_t = (*self_0).stack.size;
    while (*self_0).stack.size > 1 as libc::c_int as libc::c_uint {
        (*self_0).stack.size = (*self_0).stack.size.wrapping_sub(1);
        let mut entry: TreeCursorEntry =
            *(*self_0).stack.contents.offset((*self_0).stack.size as isize);
        let mut iterator: CursorChildIterator =
            ts_tree_cursor_iterate_children(self_0);
        iterator.child_index = entry.child_index;
        iterator.structural_child_index = entry.structural_child_index;
        iterator.position = entry.position;
        let mut visible: bool = 0 as libc::c_int != 0;
        ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry,
                                           &mut visible);
        if visible as libc::c_int != 0 &&
               (*self_0).stack.size.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) <
                   initial_size {
            break ;
        }
        while ts_tree_cursor_child_iterator_next(&mut iterator, &mut entry,
                                                 &mut visible) {
            if visible {
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_22 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh84 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh84 as isize) = entry;
                return 1 as libc::c_int != 0
            }
            if ts_subtree_visible_child_count(*entry.subtree) != 0 {
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_22 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh85 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh85 as isize) = entry;
                ts_tree_cursor_goto_first_child(_self);
                return 1 as libc::c_int != 0
            }
        }
    }
    (*self_0).stack.size = initial_size;
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_goto_parent(mut _self:
                                                        *mut TSTreeCursor)
 -> bool {
    let mut self_0: *mut TreeCursor = _self as *mut TreeCursor;
    let mut i: libc::c_uint =
        (*self_0).stack.size.wrapping_sub(2 as libc::c_int as libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) >
              0 as libc::c_int as libc::c_uint {
        let mut entry: *mut TreeCursorEntry =
            &mut *(*self_0).stack.contents.offset(i as isize) as
                *mut TreeCursorEntry;
        let mut is_aliased: bool = 0 as libc::c_int != 0;
        if i > 0 as libc::c_int as libc::c_uint {
            let mut parent_entry: *mut TreeCursorEntry =
                &mut *(*self_0).stack.contents.offset(i.wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_uint)
                                                          as isize) as
                    *mut TreeCursorEntry;
            let mut alias_sequence: *const TSSymbol =
                ts_language_alias_sequence((*(*self_0).tree).language,
                                           (*(*(*parent_entry).subtree).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                               as uint32_t);
            is_aliased =
                !alias_sequence.is_null() &&
                    *alias_sequence.offset((*entry).structural_child_index as
                                               isize) as libc::c_int != 0
        }
        if ts_subtree_visible(*(*entry).subtree) as libc::c_int != 0 ||
               is_aliased as libc::c_int != 0 {
            (*self_0).stack.size =
                i.wrapping_add(1 as libc::c_int as libc::c_uint);
            return 1 as libc::c_int != 0
        }
        i = i.wrapping_sub(1)
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_node(mut _self:
                                                         *const TSTreeCursor)
 -> TSNode {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    if (*self_0).stack.size.wrapping_sub(1 as libc::c_int as libc::c_uint) <
           (*self_0).stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->stack)->size - 1 < (&self->stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/./tree_cursor.c\x00" as *const u8 as
                          *const libc::c_char,
                      227 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 57],
                                                &[libc::c_char; 57]>(b"TSNode ts_tree_cursor_current_node(const TSTreeCursor *)\x00")).as_ptr());
    }
    let mut last_entry: *mut TreeCursorEntry =
        &mut *(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(1
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    libc::c_uint)
                                                  as isize) as
            *mut TreeCursorEntry;
    let mut alias_symbol: TSSymbol = 0 as libc::c_int as TSSymbol;
    if (*self_0).stack.size > 1 as libc::c_int as libc::c_uint {
        let mut parent_entry: *mut TreeCursorEntry =
            &mut *(*self_0).stack.contents.offset((*self_0).stack.size.wrapping_sub(2
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_uint)
                                                      as isize) as
                *mut TreeCursorEntry;
        let mut alias_sequence: *const TSSymbol =
            ts_language_alias_sequence((*(*self_0).tree).language,
                                       (*(*(*parent_entry).subtree).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                           as uint32_t);
        if !alias_sequence.is_null() &&
               !ts_subtree_extra(*(*last_entry).subtree) {
            alias_symbol =
                *alias_sequence.offset((*last_entry).structural_child_index as
                                           isize)
        }
    }
    return ts_node_new((*self_0).tree, (*last_entry).subtree,
                       (*last_entry).position, alias_symbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_status(mut _self:
                                                           *const TSTreeCursor,
                                                       mut can_have_later_siblings:
                                                           *mut bool,
                                                       mut can_have_later_siblings_with_this_field:
                                                           *mut bool)
 -> TSFieldId {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    let mut result: TSFieldId = 0 as libc::c_int as TSFieldId;
    *can_have_later_siblings = 0 as libc::c_int != 0;
    *can_have_later_siblings_with_this_field = 0 as libc::c_int != 0;
    // Walk up the tree, visiting the current node and its invisible ancestors,
  // because fields can refer to nodes through invisible *wrapper* nodes,
    let mut i: libc::c_uint =
        (*self_0).stack.size.wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i > 0 as libc::c_int as libc::c_uint {
        let mut entry: *mut TreeCursorEntry =
            &mut *(*self_0).stack.contents.offset(i as isize) as
                *mut TreeCursorEntry;
        let mut parent_entry: *mut TreeCursorEntry =
            &mut *(*self_0).stack.contents.offset(i.wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as isize) as
                *mut TreeCursorEntry;
        // Stop walking up when a visible ancestor is found.
        if i !=
               (*self_0).stack.size.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint) {
            if ts_subtree_visible(*(*entry).subtree) { break ; }
            let mut alias_sequence: *const TSSymbol =
                ts_language_alias_sequence((*(*self_0).tree).language,
                                           (*(*(*parent_entry).subtree).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                               as uint32_t);
            if !alias_sequence.is_null() &&
                   *alias_sequence.offset((*entry).structural_child_index as
                                              isize) as libc::c_int != 0 {
                break ;
            }
        }
        if ts_subtree_child_count(*(*parent_entry).subtree) >
               (*entry).child_index.wrapping_add(1 as libc::c_int as
                                                     libc::c_uint) {
            *can_have_later_siblings = 1 as libc::c_int != 0
        }
        if ts_subtree_extra(*(*entry).subtree) { break ; }
        let mut field_map: *const TSFieldMapEntry =
            0 as *const TSFieldMapEntry;
        let mut field_map_end: *const TSFieldMapEntry =
            0 as *const TSFieldMapEntry;
        ts_language_field_map((*(*self_0).tree).language,
                              (*(*(*parent_entry).subtree).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                  as uint32_t, &mut field_map,
                              &mut field_map_end);
        // Look for a field name associated with the current node.
        if result == 0 {
            let mut i_0: *const TSFieldMapEntry = field_map;
            while i_0 < field_map_end {
                if !(*i_0).inherited &&
                       (*i_0).child_index as libc::c_uint ==
                           (*entry).structural_child_index {
                    result = (*i_0).field_id;
                    *can_have_later_siblings_with_this_field =
                        0 as libc::c_int != 0;
                    break ;
                } else { i_0 = i_0.offset(1) }
            }
        }
        // Determine if there other later siblings with the same field name.
        if result != 0 {
            let mut i_1: *const TSFieldMapEntry = field_map;
            while i_1 < field_map_end {
                if (*i_1).field_id as libc::c_int == result as libc::c_int &&
                       (*i_1).child_index as libc::c_uint >
                           (*entry).structural_child_index {
                    *can_have_later_siblings_with_this_field =
                        1 as libc::c_int != 0;
                    break ;
                } else { i_1 = i_1.offset(1) }
            }
        }
        i = i.wrapping_sub(1)
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_field_id(mut _self:
                                                             *const TSTreeCursor)
 -> TSFieldId {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    // Walk up the tree, visiting the current node and its invisible ancestors.
    let mut i: libc::c_uint =
        (*self_0).stack.size.wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i > 0 as libc::c_int as libc::c_uint {
        let mut entry: *mut TreeCursorEntry =
            &mut *(*self_0).stack.contents.offset(i as isize) as
                *mut TreeCursorEntry;
        let mut parent_entry: *mut TreeCursorEntry =
            &mut *(*self_0).stack.contents.offset(i.wrapping_sub(1 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_uint)
                                                      as isize) as
                *mut TreeCursorEntry;
        // Stop walking up when another visible node is found.
        if i !=
               (*self_0).stack.size.wrapping_sub(1 as libc::c_int as
                                                     libc::c_uint) {
            if ts_subtree_visible(*(*entry).subtree) { break ; }
            let mut alias_sequence: *const TSSymbol =
                ts_language_alias_sequence((*(*self_0).tree).language,
                                           (*(*(*parent_entry).subtree).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                               as uint32_t);
            if !alias_sequence.is_null() &&
                   *alias_sequence.offset((*entry).structural_child_index as
                                              isize) as libc::c_int != 0 {
                break ;
            }
        }
        if ts_subtree_extra(*(*entry).subtree) { break ; }
        let mut field_map: *const TSFieldMapEntry =
            0 as *const TSFieldMapEntry;
        let mut field_map_end: *const TSFieldMapEntry =
            0 as *const TSFieldMapEntry;
        ts_language_field_map((*(*self_0).tree).language,
                              (*(*(*parent_entry).subtree).ptr).c2rust_unnamed.c2rust_unnamed.production_id
                                  as uint32_t, &mut field_map,
                              &mut field_map_end);
        let mut i_0: *const TSFieldMapEntry = field_map;
        while i_0 < field_map_end {
            if !(*i_0).inherited &&
                   (*i_0).child_index as libc::c_uint ==
                       (*entry).structural_child_index {
                return (*i_0).field_id
            }
            i_0 = i_0.offset(1)
        }
        i = i.wrapping_sub(1)
    }
    return 0 as libc::c_int as TSFieldId;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_current_field_name(mut _self:
                                                               *const TSTreeCursor)
 -> *const libc::c_char {
    let mut id: TSFieldId = ts_tree_cursor_current_field_id(_self);
    if id != 0 {
        let mut self_0: *const TreeCursor = _self as *const TreeCursor;
        return *(*(*(*self_0).tree).language).field_names.offset(id as isize)
    } else { return 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_cursor_copy(mut _cursor: *const TSTreeCursor)
 -> TSTreeCursor {
    let mut cursor: *const TreeCursor = _cursor as *const TreeCursor;
    let mut res: TSTreeCursor =
        {
            let mut init =
                TSTreeCursor{tree: 0 as *const libc::c_void,
                             id: 0 as *const libc::c_void,
                             context:
                                 [0 as libc::c_int as uint32_t,
                                  0 as libc::c_int as uint32_t],};
            init
        };
    let mut copy: *mut TreeCursor =
        &mut res as *mut TSTreeCursor as *mut TreeCursor;
    (*copy).tree = (*cursor).tree;
    array__splice(&mut (*copy).stack as *mut C2RustUnnamed_22 as
                      *mut VoidArray,
                  ::std::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
                  (*copy).stack.size, 0 as libc::c_int as uint32_t,
                  (*cursor).stack.size,
                  (*cursor).stack.contents as *const libc::c_void);
    return res;
}
static mut PARENT_CACHE_CAPACITY: libc::c_uint =
    32 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn ts_tree_new(mut root: Subtree,
                                     mut language: *const TSLanguage,
                                     mut included_ranges: *const TSRange,
                                     mut included_range_count: libc::c_uint)
 -> *mut TSTree {
    let mut result: *mut TSTree =
        ts_malloc(::std::mem::size_of::<TSTree>() as libc::c_ulong) as
            *mut TSTree;
    (*result).root = root;
    (*result).language = language;
    (*result).parent_cache = 0 as *mut ParentCacheEntry;
    (*result).parent_cache_start = 0 as libc::c_int as uint32_t;
    (*result).parent_cache_size = 0 as libc::c_int as uint32_t;
    (*result).included_ranges =
        ts_calloc(included_range_count as size_t,
                  ::std::mem::size_of::<TSRange>() as libc::c_ulong) as
            *mut TSRange;
    memcpy((*result).included_ranges as *mut libc::c_void,
           included_ranges as *const libc::c_void,
           (included_range_count as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<TSRange>()
                                                as libc::c_ulong));
    (*result).included_range_count = included_range_count;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_copy(mut self_0: *const TSTree)
 -> *mut TSTree {
    ts_subtree_retain((*self_0).root);
    return ts_tree_new((*self_0).root, (*self_0).language,
                       (*self_0).included_ranges,
                       (*self_0).included_range_count);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_delete(mut self_0: *mut TSTree) {
    if self_0.is_null() { return }
    let mut pool: SubtreePool =
        ts_subtree_pool_new(0 as libc::c_int as uint32_t);
    ts_subtree_release(&mut pool, (*self_0).root);
    ts_subtree_pool_delete(&mut pool);
    ts_free((*self_0).included_ranges as *mut libc::c_void);
    if !(*self_0).parent_cache.is_null() {
        ts_free((*self_0).parent_cache as *mut libc::c_void);
    }
    ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_root_node(mut self_0: *const TSTree)
 -> TSNode {
    return ts_node_new(self_0, &(*self_0).root,
                       ts_subtree_padding((*self_0).root),
                       0 as libc::c_int as TSSymbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_language(mut self_0: *const TSTree)
 -> *const TSLanguage {
    return (*self_0).language;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_edit(mut self_0: *mut TSTree,
                                      mut edit: *const TSInputEdit) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).included_range_count {
        let mut range: *mut TSRange =
            &mut *(*self_0).included_ranges.offset(i as isize) as
                *mut TSRange;
        if (*range).end_byte >= (*edit).old_end_byte {
            if (*range).end_byte != 4294967295 as libc::c_uint {
                (*range).end_byte =
                    (*edit).new_end_byte.wrapping_add((*range).end_byte.wrapping_sub((*edit).old_end_byte));
                (*range).end_point =
                    point_add((*edit).new_end_point,
                              point_sub((*range).end_point,
                                        (*edit).old_end_point));
                if (*range).end_byte < (*edit).new_end_byte {
                    (*range).end_byte = 4294967295 as libc::c_uint;
                    (*range).end_point =
                        {
                            let mut init =
                                TSPoint{row: 4294967295 as libc::c_uint,
                                        column: 4294967295 as libc::c_uint,};
                            init
                        }
                }
            }
            if (*range).start_byte >= (*edit).old_end_byte {
                (*range).start_byte =
                    (*edit).new_end_byte.wrapping_add((*range).start_byte.wrapping_sub((*edit).old_end_byte));
                (*range).start_point =
                    point_add((*edit).new_end_point,
                              point_sub((*range).start_point,
                                        (*edit).old_end_point));
                if (*range).start_byte < (*edit).new_end_byte {
                    (*range).start_byte = 4294967295 as libc::c_uint;
                    (*range).start_point =
                        {
                            let mut init =
                                TSPoint{row: 4294967295 as libc::c_uint,
                                        column: 4294967295 as libc::c_uint,};
                            init
                        }
                }
            }
        }
        i = i.wrapping_add(1)
    }
    let mut pool: SubtreePool =
        ts_subtree_pool_new(0 as libc::c_int as uint32_t);
    (*self_0).root = ts_subtree_edit((*self_0).root, edit, &mut pool);
    (*self_0).parent_cache_start = 0 as libc::c_int as uint32_t;
    (*self_0).parent_cache_size = 0 as libc::c_int as uint32_t;
    ts_subtree_pool_delete(&mut pool);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_get_changed_ranges(mut self_0: *const TSTree,
                                                    mut other: *const TSTree,
                                                    mut count: *mut uint32_t)
 -> *mut TSRange {
    let mut cursor1: TreeCursor =
        {
            let mut init =
                TreeCursor{tree: 0 as *const TSTree,
                           stack:
                               {
                                   let mut init =
                                       C2RustUnnamed_22{contents:
                                                            0 as
                                                                *mut TreeCursorEntry,
                                                        size:
                                                            0 as libc::c_int
                                                                as uint32_t,
                                                        capacity:
                                                            0 as libc::c_int
                                                                as uint32_t,};
                                   init
                               },};
            init
        };
    let mut cursor2: TreeCursor =
        {
            let mut init =
                TreeCursor{tree: 0 as *const TSTree,
                           stack:
                               {
                                   let mut init =
                                       C2RustUnnamed_22{contents:
                                                            0 as
                                                                *mut TreeCursorEntry,
                                                        size:
                                                            0 as libc::c_int
                                                                as uint32_t,
                                                        capacity:
                                                            0 as libc::c_int
                                                                as uint32_t,};
                                   init
                               },};
            init
        };
    ts_tree_cursor_init(&mut cursor1, ts_tree_root_node(self_0));
    ts_tree_cursor_init(&mut cursor2, ts_tree_root_node(other));
    let mut included_range_differences: TSRangeArray =
        {
            let mut init =
                TSRangeArray{contents: 0 as *mut TSRange,
                             size: 0 as libc::c_int as uint32_t,
                             capacity: 0 as libc::c_int as uint32_t,};
            init
        };
    ts_range_array_get_changed_ranges((*self_0).included_ranges,
                                      (*self_0).included_range_count,
                                      (*other).included_ranges,
                                      (*other).included_range_count,
                                      &mut included_range_differences);
    let mut result: *mut TSRange = 0 as *mut TSRange;
    *count =
        ts_subtree_get_changed_ranges(&(*self_0).root, &(*other).root,
                                      &mut cursor1, &mut cursor2,
                                      (*self_0).language,
                                      &mut included_range_differences,
                                      &mut result);
    array__delete(&mut included_range_differences as *mut TSRangeArray as
                      *mut VoidArray);
    array__delete(&mut cursor1.stack as *mut C2RustUnnamed_22 as
                      *mut VoidArray);
    array__delete(&mut cursor2.stack as *mut C2RustUnnamed_22 as
                      *mut VoidArray);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_print_dot_graph(mut self_0: *const TSTree,
                                                 mut file: *mut FILE) {
    ts_subtree_print_dot_graph((*self_0).root, (*self_0).language, file);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_get_cached_parent(mut self_0: *const TSTree,
                                                   mut node: *const TSNode)
 -> TSNode {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).parent_cache_size {
        let mut index: uint32_t =
            (*self_0).parent_cache_start.wrapping_add(i).wrapping_rem(PARENT_CACHE_CAPACITY);
        let mut entry: *mut ParentCacheEntry =
            &mut *(*self_0).parent_cache.offset(index as isize) as
                *mut ParentCacheEntry;
        if (*entry).child == (*node).id as *const Subtree {
            return ts_node_new(self_0, (*entry).parent, (*entry).position,
                               (*entry).alias_symbol)
        }
        i = i.wrapping_add(1)
    }
    return ts_node_new(0 as *const TSTree, 0 as *const Subtree, length_zero(),
                       0 as libc::c_int as TSSymbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_set_cached_parent(mut _self: *const TSTree,
                                                   mut node: *const TSNode,
                                                   mut parent:
                                                       *const TSNode) {
    let mut self_0: *mut TSTree = _self as *mut TSTree;
    if (*self_0).parent_cache.is_null() {
        (*self_0).parent_cache =
            ts_calloc(PARENT_CACHE_CAPACITY as size_t,
                      ::std::mem::size_of::<ParentCacheEntry>() as
                          libc::c_ulong) as *mut ParentCacheEntry
    }
    let mut index: uint32_t =
        (*self_0).parent_cache_start.wrapping_add((*self_0).parent_cache_size).wrapping_rem(PARENT_CACHE_CAPACITY);
    *(*self_0).parent_cache.offset(index as isize) =
        {
            let mut init =
                ParentCacheEntry{child: (*node).id as *const Subtree,
                                 parent: (*parent).id as *const Subtree,
                                 position:
                                     {
                                         let mut init =
                                             Length{bytes:
                                                        (*parent).context[0 as
                                                                              libc::c_int
                                                                              as
                                                                              usize],
                                                    extent:
                                                        {
                                                            let mut init =
                                                                TSPoint{row:
                                                                            (*parent).context[1
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize],
                                                                        column:
                                                                            (*parent).context[2
                                                                                                  as
                                                                                                  libc::c_int
                                                                                                  as
                                                                                                  usize],};
                                                            init
                                                        },};
                                         init
                                     },
                                 alias_symbol:
                                     (*parent).context[3 as libc::c_int as
                                                           usize] as
                                         TSSymbol,};
            init
        };
    if (*self_0).parent_cache_size == PARENT_CACHE_CAPACITY {
        (*self_0).parent_cache_start =
            (*self_0).parent_cache_start.wrapping_add(1)
    } else {
        (*self_0).parent_cache_size =
            (*self_0).parent_cache_size.wrapping_add(1)
    };
}
// The Tree-sitter library can be built by compiling this one source file.
//
// The following directories must be added to the include path:
//   - include
