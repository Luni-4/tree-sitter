#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(label_break_value, register_tool)]
extern "C" {
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(__assertion: *const libc::c_char,
                     __file: *const libc::c_char, __line: libc::c_uint,
                     __function: *const libc::c_char) -> !;
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
pub type TSSymbolType = libc::c_uint;
pub const TSSymbolTypeAuxiliary: TSSymbolType = 2;
pub const TSSymbolTypeAnonymous: TSSymbolType = 1;
pub const TSSymbolTypeRegular: TSSymbolType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableEntry {
    pub actions: *const TSParseAction,
    pub action_count: uint32_t,
    pub is_reusable: bool,
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
        let fresh0 = data;
        data = data.offset(1);
        let mut section_count: uint16_t = *fresh0;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < section_count as libc::c_uint {
            let fresh1 = data;
            data = data.offset(1);
            let mut section_value: uint16_t = *fresh1;
            let fresh2 = data;
            data = data.offset(1);
            let mut symbol_count: uint16_t = *fresh2;
            let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while i_0 < symbol_count as libc::c_uint {
                let fresh3 = data;
                data = data.offset(1);
                if *fresh3 as libc::c_int == symbol as libc::c_int {
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
                          b"lib/src/language.c\x00" as *const u8 as
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
