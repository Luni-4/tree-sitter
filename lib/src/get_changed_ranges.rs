#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, label_break_value,
           register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type TSTree;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
pub struct TSPoint {
    pub row: uint32_t,
    pub column: uint32_t,
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
pub struct Length {
    pub bytes: uint32_t,
    pub extent: TSPoint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
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
pub union Subtree {
    pub data: SubtreeInlineData,
    pub ptr: *const SubtreeHeapData,
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
pub struct TreeCursorEntry {
    pub subtree: *const Subtree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
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
pub struct TSRangeArray {
    pub contents: *mut TSRange,
    pub size: uint32_t,
    pub capacity: uint32_t,
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
#[inline]
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               0 as libc::c_int as libc::c_uint
           } else { (*self_0.ptr).child_count };
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
unsafe extern "C" fn ts_subtree_parse_state(mut self_0: Subtree)
 -> TSStateId {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.parse_state as libc::c_int
           } else { (*self_0.ptr).parse_state as libc::c_int } as TSStateId;
}
#[inline]
unsafe extern "C" fn ts_subtree_has_changes(mut self_0: Subtree) -> bool {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.has_changes() as libc::c_int
           } else { (*self_0.ptr).has_changes() as libc::c_int } != 0;
}
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
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return if self_0.data.is_inline() as libc::c_int != 0 {
               self_0.data.symbol as libc::c_int
           } else { (*self_0.ptr).symbol as libc::c_int } as TSSymbol;
}
static mut TS_TREE_STATE_NONE: TSStateId =
    (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as TSStateId;
#[inline]
unsafe extern "C" fn point__new(mut row: libc::c_uint,
                                mut column: libc::c_uint) -> TSPoint {
    let mut result: TSPoint =
        { let mut init = TSPoint{row: row, column: column,}; init };
    return result;
}
#[inline]
unsafe extern "C" fn point_add(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if b.row > 0 as libc::c_int as libc::c_uint {
        return point__new(a.row.wrapping_add(b.row), b.column)
    } else { return point__new(a.row, a.column.wrapping_add(b.column)) };
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
#[inline]
unsafe extern "C" fn length_min(mut len1: Length, mut len2: Length)
 -> Length {
    return if len1.bytes < len2.bytes { len1 } else { len2 };
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
// #define DEBUG_GET_CHANGED_RANGES
unsafe extern "C" fn ts_range_array_add(mut self_0: *mut TSRangeArray,
                                        mut start: Length, mut end: Length) {
    if (*self_0).size > 0 as libc::c_int as libc::c_uint {
        if (*self_0).size.wrapping_sub(1 as libc::c_int as libc::c_uint) <
               (*self_0).size {
        } else {
            __assert_fail(b"(uint32_t)(self)->size - 1 < (self)->size\x00" as
                              *const u8 as *const libc::c_char,
                          b"lib/src/get_changed_ranges.c\x00" as *const u8 as
                              *const libc::c_char,
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
        let fresh0 = (*self_0).size;
        (*self_0).size = (*self_0).size.wrapping_add(1);
        *(*self_0).contents.offset(fresh0 as isize) = range
    };
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
unsafe extern "C" fn iterator_new(mut cursor: *mut TreeCursor,
                                  mut tree: *const Subtree,
                                  mut language: *const TSLanguage)
 -> Iterator_0 {
    (*cursor).stack.size = 0 as libc::c_int as uint32_t;
    array__grow(&mut (*cursor).stack as *mut C2RustUnnamed_8 as
                    *mut VoidArray, 1 as libc::c_int as size_t,
                ::std::mem::size_of::<TreeCursorEntry>() as libc::c_ulong);
    let fresh1 = (*cursor).stack.size;
    (*cursor).stack.size = (*cursor).stack.size.wrapping_add(1);
    *(*cursor).stack.contents.offset(fresh1 as isize) =
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
unsafe extern "C" fn iterator_done(mut self_0: *mut Iterator_0) -> bool {
    return (*self_0).cursor.stack.size == 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn iterator_start_position(mut self_0: *mut Iterator_0)
 -> Length {
    if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) <
           (*self_0).cursor.stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/get_changed_ranges.c\x00" as *const u8 as
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
unsafe extern "C" fn iterator_end_position(mut self_0: *mut Iterator_0)
 -> Length {
    if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) <
           (*self_0).cursor.stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/get_changed_ranges.c\x00" as *const u8 as
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
unsafe extern "C" fn iterator_tree_is_visible(mut self_0: *const Iterator_0)
 -> bool {
    if (*self_0).cursor.stack.size.wrapping_sub(1 as libc::c_int as
                                                    libc::c_uint) <
           (*self_0).cursor.stack.size {
    } else {
        __assert_fail(b"(uint32_t)(&self->cursor.stack)->size - 1 < (&self->cursor.stack)->size\x00"
                          as *const u8 as *const libc::c_char,
                      b"lib/src/get_changed_ranges.c\x00" as *const u8 as
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
                      b"lib/src/get_changed_ranges.c\x00" as *const u8 as
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
                          b"lib/src/get_changed_ranges.c\x00" as *const u8 as
                              *const libc::c_char,
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
                                *mut C2RustUnnamed_8 as *mut VoidArray,
                            1 as libc::c_int as size_t,
                            ::std::mem::size_of::<TreeCursorEntry>() as
                                libc::c_ulong);
                let fresh2 = (*self_0).cursor.stack.size;
                (*self_0).cursor.stack.size =
                    (*self_0).cursor.stack.size.wrapping_add(1);
                *(*self_0).cursor.stack.contents.offset(fresh2 as isize) =
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
                          b"lib/src/get_changed_ranges.c\x00" as *const u8 as
                              *const libc::c_char,
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
        array__grow(&mut (*self_0).cursor.stack as *mut C2RustUnnamed_8 as
                        *mut VoidArray, 1 as libc::c_int as size_t,
                    ::std::mem::size_of::<TreeCursorEntry>() as
                        libc::c_ulong);
        let fresh3 = (*self_0).cursor.stack.size;
        (*self_0).cursor.stack.size =
            (*self_0).cursor.stack.size.wrapping_add(1);
        *(*self_0).cursor.stack.contents.offset(fresh3 as isize) =
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
