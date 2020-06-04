#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn ts_node_start_point(_: TSNode) -> TSPoint;
    #[no_mangle]
    fn ts_node_start_byte(_: TSNode) -> uint32_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
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
pub type __int64_t = libc::c_long;
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
pub type int64_t = __int64_t;
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
pub struct TSNode {
    pub context: [uint32_t; 4],
    pub id: *const libc::c_void,
    pub tree: *const TSTree,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSTreeCursor {
    pub tree: *const libc::c_void,
    pub id: *const libc::c_void,
    pub context: [uint32_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeCursor {
    pub tree: *const TSTree,
    pub stack: C2RustUnnamed_8,
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
pub struct TreeCursorEntry {
    pub subtree: *const Subtree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
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
pub struct CursorChildIterator {
    pub parent: Subtree,
    pub tree: *const TSTree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
    pub alias_sequence: *const TSSymbol,
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
unsafe extern "C" fn ts_free(mut buffer: *mut libc::c_void) { free(buffer); }
#[inline]
unsafe extern "C" fn ts_subtree_extra(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.extra() as libc::c_int
           } else { (*self_0.ptr).extra() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_visible(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.visible() as libc::c_int
           } else { (*self_0.ptr).visible() as libc::c_int } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_visible_child_count(mut self_0: Subtree)
 -> uint32_t {
    if ts_subtree_child_count(self_0) > 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.visible_child_count
    } else { return 0 as libc::c_int as uint32_t };
}
#[inline]
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int as libc::c_uint
           } else { (*self_0.ptr).child_count };
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
unsafe extern "C" fn array__delete(mut self_0: *mut VoidArray) {
    ts_free((*self_0).contents);
    (*self_0).contents = 0 as *mut libc::c_void;
    (*self_0).size = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
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
                      b"lib/src/tree_cursor.c\x00" as *const u8 as
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
    array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<TreeCursorEntry>() as libc::c_ulong);
    let fresh0 = (*self_0).stack.size;
    (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
    *(*self_0).stack.contents.offset(fresh0 as isize) =
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
    array__delete(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
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
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh1 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh1 as isize) = entry;
                return 1 as libc::c_int != 0
            }
            if !(ts_subtree_visible_child_count(*entry.subtree) >
                     0 as libc::c_int as libc::c_uint) {
                continue ;
            }
            array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                            *mut VoidArray, 1 as libc::c_int as size_t,
                        ::std::mem::size_of::<TreeCursorEntry>() as
                            libc::c_ulong);
            let fresh2 = (*self_0).stack.size;
            (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
            *(*self_0).stack.contents.offset(fresh2 as isize) = entry;
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
                    array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8
                                    as *mut VoidArray,
                                1 as libc::c_int as size_t,
                                ::std::mem::size_of::<TreeCursorEntry>() as
                                    libc::c_ulong);
                    let fresh3 = (*self_0).stack.size;
                    (*self_0).stack.size =
                        (*self_0).stack.size.wrapping_add(1);
                    *(*self_0).stack.contents.offset(fresh3 as isize) = entry;
                    return visible_child_index as int64_t
                }
                if !(visible_child_count > 0 as libc::c_int as libc::c_uint) {
                    continue ;
                }
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh4 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh4 as isize) = entry;
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
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh5 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh5 as isize) = entry;
                return 1 as libc::c_int != 0
            }
            if ts_subtree_visible_child_count(*entry.subtree) != 0 {
                array__grow(&mut (*self_0).stack as *mut C2RustUnnamed_8 as
                                *mut VoidArray, 1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh6 = (*self_0).stack.size;
                (*self_0).stack.size = (*self_0).stack.size.wrapping_add(1);
                *(*self_0).stack.contents.offset(fresh6 as isize) = entry;
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
                      b"lib/src/tree_cursor.c\x00" as *const u8 as
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
    array__splice(&mut (*copy).stack as *mut C2RustUnnamed_8 as
                      *mut VoidArray,
                  ::std::mem::size_of::<TreeCursorEntry>() as libc::c_ulong,
                  (*copy).stack.size, 0 as libc::c_int as uint32_t,
                  (*cursor).stack.size,
                  (*cursor).stack.contents as *const libc::c_void);
    return res;
}
