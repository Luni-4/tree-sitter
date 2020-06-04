#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn ts_subtree_pool_new(capacity: uint32_t) -> SubtreePool;
    #[no_mangle]
    fn ts_subtree_pool_delete(_: *mut SubtreePool);
    #[no_mangle]
    fn ts_subtree_retain(_: Subtree);
    #[no_mangle]
    fn ts_subtree_release(_: *mut SubtreePool, _: Subtree);
    #[no_mangle]
    fn ts_subtree_edit(_: Subtree, edit: *const TSInputEdit,
                       _: *mut SubtreePool) -> Subtree;
    #[no_mangle]
    fn ts_subtree_print_dot_graph(_: Subtree, _: *const TSLanguage,
                                  _: *mut FILE);
    #[no_mangle]
    fn ts_tree_cursor_init(_: *mut TreeCursor, _: TSNode);
    #[no_mangle]
    fn ts_range_array_get_changed_ranges(old_ranges: *const TSRange,
                                         old_range_count: libc::c_uint,
                                         new_ranges: *const TSRange,
                                         new_range_count: libc::c_uint,
                                         differences: *mut TSRangeArray);
    #[no_mangle]
    fn ts_subtree_get_changed_ranges(old_tree: *const Subtree,
                                     new_tree: *const Subtree,
                                     cursor1: *mut TreeCursor,
                                     cursor2: *mut TreeCursor,
                                     language: *const TSLanguage,
                                     included_range_differences:
                                         *const TSRangeArray,
                                     ranges: *mut *mut TSRange)
     -> libc::c_uint;
    #[no_mangle]
    fn ts_node_new(_: *const TSTree, _: *const Subtree, _: Length,
                   _: TSSymbol) -> TSNode;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct TreeCursorEntry {
    pub subtree: *const Subtree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
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
pub union MutableSubtree {
    pub data: SubtreeInlineData,
    pub ptr: *mut SubtreeHeapData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub contents: *mut TreeCursorEntry,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeCursor {
    pub tree: *const TSTree,
    pub stack: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSRangeArray {
    pub contents: *mut TSRange,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
// Private
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
unsafe extern "C" fn point_add(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if b.row > 0 as libc::c_int as libc::c_uint {
        return point__new(a.row.wrapping_add(b.row), b.column)
    } else { return point__new(a.row, a.column.wrapping_add(b.column)) };
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
unsafe extern "C" fn point__new(mut row: libc::c_uint,
                                mut column: libc::c_uint) -> TSPoint {
    let mut result: TSPoint =
        { let mut init = TSPoint{row: row, column: column,}; init };
    return result;
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
                                       C2RustUnnamed_8{contents:
                                                           0 as
                                                               *mut TreeCursorEntry,
                                                       size:
                                                           0 as libc::c_int as
                                                               uint32_t,
                                                       capacity:
                                                           0 as libc::c_int as
                                                               uint32_t,};
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
                                       C2RustUnnamed_8{contents:
                                                           0 as
                                                               *mut TreeCursorEntry,
                                                       size:
                                                           0 as libc::c_int as
                                                               uint32_t,
                                                       capacity:
                                                           0 as libc::c_int as
                                                               uint32_t,};
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
    array__delete(&mut cursor1.stack as *mut C2RustUnnamed_8 as
                      *mut VoidArray);
    array__delete(&mut cursor2.stack as *mut C2RustUnnamed_8 as
                      *mut VoidArray);
    return result;
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
