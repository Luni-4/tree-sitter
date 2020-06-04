#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn ts_tree_root_node(self_0: *const TSTree) -> TSNode;
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
    /* *
 * Advance to the next capture of the currently running query.
 *
 * If there is a capture, write its match to `*match` and its index within
 * the matche's capture list to `*capture_index`. Otherwise, return `false`.
 */
    /* *********************/
/* Section - Language */
/* *********************/
    /* *
 * Get the number of distinct node types in the language.
 */
    /* *
 * Get a node type string for the given numerical id.
 */
    #[no_mangle]
    fn ts_language_symbol_name(_: *const TSLanguage, _: TSSymbol)
     -> *const libc::c_char;
    #[no_mangle]
    fn ts_subtree_string(_: Subtree, _: *const TSLanguage, include_all: bool)
     -> *mut libc::c_char;
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
    fn ts_language_field_id_for_name(_: *const TSLanguage,
                                     _: *const libc::c_char, _: uint32_t)
     -> TSFieldId;
    #[no_mangle]
    fn ts_tree_get_cached_parent(_: *const TSTree, _: *const TSNode)
     -> TSNode;
    #[no_mangle]
    fn ts_tree_set_cached_parent(_: *const TSTree, _: *const TSNode,
                                 _: *const TSNode);
    #[no_mangle]
    fn ts_language_symbol_metadata(_: *const TSLanguage, _: TSSymbol)
     -> TSSymbolMetadata;
    #[no_mangle]
    fn ts_language_public_symbol(_: *const TSLanguage, _: TSSymbol)
     -> TSSymbol;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct ParentCacheEntry {
    pub child: *const Subtree,
    pub parent: *const Subtree,
    pub position: Length,
    pub alias_symbol: TSSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Length {
    pub bytes: uint32_t,
    pub extent: TSPoint,
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
pub struct TSNode {
    pub context: [uint32_t; 4],
    pub id: *const libc::c_void,
    pub tree: *const TSTree,
}
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
#[inline]
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.symbol as libc::c_int
           } else { (*self_0.ptr).symbol as libc::c_int } as TSSymbol;
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
unsafe extern "C" fn ts_subtree_named(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.named() as libc::c_int
           } else { (*self_0.ptr).named() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_missing(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.is_missing() as libc::c_int
           } else { (*self_0.ptr).is_missing() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_extra(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.extra() as libc::c_int
           } else { (*self_0.ptr).extra() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_has_changes(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.has_changes() as libc::c_int
           } else { (*self_0.ptr).has_changes() as libc::c_int } != 0;
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
unsafe extern "C" fn ts_subtree_visible(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.visible() as libc::c_int
           } else { (*self_0.ptr).visible() as libc::c_int } != 0;
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
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int as libc::c_uint
           } else { (*self_0.ptr).child_count };
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
unsafe extern "C" fn ts_subtree_total_bytes(mut self_0: Subtree) -> uint32_t {
    return ts_subtree_total_size(self_0).bytes;
}
#[inline]
unsafe extern "C" fn ts_subtree_total_size(mut self_0: Subtree) -> Length {
    return length_add(ts_subtree_padding(self_0), ts_subtree_size(self_0));
}
#[inline]
unsafe extern "C" fn point_lt(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row < b.row || a.row == b.row && a.column < b.column;
}
#[inline]
unsafe extern "C" fn point_lte(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row < b.row || a.row == b.row && a.column <= b.column;
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
/* *
 * Check if two nodes are identical.
 */
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
    'c_3733:
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
                        continue 'c_3733 ;
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
pub unsafe extern "C" fn ts_node_next_named_sibling(mut self_0: TSNode)
 -> TSNode {
    return ts_node__next_sibling(self_0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_prev_sibling(mut self_0: TSNode) -> TSNode {
    return ts_node__prev_sibling(self_0, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_prev_named_sibling(mut self_0: TSNode)
 -> TSNode {
    return ts_node__prev_sibling(self_0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_first_child_for_byte(mut self_0: TSNode,
                                                      mut byte: uint32_t)
 -> TSNode {
    return ts_node__first_child_for_byte(self_0, byte, 1 as libc::c_int != 0);
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
pub unsafe extern "C" fn ts_node_descendant_for_byte_range(mut self_0: TSNode,
                                                           mut start:
                                                               uint32_t,
                                                           mut end: uint32_t)
 -> TSNode {
    return ts_node__descendant_for_byte_range(self_0, start, end,
                                              1 as libc::c_int != 0);
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
