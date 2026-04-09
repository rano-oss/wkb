pub mod internal {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}
pub mod types_h {
    pub type __uint32_t = u32;
    pub type __uint64_t = u64;
    pub type __dev_t = ::core::ffi::c_ulong;
    pub type __uid_t = ::core::ffi::c_uint;
    pub type __gid_t = ::core::ffi::c_uint;
    pub type __ino_t = ::core::ffi::c_ulong;
    pub type __ino64_t = ::core::ffi::c_ulong;
    pub type __mode_t = ::core::ffi::c_uint;
    pub type __nlink_t = ::core::ffi::c_ulong;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
    pub type __time_t = ::core::ffi::c_long;
    pub type __blksize_t = ::core::ffi::c_long;
    pub type __blkcnt_t = ::core::ffi::c_long;
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
}

pub mod stdlib_h {
    pub type __compar_fn_t = Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >;

    extern "C" {
        pub fn strtol(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        pub fn calloc(__nmemb: usize, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: usize) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn secure_getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn qsort(
            __base: *mut ::core::ffi::c_void,
            __nmemb: usize,
            __size: usize,
            __compar: __compar_fn_t,
        );
    }
}
pub mod stdint_uintn_h {
    pub type u32 = __uint32_t;
    use super::types_h::__uint32_t;
}
pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
pub mod stdio_h {
    pub type va_list = __gnuc_va_list;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;

    use super::FILE_h::FILE;

    extern "C" {
        pub static mut stderr: *mut FILE;
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn vfprintf(
            __s: *mut FILE,
            __format: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
        pub fn vsnprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: usize,
            __format: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
    }
}
pub mod xmlstring_h {
    pub type xmlChar = ::core::ffi::c_uchar;
    extern "C" {
        pub fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
        pub fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    }
}
pub mod xmlmemory_h {
    pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    extern "C" {
        pub static mut xmlFree: xmlFreeFunc;
    }
}
pub mod xmlIO_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserInputBuffer {
        pub context: *mut ::core::ffi::c_void,
        pub readcallback: xmlInputReadCallback,
        pub closecallback: xmlInputCloseCallback,
        pub encoder: xmlCharEncodingHandlerPtr,
        pub buffer: xmlBufPtr,
        pub raw: xmlBufPtr,
        pub compressed: ::core::ffi::c_int,
        pub error: ::core::ffi::c_int,
        pub rawconsumed: ::core::ffi::c_ulong,
    }
    pub type xmlInputCloseCallback =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    pub type xmlInputReadCallback = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >;
    use super::encoding_h::{xmlCharEncoding, xmlCharEncodingHandlerPtr};
    use super::tree_h::{xmlBufPtr, xmlParserInputBufferPtr};
    extern "C" {
        pub fn xmlParserInputBufferCreateMem(
            mem: *const ::core::ffi::c_char,
            size: ::core::ffi::c_int,
            enc: xmlCharEncoding,
        ) -> xmlParserInputBufferPtr;
    }
}
pub mod tree_h {
    pub type xmlBufPtr = *mut xmlBuf;
    pub type xmlBuf = _xmlBuf;
    pub type xmlParserInputBuffer = _xmlParserInputBuffer;
    pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
    pub type xmlEntityPtr = *mut xmlEntity;
    pub type xmlEntity = _xmlEntity;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlDoc {
        pub _private: *mut ::core::ffi::c_void,
        pub type_0: xmlElementType,
        pub name: *mut ::core::ffi::c_char,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub compression: ::core::ffi::c_int,
        pub standalone: ::core::ffi::c_int,
        pub intSubset: *mut _xmlDtd,
        pub extSubset: *mut _xmlDtd,
        pub oldNs: *mut _xmlNs,
        pub version: *const xmlChar,
        pub encoding: *const xmlChar,
        pub ids: *mut ::core::ffi::c_void,
        pub refs: *mut ::core::ffi::c_void,
        pub URL: *const xmlChar,
        pub charset: ::core::ffi::c_int,
        pub dict: *mut _xmlDict,
        pub psvi: *mut ::core::ffi::c_void,
        pub parseFlags: ::core::ffi::c_int,
        pub properties: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlNs {
        pub next: *mut _xmlNs,
        pub type_0: xmlNsType,
        pub href: *const xmlChar,
        pub prefix: *const xmlChar,
        pub _private: *mut ::core::ffi::c_void,
        pub context: *mut _xmlDoc,
    }
    pub type xmlElementType = xmlNsType;
    pub type xmlNsType = ::core::ffi::c_uint;
    pub const XML_XINCLUDE_END: xmlNsType = 20;
    pub const XML_XINCLUDE_START: xmlNsType = 19;
    pub const XML_NAMESPACE_DECL: xmlNsType = 18;
    pub const XML_ENTITY_DECL: xmlNsType = 17;
    pub const XML_ATTRIBUTE_DECL: xmlNsType = 16;
    pub const XML_ELEMENT_DECL: xmlNsType = 15;
    pub const XML_DTD_NODE: xmlNsType = 14;
    pub const XML_HTML_DOCUMENT_NODE: xmlNsType = 13;
    pub const XML_NOTATION_NODE: xmlNsType = 12;
    pub const XML_DOCUMENT_FRAG_NODE: xmlNsType = 11;
    pub const XML_DOCUMENT_TYPE_NODE: xmlNsType = 10;
    pub const XML_DOCUMENT_NODE: xmlNsType = 9;
    pub const XML_COMMENT_NODE: xmlNsType = 8;
    pub const XML_PI_NODE: xmlNsType = 7;
    pub const XML_ENTITY_NODE: xmlNsType = 6;
    pub const XML_ENTITY_REF_NODE: xmlNsType = 5;
    pub const XML_CDATA_SECTION_NODE: xmlNsType = 4;
    pub const XML_TEXT_NODE: xmlNsType = 3;
    pub const XML_ATTRIBUTE_NODE: xmlNsType = 2;
    pub const XML_ELEMENT_NODE: xmlNsType = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlDtd {
        pub _private: *mut ::core::ffi::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlDoc,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub notations: *mut ::core::ffi::c_void,
        pub elements: *mut ::core::ffi::c_void,
        pub attributes: *mut ::core::ffi::c_void,
        pub entities: *mut ::core::ffi::c_void,
        pub ExternalID: *const xmlChar,
        pub SystemID: *const xmlChar,
        pub pentities: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlNode {
        pub _private: *mut ::core::ffi::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub ns: *mut xmlNs,
        pub content: *mut xmlChar,
        pub properties: *mut _xmlAttr,
        pub nsDef: *mut xmlNs,
        pub psvi: *mut ::core::ffi::c_void,
        pub line: ::core::ffi::c_ushort,
        pub extra: ::core::ffi::c_ushort,
    }
    pub type xmlNs = _xmlNs;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlAttr {
        pub _private: *mut ::core::ffi::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlNode,
        pub next: *mut _xmlAttr,
        pub prev: *mut _xmlAttr,
        pub doc: *mut _xmlDoc,
        pub ns: *mut xmlNs,
        pub atype: xmlAttributeType,
        pub psvi: *mut ::core::ffi::c_void,
    }
    pub type xmlAttributeType = ::core::ffi::c_uint;
    pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
    pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
    pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
    pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
    pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
    pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
    pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
    pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
    pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
    pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
    pub type xmlParserInput = _xmlParserInput;
    pub type xmlParserInputPtr = *mut xmlParserInput;
    pub type xmlAttrPtr = *mut xmlAttr;
    pub type xmlAttr = _xmlAttr;
    pub type xmlNodePtr = *mut xmlNode;
    pub type xmlNode = _xmlNode;
    pub type xmlDocPtr = *mut xmlDoc;
    pub type xmlDoc = _xmlDoc;
    pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
    pub type xmlSAXLocator = _xmlSAXLocator;
    pub type xmlElementContentPtr = *mut xmlElementContent;
    pub type xmlElementContent = _xmlElementContent;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlElementContent {
        pub type_0: xmlElementContentType,
        pub ocur: xmlElementContentOccur,
        pub name: *const xmlChar,
        pub c1: *mut _xmlElementContent,
        pub c2: *mut _xmlElementContent,
        pub parent: *mut _xmlElementContent,
        pub prefix: *const xmlChar,
    }
    pub type xmlElementContentOccur = ::core::ffi::c_uint;
    pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
    pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
    pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
    pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
    pub type xmlElementContentType = ::core::ffi::c_uint;
    pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
    pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
    pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
    pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
    pub type xmlEnumerationPtr = *mut xmlEnumeration;
    pub type xmlEnumeration = _xmlEnumeration;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlEnumeration {
        pub next: *mut _xmlEnumeration,
        pub name: *const xmlChar,
    }
    pub type xmlParserCtxt = _xmlParserCtxt;
    pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
    pub type xmlSAXHandler = _xmlSAXHandler;
    pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
    pub type xmlDtd = _xmlDtd;
    pub type xmlDtdPtr = *mut xmlDtd;
    use super::entities_h::_xmlEntity;
    use super::parser_h::{_xmlParserCtxt, _xmlParserInput, _xmlSAXHandler, _xmlSAXLocator};
    use super::xmlIO_h::_xmlParserInputBuffer;
    use super::xmlstring_h::xmlChar;
    extern "C" {
        pub type _xmlBuf;
        pub type _xmlDict;
        pub fn xmlFreeDtd(cur: xmlDtdPtr);
        pub fn xmlFreeDoc(cur: xmlDocPtr);
        pub fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
        pub fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    }
}
pub mod encoding_h {
    pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
    pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlCharEncodingHandler {
        pub name: *mut ::core::ffi::c_char,
        pub input: xmlCharEncodingInputFunc,
        pub output: xmlCharEncodingOutputFunc,
        pub iconv_in: iconv_t,
        pub iconv_out: iconv_t,
    }
    pub type xmlCharEncodingOutputFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_uchar,
            *mut ::core::ffi::c_int,
            *const ::core::ffi::c_uchar,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >;
    pub type xmlCharEncodingInputFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_uchar,
            *mut ::core::ffi::c_int,
            *const ::core::ffi::c_uchar,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >;
    pub type xmlCharEncoding = ::core::ffi::c_int;
    pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
    pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
    pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
    pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
    pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
    pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
    pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
    pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
    pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
    pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
    pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
    pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
    pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
    pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
    pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
    pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
    pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
    pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
    pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
    pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
    pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
    pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
    pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
    pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
    use super::iconv_h::iconv_t;
}
pub mod iconv_h {
    pub type iconv_t = *mut ::core::ffi::c_void;
}
pub mod parser_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserInput {
        pub buf: xmlParserInputBufferPtr,
        pub filename: *const ::core::ffi::c_char,
        pub directory: *const ::core::ffi::c_char,
        pub base: *const xmlChar,
        pub cur: *const xmlChar,
        pub end: *const xmlChar,
        pub length: ::core::ffi::c_int,
        pub line: ::core::ffi::c_int,
        pub col: ::core::ffi::c_int,
        pub consumed: ::core::ffi::c_ulong,
        pub free: xmlParserInputDeallocate,
        pub encoding: *const xmlChar,
        pub version: *const xmlChar,
        pub flags: ::core::ffi::c_int,
        pub id: ::core::ffi::c_int,
        pub parentConsumed: ::core::ffi::c_ulong,
        pub entity: xmlEntityPtr,
    }
    pub type xmlParserInputDeallocate = Option<unsafe extern "C" fn(*mut xmlChar) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserCtxt {
        pub sax: *mut _xmlSAXHandler,
        pub userData: *mut ::core::ffi::c_void,
        pub myDoc: xmlDocPtr,
        pub wellFormed: ::core::ffi::c_int,
        pub replaceEntities: ::core::ffi::c_int,
        pub version: *const xmlChar,
        pub encoding: *const xmlChar,
        pub standalone: ::core::ffi::c_int,
        pub html: ::core::ffi::c_int,
        pub input: xmlParserInputPtr,
        pub inputNr: ::core::ffi::c_int,
        pub inputMax: ::core::ffi::c_int,
        pub inputTab: *mut xmlParserInputPtr,
        pub node: xmlNodePtr,
        pub nodeNr: ::core::ffi::c_int,
        pub nodeMax: ::core::ffi::c_int,
        pub nodeTab: *mut xmlNodePtr,
        pub record_info: ::core::ffi::c_int,
        pub node_seq: xmlParserNodeInfoSeq,
        pub errNo: ::core::ffi::c_int,
        pub hasExternalSubset: ::core::ffi::c_int,
        pub hasPErefs: ::core::ffi::c_int,
        pub external: ::core::ffi::c_int,
        pub valid: ::core::ffi::c_int,
        pub validate: ::core::ffi::c_int,
        pub vctxt: xmlValidCtxt,
        pub instate: xmlParserInputState,
        pub token: ::core::ffi::c_int,
        pub directory: *mut ::core::ffi::c_char,
        pub name: *const xmlChar,
        pub nameNr: ::core::ffi::c_int,
        pub nameMax: ::core::ffi::c_int,
        pub nameTab: *mut *const xmlChar,
        pub nbChars: ::core::ffi::c_long,
        pub checkIndex: ::core::ffi::c_long,
        pub keepBlanks: ::core::ffi::c_int,
        pub disableSAX: ::core::ffi::c_int,
        pub inSubset: ::core::ffi::c_int,
        pub intSubName: *const xmlChar,
        pub extSubURI: *mut xmlChar,
        pub extSubSystem: *mut xmlChar,
        pub space: *mut ::core::ffi::c_int,
        pub spaceNr: ::core::ffi::c_int,
        pub spaceMax: ::core::ffi::c_int,
        pub spaceTab: *mut ::core::ffi::c_int,
        pub depth: ::core::ffi::c_int,
        pub entity: xmlParserInputPtr,
        pub charset: ::core::ffi::c_int,
        pub nodelen: ::core::ffi::c_int,
        pub nodemem: ::core::ffi::c_int,
        pub pedantic: ::core::ffi::c_int,
        pub _private: *mut ::core::ffi::c_void,
        pub loadsubset: ::core::ffi::c_int,
        pub linenumbers: ::core::ffi::c_int,
        pub catalogs: *mut ::core::ffi::c_void,
        pub recovery: ::core::ffi::c_int,
        pub progressive: ::core::ffi::c_int,
        pub dict: xmlDictPtr,
        pub atts: *mut *const xmlChar,
        pub maxatts: ::core::ffi::c_int,
        pub docdict: ::core::ffi::c_int,
        pub str_xml: *const xmlChar,
        pub str_xmlns: *const xmlChar,
        pub str_xml_ns: *const xmlChar,
        pub sax2: ::core::ffi::c_int,
        pub nsNr: ::core::ffi::c_int,
        pub nsMax: ::core::ffi::c_int,
        pub nsTab: *mut *const xmlChar,
        pub attallocs: *mut ::core::ffi::c_uint,
        pub pushTab: *mut xmlStartTag,
        pub attsDefault: xmlHashTablePtr,
        pub attsSpecial: xmlHashTablePtr,
        pub nsWellFormed: ::core::ffi::c_int,
        pub options: ::core::ffi::c_int,
        pub dictNames: ::core::ffi::c_int,
        pub freeElemsNr: ::core::ffi::c_int,
        pub freeElems: xmlNodePtr,
        pub freeAttrsNr: ::core::ffi::c_int,
        pub freeAttrs: xmlAttrPtr,
        pub lastError: xmlError,
        pub parseMode: xmlParserMode,
        pub nbentities: ::core::ffi::c_ulong,
        pub sizeentities: ::core::ffi::c_ulong,
        pub nodeInfo: *mut xmlParserNodeInfo,
        pub nodeInfoNr: ::core::ffi::c_int,
        pub nodeInfoMax: ::core::ffi::c_int,
        pub nodeInfoTab: *mut xmlParserNodeInfo,
        pub input_id: ::core::ffi::c_int,
        pub sizeentcopy: ::core::ffi::c_ulong,
        pub endCheckState: ::core::ffi::c_int,
        pub nbErrors: ::core::ffi::c_ushort,
        pub nbWarnings: ::core::ffi::c_ushort,
        pub maxAmpl: ::core::ffi::c_uint,
        pub nsdb: *mut xmlParserNsData,
        pub attrHashMax: ::core::ffi::c_uint,
        pub attrHash: *mut xmlAttrHashBucket,
    }
    pub type xmlAttrHashBucket = _xmlAttrHashBucket;
    pub type xmlParserNsData = _xmlParserNsData;
    pub type xmlParserNodeInfo = _xmlParserNodeInfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserNodeInfo {
        pub node: *const _xmlNode,
        pub begin_pos: ::core::ffi::c_ulong,
        pub begin_line: ::core::ffi::c_ulong,
        pub end_pos: ::core::ffi::c_ulong,
        pub end_line: ::core::ffi::c_ulong,
    }
    pub type xmlParserMode = ::core::ffi::c_uint;
    pub const XML_PARSE_READER: xmlParserMode = 5;
    pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
    pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
    pub const XML_PARSE_SAX: xmlParserMode = 2;
    pub const XML_PARSE_DOM: xmlParserMode = 1;
    pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
    pub type xmlStartTag = _xmlStartTag;
    pub type xmlParserInputState = ::core::ffi::c_int;
    pub const XML_PARSER_XML_DECL: xmlParserInputState = 17;
    pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
    pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
    pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
    pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
    pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
    pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
    pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
    pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
    pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
    pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
    pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
    pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
    pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
    pub const XML_PARSER_DTD: xmlParserInputState = 3;
    pub const XML_PARSER_PI: xmlParserInputState = 2;
    pub const XML_PARSER_MISC: xmlParserInputState = 1;
    pub const XML_PARSER_START: xmlParserInputState = 0;
    pub const XML_PARSER_EOF: xmlParserInputState = -1;
    pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserNodeInfoSeq {
        pub maximum: ::core::ffi::c_ulong,
        pub length: ::core::ffi::c_ulong,
        pub buffer: *mut xmlParserNodeInfo,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlSAXHandler {
        pub internalSubset: internalSubsetSAXFunc,
        pub isStandalone: isStandaloneSAXFunc,
        pub hasInternalSubset: hasInternalSubsetSAXFunc,
        pub hasExternalSubset: hasExternalSubsetSAXFunc,
        pub resolveEntity: resolveEntitySAXFunc,
        pub getEntity: getEntitySAXFunc,
        pub entityDecl: entityDeclSAXFunc,
        pub notationDecl: notationDeclSAXFunc,
        pub attributeDecl: attributeDeclSAXFunc,
        pub elementDecl: elementDeclSAXFunc,
        pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
        pub setDocumentLocator: setDocumentLocatorSAXFunc,
        pub startDocument: startDocumentSAXFunc,
        pub endDocument: endDocumentSAXFunc,
        pub startElement: startElementSAXFunc,
        pub endElement: endElementSAXFunc,
        pub reference: referenceSAXFunc,
        pub characters: charactersSAXFunc,
        pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
        pub processingInstruction: processingInstructionSAXFunc,
        pub comment: commentSAXFunc,
        pub warning: warningSAXFunc,
        pub error: errorSAXFunc,
        pub fatalError: fatalErrorSAXFunc,
        pub getParameterEntity: getParameterEntitySAXFunc,
        pub cdataBlock: cdataBlockSAXFunc,
        pub externalSubset: externalSubsetSAXFunc,
        pub initialized: ::core::ffi::c_uint,
        pub _private: *mut ::core::ffi::c_void,
        pub startElementNs: startElementNsSAX2Func,
        pub endElementNs: endElementNsSAX2Func,
        pub serror: xmlStructuredErrorFunc,
    }
    pub type endElementNsSAX2Func = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
        ) -> (),
    >;
    pub type startElementNsSAX2Func = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
            ::core::ffi::c_int,
            *mut *const xmlChar,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut *const xmlChar,
        ) -> (),
    >;
    pub type externalSubsetSAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
        ) -> (),
    >;
    pub type cdataBlockSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
    >;
    pub type getParameterEntitySAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
    pub type fatalErrorSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >;
    pub type errorSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >;
    pub type warningSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >;
    pub type commentSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
    pub type processingInstructionSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar) -> (),
    >;
    pub type ignorableWhitespaceSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
    >;
    pub type charactersSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, ::core::ffi::c_int) -> (),
    >;
    pub type referenceSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
    pub type endElementSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
    pub type startElementSAXFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, *mut *const xmlChar) -> (),
    >;
    pub type endDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    pub type startDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    pub type setDocumentLocatorSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlSAXLocator {
        pub getPublicId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
        pub getSystemId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
        pub getLineNumber:
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        pub getColumnNumber:
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    }
    pub type unparsedEntityDeclSAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
        ) -> (),
    >;
    pub type elementDeclSAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            ::core::ffi::c_int,
            xmlElementContentPtr,
        ) -> (),
    >;
    pub type attributeDeclSAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *const xmlChar,
            xmlEnumerationPtr,
        ) -> (),
    >;
    pub type notationDeclSAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
        ) -> (),
    >;
    pub type entityDeclSAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            ::core::ffi::c_int,
            *const xmlChar,
            *const xmlChar,
            *mut xmlChar,
        ) -> (),
    >;
    pub type getEntitySAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
    pub type resolveEntitySAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
        ) -> xmlParserInputPtr,
    >;
    pub type hasExternalSubsetSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    pub type hasInternalSubsetSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    pub type isStandaloneSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
    pub type internalSubsetSAXFunc = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
        ) -> (),
    >;
    pub type C2Rust_Unnamed = ::core::ffi::c_uint;
    pub const XML_PARSE_BIG_LINES: C2Rust_Unnamed = 4194304;
    pub const XML_PARSE_IGNORE_ENC: C2Rust_Unnamed = 2097152;
    pub const XML_PARSE_OLDSAX: C2Rust_Unnamed = 1048576;
    pub const XML_PARSE_HUGE: C2Rust_Unnamed = 524288;
    pub const XML_PARSE_NOBASEFIX: C2Rust_Unnamed = 262144;
    pub const XML_PARSE_OLD10: C2Rust_Unnamed = 131072;
    pub const XML_PARSE_COMPACT: C2Rust_Unnamed = 65536;
    pub const XML_PARSE_NOXINCNODE: C2Rust_Unnamed = 32768;
    pub const XML_PARSE_NOCDATA: C2Rust_Unnamed = 16384;
    pub const XML_PARSE_NSCLEAN: C2Rust_Unnamed = 8192;
    pub const XML_PARSE_NODICT: C2Rust_Unnamed = 4096;
    pub const XML_PARSE_NONET: C2Rust_Unnamed = 2048;
    pub const XML_PARSE_XINCLUDE: C2Rust_Unnamed = 1024;
    pub const XML_PARSE_SAX1: C2Rust_Unnamed = 512;
    pub const XML_PARSE_NOBLANKS: C2Rust_Unnamed = 256;
    pub const XML_PARSE_PEDANTIC: C2Rust_Unnamed = 128;
    pub const XML_PARSE_NOWARNING: C2Rust_Unnamed = 64;
    pub const XML_PARSE_NOERROR: C2Rust_Unnamed = 32;
    pub const XML_PARSE_DTDVALID: C2Rust_Unnamed = 16;
    pub const XML_PARSE_DTDATTR: C2Rust_Unnamed = 8;
    pub const XML_PARSE_DTDLOAD: C2Rust_Unnamed = 4;
    pub const XML_PARSE_NOENT: C2Rust_Unnamed = 2;
    pub const XML_PARSE_RECOVER: C2Rust_Unnamed = 1;
    use super::dict_h::xmlDictPtr;
    use super::encoding_h::xmlCharEncoding;
    use super::hash_h::xmlHashTablePtr;
    use super::tree_h::{
        _xmlNode, xmlAttrPtr, xmlDocPtr, xmlDtdPtr, xmlElementContentPtr, xmlEntityPtr,
        xmlEnumerationPtr, xmlNodePtr, xmlParserCtxtPtr, xmlParserInputBufferPtr,
        xmlParserInputPtr, xmlSAXHandlerPtr, xmlSAXLocatorPtr,
    };
    use super::valid_h::xmlValidCtxt;
    use super::xmlerror_h::{xmlError, xmlStructuredErrorFunc};
    use super::xmlstring_h::xmlChar;
    extern "C" {
        pub type _xmlAttrHashBucket;
        pub type _xmlParserNsData;
        pub type _xmlStartTag;
        pub fn xmlIOParseDTD(
            sax: xmlSAXHandlerPtr,
            input: xmlParserInputBufferPtr,
            enc: xmlCharEncoding,
        ) -> xmlDtdPtr;
        pub fn xmlNewParserCtxt() -> xmlParserCtxtPtr;
        pub fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
        pub fn xmlCtxtUseOptions(
            ctxt: xmlParserCtxtPtr,
            options: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub fn xmlCtxtReadFile(
            ctxt: xmlParserCtxtPtr,
            filename: *const ::core::ffi::c_char,
            encoding: *const ::core::ffi::c_char,
            options: ::core::ffi::c_int,
        ) -> xmlDocPtr;
    }
}
pub mod entities_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlEntity {
        pub _private: *mut ::core::ffi::c_void,
        pub type_0: xmlElementType,
        pub name: *const xmlChar,
        pub children: *mut _xmlNode,
        pub last: *mut _xmlNode,
        pub parent: *mut _xmlDtd,
        pub next: *mut _xmlNode,
        pub prev: *mut _xmlNode,
        pub doc: *mut _xmlDoc,
        pub orig: *mut xmlChar,
        pub content: *mut xmlChar,
        pub length: ::core::ffi::c_int,
        pub etype: xmlEntityType,
        pub ExternalID: *const xmlChar,
        pub SystemID: *const xmlChar,
        pub nexte: *mut _xmlEntity,
        pub URI: *const xmlChar,
        pub owner: ::core::ffi::c_int,
        pub flags: ::core::ffi::c_int,
        pub expandedSize: ::core::ffi::c_ulong,
    }
    pub type xmlEntityType = ::core::ffi::c_uint;
    pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
    pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
    pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
    pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
    pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
    pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
    use super::tree_h::{_xmlDoc, _xmlDtd, _xmlNode, xmlElementType};
    use super::xmlstring_h::xmlChar;
}
pub mod xmlerror_h {
    pub type xmlError = _xmlError;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlError {
        pub domain: ::core::ffi::c_int,
        pub code: ::core::ffi::c_int,
        pub message: *mut ::core::ffi::c_char,
        pub level: xmlErrorLevel,
        pub file: *mut ::core::ffi::c_char,
        pub line: ::core::ffi::c_int,
        pub str1: *mut ::core::ffi::c_char,
        pub str2: *mut ::core::ffi::c_char,
        pub str3: *mut ::core::ffi::c_char,
        pub int1: ::core::ffi::c_int,
        pub int2: ::core::ffi::c_int,
        pub ctxt: *mut ::core::ffi::c_void,
        pub node: *mut ::core::ffi::c_void,
    }
    pub type xmlErrorLevel = ::core::ffi::c_uint;
    pub const XML_ERR_FATAL: xmlErrorLevel = 3;
    pub const XML_ERR_ERROR: xmlErrorLevel = 2;
    pub const XML_ERR_WARNING: xmlErrorLevel = 1;
    pub const XML_ERR_NONE: xmlErrorLevel = 0;
    pub type xmlStructuredErrorFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> ()>;
    pub type xmlGenericErrorFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >;
    extern "C" {
        pub fn xmlSetGenericErrorFunc(ctx: *mut ::core::ffi::c_void, handler: xmlGenericErrorFunc);
    }
}
pub mod hash_h {
    pub type xmlHashTablePtr = *mut xmlHashTable;
    pub type xmlHashTable = _xmlHashTable;
    extern "C" {
        pub type _xmlHashTable;
    }
}
pub mod dict_h {
    pub type xmlDictPtr = *mut xmlDict;
    pub type xmlDict = _xmlDict;
    use super::tree_h::_xmlDict;
}
pub mod valid_h {
    pub type xmlValidCtxt = _xmlValidCtxt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlValidCtxt {
        pub userData: *mut ::core::ffi::c_void,
        pub error: xmlValidityErrorFunc,
        pub warning: xmlValidityWarningFunc,
        pub node: xmlNodePtr,
        pub nodeNr: ::core::ffi::c_int,
        pub nodeMax: ::core::ffi::c_int,
        pub nodeTab: *mut xmlNodePtr,
        pub flags: ::core::ffi::c_uint,
        pub doc: xmlDocPtr,
        pub valid: ::core::ffi::c_int,
        pub vstate: *mut xmlValidState,
        pub vstateNr: ::core::ffi::c_int,
        pub vstateMax: ::core::ffi::c_int,
        pub vstateTab: *mut xmlValidState,
        pub am: xmlAutomataPtr,
        pub state: xmlAutomataStatePtr,
    }
    pub type xmlValidState = _xmlValidState;
    pub type xmlValidityWarningFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >;
    pub type xmlValidityErrorFunc = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >;
    pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
    use super::tree_h::{xmlDocPtr, xmlDtdPtr, xmlNodePtr};
    use super::xmlautomata_h::{xmlAutomataPtr, xmlAutomataStatePtr};
    extern "C" {
        pub type _xmlValidState;
        pub fn xmlNewValidCtxt() -> xmlValidCtxtPtr;
        pub fn xmlFreeValidCtxt(_: xmlValidCtxtPtr);
        pub fn xmlValidateDtd(
            ctxt: xmlValidCtxtPtr,
            doc: xmlDocPtr,
            dtd: xmlDtdPtr,
        ) -> ::core::ffi::c_int;
    }
}
pub mod xmlautomata_h {
    pub type xmlAutomataStatePtr = *mut xmlAutomataState;
    pub type xmlAutomataState = _xmlAutomataState;
    pub type xmlAutomataPtr = *mut xmlAutomata;
    pub type xmlAutomata = _xmlAutomata;
    extern "C" {
        pub type _xmlAutomataState;
        pub type _xmlAutomata;
    }
}
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: __ino64_t,
        pub d_off: __off64_t,
        pub d_reclen: ::core::ffi::c_ushort,
        pub d_type: ::core::ffi::c_uchar,
        pub d_name: [::core::ffi::c_char; 256],
    }
    use super::types_h::{__ino64_t, __off64_t};
}
pub mod include_dirent_h {
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        pub type __dirstream;
        pub fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
        pub fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
pub mod xkbregistry_h {
    pub type rxkb_log_level = ::core::ffi::c_uint;
    pub const RXKB_LOG_LEVEL_DEBUG: rxkb_log_level = 50;
    pub const RXKB_LOG_LEVEL_INFO: rxkb_log_level = 40;
    pub const RXKB_LOG_LEVEL_WARNING: rxkb_log_level = 30;
    pub const RXKB_LOG_LEVEL_ERROR: rxkb_log_level = 20;
    pub const RXKB_LOG_LEVEL_CRITICAL: rxkb_log_level = 10;
    pub type rxkb_popularity = ::core::ffi::c_uint;
    pub const RXKB_POPULARITY_EXOTIC: rxkb_popularity = 2;
    pub const RXKB_POPULARITY_STANDARD: rxkb_popularity = 1;
    pub type rxkb_context_flags = ::core::ffi::c_uint;
    pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;
    pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
    pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
    pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;
}
pub mod darray_h {
    pub type darray_size_t = ::core::ffi::c_uint;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct darray_string {
        pub size: darray_size_t,
        pub alloc: darray_size_t,
        pub item: *mut *mut ::core::ffi::c_char,
    }
    #[inline]
    pub unsafe extern "C" fn darray_next_alloc(
        mut alloc: darray_size_t,
        mut need: darray_size_t,
        mut itemSize: usize,
    ) -> darray_size_t {
        unsafe {
            if (need as usize)
                < ((2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_mul(2 as ::core::ffi::c_uint)
                    .wrapping_add(1 as ::core::ffi::c_uint) as usize)
                    .wrapping_div(itemSize)
                    .wrapping_div(2 as usize)
            {
            } else {
                __assert_fail(
                    b"need < darray_max_alloc(itemSize) / 2\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"../src/darray.h\0".as_ptr() as *const ::core::ffi::c_char,
                    220 as ::core::ffi::c_uint,
                    b"darray_size_t darray_next_alloc(darray_size_t, darray_size_t, usize)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if alloc == 0 as darray_size_t {
                alloc = 4 as darray_size_t;
            }
            while alloc < need {
                alloc = alloc.wrapping_mul(2 as darray_size_t);
            }
            return alloc;
        }
    }

    use super::assert_h::__assert_fail;
}
pub mod util_list_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct list {
        pub prev: *mut list,
        pub next: *mut list,
    }
    extern "C" {
        pub fn list_init(list: *mut list);
        pub fn list_append(list: *mut list, elm: *mut list);
        pub fn list_remove(elm: *mut list);
        pub fn list_empty(list: *const list) -> bool;
        pub fn list_is_last(list: *const list, elm: *const list) -> bool;
    }
}
pub mod messages_codes_h {
    pub const XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH: xkb_message_code = 632;
    pub const XKB_ERROR_INVALID_PATH: xkb_message_code = 161;
    pub const XKB_ERROR_INSUFFICIENT_BUFFER_SIZE: xkb_message_code = 134;
    pub type xkb_message_code = ::core::ffi::c_uint;
    pub const _XKB_LOG_MESSAGE_MAX_CODE: xkb_message_code = 971;
    pub const XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE: xkb_message_code = 971;
    pub const XKB_ERROR_INVALID_RULES_SYNTAX: xkb_message_code = 967;
    pub const XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL: xkb_message_code = 965;
    pub const XKB_ERROR_INVALID_IDENTIFIER: xkb_message_code = 949;
    pub const XKB_WARNING_CONFLICTING_KEY_FIELDS: xkb_message_code = 935;
    pub const XKB_ERROR_ABI_BACKWARD_COMPAT_: xkb_message_code = 914;
    pub const XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX: xkb_message_code = 903;
    pub const XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY: xkb_message_code = 901;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS: xkb_message_code = 893;
    pub const XKB_WARNING_CONFLICTING_KEY_ACTION: xkb_message_code = 883;
    pub const XKB_ERROR_ABI_FORWARD_COMPAT_: xkb_message_code = 876;
    pub const XKB_ERROR_UNKNOWN_ACTION_TYPE: xkb_message_code = 844;
    pub const XKB_ERROR_KEYMAP_COMPILATION_FAILED: xkb_message_code = 822;
    pub const XKB_ERROR_UNKNOWN_FIELD: xkb_message_code = 812;
    pub const XKB_WARNING_CONFLICTING_MODMAP: xkb_message_code = 800;
    pub const XKB_ERROR_INVALID_VALUE: xkb_message_code = 796;
    pub const XKB_ERROR_INVALID_EXPRESSION_TYPE: xkb_message_code = 784;
    pub const XKB_WARNING_UNDEFINED_KEYCODE: xkb_message_code = 770;
    pub const XKB_ERROR_INVALID_XKB_SYNTAX: xkb_message_code = 769;
    pub const XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION: xkb_message_code = 762;
    pub const XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT: xkb_message_code = 742;
    pub const XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD: xkb_message_code = 711;
    pub const XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE: xkb_message_code = 700;
    pub const XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT: xkb_message_code = 693;
    pub const XKB_ERROR_INVALID_COMPOSE_SYNTAX: xkb_message_code = 685;
    pub const XKB_ERROR_INVALID_COMPOSE_LOCALE: xkb_message_code = 679;
    pub const XKB_ERROR_INVALID_INCLUDED_FILE: xkb_message_code = 661;
    pub const XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE: xkb_message_code = 645;
    pub const XKB_ERROR_UNKNOWN_DEFAULT_FIELD: xkb_message_code = 639;
    pub const XKB_ERROR_INVALID_REAL_MODIFIER: xkb_message_code = 623;
    pub const XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE: xkb_message_code = 607;
    pub const XKB_ERROR_CANNOT_RESOLVE_RMLVO: xkb_message_code = 595;
    pub const XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX: xkb_message_code = 588;
    pub const XKB_ERROR_WRONG_FIELD_TYPE: xkb_message_code = 578;
    pub const XKB_ERROR_INVALID_ACTION_FIELD: xkb_message_code = 563;
    pub const XKB_ERROR_ALLOCATION_ERROR: xkb_message_code = 550;
    pub const XKB_ERROR_INVALID_FILE_ENCODING: xkb_message_code = 542;
    pub const XKB_WARNING_CONFLICTING_KEY_NAME: xkb_message_code = 523;
    pub const XKB_WARNING_EXTRA_SYMBOLS_IGNORED: xkb_message_code = 516;
    pub const XKB_WARNING_NUMERIC_KEYSYM: xkb_message_code = 489;
    pub const XKB_ERROR_INVALID_OPERATION: xkb_message_code = 478;
    pub const XKB_WARNING_CONFLICTING_KEY_SYMBOL: xkb_message_code = 461;
    pub const XKB_ERROR_ABI_INVALID_STRUCT_SIZE_: xkb_message_code = 450;
    pub const XKB_WARNING_MISSING_DEFAULT_SECTION: xkb_message_code = 433;
    pub const XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE: xkb_message_code = 428;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS: xkb_message_code = 407;
    pub const XKB_ERROR_RECURSIVE_INCLUDE: xkb_message_code = 386;
    pub const XKB_WARNING_DUPLICATE_ENTRY: xkb_message_code = 378;
    pub const XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_: xkb_message_code = 371;
    pub const XKB_WARNING_UNSUPPORTED_LEGACY_ACTION: xkb_message_code = 362;
    pub const XKB_ERROR_OVERLAPPING_OVERLAY: xkb_message_code = 355;
    pub const XKB_ERROR_UNKNOWN_OPERATOR: xkb_message_code = 345;
    pub const XKB_ERROR_INCLUDED_FILE_NOT_FOUND: xkb_message_code = 338;
    pub const XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL: xkb_message_code = 312;
    pub const XKB_WARNING_NON_BASE_GROUP_NAME: xkb_message_code = 305;
    pub const XKB_WARNING_DEPRECATED_KEYSYM_NAME: xkb_message_code = 302;
    pub const XKB_WARNING_DEPRECATED_KEYSYM: xkb_message_code = 301;
    pub const XKB_WARNING_UNDEFINED_KEY_TYPE: xkb_message_code = 286;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY: xkb_message_code = 266;
    pub const XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT: xkb_message_code = 254;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES: xkb_message_code = 239;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_: xkb_message_code = 237;
    pub const XKB_ERROR_UNKNOWN_STATEMENT: xkb_message_code = 222;
    pub const XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_: xkb_message_code = 214;
    pub const XKB_ERROR_INVALID_MODMAP_ENTRY: xkb_message_code = 206;
    pub const XKB_ERROR_INVALID_INCLUDE_STATEMENT: xkb_message_code = 203;
    pub const XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT: xkb_message_code = 195;
    pub const XKB_WARNING_INVALID_ESCAPE_SEQUENCE: xkb_message_code = 193;
    pub const XKB_WARNING_CANNOT_INFER_KEY_TYPE: xkb_message_code = 183;
    pub const XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION: xkb_message_code = 172;
    pub const XKB_ERROR_WRONG_STATEMENT_TYPE: xkb_message_code = 150;
    pub const XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER: xkb_message_code = 123;
    pub const XKB_WARNING_UNRECOGNIZED_KEYSYM: xkb_message_code = 107;
    pub const XKB_WARNING_ILLEGAL_KEYCODE_ALIAS: xkb_message_code = 101;
    pub const XKB_ERROR_INVALID_NUMERIC_KEYSYM: xkb_message_code = 82;
    pub const XKB_ERROR_EXPECTED_ARRAY_ENTRY: xkb_message_code = 77;
    pub const XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_: xkb_message_code = 60;
    pub const XKB_ERROR_INTEGER_OVERFLOW: xkb_message_code = 52;
    pub const XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES: xkb_message_code = 43;
    pub const XKB_ERROR_MALFORMED_NUMBER_LITERAL: xkb_message_code = 34;
    pub const _XKB_LOG_MESSAGE_MIN_CODE: xkb_message_code = 34;
}
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        pub fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
pub mod string_h {

    extern "C" {
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: usize,
        ) -> *mut ::core::ffi::c_void;
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> usize;
        pub fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    }
}
pub mod utils_h {
    #[inline]
    pub unsafe extern "C" fn streq(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if !s1.is_null() && !s2.is_null() {
            } else {
                __assert_fail(
                    b"s1 && s2\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../src/utils.h\0".as_ptr() as *const ::core::ffi::c_char,
                    94 as ::core::ffi::c_uint,
                    b"_Bool streq(const char *, const char *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn streq_null(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
    ) -> bool {
        unsafe {
            if s1.is_null() || s2.is_null() {
                return s1 == s2;
            }
            return streq(s1, s2);
        }
    }
    #[inline]
    pub unsafe extern "C" fn istrneq(
        mut s1: *const ::core::ffi::c_char,
        mut s2: *const ::core::ffi::c_char,
        mut len: usize,
    ) -> bool {
        unsafe {
            return istrncmp(s1, s2, len) == 0 as ::core::ffi::c_int;
        }
    }
    #[inline]
    pub unsafe extern "C" fn strdup_safe(
        mut s: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        unsafe {
            return if !s.is_null() {
                strdup(s)
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            };
        }
    }
    #[inline]
    pub unsafe extern "C" fn is_space(mut ch: ::core::ffi::c_char) -> bool {
        unsafe {
            return ch as ::core::ffi::c_int == ' ' as i32
                || ch as ::core::ffi::c_int >= '\t' as i32
                    && ch as ::core::ffi::c_int <= '\r' as i32;
        }
    }
    #[inline]
    pub unsafe extern "C" fn check_eaccess(
        mut path: *const ::core::ffi::c_char,
        mut mode: ::core::ffi::c_int,
    ) -> bool {
        unsafe {
            if eaccess(path, mode) != 0 as ::core::ffi::c_int {
                return false;
            }
            return true;
        }
    }
    #[inline]
    pub unsafe extern "C" fn snprintf_safe(
        mut buf: *mut ::core::ffi::c_char,
        mut sz: usize,
        mut format: *const ::core::ffi::c_char,
        mut c2rust_args: ...
    ) -> bool {
        unsafe {
            let mut ap: ::core::ffi::VaList;
            let mut rc: ::core::ffi::c_int = 0;
            ap = c2rust_args.clone();
            rc = vsnprintf(buf, sz, format, ap);
            return rc >= 0 as ::core::ffi::c_int && (rc as usize) < sz;
        }
    }

    use super::assert_h::__assert_fail;
    use super::stdio_h::vsnprintf;
    use super::string_h::{strcmp, strdup};
    use super::unistd_h::eaccess;
    extern "C" {
        pub fn istrncmp(
            a: *const ::core::ffi::c_char,
            b: *const ::core::ffi::c_char,
            n: usize,
        ) -> ::core::ffi::c_int;
    }
}
pub mod util_mem_h {
    #[inline]
    pub unsafe extern "C" fn _steal(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
        unsafe {
            let mut original: *mut *mut ::core::ffi::c_void = ptr as *mut *mut ::core::ffi::c_void;
            let mut swapped: *mut ::core::ffi::c_void = *original;
            *original = NULL;
            return swapped;
        }
    }
    use super::__stddef_null_h::NULL;
}
pub mod assert_h {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub mod xmlversion_h {
    extern "C" {
        pub fn xmlCheckVersion(version: ::core::ffi::c_int);
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod errno_base_h {
    pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
    pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
}
pub mod unistd_h {
    pub const R_OK: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {
        pub fn eaccess(
            __name: *const ::core::ffi::c_char,
            __type: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
pub mod errno_h {
    extern "C" {
        pub fn __errno_location() -> *mut ::core::ffi::c_int;
    }
}
pub mod config_h {
    pub const DEFAULT_XKB_RULES: [::core::ffi::c_char; 6] =
        unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"evdev\0") };
    pub const DFLT_XKB_CONFIG_EXTRA_PATH: [::core::ffi::c_char; 19] = unsafe {
        ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"/usr/local/etc/xkb\0")
    };
    pub const DFLT_XKB_CONFIG_ROOT: [::core::ffi::c_char; 30] = unsafe {
        ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
            *b"/usr/share/xkeyboard-config-2\0",
        )
    };
    pub const DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH: [::core::ffi::c_char; 30] = unsafe {
        ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
            *b"/usr/share/xkeyboard-config.d\0",
        )
    };
    pub const DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH: [::core::ffi::c_char; 32] = unsafe {
        ::core::mem::transmute::<[u8; 32], [::core::ffi::c_char; 32]>(
            *b"/usr/share/xkeyboard-config-2.d\0",
        )
    };
    pub const DFLT_XKB_LEGACY_ROOT: [::core::ffi::c_char; 19] = unsafe {
        ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"/usr/share/X11/xkb\0")
    };
}
pub mod bits_stat_h {
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
pub mod stdbool_h {}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::NULL;

use self::assert_h::__assert_fail;
pub use self::bits_stat_h::__S_IFMT;
pub use self::config_h::{
    DEFAULT_XKB_RULES, DFLT_XKB_CONFIG_EXTRA_PATH, DFLT_XKB_CONFIG_ROOT,
    DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH, DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH,
    DFLT_XKB_LEGACY_ROOT,
};
pub use self::darray_h::{darray_next_alloc, darray_size_t, darray_string};
pub use self::dict_h::{xmlDict, xmlDictPtr};
pub use self::dirent_h::dirent;
pub use self::encoding_h::{
    _xmlCharEncodingHandler, xmlCharEncoding, xmlCharEncodingHandler, xmlCharEncodingHandlerPtr,
    xmlCharEncodingInputFunc, xmlCharEncodingOutputFunc, XML_CHAR_ENCODING_2022_JP,
    XML_CHAR_ENCODING_8859_1, XML_CHAR_ENCODING_8859_2, XML_CHAR_ENCODING_8859_3,
    XML_CHAR_ENCODING_8859_4, XML_CHAR_ENCODING_8859_5, XML_CHAR_ENCODING_8859_6,
    XML_CHAR_ENCODING_8859_7, XML_CHAR_ENCODING_8859_8, XML_CHAR_ENCODING_8859_9,
    XML_CHAR_ENCODING_ASCII, XML_CHAR_ENCODING_EBCDIC, XML_CHAR_ENCODING_ERROR,
    XML_CHAR_ENCODING_EUC_JP, XML_CHAR_ENCODING_NONE, XML_CHAR_ENCODING_SHIFT_JIS,
    XML_CHAR_ENCODING_UCS2, XML_CHAR_ENCODING_UCS4BE, XML_CHAR_ENCODING_UCS4LE,
    XML_CHAR_ENCODING_UCS4_2143, XML_CHAR_ENCODING_UCS4_3412, XML_CHAR_ENCODING_UTF16BE,
    XML_CHAR_ENCODING_UTF16LE, XML_CHAR_ENCODING_UTF8,
};
pub use self::entities_h::{
    _xmlEntity, xmlEntityType, XML_EXTERNAL_GENERAL_PARSED_ENTITY,
    XML_EXTERNAL_GENERAL_UNPARSED_ENTITY, XML_EXTERNAL_PARAMETER_ENTITY,
    XML_INTERNAL_GENERAL_ENTITY, XML_INTERNAL_PARAMETER_ENTITY, XML_INTERNAL_PREDEFINED_ENTITY,
};
pub use self::errno_base_h::{EACCES, ENOMEM, ENOTDIR};
use self::errno_h::__errno_location;
pub use self::hash_h::{_xmlHashTable, xmlHashTable, xmlHashTablePtr};
pub use self::iconv_h::iconv_t;
pub use self::include_dirent_h::{__dirstream, closedir, opendir, readdir, DIR};
pub use self::internal::{__builtin_va_list, __va_list_tag};
pub use self::messages_codes_h::{
    xkb_message_code, _XKB_LOG_MESSAGE_MAX_CODE, _XKB_LOG_MESSAGE_MIN_CODE,
    XKB_ERROR_ABI_BACKWARD_COMPAT_, XKB_ERROR_ABI_FORWARD_COMPAT_,
    XKB_ERROR_ABI_INVALID_STRUCT_SIZE_, XKB_ERROR_ALLOCATION_ERROR, XKB_ERROR_CANNOT_RESOLVE_RMLVO,
    XKB_ERROR_CONFLICTING_KEY_SYMBOLS_ENTRY, XKB_ERROR_EXPECTED_ARRAY_ENTRY,
    XKB_ERROR_GLOBAL_DEFAULTS_WRONG_SCOPE, XKB_ERROR_INCLUDED_FILE_NOT_FOUND,
    XKB_ERROR_INCOMPATIBLE_ACTIONS_AND_KEYSYMS_COUNT, XKB_ERROR_INCOMPATIBLE_KEYMAP_TEXT_FORMAT,
    XKB_ERROR_INSUFFICIENT_BUFFER_SIZE, XKB_ERROR_INTEGER_OVERFLOW, XKB_ERROR_INVALID_ACTION_FIELD,
    XKB_ERROR_INVALID_COMPOSE_LOCALE, XKB_ERROR_INVALID_COMPOSE_SYNTAX,
    XKB_ERROR_INVALID_EXPRESSION_TYPE, XKB_ERROR_INVALID_FILE_ENCODING,
    XKB_ERROR_INVALID_IDENTIFIER, XKB_ERROR_INVALID_INCLUDED_FILE,
    XKB_ERROR_INVALID_INCLUDE_STATEMENT, XKB_ERROR_INVALID_MODMAP_ENTRY,
    XKB_ERROR_INVALID_NUMERIC_KEYSYM, XKB_ERROR_INVALID_OPERATION, XKB_ERROR_INVALID_PATH,
    XKB_ERROR_INVALID_REAL_MODIFIER, XKB_ERROR_INVALID_RULES_SYNTAX,
    XKB_ERROR_INVALID_SET_DEFAULT_STATEMENT, XKB_ERROR_INVALID_VALUE, XKB_ERROR_INVALID_XKB_SYNTAX,
    XKB_ERROR_KEYMAP_COMPILATION_FAILED, XKB_ERROR_MALFORMED_NUMBER_LITERAL,
    XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH, XKB_ERROR_OVERLAPPING_OVERLAY,
    XKB_ERROR_RECURSIVE_INCLUDE, XKB_ERROR_RULES_INVALID_LAYOUT_INDEX_PERCENT_EXPANSION,
    XKB_ERROR_UNDECLARED_VIRTUAL_MODIFIER, XKB_ERROR_UNKNOWN_ACTION_TYPE,
    XKB_ERROR_UNKNOWN_DEFAULT_FIELD, XKB_ERROR_UNKNOWN_FIELD, XKB_ERROR_UNKNOWN_OPERATOR,
    XKB_ERROR_UNKNOWN_STATEMENT, XKB_ERROR_UNSUPPORTED_A11Y_FLAGS_,
    XKB_ERROR_UNSUPPORTED_LAYOUT_INDEX_, XKB_ERROR_UNSUPPORTED_LAYOUT_OUT_OF_RANGE_POLICY_,
    XKB_ERROR_UNSUPPORTED_MODIFIER_MASK_, XKB_ERROR_UNSUPPORTED_OVERLAY_INDEX,
    XKB_ERROR_UNSUPPORTED_SHIFT_LEVEL, XKB_ERROR_WRONG_FIELD_TYPE, XKB_ERROR_WRONG_STATEMENT_TYPE,
    XKB_WARNING_CANNOT_INFER_KEY_TYPE, XKB_WARNING_CONFLICTING_KEY_ACTION,
    XKB_WARNING_CONFLICTING_KEY_FIELDS, XKB_WARNING_CONFLICTING_KEY_NAME,
    XKB_WARNING_CONFLICTING_KEY_SYMBOL, XKB_WARNING_CONFLICTING_KEY_TYPE_DEFINITIONS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_LEVEL_NAMES, XKB_WARNING_CONFLICTING_KEY_TYPE_MAP_ENTRY,
    XKB_WARNING_CONFLICTING_KEY_TYPE_MERGING_GROUPS,
    XKB_WARNING_CONFLICTING_KEY_TYPE_PRESERVE_ENTRIES, XKB_WARNING_CONFLICTING_MODMAP,
    XKB_WARNING_DEPRECATED_KEYSYM, XKB_WARNING_DEPRECATED_KEYSYM_NAME, XKB_WARNING_DUPLICATE_ENTRY,
    XKB_WARNING_EXTRA_SYMBOLS_IGNORED, XKB_WARNING_ILLEGAL_KEYCODE_ALIAS,
    XKB_WARNING_ILLEGAL_KEY_TYPE_PRESERVE_RESULT, XKB_WARNING_INVALID_ESCAPE_SEQUENCE,
    XKB_WARNING_INVALID_UNICODE_ESCAPE_SEQUENCE, XKB_WARNING_MISSING_DEFAULT_SECTION,
    XKB_WARNING_MISSING_SYMBOLS_GROUP_NAME_INDEX, XKB_WARNING_MULTIPLE_GROUPS_AT_ONCE,
    XKB_WARNING_NON_BASE_GROUP_NAME, XKB_WARNING_NUMERIC_KEYSYM,
    XKB_WARNING_UNDECLARED_MODIFIERS_IN_KEY_TYPE, XKB_WARNING_UNDEFINED_KEYCODE,
    XKB_WARNING_UNDEFINED_KEY_TYPE, XKB_WARNING_UNKNOWN_CHAR_ESCAPE_SEQUENCE,
    XKB_WARNING_UNRECOGNIZED_KEYSYM, XKB_WARNING_UNRESOLVED_KEYMAP_SYMBOL,
    XKB_WARNING_UNSUPPORTED_GEOMETRY_SECTION, XKB_WARNING_UNSUPPORTED_LEGACY_ACTION,
    XKB_WARNING_UNSUPPORTED_SYMBOLS_FIELD,
};
pub use self::parser_h::{
    _xmlAttrHashBucket, _xmlParserCtxt, _xmlParserInput, _xmlParserNodeInfo, _xmlParserNodeInfoSeq,
    _xmlParserNsData, _xmlSAXHandler, _xmlSAXLocator, _xmlStartTag, attributeDeclSAXFunc,
    cdataBlockSAXFunc, charactersSAXFunc, commentSAXFunc, elementDeclSAXFunc, endDocumentSAXFunc,
    endElementNsSAX2Func, endElementSAXFunc, entityDeclSAXFunc, errorSAXFunc,
    externalSubsetSAXFunc, fatalErrorSAXFunc, getEntitySAXFunc, getParameterEntitySAXFunc,
    hasExternalSubsetSAXFunc, hasInternalSubsetSAXFunc, ignorableWhitespaceSAXFunc,
    internalSubsetSAXFunc, isStandaloneSAXFunc, notationDeclSAXFunc, processingInstructionSAXFunc,
    referenceSAXFunc, resolveEntitySAXFunc, setDocumentLocatorSAXFunc, startDocumentSAXFunc,
    startElementNsSAX2Func, startElementSAXFunc, unparsedEntityDeclSAXFunc, warningSAXFunc,
    xmlAttrHashBucket, xmlCtxtReadFile, xmlCtxtUseOptions, xmlFreeParserCtxt, xmlIOParseDTD,
    xmlNewParserCtxt, xmlParserInputDeallocate, xmlParserInputState, xmlParserMode,
    xmlParserNodeInfo, xmlParserNodeInfoSeq, xmlParserNsData, xmlStartTag, C2Rust_Unnamed,
    XML_PARSER_ATTRIBUTE_VALUE, XML_PARSER_CDATA_SECTION, XML_PARSER_COMMENT, XML_PARSER_CONTENT,
    XML_PARSER_DTD, XML_PARSER_END_TAG, XML_PARSER_ENTITY_DECL, XML_PARSER_ENTITY_VALUE,
    XML_PARSER_EOF, XML_PARSER_EPILOG, XML_PARSER_IGNORE, XML_PARSER_MISC, XML_PARSER_PI,
    XML_PARSER_PROLOG, XML_PARSER_PUBLIC_LITERAL, XML_PARSER_START, XML_PARSER_START_TAG,
    XML_PARSER_SYSTEM_LITERAL, XML_PARSER_XML_DECL, XML_PARSE_BIG_LINES, XML_PARSE_COMPACT,
    XML_PARSE_DOM, XML_PARSE_DTDATTR, XML_PARSE_DTDLOAD, XML_PARSE_DTDVALID, XML_PARSE_HUGE,
    XML_PARSE_IGNORE_ENC, XML_PARSE_NOBASEFIX, XML_PARSE_NOBLANKS, XML_PARSE_NOCDATA,
    XML_PARSE_NODICT, XML_PARSE_NOENT, XML_PARSE_NOERROR, XML_PARSE_NONET, XML_PARSE_NOWARNING,
    XML_PARSE_NOXINCNODE, XML_PARSE_NSCLEAN, XML_PARSE_OLD10, XML_PARSE_OLDSAX, XML_PARSE_PEDANTIC,
    XML_PARSE_PUSH_DOM, XML_PARSE_PUSH_SAX, XML_PARSE_READER, XML_PARSE_RECOVER, XML_PARSE_SAX,
    XML_PARSE_SAX1, XML_PARSE_UNKNOWN, XML_PARSE_XINCLUDE,
};
use self::stat_h::stat;
pub use self::stdint_uintn_h::u32;
pub use self::stdio_h::{fprintf, stderr, va_list, vfprintf, vsnprintf};
pub use self::stdlib_h::{
    __compar_fn_t, calloc, free, getenv, qsort, realloc, secure_getenv, strtol,
};
use self::string_h::{memset, strcmp, strdup, strerror, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::struct_stat_h::stat;
pub use self::struct_timespec_h::timespec;
pub use self::tree_h::{
    _xmlAttr, _xmlBuf, _xmlDict, _xmlDoc, _xmlDtd, _xmlElementContent, _xmlEnumeration, _xmlNode,
    _xmlNs, xmlAttr, xmlAttrPtr, xmlAttributeType, xmlBuf, xmlBufPtr, xmlDoc, xmlDocGetRootElement,
    xmlDocPtr, xmlDtd, xmlDtdPtr, xmlElementContent, xmlElementContentOccur, xmlElementContentPtr,
    xmlElementContentType, xmlElementType, xmlEntity, xmlEntityPtr, xmlEnumeration,
    xmlEnumerationPtr, xmlFreeDoc, xmlFreeDtd, xmlGetProp, xmlNode, xmlNodePtr, xmlNs, xmlNsType,
    xmlParserCtxt, xmlParserCtxtPtr, xmlParserInput, xmlParserInputBuffer, xmlParserInputBufferPtr,
    xmlParserInputPtr, xmlSAXHandler, xmlSAXHandlerPtr, xmlSAXLocator, xmlSAXLocatorPtr,
    XML_ATTRIBUTE_CDATA, XML_ATTRIBUTE_DECL, XML_ATTRIBUTE_ENTITIES, XML_ATTRIBUTE_ENTITY,
    XML_ATTRIBUTE_ENUMERATION, XML_ATTRIBUTE_ID, XML_ATTRIBUTE_IDREF, XML_ATTRIBUTE_IDREFS,
    XML_ATTRIBUTE_NMTOKEN, XML_ATTRIBUTE_NMTOKENS, XML_ATTRIBUTE_NODE, XML_ATTRIBUTE_NOTATION,
    XML_CDATA_SECTION_NODE, XML_COMMENT_NODE, XML_DOCUMENT_FRAG_NODE, XML_DOCUMENT_NODE,
    XML_DOCUMENT_TYPE_NODE, XML_DTD_NODE, XML_ELEMENT_CONTENT_ELEMENT, XML_ELEMENT_CONTENT_MULT,
    XML_ELEMENT_CONTENT_ONCE, XML_ELEMENT_CONTENT_OPT, XML_ELEMENT_CONTENT_OR,
    XML_ELEMENT_CONTENT_PCDATA, XML_ELEMENT_CONTENT_PLUS, XML_ELEMENT_CONTENT_SEQ,
    XML_ELEMENT_DECL, XML_ELEMENT_NODE, XML_ENTITY_DECL, XML_ENTITY_NODE, XML_ENTITY_REF_NODE,
    XML_HTML_DOCUMENT_NODE, XML_NAMESPACE_DECL, XML_NOTATION_NODE, XML_PI_NODE, XML_TEXT_NODE,
    XML_XINCLUDE_END, XML_XINCLUDE_START,
};
pub use self::types_h::{
    __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino64_t, __ino_t, __mode_t, __nlink_t, __off64_t,
    __off_t, __syscall_slong_t, __time_t, __uid_t, __uint32_t, __uint64_t,
};
pub use self::unistd_h::{eaccess, R_OK, X_OK};
pub use self::util_list_h::{list, list_append, list_empty, list_init, list_is_last, list_remove};
pub use self::util_mem_h::_steal;
pub use self::utils_h::{
    check_eaccess, is_space, istrncmp, istrneq, snprintf_safe, strdup_safe, streq, streq_null,
};
pub use self::valid_h::{
    _xmlValidCtxt, _xmlValidState, xmlFreeValidCtxt, xmlNewValidCtxt, xmlValidCtxt,
    xmlValidCtxtPtr, xmlValidState, xmlValidateDtd, xmlValidityErrorFunc, xmlValidityWarningFunc,
};
pub use self::xkbregistry_h::{
    rxkb_context_flags, rxkb_log_level, rxkb_popularity, RXKB_CONTEXT_LOAD_EXOTIC_RULES,
    RXKB_CONTEXT_NO_DEFAULT_INCLUDES, RXKB_CONTEXT_NO_FLAGS, RXKB_CONTEXT_NO_SECURE_GETENV,
    RXKB_LOG_LEVEL_CRITICAL, RXKB_LOG_LEVEL_DEBUG, RXKB_LOG_LEVEL_ERROR, RXKB_LOG_LEVEL_INFO,
    RXKB_LOG_LEVEL_WARNING, RXKB_POPULARITY_EXOTIC, RXKB_POPULARITY_STANDARD,
};
pub use self::xmlIO_h::{
    _xmlParserInputBuffer, xmlInputCloseCallback, xmlInputReadCallback,
    xmlParserInputBufferCreateMem,
};
pub use self::xmlautomata_h::{
    _xmlAutomata, _xmlAutomataState, xmlAutomata, xmlAutomataPtr, xmlAutomataState,
    xmlAutomataStatePtr,
};
pub use self::xmlerror_h::{
    _xmlError, xmlError, xmlErrorLevel, xmlGenericErrorFunc, xmlSetGenericErrorFunc,
    xmlStructuredErrorFunc, XML_ERR_ERROR, XML_ERR_FATAL, XML_ERR_NONE, XML_ERR_WARNING,
};
pub use self::xmlmemory_h::{xmlFree, xmlFreeFunc};
pub use self::xmlstring_h::{xmlChar, xmlStrEqual, xmlStrdup};
use self::xmlversion_h::xmlCheckVersion;
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_context {
    pub base: rxkb_object,
    pub context_state: context_state,
    pub load_extra_rules_files: bool,
    pub use_secure_getenv: bool,
    pub models: list,
    pub layouts: list,
    pub option_groups: list,
    pub includes: C2Rust_Unnamed_0,
    pub log_fn: Option<
        unsafe extern "C" fn(
            *mut rxkb_context,
            rxkb_log_level,
            *const ::core::ffi::c_char,
            ::core::ffi::VaList,
        ) -> (),
    >,
    pub log_level: rxkb_log_level,
    pub userdata: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *mut ::core::ffi::c_char,
}
pub type context_state = ::core::ffi::c_uint;
pub const CONTEXT_FAILED: context_state = 2;
pub const CONTEXT_PARSED: context_state = 1;
pub const CONTEXT_NEW: context_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_object {
    pub parent: *mut rxkb_object,
    pub refcount: u32,
    pub link: list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_model {
    pub base: rxkb_object,
    pub name: *mut ::core::ffi::c_char,
    pub vendor: *mut ::core::ffi::c_char,
    pub description: *mut ::core::ffi::c_char,
    pub popularity: rxkb_popularity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_layout {
    pub base: rxkb_object,
    pub name: *mut ::core::ffi::c_char,
    pub brief: *mut ::core::ffi::c_char,
    pub description: *mut ::core::ffi::c_char,
    pub variant: *mut ::core::ffi::c_char,
    pub popularity: rxkb_popularity,
    pub iso639s: list,
    pub iso3166s: list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_option_group {
    pub base: rxkb_object,
    pub allow_multiple: bool,
    pub options: list,
    pub name: *mut ::core::ffi::c_char,
    pub description: *mut ::core::ffi::c_char,
    pub popularity: rxkb_popularity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_option {
    pub base: rxkb_object,
    pub name: *mut ::core::ffi::c_char,
    pub brief: *mut ::core::ffi::c_char,
    pub description: *mut ::core::ffi::c_char,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_iso639_code {
    pub base: rxkb_object,
    pub code: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_iso3166_code {
    pub base: rxkb_object,
    pub code: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_item {
    pub name: *mut ::core::ffi::c_char,
    pub description: *mut ::core::ffi::c_char,
    pub brief: *mut ::core::ffi::c_char,
    pub vendor: *mut ::core::ffi::c_char,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}
unsafe extern "C" fn rxkb_log(
    mut ctx: *mut rxkb_context,
    mut level: rxkb_log_level,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        let mut args: ::core::ffi::VaList;
        if ((*ctx).log_level as ::core::ffi::c_uint) < level as ::core::ffi::c_uint {
            return;
        }
        args = c2rust_args.clone();
        (*ctx).log_fn.expect("non-null function pointer")(ctx, level, fmt, args);
    }
}
unsafe extern "C" fn rxkb_object_init(mut object: *mut rxkb_object, mut parent: *mut rxkb_object) {
    unsafe {
        (*object).refcount = 1 as u32;
        (*object).parent = parent;
        list_init(&raw mut (*object).link);
    }
}
unsafe extern "C" fn rxkb_object_ref(mut object: *mut rxkb_object) -> *mut ::core::ffi::c_void {
    unsafe {
        if (*object).refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                226 as ::core::ffi::c_uint,
                b"void *rxkb_object_ref(struct rxkb_object *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*object).refcount = (*object).refcount.wrapping_add(1);
        return object as *mut ::core::ffi::c_void;
    }
}
unsafe extern "C" fn rxkb_iso639_code_destroy(mut code: *mut rxkb_iso639_code) {
    unsafe {
        free((*code).code as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_get_iso639_first(
    mut layout: *mut rxkb_layout,
) -> *mut rxkb_iso639_code {
    unsafe {
        let mut code: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        if !list_empty(&raw mut (*layout).iso639s) {
            code = ((*layout).iso639s.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        }
        return code;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso639_code_next(
    mut code: *mut rxkb_iso639_code,
) -> *mut rxkb_iso639_code {
    unsafe {
        let mut next: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        let mut layout: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        layout = ((*code).base.parent as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*layout).iso639s, &raw mut (*code).base.link) {
            return ::core::ptr::null_mut::<rxkb_iso639_code>();
        }
        next = ((*code).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_iso639_code
            as *mut rxkb_iso639_code;
        return next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso639_code_ref(
    mut object: *mut rxkb_iso639_code,
) -> *mut rxkb_iso639_code {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso639_code_unref(
    mut object: *mut rxkb_iso639_code,
) -> *mut rxkb_iso639_code {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_iso639_code>();
        }
        if (*object).base.refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->base.refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                264 as ::core::ffi::c_uint,
                b"struct rxkb_iso639_code *rxkb_iso639_code_unref(struct rxkb_iso639_code *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_iso639_code_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_iso639_code>();
    }
}
#[inline]
unsafe extern "C" fn rxkb_iso639_code_create(
    mut parent: *mut rxkb_object,
) -> *mut rxkb_iso639_code {
    unsafe {
        let mut t: *mut rxkb_iso639_code = calloc(
            1 as usize,
            ::core::mem::size_of::<rxkb_iso639_code>() as usize,
        ) as *mut rxkb_iso639_code;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso639_code_get_code(
    mut object: *mut rxkb_iso639_code,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).code;
    }
}
unsafe extern "C" fn rxkb_iso3166_code_destroy(mut code: *mut rxkb_iso3166_code) {
    unsafe {
        free((*code).code as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_get_iso3166_first(
    mut layout: *mut rxkb_layout,
) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut code: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        if !list_empty(&raw mut (*layout).iso3166s) {
            code = ((*layout).iso3166s.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        }
        return code;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso3166_code_next(
    mut code: *mut rxkb_iso3166_code,
) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut next: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        let mut layout: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        layout = ((*code).base.parent as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*layout).iso3166s, &raw mut (*code).base.link) {
            return ::core::ptr::null_mut::<rxkb_iso3166_code>();
        }
        next = ((*code).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_iso3166_code
            as *mut rxkb_iso3166_code;
        return next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso3166_code_unref(
    mut object: *mut rxkb_iso3166_code,
) -> *mut rxkb_iso3166_code {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_iso3166_code>();
        }
        if (*object).base.refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->base.refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_uint,
                b"struct rxkb_iso3166_code *rxkb_iso3166_code_unref(struct rxkb_iso3166_code *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_iso3166_code_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_iso3166_code>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso3166_code_ref(
    mut object: *mut rxkb_iso3166_code,
) -> *mut rxkb_iso3166_code {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[inline]
unsafe extern "C" fn rxkb_iso3166_code_create(
    mut parent: *mut rxkb_object,
) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut t: *mut rxkb_iso3166_code = calloc(
            1 as usize,
            ::core::mem::size_of::<rxkb_iso3166_code>() as usize,
        ) as *mut rxkb_iso3166_code;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_iso3166_code_get_code(
    mut object: *mut rxkb_iso3166_code,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).code;
    }
}
unsafe extern "C" fn rxkb_option_destroy(mut o: *mut rxkb_option) {
    unsafe {
        free((*o).name as *mut ::core::ffi::c_void);
        free((*o).brief as *mut ::core::ffi::c_void);
        free((*o).description as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_unref(mut object: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_option>();
        }
        if (*object).base.refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->base.refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                313 as ::core::ffi::c_uint,
                b"struct rxkb_option *rxkb_option_unref(struct rxkb_option *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_option_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_option>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_ref(mut object: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[inline]
unsafe extern "C" fn rxkb_option_create(mut parent: *mut rxkb_object) -> *mut rxkb_option {
    unsafe {
        let mut t: *mut rxkb_option =
            calloc(1 as usize, ::core::mem::size_of::<rxkb_option>() as usize) as *mut rxkb_option;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_get_name(
    mut object: *mut rxkb_option,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).name;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_get_brief(
    mut object: *mut rxkb_option,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).brief;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_get_description(
    mut object: *mut rxkb_option,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).description;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_get_popularity(
    mut object: *mut rxkb_option,
) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_is_layout_specific(mut object: *mut rxkb_option) -> bool {
    unsafe {
        return (*object).layout_specific;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_next(mut o: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        let mut parent: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut next: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        parent = ((*o).base.parent as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        next = ((*o).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option
            as *mut rxkb_option;
        if list_is_last(&raw mut (*parent).options, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_option>();
        }
        return next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_first(mut parent: *mut rxkb_option_group) -> *mut rxkb_option {
    unsafe {
        let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        if !list_empty(&raw mut (*parent).options) {
            o = ((*parent).options.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option
                as *mut rxkb_option;
        }
        return o;
    }
}
unsafe extern "C" fn rxkb_layout_destroy(mut l: *mut rxkb_layout) {
    unsafe {
        let mut iso639: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        let mut tmp_639: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        let mut iso3166: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        let mut tmp_3166: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        free((*l).name as *mut ::core::ffi::c_void);
        free((*l).brief as *mut ::core::ffi::c_void);
        free((*l).description as *mut ::core::ffi::c_void);
        free((*l).variant as *mut ::core::ffi::c_void);
        iso639 = ::core::ptr::null_mut::<rxkb_iso639_code>();
        tmp_639 = ::core::ptr::null_mut::<rxkb_iso639_code>();
        iso639 = ((*l).iso639s.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        tmp_639 = ((*iso639).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        while &raw mut (*iso639).base.link != &raw mut (*l).iso639s {
            rxkb_iso639_code_unref(iso639);
            iso639 = tmp_639;
            tmp_639 = ((*iso639).base.link.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        }
        iso3166 = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        tmp_3166 = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        iso3166 = ((*l).iso3166s.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        tmp_3166 = ((*iso3166).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        while &raw mut (*iso3166).base.link != &raw mut (*l).iso3166s {
            rxkb_iso3166_code_unref(iso3166);
            iso3166 = tmp_3166;
            tmp_3166 = ((*iso3166).base.link.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_ref(mut object: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_unref(mut object: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_layout>();
        }
        if (*object).base.refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->base.refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                343 as ::core::ffi::c_uint,
                b"struct rxkb_layout *rxkb_layout_unref(struct rxkb_layout *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_layout_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_layout>();
    }
}
#[inline]
unsafe extern "C" fn rxkb_layout_create(mut parent: *mut rxkb_object) -> *mut rxkb_layout {
    unsafe {
        let mut t: *mut rxkb_layout =
            calloc(1 as usize, ::core::mem::size_of::<rxkb_layout>() as usize) as *mut rxkb_layout;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_get_name(
    mut object: *mut rxkb_layout,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).name;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_get_brief(
    mut object: *mut rxkb_layout,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).brief;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_get_description(
    mut object: *mut rxkb_layout,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).description;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_get_variant(
    mut object: *mut rxkb_layout,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).variant;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_get_popularity(
    mut object: *mut rxkb_layout,
) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_first(mut parent: *mut rxkb_context) -> *mut rxkb_layout {
    unsafe {
        let mut o: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        if !list_empty(&raw mut (*parent).layouts) {
            o = ((*parent).layouts.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
                as *mut rxkb_layout;
        }
        return o;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_layout_next(mut o: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        let mut parent: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut next: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        parent = ((*o).base.parent as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*parent).layouts, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_layout>();
        }
        return next;
    }
}
unsafe extern "C" fn rxkb_model_destroy(mut m: *mut rxkb_model) {
    unsafe {
        free((*m).name as *mut ::core::ffi::c_void);
        free((*m).vendor as *mut ::core::ffi::c_void);
        free((*m).description as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_ref(mut object: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_unref(mut object: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_model>();
        }
        if (*object).base.refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->base.refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                360 as ::core::ffi::c_uint,
                b"struct rxkb_model *rxkb_model_unref(struct rxkb_model *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_model_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_model>();
    }
}
#[inline]
unsafe extern "C" fn rxkb_model_create(mut parent: *mut rxkb_object) -> *mut rxkb_model {
    unsafe {
        let mut t: *mut rxkb_model =
            calloc(1 as usize, ::core::mem::size_of::<rxkb_model>() as usize) as *mut rxkb_model;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_get_name(
    mut object: *mut rxkb_model,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).name;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_get_vendor(
    mut object: *mut rxkb_model,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).vendor;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_get_description(
    mut object: *mut rxkb_model,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).description;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_get_popularity(mut object: *mut rxkb_model) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_next(mut o: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        let mut parent: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut next: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        parent = ((*o).base.parent as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        if list_is_last(&raw mut (*parent).models, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_model>();
        }
        return next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_model_first(mut parent: *mut rxkb_context) -> *mut rxkb_model {
    unsafe {
        let mut o: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        if !list_empty(&raw mut (*parent).models) {
            o = ((*parent).models.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_model
                as *mut rxkb_model;
        }
        return o;
    }
}
unsafe extern "C" fn rxkb_option_group_destroy(mut og: *mut rxkb_option_group) {
    unsafe {
        let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        let mut otmp: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        free((*og).name as *mut ::core::ffi::c_void);
        free((*og).description as *mut ::core::ffi::c_void);
        o = ::core::ptr::null_mut::<rxkb_option>();
        otmp = ::core::ptr::null_mut::<rxkb_option>();
        o = ((*og).options.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option
            as *mut rxkb_option;
        otmp = ((*o).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option
            as *mut rxkb_option;
        while &raw mut (*o).base.link != &raw mut (*og).options {
            rxkb_option_unref(o);
            o = otmp;
            otmp = ((*o).base.link.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_option as *mut rxkb_option;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_allows_multiple(mut g: *mut rxkb_option_group) -> bool {
    unsafe {
        return (*g).allow_multiple;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_ref(
    mut object: *mut rxkb_option_group,
) -> *mut rxkb_option_group {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_unref(
    mut object: *mut rxkb_option_group,
) -> *mut rxkb_option_group {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_option_group>();
        }
        if (*object).base.refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->base.refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                387 as ::core::ffi::c_uint,
                b"struct rxkb_option_group *rxkb_option_group_unref(struct rxkb_option_group *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_option_group_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_option_group>();
    }
}
#[inline]
unsafe extern "C" fn rxkb_option_group_create(
    mut parent: *mut rxkb_object,
) -> *mut rxkb_option_group {
    unsafe {
        let mut t: *mut rxkb_option_group = calloc(
            1 as usize,
            ::core::mem::size_of::<rxkb_option_group>() as usize,
        ) as *mut rxkb_option_group;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_get_name(
    mut object: *mut rxkb_option_group,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).name;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_get_description(
    mut object: *mut rxkb_option_group,
) -> *const ::core::ffi::c_char {
    unsafe {
        return (*object).description;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_get_popularity(
    mut object: *mut rxkb_option_group,
) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_first(
    mut parent: *mut rxkb_context,
) -> *mut rxkb_option_group {
    unsafe {
        let mut o: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        if !list_empty(&raw mut (*parent).option_groups) {
            o = ((*parent).option_groups.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_option_group as *mut rxkb_option_group;
        }
        return o;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_option_group_next(
    mut o: *mut rxkb_option_group,
) -> *mut rxkb_option_group {
    unsafe {
        let mut parent: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut next: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        parent = ((*o).base.parent as *mut ::core::ffi::c_char)
            .offset(-(0 as ::core::ffi::c_ulong as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option_group
            as *mut rxkb_option_group;
        if list_is_last(&raw mut (*parent).option_groups, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_option_group>();
        }
        return next;
    }
}
unsafe extern "C" fn rxkb_context_destroy(mut ctx: *mut rxkb_context) {
    unsafe {
        let mut m: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        let mut mtmp: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        let mut l: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        let mut ltmp: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        let mut og: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut ogtmp: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut path: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        m = ::core::ptr::null_mut::<rxkb_model>();
        mtmp = ::core::ptr::null_mut::<rxkb_model>();
        m = ((*ctx).models.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        mtmp = ((*m).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        while &raw mut (*m).base.link != &raw mut (*ctx).models {
            rxkb_model_unref(m);
            m = mtmp;
            mtmp = ((*m).base.link.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_model as *mut rxkb_model;
        }
        if list_empty(&raw mut (*ctx).models) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"list_empty(&ctx->models)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                404 as ::core::ffi::c_uint,
                b"void rxkb_context_destroy(struct rxkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        l = ::core::ptr::null_mut::<rxkb_layout>();
        ltmp = ::core::ptr::null_mut::<rxkb_layout>();
        l = ((*ctx).layouts.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        ltmp = ((*l).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        while &raw mut (*l).base.link != &raw mut (*ctx).layouts {
            rxkb_layout_unref(l);
            l = ltmp;
            ltmp = ((*l).base.link.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_layout as *mut rxkb_layout;
        }
        if list_empty(&raw mut (*ctx).layouts) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"list_empty(&ctx->layouts)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                408 as ::core::ffi::c_uint,
                b"void rxkb_context_destroy(struct rxkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        og = ::core::ptr::null_mut::<rxkb_option_group>();
        ogtmp = ::core::ptr::null_mut::<rxkb_option_group>();
        og = ((*ctx).option_groups.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option_group
            as *mut rxkb_option_group;
        ogtmp = ((*og).base.link.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        while &raw mut (*og).base.link != &raw mut (*ctx).option_groups {
            rxkb_option_group_unref(og);
            og = ogtmp;
            ogtmp = ((*og).base.link.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize))
                as *mut rxkb_option_group as *mut rxkb_option_group;
        }
        if list_empty(&raw mut (*ctx).option_groups) as ::core::ffi::c_int != 0 {
        } else {
            __assert_fail(
                b"list_empty(&ctx->option_groups)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                412 as ::core::ffi::c_uint,
                b"void rxkb_context_destroy(struct rxkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if !(*ctx).includes.item.is_null() {
            path = (*ctx)
                .includes
                .item
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut ::core::ffi::c_char;
            while path
                < (*ctx).includes.item.offset((*ctx).includes.size as isize)
                    as *mut *mut ::core::ffi::c_char
            {
                free(*path as *mut ::core::ffi::c_void);
                path = path.offset(1);
            }
        }
        free((*ctx).includes.item as *mut ::core::ffi::c_void);
        (*ctx).includes.item = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*ctx).includes.size = 0 as darray_size_t;
        (*ctx).includes.alloc = 0 as darray_size_t;
        if (*ctx).includes.size == 0 as darray_size_t {
        } else {
            __assert_fail(
                b"darray_empty(ctx->includes)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                418 as ::core::ffi::c_uint,
                b"void rxkb_context_destroy(struct rxkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_ref(mut object: *mut rxkb_context) -> *mut rxkb_context {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_unref(mut object: *mut rxkb_context) -> *mut rxkb_context {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_context>();
        }
        if (*object).base.refcount >= 1 as u32 {
        } else {
            __assert_fail(
                b"object->base.refcount >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../src/registry.c\0".as_ptr() as *const ::core::ffi::c_char,
                421 as ::core::ffi::c_uint,
                b"struct rxkb_context *rxkb_context_unref(struct rxkb_context *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_context_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_context>();
    }
}
#[inline]
unsafe extern "C" fn rxkb_context_create(mut parent: *mut rxkb_object) -> *mut rxkb_context {
    unsafe {
        let mut t: *mut rxkb_context =
            calloc(1 as usize, ::core::mem::size_of::<rxkb_context>() as usize)
                as *mut rxkb_context;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_get_log_level(
    mut object: *mut rxkb_context,
) -> rxkb_log_level {
    unsafe {
        return (*object).log_level;
    }
}
unsafe extern "C" fn rxkb_context_getenv(
    mut ctx: *mut rxkb_context,
    mut name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if (*ctx).use_secure_getenv {
            return secure_getenv(name);
        } else {
            return getenv(name);
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_set_log_level(
    mut ctx: *mut rxkb_context,
    mut level: rxkb_log_level,
) {
    unsafe {
        (*ctx).log_level = level;
    }
}
unsafe extern "C" fn log_level_to_prefix(mut level: rxkb_log_level) -> *const ::core::ffi::c_char {
    unsafe {
        match level as ::core::ffi::c_uint {
            50 => return b"xkbregistry: DEBUG: \0".as_ptr() as *const ::core::ffi::c_char,
            40 => return b"xkbregistry: INFO: \0".as_ptr() as *const ::core::ffi::c_char,
            30 => {
                return b"xkbregistry: WARNING: \0".as_ptr() as *const ::core::ffi::c_char;
            }
            20 => return b"xkbregistry: ERROR: \0".as_ptr() as *const ::core::ffi::c_char,
            10 => {
                return b"xkbregistry: CRITICAL: \0".as_ptr() as *const ::core::ffi::c_char;
            }
            _ => return ::core::ptr::null::<::core::ffi::c_char>(),
        };
    }
}
unsafe extern "C" fn default_log_fn(
    mut ctx: *mut rxkb_context,
    mut level: rxkb_log_level,
    mut fmt: *const ::core::ffi::c_char,
    mut args: ::core::ffi::VaList,
) {
    unsafe {
        let mut prefix: *const ::core::ffi::c_char = log_level_to_prefix(level);
        if !prefix.is_null() {
            fprintf(
                stderr,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                prefix,
            );
        }
        vfprintf(stderr, fmt, args);
    }
}
unsafe extern "C" fn log_level(mut level: *const ::core::ffi::c_char) -> rxkb_log_level {
    unsafe {
        let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut lvl: rxkb_log_level = 0 as rxkb_log_level;
        *__errno_location() = 0 as ::core::ffi::c_int;
        lvl = strtol(level, &raw mut endptr, 10 as ::core::ffi::c_int) as rxkb_log_level;
        if *__errno_location() == 0 as ::core::ffi::c_int
            && (*endptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32
                || is_space(*endptr.offset(0 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
                    != 0)
        {
            return lvl;
        }
        if istrneq(
            b"crit\0".as_ptr() as *const ::core::ffi::c_char,
            level,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_CRITICAL;
        }
        if istrneq(
            b"err\0".as_ptr() as *const ::core::ffi::c_char,
            level,
            (::core::mem::size_of::<[::core::ffi::c_char; 4]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_ERROR;
        }
        if istrneq(
            b"warn\0".as_ptr() as *const ::core::ffi::c_char,
            level,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_WARNING;
        }
        if istrneq(
            b"info\0".as_ptr() as *const ::core::ffi::c_char,
            level,
            (::core::mem::size_of::<[::core::ffi::c_char; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_INFO;
        }
        if istrneq(
            b"debug\0".as_ptr() as *const ::core::ffi::c_char,
            level,
            (::core::mem::size_of::<[::core::ffi::c_char; 6]>() as usize).wrapping_sub(1 as usize),
        ) as ::core::ffi::c_int
            != 0
            || istrneq(
                b"dbg\0".as_ptr() as *const ::core::ffi::c_char,
                level,
                (::core::mem::size_of::<[::core::ffi::c_char; 4]>() as usize)
                    .wrapping_sub(1 as usize),
            ) as ::core::ffi::c_int
                != 0
        {
            return RXKB_LOG_LEVEL_DEBUG;
        }
        return RXKB_LOG_LEVEL_ERROR;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_new(mut flags: rxkb_context_flags) -> *mut rxkb_context {
    unsafe {
        let mut ctx: *mut rxkb_context =
            rxkb_context_create(::core::ptr::null_mut::<rxkb_object>());
        let mut env: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ctx.is_null() {
            return ::core::ptr::null_mut::<rxkb_context>();
        }
        (*ctx).context_state = CONTEXT_NEW;
        (*ctx).load_extra_rules_files = flags as ::core::ffi::c_uint
            & RXKB_CONTEXT_LOAD_EXOTIC_RULES as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0;
        (*ctx).use_secure_getenv = flags as ::core::ffi::c_uint
            & RXKB_CONTEXT_NO_SECURE_GETENV as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0;
        (*ctx).log_fn = Some(
            default_log_fn
                as unsafe extern "C" fn(
                    *mut rxkb_context,
                    rxkb_log_level,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut rxkb_context,
                    rxkb_log_level,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
            >;
        (*ctx).log_level = RXKB_LOG_LEVEL_ERROR;
        env = rxkb_context_getenv(
            ctx,
            b"RXKB_LOG_LEVEL\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !env.is_null() {
            rxkb_context_set_log_level(ctx, log_level(env));
        }
        static mut RXKB_CONTEXT_FLAGS: rxkb_context_flags = (RXKB_CONTEXT_NO_DEFAULT_INCLUDES
            as ::core::ffi::c_int
            | RXKB_CONTEXT_LOAD_EXOTIC_RULES as ::core::ffi::c_int
            | RXKB_CONTEXT_NO_SECURE_GETENV as ::core::ffi::c_int)
            as rxkb_context_flags;
        if flags as ::core::ffi::c_uint & !(RXKB_CONTEXT_FLAGS as ::core::ffi::c_uint) != 0 {
            rxkb_log(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                b"%s: Invalid context flags: 0x%x\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"rxkb_context_new\0".as_ptr() as *const ::core::ffi::c_char,
                flags as ::core::ffi::c_uint & !(RXKB_CONTEXT_FLAGS as ::core::ffi::c_uint),
            );
            free(ctx as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<rxkb_context>();
        }
        list_init(&raw mut (*ctx).models);
        list_init(&raw mut (*ctx).layouts);
        list_init(&raw mut (*ctx).option_groups);
        if flags as ::core::ffi::c_uint
            & RXKB_CONTEXT_NO_DEFAULT_INCLUDES as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
            && !rxkb_context_include_path_append_default(ctx)
        {
            rxkb_log(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                b"[XKB-%03d] Failed to add any default include path (default system path: %s)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as ::core::ffi::c_int,
                b"/usr/share/xkeyboard-config-2\0".as_ptr() as *const ::core::ffi::c_char,
            );
            rxkb_context_unref(ctx);
            return ::core::ptr::null_mut::<rxkb_context>();
        }
        return ctx;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_set_log_fn(
    mut ctx: *mut rxkb_context,
    mut log_fn: Option<
        unsafe extern "C" fn(
            *mut rxkb_context,
            rxkb_log_level,
            *const ::core::ffi::c_char,
            ::core::ffi::VaList,
        ) -> (),
    >,
) {
    unsafe {
        (*ctx).log_fn = (if log_fn.is_some() {
            log_fn
                as Option<
                    unsafe extern "C" fn(
                        *mut rxkb_context,
                        rxkb_log_level,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
                >
        } else {
            Some(
                default_log_fn
                    as unsafe extern "C" fn(
                        *mut rxkb_context,
                        rxkb_log_level,
                        *const ::core::ffi::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            )
        })
            as Option<
                unsafe extern "C" fn(
                    *mut rxkb_context,
                    rxkb_log_level,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
            >;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_include_path_append(
    mut ctx: *mut rxkb_context,
    mut path: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        let mut stat_buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut rules: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*ctx).context_state as ::core::ffi::c_uint
            != CONTEXT_NEW as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            rxkb_log(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                b"include paths can only be appended to a new context\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            stat_buf = stat {
                st_dev: 0,
                st_ino: 0,
                st_nlink: 0,
                st_mode: 0,
                st_uid: 0,
                st_gid: 0,
                __pad0: 0,
                st_rdev: 0,
                st_size: 0,
                st_blksize: 0,
                st_blocks: 0,
                st_atim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_mtim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                st_ctim: timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                __glibc_reserved: [0; 3],
            };
            err = stat(path, &raw mut stat_buf);
            if err != 0 as ::core::ffi::c_int {
                err = *__errno_location();
            } else if !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
                err = ENOTDIR;
            } else if !check_eaccess(path, R_OK | X_OK) {
                err = EACCES;
            } else {
                rules = [0; 4096];
                if !snprintf_safe(
                    &raw mut rules as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                    b"%s/rules/%s.xml\0".as_ptr() as *const ::core::ffi::c_char,
                    path,
                    DEFAULT_XKB_RULES.as_ptr(),
                ) {
                    rxkb_log(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        b"[XKB-%03d] Path is too long: expected max length of %zu, got: %s/rules/%s.xml\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        XKB_ERROR_INVALID_PATH as ::core::ffi::c_int,
                        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
                        path,
                        b"evdev\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else {
                    tmp = strdup(path) as *mut ::core::ffi::c_char;
                    if tmp.is_null() {
                        err = ENOMEM;
                    } else {
                        (*ctx).includes.size =
                            (*ctx).includes.size.wrapping_add(1 as darray_size_t);
                        let mut __need: darray_size_t = (*ctx).includes.size;
                        if __need > (*ctx).includes.alloc {
                            (*ctx).includes.alloc = darray_next_alloc(
                                (*ctx).includes.alloc,
                                __need,
                                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                            );
                            (*ctx).includes.item = realloc(
                                (*ctx).includes.item as *mut ::core::ffi::c_void,
                                ((*ctx).includes.alloc as usize).wrapping_mul(
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                ),
                            )
                                as *mut *mut ::core::ffi::c_char;
                        }
                        let ref mut c2rust_fresh0 = *(*ctx)
                            .includes
                            .item
                            .offset((*ctx).includes.size.wrapping_sub(1 as darray_size_t) as isize);
                        *c2rust_fresh0 = tmp;
                        rxkb_log(
                            ctx,
                            RXKB_LOG_LEVEL_INFO,
                            b"Include path added: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            tmp,
                        );
                        return true;
                    }
                }
            }
        }
        rxkb_log(
            ctx,
            RXKB_LOG_LEVEL_INFO,
            b"Include path failed: \"%s\" (%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(err),
        );
        return false;
    }
}
unsafe extern "C" fn compare_str(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    unsafe {
        return strcmp(
            *(a as *mut *mut ::core::ffi::c_char),
            *(b as *mut *mut ::core::ffi::c_char),
        );
    }
}
unsafe extern "C" fn add_direct_subdirectories(
    mut ctx: *mut rxkb_context,
    mut path: *const ::core::ffi::c_char,
    mut extensions: *mut darray_string,
    mut versioned_count: darray_size_t,
    mut versioned_path_length: usize,
) -> ::core::ffi::c_int {
    unsafe {
        let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
        let mut path_buf: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut c2rust_current_block: u64;
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut err: ::core::ffi::c_int = ENOMEM;
        let mut dir: *mut DIR = ::core::ptr::null_mut::<DIR>();
        let mut stat_buf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        err = stat(path, &raw mut stat_buf);
        if err != 0 as ::core::ffi::c_int {
            err = *__errno_location();
        } else if !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
            err = ENOTDIR;
        } else if !check_eaccess(path, R_OK | X_OK) {
            err = EACCES;
        } else {
            dir = opendir(path);
            if dir.is_null() {
                err = EACCES;
            } else {
                entry = ::core::ptr::null_mut::<dirent>();
                path_buf = ::core::mem::transmute::<
                    [u8; 4096],
                    [::core::ffi::c_char; 4096],
                >(
                    *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                );
                versioned_path_length = versioned_path_length.wrapping_add(1);
                's_62: loop {
                    entry = readdir(dir);
                    if entry.is_null() {
                        c2rust_current_block = 14434620278749266018;
                        break;
                    }
                    let mut name: *const ::core::ffi::c_char =
                        &raw mut (*entry).d_name as *mut ::core::ffi::c_char;
                    if strcmp(name, b".\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                        || strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                    {
                        continue;
                    }
                    if !snprintf_safe(
                        &raw mut path_buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                        b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
                        path,
                        name,
                    ) {
                        err = ENOMEM;
                        c2rust_current_block = 17009998909239196508;
                        break;
                    } else {
                        if stat(
                            &raw mut path_buf as *mut ::core::ffi::c_char,
                            &raw mut stat_buf,
                        ) != 0 as ::core::ffi::c_int
                            || !(stat_buf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
                        {
                            continue;
                        }
                        let mut i: darray_size_t = 0 as darray_size_t;
                        while i < versioned_count {
                            let prev_name: *const ::core::ffi::c_char =
                                (*(*extensions).item.offset(i as isize))
                                    .offset(versioned_path_length as isize);
                            if strcmp(name, prev_name) == 0 as ::core::ffi::c_int {
                                continue 's_62;
                            }
                            i = i.wrapping_add(1);
                        }
                        let mut ext_path: *mut ::core::ffi::c_char =
                            strdup_safe(&raw mut path_buf as *mut ::core::ffi::c_char);
                        if ext_path.is_null() {
                            err = ENOMEM;
                            c2rust_current_block = 17009998909239196508;
                            break;
                        } else {
                            (*extensions).size =
                                (*extensions).size.wrapping_add(1 as darray_size_t);
                            let mut __need: darray_size_t = (*extensions).size;
                            if __need > (*extensions).alloc {
                                (*extensions).alloc = darray_next_alloc(
                                    (*extensions).alloc,
                                    __need,
                                    ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                );
                                (*extensions).item = realloc(
                                    (*extensions).item as *mut ::core::ffi::c_void,
                                    ((*extensions).alloc as usize).wrapping_mul(
                                        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                    ),
                                )
                                    as *mut *mut ::core::ffi::c_char;
                            }
                            let ref mut c2rust_fresh1 =
                                *(*extensions)
                                    .item
                                    .offset((*extensions).size.wrapping_sub(1 as darray_size_t)
                                        as isize);
                            *c2rust_fresh1 = ext_path;
                        }
                    }
                }
                match c2rust_current_block {
                    17009998909239196508 => {}
                    _ => {
                        closedir(dir);
                        if (*extensions).size > versioned_count {
                            qsort(
                                (*extensions).item.offset(versioned_count as isize)
                                    as *mut ::core::ffi::c_void,
                                (*extensions).size.wrapping_sub(versioned_count) as usize,
                                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize,
                                Some(
                                    compare_str
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_void,
                                            *const ::core::ffi::c_void,
                                        )
                                            -> ::core::ffi::c_int,
                                ),
                            );
                            let mut ext_path_0: *mut *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                            if !(*extensions).item.is_null() {
                                ext_path_0 = (*extensions).item.offset(versioned_count as isize)
                                    as *mut *mut ::core::ffi::c_char;
                                while ext_path_0
                                    < (*extensions).item.offset((*extensions).size as isize)
                                        as *mut *mut ::core::ffi::c_char
                                {
                                    ret |= rxkb_context_include_path_append(ctx, *ext_path_0)
                                        as ::core::ffi::c_int;
                                    ext_path_0 = ext_path_0.offset(1);
                                }
                            }
                        }
                        return ret;
                    }
                }
            }
        }
        rxkb_log(
            ctx,
            RXKB_LOG_LEVEL_DEBUG,
            b"Include extensions path failed: %s (%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(err),
        );
        if !dir.is_null() {
            closedir(dir);
        }
        return ret;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_include_path_append_default(
    mut ctx: *mut rxkb_context,
) -> bool {
    unsafe {
        let mut user_path: [::core::ffi::c_char; 4096] = [0; 4096];
        let mut ret: ::core::ffi::c_int = if false { 1 } else { 0 };
        if (*ctx).context_state as ::core::ffi::c_uint
            != CONTEXT_NEW as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            rxkb_log(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                b"include paths can only be appended to a new context\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return false;
        }
        let home: *const ::core::ffi::c_char =
            rxkb_context_getenv(ctx, b"HOME\0".as_ptr() as *const ::core::ffi::c_char);
        let xdg: *const ::core::ffi::c_char = rxkb_context_getenv(
            ctx,
            b"XDG_CONFIG_HOME\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !xdg.is_null() {
            if snprintf_safe(
                &raw mut user_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                b"%s/xkb\0".as_ptr() as *const ::core::ffi::c_char,
                xdg,
            ) {
                ret = ret as ::core::ffi::c_int
                    | rxkb_context_include_path_append(
                        ctx,
                        &raw mut user_path as *mut ::core::ffi::c_char,
                    ) as ::core::ffi::c_int;
            }
        } else if !home.is_null() {
            if snprintf_safe(
                &raw mut user_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                b"%s/.config/xkb\0".as_ptr() as *const ::core::ffi::c_char,
                home,
            ) {
                ret = ret as ::core::ffi::c_int
                    | rxkb_context_include_path_append(
                        ctx,
                        &raw mut user_path as *mut ::core::ffi::c_char,
                    ) as ::core::ffi::c_int;
            }
        }
        if !home.is_null() {
            if snprintf_safe(
                &raw mut user_path as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                b"%s/.xkb\0".as_ptr() as *const ::core::ffi::c_char,
                home,
            ) {
                ret = ret as ::core::ffi::c_int
                    | rxkb_context_include_path_append(
                        ctx,
                        &raw mut user_path as *mut ::core::ffi::c_char,
                    ) as ::core::ffi::c_int;
            }
        }
        let extra: *const ::core::ffi::c_char = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_EXTRA_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        );
        ret = ret as ::core::ffi::c_int
            | rxkb_context_include_path_append(
                ctx,
                if !extra.is_null() {
                    extra
                } else {
                    DFLT_XKB_CONFIG_EXTRA_PATH.as_ptr()
                },
            ) as ::core::ffi::c_int;
        let mut extensions: darray_string = darray_string {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        };
        let mut extensions_path: *const ::core::ffi::c_char = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_VERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if extensions_path.is_null() {
            extensions_path = DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH.as_ptr();
        }
        let mut versioned_path_length: usize = 0 as usize;
        if !extensions_path.is_null() {
            ret = ret as ::core::ffi::c_int
                | add_direct_subdirectories(
                    ctx,
                    extensions_path,
                    &raw mut extensions,
                    0 as darray_size_t,
                    0 as usize,
                );
            versioned_path_length = strlen(extensions_path);
        }
        extensions_path = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if extensions_path.is_null() {
            extensions_path = DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH.as_ptr();
        }
        if !extensions_path.is_null() {
            ret = ret as ::core::ffi::c_int
                | add_direct_subdirectories(
                    ctx,
                    extensions_path,
                    &raw mut extensions,
                    extensions.size,
                    versioned_path_length,
                );
        }
        let mut ext_path: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        if !extensions.item.is_null() {
            ext_path = extensions.item.offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut ::core::ffi::c_char;
            while ext_path
                < extensions.item.offset(extensions.size as isize) as *mut *mut ::core::ffi::c_char
            {
                free(*ext_path as *mut ::core::ffi::c_void);
                ext_path = ext_path.offset(1);
            }
        }
        free(extensions.item as *mut ::core::ffi::c_void);
        extensions.item = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        extensions.size = 0 as darray_size_t;
        extensions.alloc = 0 as darray_size_t;
        let root: *const ::core::ffi::c_char = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_ROOT\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let has_root: bool = rxkb_context_include_path_append(
            ctx,
            if !root.is_null() {
                root
            } else {
                DFLT_XKB_CONFIG_ROOT.as_ptr()
            },
        ) as bool;
        ret = ret as ::core::ffi::c_int | has_root as ::core::ffi::c_int;
        if !has_root
            && (root.is_null()
                || *root.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != '\0' as i32)
        {
            rxkb_log(
                ctx,
                RXKB_LOG_LEVEL_WARNING,
                b"Root include path failed; fallback to \"%s\". The setup is probably misconfigured. Please ensure that \"%s\" is available in the environment.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"/usr/share/X11/xkb\0".as_ptr() as *const ::core::ffi::c_char,
                if root.is_null() {
                    b"/usr/share/xkeyboard-config-2\0".as_ptr()
                        as *const ::core::ffi::c_char
                } else {
                    root
                },
            );
            ret = ret as ::core::ffi::c_int
                | rxkb_context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT.as_ptr())
                    as ::core::ffi::c_int;
        }
        return ret != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_parse_default_ruleset(mut ctx: *mut rxkb_context) -> bool {
    unsafe {
        return rxkb_context_parse(ctx, DEFAULT_XKB_RULES.as_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_parse(
    mut ctx: *mut rxkb_context,
    mut ruleset: *const ::core::ffi::c_char,
) -> bool {
    unsafe {
        let mut path: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut success: bool = false;
        if (*ctx).context_state as ::core::ffi::c_uint
            != CONTEXT_NEW as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            rxkb_log(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                b"parse must only be called on a new context\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return false;
        }
        if !(*ctx).includes.item.is_null() && (*ctx).includes.size != 0 {
            path = (*ctx)
                .includes
                .item
                .offset((*ctx).includes.size.wrapping_sub(1 as darray_size_t) as isize)
                as *mut *mut ::core::ffi::c_char;
            while (*ctx).includes.size > 0 as darray_size_t
                && path
                    >= (*ctx)
                        .includes
                        .item
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut *mut ::core::ffi::c_char
            {
                let mut rules: [::core::ffi::c_char; 4096] = [0; 4096];
                if snprintf_safe(
                    &raw mut rules as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                    b"%s/rules/%s.xml\0".as_ptr() as *const ::core::ffi::c_char,
                    *path,
                    ruleset,
                ) {
                    rxkb_log(
                        ctx,
                        RXKB_LOG_LEVEL_DEBUG,
                        b"Parsing %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut rules as *mut ::core::ffi::c_char,
                    );
                    if parse(
                        ctx,
                        &raw mut rules as *mut ::core::ffi::c_char,
                        RXKB_POPULARITY_STANDARD,
                    ) {
                        success = true;
                    }
                }
                if (*ctx).load_extra_rules_files as ::core::ffi::c_int != 0
                    && snprintf_safe(
                        &raw mut rules as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
                        b"%s/rules/%s.extras.xml\0".as_ptr() as *const ::core::ffi::c_char,
                        *path,
                        ruleset,
                    ) as ::core::ffi::c_int
                        != 0
                {
                    rxkb_log(
                        ctx,
                        RXKB_LOG_LEVEL_DEBUG,
                        b"Parsing %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut rules as *mut ::core::ffi::c_char,
                    );
                    if parse(
                        ctx,
                        &raw mut rules as *mut ::core::ffi::c_char,
                        RXKB_POPULARITY_EXOTIC,
                    ) {
                        success = true;
                    }
                }
                path = path.offset(-1);
            }
        }
        (*ctx).context_state = (if success as ::core::ffi::c_int != 0 {
            CONTEXT_PARSED as ::core::ffi::c_int
        } else {
            CONTEXT_FAILED as ::core::ffi::c_int
        }) as context_state;
        return success;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_set_user_data(
    mut ctx: *mut rxkb_context,
    mut userdata: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*ctx).userdata = userdata;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rxkb_context_get_user_data(
    mut ctx: *mut rxkb_context,
) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*ctx).userdata;
    }
}
#[inline]
unsafe extern "C" fn is_node(mut node: *mut xmlNode, mut name: *const ::core::ffi::c_char) -> bool {
    unsafe {
        return (*node).type_0 as ::core::ffi::c_uint
            == XML_ELEMENT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            && xmlStrEqual((*node).name, name as *const xmlChar) != 0;
    }
}
unsafe extern "C" fn extract_text(mut node: *mut xmlNode) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut n: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        n = (*node).children as *mut xmlNode;
        while !n.is_null() {
            if (*n).type_0 as ::core::ffi::c_uint
                == XML_TEXT_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return xmlStrdup((*n).content) as *mut ::core::ffi::c_char;
            }
            n = (*n).next as *mut xmlNode;
        }
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn config_item_free(mut config: *mut config_item) {
    unsafe {
        free((*config).name as *mut ::core::ffi::c_void);
        free((*config).description as *mut ::core::ffi::c_void);
        free((*config).brief as *mut ::core::ffi::c_void);
        free((*config).vendor as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn parse_config_item(
    mut ctx: *mut rxkb_context,
    mut parent: *mut xmlNode,
    mut config: *mut config_item,
) -> bool {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut ci: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        ci = (*parent).children as *mut xmlNode;
        while !ci.is_null() {
            if is_node(ci, b"configItem\0".as_ptr() as *const ::core::ffi::c_char) {
                let mut raw_popularity: *mut xmlChar = xmlGetProp(
                    ci,
                    b"popularity\0".as_ptr() as *const ::core::ffi::c_char as *const xmlChar,
                );
                if !raw_popularity.is_null() {
                    if xmlStrEqual(
                        raw_popularity,
                        b"standard\0".as_ptr() as *const ::core::ffi::c_char as *const xmlChar,
                    ) != 0
                    {
                        (*config).popularity = RXKB_POPULARITY_STANDARD;
                    } else if xmlStrEqual(
                        raw_popularity,
                        b"exotic\0".as_ptr() as *const ::core::ffi::c_char as *const xmlChar,
                    ) != 0
                    {
                        (*config).popularity = RXKB_POPULARITY_EXOTIC;
                    } else {
                        rxkb_log(
                            ctx,
                            RXKB_LOG_LEVEL_ERROR,
                            b"xml:%u: invalid popularity attribute: expected 'standard' or 'exotic', got: '%s'\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            (*ci).line as ::core::ffi::c_int,
                            raw_popularity,
                        );
                    }
                }
                xmlFree.expect("non-null function pointer")(
                    raw_popularity as *mut ::core::ffi::c_void,
                );
                let mut raw_layout_specific: *mut xmlChar = xmlGetProp(
                    ci,
                    b"layout-specific\0".as_ptr() as *const ::core::ffi::c_char as *const xmlChar,
                );
                if !raw_layout_specific.is_null()
                    && xmlStrEqual(
                        raw_layout_specific,
                        b"true\0".as_ptr() as *const ::core::ffi::c_char as *const xmlChar,
                    ) != 0
                {
                    (*config).layout_specific = true;
                }
                xmlFree.expect("non-null function pointer")(
                    raw_layout_specific as *mut ::core::ffi::c_void,
                );
                node = (*ci).children as *mut xmlNode;
                while !node.is_null() {
                    if is_node(node, b"name\0".as_ptr() as *const ::core::ffi::c_char) {
                        (*config).name = extract_text(node);
                    } else if is_node(
                        node,
                        b"description\0".as_ptr() as *const ::core::ffi::c_char,
                    ) {
                        (*config).description = extract_text(node);
                    } else if is_node(
                        node,
                        b"shortDescription\0".as_ptr() as *const ::core::ffi::c_char,
                    ) {
                        (*config).brief = extract_text(node);
                    } else if is_node(node, b"vendor\0".as_ptr() as *const ::core::ffi::c_char) {
                        (*config).vendor = extract_text(node);
                    }
                    node = (*node).next as *mut xmlNode;
                }
                if (*config).name.is_null() || strlen((*config).name) == 0 {
                    rxkb_log(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        b"xml:%u: missing required element 'name'\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*ci).line as ::core::ffi::c_int,
                    );
                    config_item_free(config);
                    return false;
                }
                return true;
            }
            ci = (*ci).next as *mut xmlNode;
        }
        return false;
    }
}
unsafe extern "C" fn parse_model(
    mut ctx: *mut rxkb_context,
    mut model: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            description: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            brief: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            vendor: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, model, &raw mut config) {
            let mut m: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
            m = ::core::ptr::null_mut::<rxkb_model>();
            m = ((*ctx).models.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_model
                as *mut rxkb_model;
            while &raw mut (*m).base.link != &raw mut (*ctx).models {
                if streq((*m).name, config.name) {
                    config_item_free(&raw mut config);
                    return;
                }
                m = ((*m).base.link.next as *mut ::core::ffi::c_char)
                    .offset(-(16 as ::core::ffi::c_ulong as isize))
                    as *mut rxkb_model as *mut rxkb_model;
            }
            m = rxkb_model_create(&raw mut (*ctx).base);
            (*m).name = _steal(&raw mut config.name as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*m).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            (*m).vendor = _steal(&raw mut config.vendor as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*m).popularity = config.popularity;
            list_append(&raw mut (*ctx).models, &raw mut (*m).base.link);
        }
    }
}
unsafe extern "C" fn parse_model_list(
    mut ctx: *mut rxkb_context,
    mut model_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*model_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"model\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_model(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_language_list(
    mut language_list: *mut xmlNode,
    mut layout: *mut rxkb_layout,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut code: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        node = (*language_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"iso639Id\0".as_ptr() as *const ::core::ffi::c_char) {
                let mut str: *mut ::core::ffi::c_char = extract_text(node);
                let mut parent: *mut rxkb_object = ::core::ptr::null_mut::<rxkb_object>();
                if str.is_null() || strlen(str) != 3 as usize {
                    free(str as *mut ::core::ffi::c_void);
                } else {
                    parent = &raw mut (*layout).base;
                    code = rxkb_iso639_code_create(parent);
                    (*code).code = str;
                    list_append(&raw mut (*layout).iso639s, &raw mut (*code).base.link);
                }
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_country_list(
    mut country_list: *mut xmlNode,
    mut layout: *mut rxkb_layout,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut code: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        node = (*country_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"iso3166Id\0".as_ptr() as *const ::core::ffi::c_char) {
                let mut str: *mut ::core::ffi::c_char = extract_text(node);
                let mut parent: *mut rxkb_object = ::core::ptr::null_mut::<rxkb_object>();
                if str.is_null() || strlen(str) != 2 as usize {
                    free(str as *mut ::core::ffi::c_void);
                } else {
                    parent = &raw mut (*layout).base;
                    code = rxkb_iso3166_code_create(parent);
                    (*code).code = str;
                    list_append(&raw mut (*layout).iso3166s, &raw mut (*code).base.link);
                }
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_variant(
    mut ctx: *mut rxkb_context,
    mut l: *mut rxkb_layout,
    mut variant: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut ci: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            description: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            brief: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            vendor: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, variant, &raw mut config) {
            let mut v: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
            let mut exists: bool = false;
            v = ::core::ptr::null_mut::<rxkb_layout>();
            v = ((*ctx).layouts.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
                as *mut rxkb_layout;
            while &raw mut (*v).base.link != &raw mut (*ctx).layouts {
                if streq_null((*v).variant, config.name) as ::core::ffi::c_int != 0
                    && streq((*v).name, (*l).name) as ::core::ffi::c_int != 0
                {
                    exists = true;
                    break;
                } else {
                    v = ((*v).base.link.next as *mut ::core::ffi::c_char)
                        .offset(-(16 as ::core::ffi::c_ulong as isize))
                        as *mut rxkb_layout as *mut rxkb_layout;
                }
            }
            if !exists {
                v = rxkb_layout_create(&raw mut (*ctx).base);
                list_init(&raw mut (*v).iso639s);
                list_init(&raw mut (*v).iso3166s);
                (*v).name = strdup((*l).name);
                (*v).variant = _steal(&raw mut config.name as *mut ::core::ffi::c_void)
                    as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                (*v).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                    as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_char;
                (*v).brief = if config.brief.is_null() {
                    strdup_safe((*l).brief)
                } else {
                    _steal(&raw mut config.brief as *mut ::core::ffi::c_void)
                        as *mut ::core::ffi::c_char
                };
                (*v).popularity = config.popularity;
                list_append(&raw mut (*ctx).layouts, &raw mut (*v).base.link);
                ci = (*variant).children as *mut xmlNode;
                while !ci.is_null() {
                    let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
                    if is_node(ci, b"configItem\0".as_ptr() as *const ::core::ffi::c_char) {
                        let mut found_language_list: bool = false;
                        let mut found_country_list: bool = false;
                        node = (*ci).children as *mut xmlNode;
                        while !node.is_null() {
                            if is_node(
                                node,
                                b"languageList\0".as_ptr() as *const ::core::ffi::c_char,
                            ) {
                                parse_language_list(node, v);
                                found_language_list = true;
                            }
                            if is_node(
                                node,
                                b"countryList\0".as_ptr() as *const ::core::ffi::c_char,
                            ) {
                                parse_country_list(node, v);
                                found_country_list = true;
                            }
                            node = (*node).next as *mut xmlNode;
                        }
                        if !found_language_list {
                            let mut x: *mut rxkb_iso639_code =
                                ::core::ptr::null_mut::<rxkb_iso639_code>();
                            x = ::core::ptr::null_mut::<rxkb_iso639_code>();
                            x = ((*l).iso639s.next as *mut ::core::ffi::c_char)
                                .offset(-(16 as ::core::ffi::c_ulong as isize))
                                as *mut rxkb_iso639_code
                                as *mut rxkb_iso639_code;
                            while &raw mut (*x).base.link != &raw mut (*l).iso639s {
                                let mut code: *mut rxkb_iso639_code =
                                    rxkb_iso639_code_create(&raw mut (*v).base);
                                (*code).code = strdup((*x).code);
                                list_append(&raw mut (*v).iso639s, &raw mut (*code).base.link);
                                x = ((*x).base.link.next as *mut ::core::ffi::c_char)
                                    .offset(-(16 as ::core::ffi::c_ulong as isize))
                                    as *mut rxkb_iso639_code
                                    as *mut rxkb_iso639_code;
                            }
                        }
                        if !found_country_list {
                            let mut x_0: *mut rxkb_iso3166_code =
                                ::core::ptr::null_mut::<rxkb_iso3166_code>();
                            x_0 = ::core::ptr::null_mut::<rxkb_iso3166_code>();
                            x_0 = ((*l).iso3166s.next as *mut ::core::ffi::c_char)
                                .offset(-(16 as ::core::ffi::c_ulong as isize))
                                as *mut rxkb_iso3166_code
                                as *mut rxkb_iso3166_code;
                            while &raw mut (*x_0).base.link != &raw mut (*l).iso3166s {
                                let mut code_0: *mut rxkb_iso3166_code =
                                    rxkb_iso3166_code_create(&raw mut (*v).base);
                                (*code_0).code = strdup((*x_0).code);
                                list_append(&raw mut (*v).iso3166s, &raw mut (*code_0).base.link);
                                x_0 = ((*x_0).base.link.next as *mut ::core::ffi::c_char)
                                    .offset(-(16 as ::core::ffi::c_ulong as isize))
                                    as *mut rxkb_iso3166_code
                                    as *mut rxkb_iso3166_code;
                            }
                        }
                    }
                    ci = (*ci).next as *mut xmlNode;
                }
            } else {
                config_item_free(&raw mut config);
            }
        }
    }
}
unsafe extern "C" fn parse_variant_list(
    mut ctx: *mut rxkb_context,
    mut l: *mut rxkb_layout,
    mut variant_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*variant_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"variant\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_variant(ctx, l, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_layout(
    mut ctx: *mut rxkb_context,
    mut layout: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            description: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            brief: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            vendor: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            popularity: popularity,
            layout_specific: false,
        };
        let mut l: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut exists: bool = false;
        if !parse_config_item(ctx, layout, &raw mut config) {
            return;
        }
        l = ::core::ptr::null_mut::<rxkb_layout>();
        l = ((*ctx).layouts.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        while &raw mut (*l).base.link != &raw mut (*ctx).layouts {
            if streq((*l).name, config.name) as ::core::ffi::c_int != 0 && (*l).variant.is_null() {
                exists = true;
                break;
            } else {
                l = ((*l).base.link.next as *mut ::core::ffi::c_char)
                    .offset(-(16 as ::core::ffi::c_ulong as isize))
                    as *mut rxkb_layout as *mut rxkb_layout;
            }
        }
        if !exists {
            l = rxkb_layout_create(&raw mut (*ctx).base);
            list_init(&raw mut (*l).iso639s);
            list_init(&raw mut (*l).iso3166s);
            (*l).name = _steal(&raw mut config.name as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*l).variant = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*l).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            (*l).brief = _steal(&raw mut config.brief as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*l).popularity = config.popularity;
            list_append(&raw mut (*ctx).layouts, &raw mut (*l).base.link);
        } else {
            config_item_free(&raw mut config);
        }
        node = (*layout).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(
                node,
                b"variantList\0".as_ptr() as *const ::core::ffi::c_char,
            ) {
                parse_variant_list(ctx, l, node, popularity);
            }
            if !exists
                && is_node(node, b"configItem\0".as_ptr() as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int
                    != 0
            {
                let mut ll: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
                ll = (*node).children as *mut xmlNode;
                while !ll.is_null() {
                    if is_node(ll, b"languageList\0".as_ptr() as *const ::core::ffi::c_char) {
                        parse_language_list(ll, l);
                    }
                    if is_node(ll, b"countryList\0".as_ptr() as *const ::core::ffi::c_char) {
                        parse_country_list(ll, l);
                    }
                    ll = (*ll).next as *mut xmlNode;
                }
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_layout_list(
    mut ctx: *mut rxkb_context,
    mut layout_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*layout_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"layout\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_layout(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_option(
    mut ctx: *mut rxkb_context,
    mut group: *mut rxkb_option_group,
    mut option: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            description: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            brief: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            vendor: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, option, &raw mut config) {
            let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
            o = ::core::ptr::null_mut::<rxkb_option>();
            o = ((*group).options.next as *mut ::core::ffi::c_char)
                .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option
                as *mut rxkb_option;
            while &raw mut (*o).base.link != &raw mut (*group).options {
                if streq((*o).name, config.name) {
                    config_item_free(&raw mut config);
                    return;
                }
                o = ((*o).base.link.next as *mut ::core::ffi::c_char)
                    .offset(-(16 as ::core::ffi::c_ulong as isize))
                    as *mut rxkb_option as *mut rxkb_option;
            }
            o = rxkb_option_create(&raw mut (*group).base);
            (*o).name = _steal(&raw mut config.name as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*o).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            (*o).popularity = config.popularity;
            (*o).layout_specific = config.layout_specific;
            list_append(&raw mut (*group).options, &raw mut (*o).base.link);
        }
    }
}
unsafe extern "C" fn parse_group(
    mut ctx: *mut rxkb_context,
    mut group: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            description: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            brief: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            vendor: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            popularity: popularity,
            layout_specific: false,
        };
        let mut g: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut multiple: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
        let mut exists: bool = false;
        if !parse_config_item(ctx, group, &raw mut config) {
            return;
        }
        g = ::core::ptr::null_mut::<rxkb_option_group>();
        g = ((*ctx).option_groups.next as *mut ::core::ffi::c_char)
            .offset(-(16 as ::core::ffi::c_ulong as isize)) as *mut rxkb_option_group
            as *mut rxkb_option_group;
        while &raw mut (*g).base.link != &raw mut (*ctx).option_groups {
            if streq((*g).name, config.name) {
                exists = true;
                break;
            } else {
                g = ((*g).base.link.next as *mut ::core::ffi::c_char)
                    .offset(-(16 as ::core::ffi::c_ulong as isize))
                    as *mut rxkb_option_group as *mut rxkb_option_group;
            }
        }
        if !exists {
            g = rxkb_option_group_create(&raw mut (*ctx).base);
            (*g).name = _steal(&raw mut config.name as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
            (*g).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            (*g).popularity = config.popularity;
            multiple = xmlGetProp(
                group,
                b"allowMultipleSelection\0".as_ptr() as *const ::core::ffi::c_char
                    as *const xmlChar,
            );
            if !multiple.is_null()
                && xmlStrEqual(
                    multiple,
                    b"true\0".as_ptr() as *const ::core::ffi::c_char as *const xmlChar,
                ) != 0
            {
                (*g).allow_multiple = true;
            }
            xmlFree.expect("non-null function pointer")(multiple as *mut ::core::ffi::c_void);
            list_init(&raw mut (*g).options);
            list_append(&raw mut (*ctx).option_groups, &raw mut (*g).base.link);
        } else {
            config_item_free(&raw mut config);
        }
        node = (*group).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"option\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_option(ctx, g, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_option_list(
    mut ctx: *mut rxkb_context,
    mut option_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*option_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"group\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_group(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn parse_rules_xml(
    mut ctx: *mut rxkb_context,
    mut root: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*root).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"modelList\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_model_list(ctx, node, popularity);
            } else if is_node(node, b"layoutList\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_layout_list(ctx, node, popularity);
            } else if is_node(node, b"optionList\0".as_ptr() as *const ::core::ffi::c_char) {
                parse_option_list(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn xml_error_func(
    mut ctx: *mut ::core::ffi::c_void,
    mut msg: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    unsafe {
        static mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
        static mut slen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut args: ::core::ffi::VaList;
        let mut rc: ::core::ffi::c_int = 0;
        args = c2rust_args.clone();
        rc = vsnprintf(
            (&raw mut buf as *mut ::core::ffi::c_char).offset(slen as isize)
                as *mut ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize)
                .wrapping_sub(slen as usize),
            msg,
            args,
        );
        if rc < 0 as ::core::ffi::c_int {
            rxkb_log(
                ctx as *mut rxkb_context,
                RXKB_LOG_LEVEL_ERROR,
                b"[XKB-%03d] +++ out of cheese error. redo from start +++\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as ::core::ffi::c_int,
            );
            slen = 0 as ::core::ffi::c_int;
            memset(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            );
            return;
        }
        slen += rc;
        if slen >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int {
            buf[(::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize)
                .wrapping_sub(1 as usize) as usize] = '\n' as i32 as ::core::ffi::c_char;
            slen = ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as ::core::ffi::c_int;
        }
        if buf[(slen - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int == '\n' as i32 {
            rxkb_log(
                ctx as *mut rxkb_context,
                RXKB_LOG_LEVEL_ERROR,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            memset(
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as usize,
            );
            slen = 0 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn validate(mut ctx: *mut rxkb_context, mut doc: *mut xmlDoc) -> bool {
    unsafe {
        let mut dtd: *mut xmlDtd = ::core::ptr::null_mut::<xmlDtd>();
        let mut dtdvalid: *mut xmlValidCtxt = ::core::ptr::null_mut::<xmlValidCtxt>();
        let mut success: bool = false;
        let dtdstr: [::core::ffi::c_char; 1061] = ::core::mem::transmute::<
            [u8; 1061],
            [::core::ffi::c_char; 1061],
        >(
            *b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!ELEMENT xkbConfigRegistry (modelList?, layoutList?, optionList?)>\n<!ATTLIST xkbConfigRegistry version CDATA \"1.1\">\n<!ELEMENT modelList (model*)>\n<!ELEMENT model (configItem)>\n<!ELEMENT layoutList (layout*)>\n<!ELEMENT layout (configItem,  variantList?)>\n<!ELEMENT optionList (group*)>\n<!ELEMENT variantList (variant*)>\n<!ELEMENT variant (configItem)>\n<!ELEMENT group (configItem, option*)>\n<!ATTLIST group allowMultipleSelection (true|false) \"false\">\n<!ELEMENT option (configItem)>\n<!ELEMENT configItem (name, shortDescription?, description?, vendor?, countryList?, languageList?, hwList?)>\n<!ATTLIST configItem layout-specific (true|false) \"false\">\n<!ATTLIST configItem popularity (standard|exotic) #IMPLIED>\n<!ELEMENT name (#PCDATA)>\n<!ELEMENT shortDescription (#PCDATA)>\n<!ELEMENT description (#PCDATA)>\n<!ELEMENT vendor (#PCDATA)>\n<!ELEMENT countryList (iso3166Id+)>\n<!ELEMENT iso3166Id (#PCDATA)>\n<!ELEMENT languageList (iso639Id+)>\n<!ELEMENT iso639Id (#PCDATA)>\n<!ELEMENT hwList (hwId+)>\n<!ELEMENT hwId (#PCDATA)>\n\0",
        );
        let mut buf: xmlParserInputBufferPtr = xmlParserInputBufferCreateMem(
            &raw const dtdstr as *const ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 1061]>() as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                .wrapping_sub(1 as usize) as ::core::ffi::c_int,
            XML_CHAR_ENCODING_NONE,
        );
        if !buf.is_null() {
            dtd = xmlIOParseDTD(
                ::core::ptr::null_mut::<xmlSAXHandler>(),
                buf,
                XML_CHAR_ENCODING_UTF8,
            ) as *mut xmlDtd;
            if dtd.is_null() {
                rxkb_log(
                    ctx,
                    RXKB_LOG_LEVEL_ERROR,
                    b"Failed to load DTD\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else {
                dtdvalid = xmlNewValidCtxt() as *mut xmlValidCtxt;
                success = xmlValidateDtd(
                    dtdvalid as xmlValidCtxtPtr,
                    doc as xmlDocPtr,
                    dtd as xmlDtdPtr,
                ) != 0;
                if !dtdvalid.is_null() {
                    xmlFreeValidCtxt(dtdvalid as xmlValidCtxtPtr);
                }
                xmlFreeDtd(dtd as xmlDtdPtr);
            }
        }
        return success;
    }
}
unsafe extern "C" fn parse(
    mut ctx: *mut rxkb_context,
    mut path: *const ::core::ffi::c_char,
    mut popularity: rxkb_popularity,
) -> bool {
    unsafe {
        let mut success: bool = false;
        let mut doc: *mut xmlDoc = ::core::ptr::null_mut::<xmlDoc>();
        let mut root: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        if !check_eaccess(path, R_OK) {
            return false;
        }
        xmlCheckVersion(21210 as ::core::ffi::c_int);
        let mut xmlCtxt: xmlParserCtxtPtr = xmlNewParserCtxt();
        if xmlCtxt.is_null() {
            return false;
        }
        xmlCtxtUseOptions(xmlCtxt, XML_PARSE_NONET as ::core::ffi::c_int);
        xmlSetGenericErrorFunc(
            ctx as *mut ::core::ffi::c_void,
            Some(
                xml_error_func
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            ),
        );
        doc = xmlCtxtReadFile(
            xmlCtxt,
            path,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        ) as *mut xmlDoc;
        if !doc.is_null() {
            if !validate(ctx, doc) {
                rxkb_log(
                    ctx,
                    RXKB_LOG_LEVEL_ERROR,
                    b"XML error: failed to validate document at %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    path,
                );
            } else {
                root = xmlDocGetRootElement(doc) as *mut xmlNode;
                parse_rules_xml(ctx, root, popularity);
                success = true;
            }
            xmlFreeDoc(doc as xmlDocPtr);
        }
        xmlSetGenericErrorFunc(NULL, None);
        xmlFreeParserCtxt(xmlCtxt);
        return success;
    }
}
