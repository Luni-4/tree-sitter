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
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type TSSymbol = uint16_t;
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
pub type TSInputEncoding = libc::c_uint;
pub const TSInputEncodingUTF16: TSInputEncoding = 1;
pub const TSInputEncodingUTF8: TSInputEncoding = 0;
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
pub struct TSInput {
    pub payload: *mut libc::c_void,
    pub read: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: uint32_t,
                                          _: TSPoint, _: *mut uint32_t)
                         -> *const libc::c_char>,
    pub encoding: TSInputEncoding,
}
pub type TSLogType = libc::c_uint;
pub const TSLogTypeLex: TSLogType = 1;
pub const TSLogTypeParse: TSLogType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLogger {
    pub payload: *mut libc::c_void,
    pub log: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: TSLogType,
                                         _: *const libc::c_char) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Length {
    pub bytes: uint32_t,
    pub extent: TSPoint,
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
// These functions read one unicode code point from the given string,
// returning the number of bytes consumed.
pub type UnicodeDecodeFunction
    =
    Option<unsafe extern "C" fn(_: *const uint8_t, _: uint32_t,
                                _: *mut int32_t) -> uint32_t>;
// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
******************************************************************************
*
*   Copyright (C) 1999-2015, International Business Machines
*   Corporation and others.  All Rights Reserved.
*
******************************************************************************
*   file name:  umachine.h
*   encoding:   UTF-8
*   tab size:   8 (not used)
*   indentation:4
*
*   created on: 1999sep13
*   created by: Markus W. Scherer
*
*   This file defines basic types and constants for ICU to be
*   platform-independent. umachine.h and utf.h are included into
*   utypes.h to provide all the general definitions for ICU.
*   All of these definitions used to be in utypes.h before
*   the UTF-handling macros made this unmaintainable.
*/
/* *
 * \file
 * \brief Basic types and constants for UTF
 *
 * <h2> Basic types and constants for UTF </h2>
 *   This file defines basic types and constants for utf.h to be
 *   platform-independent. umachine.h and utf.h are included into
 *   utypes.h to provide all the general definitions for ICU.
 *   All of these definitions used to be in utypes.h before
 *   the UTF-handling macros made this unmaintainable.
 *
 */
/*==========================================================================*/
/* Include platform-dependent definitions                                   */
/* which are contained in the platform-specific file platform.h             */
/*==========================================================================*/
/*
 * ANSI C headers:
 * stddef.h defines wchar_t
 */
/*==========================================================================*/
/* For C wrappers, we use the symbol U_STABLE.                                */
/* This works properly if the includer is C or C++.                         */
/* Functions are declared   U_STABLE return-type U_EXPORT2 function-name()... */
/*==========================================================================*/
/* *
 * \def U_CFUNC
 * This is used in a declaration of a library private ICU C function.
 * @stable ICU 2.4
 */
/* *
 * \def U_CDECL_BEGIN
 * This is used to begin a declaration of a library private ICU C API.
 * @stable ICU 2.4
 */
/* *
 * \def U_CDECL_END
 * This is used to end a declaration of a library private ICU C API
 * @stable ICU 2.4
 */
/* *
 * \def U_ATTRIBUTE_DEPRECATED
 *  This is used for GCC specific attributes
 * @internal
 */
/* * This is used to declare a function as a public ICU C API @stable ICU 2.0*/
/* * This is used to declare a function as a stable public ICU C API*/
/* * This is used to declare a function as a draft public ICU C API  */
/* * This is used to declare a function as a deprecated public ICU C API  */
/* * This is used to declare a function as an obsolete public ICU C API  */
/* * This is used to declare a function as an internal ICU C API  */
/* *
 * \def U_OVERRIDE
 * Defined to the C++11 "override" keyword if available.
 * Denotes a class or member which is an override of the base class.
 * May result in an error if it applied to something not an override.
 * @internal
 */
/* *
 * \def U_FINAL
 * Defined to the C++11 "final" keyword if available.
 * Denotes a class or member which may not be overridden in subclasses.
 * May result in an error if subclasses attempt to override.
 * @internal
 */
// Before ICU 65, function-like, multi-statement ICU macros were just defined as
// series of statements wrapped in { } blocks and the caller could choose to
// either treat them as if they were actual functions and end the invocation
// with a trailing ; creating an empty statement after the block or else omit
// this trailing ; using the knowledge that the macro would expand to { }.
//
// But doing so doesn't work well with macros that look like functions and
// compiler warnings about empty statements (ICU-20601) and ICU 65 therefore
// switches to the standard solution of wrapping such macros in do { } while.
//
// This will however break existing code that depends on being able to invoke
// these macros without a trailing ; so to be able to remain compatible with
// such code the wrapper is itself defined as macros so that it's possible to
// build ICU 65 and later with the old macro behaviour, like this:
//
// CPPFLAGS='-DUPRV_BLOCK_MACRO_BEGIN="" -DUPRV_BLOCK_MACRO_END=""'
// runConfigureICU ...
/* *
 * \def UPRV_BLOCK_MACRO_BEGIN
 * Defined as the "do" keyword by default.
 * @internal
 */
/* *
 * \def UPRV_BLOCK_MACRO_END
 * Defined as "while (FALSE)" by default.
 * @internal
 */
/*==========================================================================*/
/* limits for int32_t etc., like in POSIX inttypes.h                        */
/*==========================================================================*/
/* * The smallest value a 64 bit signed integer can hold @stable ICU 2.8 */
/* * The largest value a 64 bit signed integer can hold @stable ICU 2.8 */
/* * The largest value a 64 bit unsigned integer can hold @stable ICU 2.8 */
/*==========================================================================*/
/* Boolean data type                                                        */
/*==========================================================================*/
/* * The ICU boolean type @stable ICU 2.0 */
/* * The TRUE value of a UBool @stable ICU 2.0 */
/* * The FALSE value of a UBool @stable ICU 2.0 */
/*==========================================================================*/
/* Unicode data types                                                       */
/*==========================================================================*/
/* wchar_t-related definitions -------------------------------------------- */
/*
 * \def U_WCHAR_IS_UTF16
 * Defined if wchar_t uses UTF-16.
 *
 * @stable ICU 2.0
 */
/*
 * \def U_WCHAR_IS_UTF32
 * Defined if wchar_t uses UTF-32.
 *
 * @stable ICU 2.0
 */
/* UChar and UChar32 definitions -------------------------------------------- */
/* * Number of bytes in a UChar. @stable ICU 2.0 */
/* *
 * \def U_CHAR16_IS_TYPEDEF
 * If 1, then char16_t is a typedef and not a real type (yet)
 * @internal
 */
/* *
 * \var UChar
 *
 * The base type for UTF-16 code units and pointers.
 * Unsigned 16-bit integer.
 * Starting with ICU 59, C++ API uses char16_t directly, while C API continues to use UChar.
 *
 * UChar is configurable by defining the macro UCHAR_TYPE
 * on the preprocessor or compiler command line:
 * -DUCHAR_TYPE=uint16_t or -DUCHAR_TYPE=wchar_t (if U_SIZEOF_WCHAR_T==2) etc.
 * (The UCHAR_TYPE can also be \#defined earlier in this file, for outside the ICU library code.)
 * This is for transitional use from application code that uses uint16_t or wchar_t for UTF-16.
 *
 * The default is UChar=char16_t.
 *
 * C++11 defines char16_t as bit-compatible with uint16_t, but as a distinct type.
 *
 * In C, char16_t is a simple typedef of uint_least16_t.
 * ICU requires uint_least16_t=uint16_t for data memory mapping.
 * On macOS, char16_t is not available because the uchar.h standard header is missing.
 *
 * @stable ICU 4.4
 */
// #if 1 is normal. UChar defaults to char16_t in C++.
    // For configuration testing of UChar=uint16_t temporarily change this to #if 0.
    // The intltest Makefile #defines UCHAR_TYPE=char16_t,
    // so we only #define it to uint16_t if it is undefined so far.
/* *
 * \var OldUChar
 * Default ICU 58 definition of UChar.
 * A base type for UTF-16 code units and pointers.
 * Unsigned 16-bit integer.
 *
 * Define OldUChar to be wchar_t if that is 16 bits wide.
 * If wchar_t is not 16 bits wide, then define UChar to be uint16_t.
 *
 * This makes the definition of OldUChar platform-dependent
 * but allows direct string type compatibility with platforms with
 * 16-bit wchar_t types.
 *
 * This is how UChar was defined in ICU 58, for transition convenience.
 * Exception: ICU 58 UChar was defined to UCHAR_TYPE if that macro was defined.
 * The current UChar responds to UCHAR_TYPE but OldUChar does not.
 *
 * @stable ICU 59
 */
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
unsafe extern "C" fn length_is_undefined(mut length: Length) -> bool {
    return length.bytes == 0 as libc::c_int as libc::c_uint &&
               length.extent.column != 0 as libc::c_int as libc::c_uint;
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
static mut TS_DECODE_ERROR: int32_t = -(1 as libc::c_int);
#[inline]
unsafe extern "C" fn ts_decode_utf8(mut string: *const uint8_t,
                                    mut length: uint32_t,
                                    mut code_point: *mut int32_t)
 -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh0 = i;
    i = i.wrapping_add(1);
    *code_point = *string.offset(fresh0 as isize) as int32_t;
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
#[inline]
unsafe extern "C" fn ts_decode_utf16(mut string: *const uint8_t,
                                     mut length: uint32_t,
                                     mut code_point: *mut int32_t)
 -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh1 = i;
    i = i.wrapping_add(1);
    *code_point =
        *(string as *mut uint16_t).offset(fresh1 as isize) as int32_t;
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
// Is the lexer at a boundary between two disjoint included ranges of
// source code? This is exposed as an API because some languages' external
// scanners need to perform custom actions at these bounaries.
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
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_delete(mut self_0: *mut Lexer) {
    ts_free((*self_0).included_ranges as *mut libc::c_void);
}
unsafe extern "C" fn ts_lexer_goto(mut self_0: *mut Lexer,
                                   mut position: Length) {
    (*self_0).current_position = position;
    let mut found_included_range: bool = 0 as libc::c_int != 0;
    // Move to the first valid position at or after the given position.
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
        // If the current position is outside of the current chunk of text,
    // then clear out the current chunk of text.
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
        // If the given position is beyond any of included ranges, move to the EOF
  // state - past the end of the included ranges.
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
pub unsafe extern "C" fn ts_lexer_set_input(mut self_0: *mut Lexer,
                                            mut input: TSInput) {
    (*self_0).input = input;
    ts_lexer__clear_chunk(self_0);
    ts_lexer_goto(self_0, (*self_0).current_position);
}
// Move the lexer to the given position. This doesn't do any work
// if the parser is already at the given position.
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_reset(mut self_0: *mut Lexer,
                                        mut position: Length) {
    if position.bytes != (*self_0).current_position.bytes {
        ts_lexer_goto(self_0, position);
    };
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
    // In order to determine that a byte sequence is invalid UTF8 or UTF16,
  // the character decoding algorithm may have looked at the following byte.
  // Therefore, the next byte *after* the current (invalid) character
  // affects the interpretation of the current character.
    if (*self_0).data.lookahead == TS_DECODE_ERROR {
        current_lookahead_end_byte =
            current_lookahead_end_byte.wrapping_add(1)
    }
    if current_lookahead_end_byte > *lookahead_end_byte {
        *lookahead_end_byte = current_lookahead_end_byte
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_advance_to_end(mut self_0: *mut Lexer) {
    while !(*self_0).chunk.is_null() {
        ts_lexer__advance(&mut (*self_0).data, 0 as libc::c_int != 0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_mark_end(mut self_0: *mut Lexer) {
    ts_lexer__mark_end(&mut (*self_0).data);
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
pub unsafe extern "C" fn ts_lexer_included_ranges(mut self_0: *const Lexer,
                                                  mut count: *mut uint32_t)
 -> *mut TSRange {
    *count = (*self_0).included_range_count as uint32_t;
    return (*self_0).included_ranges;
}
