pub type xmlChar = ::core::ffi::c_uchar;
extern "C" {
    pub fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    pub fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> i32;
}
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
extern "C" {
    pub static mut xmlFree: xmlFreeFunc;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut ::core::ffi::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: i32,
    pub error: i32,
    pub rawconsumed: u64,
}
pub type xmlInputCloseCallback = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> i32>;
pub type xmlInputReadCallback = Option<unsafe fn(*mut ::core::ffi::c_void, *mut i8, i32) -> i32>;
extern "C" {
    pub fn xmlParserInputBufferCreateMem(
        mem: *const i8,
        size: i32,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
}
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
    pub name: *mut i8,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: i32,
    pub standalone: i32,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut ::core::ffi::c_void,
    pub refs: *mut ::core::ffi::c_void,
    pub URL: *const xmlChar,
    pub charset: i32,
    pub dict: *mut _xmlDict,
    pub psvi: *mut ::core::ffi::c_void,
    pub parseFlags: i32,
    pub properties: i32,
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
pub type xmlNsType = u32;
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
    pub line: u16,
    pub extra: u16,
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
pub type xmlAttributeType = u32;
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
pub type xmlElementContentOccur = u32;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = u32;
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
extern "C" {
    pub type _xmlBuf;
    pub type _xmlDict;
    pub fn xmlFreeDtd(cur: xmlDtdPtr);
    pub fn xmlFreeDoc(cur: xmlDocPtr);
    pub fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    pub fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
}
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut i8,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type xmlCharEncodingOutputFunc = Option<
    unsafe fn(*mut ::core::ffi::c_uchar, *mut i32, *const ::core::ffi::c_uchar, *mut i32) -> i32,
>;
pub type xmlCharEncodingInputFunc = Option<
    unsafe fn(*mut ::core::ffi::c_uchar, *mut i32, *const ::core::ffi::c_uchar, *mut i32) -> i32,
>;
pub type xmlCharEncoding = i32;
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
pub type iconv_t = *mut ::core::ffi::c_void;
pub mod parser_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserInput {
        pub buf: xmlParserInputBufferPtr,
        pub filename: *const i8,
        pub directory: *const i8,
        pub base: *const xmlChar,
        pub cur: *const xmlChar,
        pub end: *const xmlChar,
        pub length: i32,
        pub line: i32,
        pub col: i32,
        pub consumed: u64,
        pub free: xmlParserInputDeallocate,
        pub encoding: *const xmlChar,
        pub version: *const xmlChar,
        pub flags: i32,
        pub id: i32,
        pub parentConsumed: u64,
        pub entity: xmlEntityPtr,
    }
    pub type xmlParserInputDeallocate = Option<unsafe extern "C" fn(*mut xmlChar) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserCtxt {
        pub sax: *mut _xmlSAXHandler,
        pub userData: *mut ::core::ffi::c_void,
        pub myDoc: xmlDocPtr,
        pub wellFormed: i32,
        pub replaceEntities: i32,
        pub version: *const xmlChar,
        pub encoding: *const xmlChar,
        pub standalone: i32,
        pub html: i32,
        pub input: xmlParserInputPtr,
        pub inputNr: i32,
        pub inputMax: i32,
        pub inputTab: *mut xmlParserInputPtr,
        pub node: xmlNodePtr,
        pub nodeNr: i32,
        pub nodeMax: i32,
        pub nodeTab: *mut xmlNodePtr,
        pub record_info: i32,
        pub node_seq: xmlParserNodeInfoSeq,
        pub errNo: i32,
        pub hasExternalSubset: i32,
        pub hasPErefs: i32,
        pub external: i32,
        pub valid: i32,
        pub validate: i32,
        pub vctxt: xmlValidCtxt,
        pub instate: xmlParserInputState,
        pub token: i32,
        pub directory: *mut i8,
        pub name: *const xmlChar,
        pub nameNr: i32,
        pub nameMax: i32,
        pub nameTab: *mut *const xmlChar,
        pub nbChars: i64,
        pub checkIndex: i64,
        pub keepBlanks: i32,
        pub disableSAX: i32,
        pub inSubset: i32,
        pub intSubName: *const xmlChar,
        pub extSubURI: *mut xmlChar,
        pub extSubSystem: *mut xmlChar,
        pub space: *mut i32,
        pub spaceNr: i32,
        pub spaceMax: i32,
        pub spaceTab: *mut i32,
        pub depth: i32,
        pub entity: xmlParserInputPtr,
        pub charset: i32,
        pub nodelen: i32,
        pub nodemem: i32,
        pub pedantic: i32,
        pub _private: *mut ::core::ffi::c_void,
        pub loadsubset: i32,
        pub linenumbers: i32,
        pub catalogs: *mut ::core::ffi::c_void,
        pub recovery: i32,
        pub progressive: i32,
        pub dict: xmlDictPtr,
        pub atts: *mut *const xmlChar,
        pub maxatts: i32,
        pub docdict: i32,
        pub str_xml: *const xmlChar,
        pub str_xmlns: *const xmlChar,
        pub str_xml_ns: *const xmlChar,
        pub sax2: i32,
        pub nsNr: i32,
        pub nsMax: i32,
        pub nsTab: *mut *const xmlChar,
        pub attallocs: *mut u32,
        pub pushTab: *mut xmlStartTag,
        pub attsDefault: xmlHashTablePtr,
        pub attsSpecial: xmlHashTablePtr,
        pub nsWellFormed: i32,
        pub options: i32,
        pub dictNames: i32,
        pub freeElemsNr: i32,
        pub freeElems: xmlNodePtr,
        pub freeAttrsNr: i32,
        pub freeAttrs: xmlAttrPtr,
        pub lastError: xmlError,
        pub parseMode: xmlParserMode,
        pub nbentities: u64,
        pub sizeentities: u64,
        pub nodeInfo: *mut xmlParserNodeInfo,
        pub nodeInfoNr: i32,
        pub nodeInfoMax: i32,
        pub nodeInfoTab: *mut xmlParserNodeInfo,
        pub input_id: i32,
        pub sizeentcopy: u64,
        pub endCheckState: i32,
        pub nbErrors: u16,
        pub nbWarnings: u16,
        pub maxAmpl: u32,
        pub nsdb: *mut xmlParserNsData,
        pub attrHashMax: u32,
        pub attrHash: *mut xmlAttrHashBucket,
    }
    pub type xmlAttrHashBucket = _xmlAttrHashBucket;
    pub type xmlParserNsData = _xmlParserNsData;
    pub type xmlParserNodeInfo = _xmlParserNodeInfo;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlParserNodeInfo {
        pub node: *const _xmlNode,
        pub begin_pos: u64,
        pub begin_line: u64,
        pub end_pos: u64,
        pub end_line: u64,
    }
    pub type xmlParserMode = u32;
    pub const XML_PARSE_READER: xmlParserMode = 5;
    pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
    pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
    pub const XML_PARSE_SAX: xmlParserMode = 2;
    pub const XML_PARSE_DOM: xmlParserMode = 1;
    pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
    pub type xmlStartTag = _xmlStartTag;
    pub type xmlParserInputState = i32;
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
        pub maximum: u64,
        pub length: u64,
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
        pub initialized: u32,
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
            i32,
            *mut *const xmlChar,
            i32,
            i32,
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
    pub type cdataBlockSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar, i32) -> ()>;
    pub type getParameterEntitySAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
    pub type fatalErrorSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const i8, ...) -> ()>;
    pub type errorSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const i8, ...) -> ()>;
    pub type warningSAXFunc =
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const i8, ...) -> ()>;
    pub type commentSAXFunc = Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
    pub type processingInstructionSAXFunc =
        Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar) -> ()>;
    pub type ignorableWhitespaceSAXFunc =
        Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, i32) -> ()>;
    pub type charactersSAXFunc =
        Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, i32) -> ()>;
    pub type referenceSAXFunc = Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
    pub type endElementSAXFunc = Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
    pub type startElementSAXFunc =
        Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, *mut *const xmlChar) -> ()>;
    pub type endDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    pub type startDocumentSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    pub type setDocumentLocatorSAXFunc =
        Option<unsafe fn(*mut ::core::ffi::c_void, xmlSAXLocatorPtr) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct _xmlSAXLocator {
        pub getPublicId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
        pub getSystemId: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const xmlChar>,
        pub getLineNumber: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> i32>,
        pub getColumnNumber: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> i32>,
    }
    pub type unparsedEntityDeclSAXFunc = Option<
        unsafe fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
            *const xmlChar,
        ) -> (),
    >;
    pub type elementDeclSAXFunc = Option<
        unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, i32, xmlElementContentPtr) -> (),
    >;
    pub type attributeDeclSAXFunc = Option<
        unsafe fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            *const xmlChar,
            i32,
            i32,
            *const xmlChar,
            xmlEnumerationPtr,
        ) -> (),
    >;
    pub type notationDeclSAXFunc = Option<
        unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar, *const xmlChar) -> (),
    >;
    pub type entityDeclSAXFunc = Option<
        unsafe fn(
            *mut ::core::ffi::c_void,
            *const xmlChar,
            i32,
            *const xmlChar,
            *const xmlChar,
            *mut xmlChar,
        ) -> (),
    >;
    pub type getEntitySAXFunc =
        Option<unsafe fn(*mut ::core::ffi::c_void, *const xmlChar) -> xmlEntityPtr>;
    pub type resolveEntitySAXFunc = Option<
        unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar) -> xmlParserInputPtr,
    >;
    pub type hasExternalSubsetSAXFunc = Option<unsafe fn(*mut ::core::ffi::c_void) -> i32>;
    pub type hasInternalSubsetSAXFunc = Option<unsafe fn(*mut ::core::ffi::c_void) -> i32>;
    pub type isStandaloneSAXFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> i32>;
    pub type internalSubsetSAXFunc = Option<
        unsafe fn(*mut ::core::ffi::c_void, *const xmlChar, *const xmlChar, *const xmlChar) -> (),
    >;
    pub type C2Rust_Unnamed = u32;
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
    use super::xmlChar;
    use super::xmlCharEncoding;
    use super::xmlDictPtr;
    use super::xmlHashTablePtr;
    use super::xmlValidCtxt;
    use super::{
        _xmlNode, xmlAttrPtr, xmlDocPtr, xmlDtdPtr, xmlElementContentPtr, xmlEntityPtr,
        xmlEnumerationPtr, xmlNodePtr, xmlParserCtxtPtr, xmlParserInputBufferPtr,
        xmlParserInputPtr, xmlSAXHandlerPtr, xmlSAXLocatorPtr,
    };
    use super::{xmlError, xmlStructuredErrorFunc};
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
        pub fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: i32) -> i32;
        pub fn xmlCtxtReadFile(
            ctxt: xmlParserCtxtPtr,
            filename: *const i8,
            encoding: *const i8,
            options: i32,
        ) -> xmlDocPtr;
    }
}
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
    pub length: i32,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: i32,
    pub flags: i32,
    pub expandedSize: u64,
}
pub type xmlEntityType = u32;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: i32,
    pub code: i32,
    pub message: *mut i8,
    pub level: xmlErrorLevel,
    pub file: *mut i8,
    pub line: i32,
    pub str1: *mut i8,
    pub str2: *mut i8,
    pub str3: *mut i8,
    pub int1: i32,
    pub int2: i32,
    pub ctxt: *mut ::core::ffi::c_void,
    pub node: *mut ::core::ffi::c_void,
}
pub type xmlErrorLevel = u32;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlStructuredErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlError) -> ()>;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const i8, ...) -> ()>;
extern "C" {
    pub fn xmlSetGenericErrorFunc(ctx: *mut ::core::ffi::c_void, handler: xmlGenericErrorFunc);
}
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
extern "C" {
    pub type _xmlHashTable;
}
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut ::core::ffi::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: i32,
    pub nodeMax: i32,
    pub nodeTab: *mut xmlNodePtr,
    pub flags: u32,
    pub doc: xmlDocPtr,
    pub valid: i32,
    pub vstate: *mut xmlValidState,
    pub vstateNr: i32,
    pub vstateMax: i32,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlValidState = _xmlValidState;
pub type xmlValidityWarningFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const i8, ...) -> ()>;
pub type xmlValidityErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const i8, ...) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
extern "C" {
    pub type _xmlValidState;
    pub fn xmlNewValidCtxt() -> xmlValidCtxtPtr;
    pub fn xmlFreeValidCtxt(_: xmlValidCtxtPtr);
    pub fn xmlValidateDtd(ctxt: xmlValidCtxtPtr, doc: xmlDocPtr, dtd: xmlDtdPtr) -> i32;
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
extern "C" {
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
}
pub type rxkb_log_level = u32;
pub const RXKB_LOG_LEVEL_DEBUG: rxkb_log_level = 50;
pub const RXKB_LOG_LEVEL_INFO: rxkb_log_level = 40;
pub const RXKB_LOG_LEVEL_WARNING: rxkb_log_level = 30;
pub const RXKB_LOG_LEVEL_ERROR: rxkb_log_level = 20;
pub const RXKB_LOG_LEVEL_CRITICAL: rxkb_log_level = 10;
pub type rxkb_popularity = u32;
pub const RXKB_POPULARITY_EXOTIC: rxkb_popularity = 2;
pub const RXKB_POPULARITY_STANDARD: rxkb_popularity = 1;
pub type rxkb_context_flags = u32;
pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;
pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;

pub use crate::xkb::util_list::{
    list, list_append, list_empty, list_init, list_is_last, list_remove,
};
extern "C" {
    pub fn xmlCheckVersion(version: i32);
}

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
pub use crate::xkb::messages::{
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
pub use crate::xkb::shared_types::dirent;
pub use crate::xkb::shared_types::stat;
pub use crate::xkb::shared_types::timespec;
pub use crate::xkb::utils::eaccess;
use crate::xkb::utils::xkb_stat;
pub use crate::xkb::utils::{__dirstream, closedir, opendir, readdir, DIR};
use crate::xkb::utils::{__errno_location, _steal, cstr_dup};
pub use crate::xkb::utils::{
    check_eaccess, is_space, istrncmp, istrneq, strdup_safe, streq, streq_null,
};
use crate::xkb::utils::{cstr_cmp, cstr_len, darray_append, darray_free};
use libc::{calloc, free, getenv, qsort, strtol};
extern "C" {
    fn vsnprintf(s: *mut i8, maxlen: usize, format: *const i8, ...) -> i32;
}
extern "C" {
    pub fn secure_getenv(name: *const i8) -> *mut i8;
}
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
    pub log_fn: Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>,
    pub log_level: rxkb_log_level,
    pub userdata: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub size: darray_size_t,
    pub alloc: darray_size_t,
    pub item: *mut *mut i8,
}
pub type context_state = u32;
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
    pub name: *mut i8,
    pub vendor: *mut i8,
    pub description: *mut i8,
    pub popularity: rxkb_popularity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_layout {
    pub base: rxkb_object,
    pub name: *mut i8,
    pub brief: *mut i8,
    pub description: *mut i8,
    pub variant: *mut i8,
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
    pub name: *mut i8,
    pub description: *mut i8,
    pub popularity: rxkb_popularity,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_option {
    pub base: rxkb_object,
    pub name: *mut i8,
    pub brief: *mut i8,
    pub description: *mut i8,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_iso639_code {
    pub base: rxkb_object,
    pub code: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rxkb_iso3166_code {
    pub base: rxkb_object,
    pub code: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_item {
    pub name: *mut i8,
    pub description: *mut i8,
    pub brief: *mut i8,
    pub vendor: *mut i8,
    pub popularity: rxkb_popularity,
    pub layout_specific: bool,
}

macro_rules! rxkb_logf {
    ($ctx:expr, $level:expr, $($arg:tt)*) => {{
        let mut _rxkb_log_buf = [0u8; 2048];
        {
            let mut _w = crate::xkb::utils::LogBuf::new(&mut _rxkb_log_buf[..2047]);
            let _ = core::fmt::Write::write_fmt(&mut _w, format_args!($($arg)*));
        }
        rxkb_log($ctx, $level, _rxkb_log_buf.as_ptr() as *const i8)
    }};
}

unsafe fn rxkb_log(mut ctx: *mut rxkb_context, mut level: rxkb_log_level, mut msg: *const i8) {
    unsafe {
        if ((*ctx).log_level as u32) < level as u32 {
            return;
        }
        (*ctx).log_fn.expect("non-null function pointer")(ctx, level, msg);
    }
}
unsafe fn rxkb_object_init(mut object: *mut rxkb_object, mut parent: *mut rxkb_object) {
    unsafe {
        (*object).refcount = 1 as u32;
        (*object).parent = parent;
        list_init(&raw mut (*object).link);
    }
}
unsafe fn rxkb_object_ref(mut object: *mut rxkb_object) -> *mut ::core::ffi::c_void {
    unsafe {
        (*object).refcount = (*object).refcount.wrapping_add(1);
        return object as *mut ::core::ffi::c_void;
    }
}
unsafe fn rxkb_iso639_code_destroy(mut code: *mut rxkb_iso639_code) {
    unsafe {
        free((*code).code as *mut ::core::ffi::c_void);
    }
}
pub unsafe fn rxkb_layout_get_iso639_first(mut layout: *mut rxkb_layout) -> *mut rxkb_iso639_code {
    unsafe {
        let mut code: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        if !list_empty(&raw mut (*layout).iso639s) {
            code = ((*layout).iso639s.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        }
        return code;
    }
}
pub unsafe fn rxkb_iso639_code_next(mut code: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code {
    unsafe {
        let mut next: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        let mut layout: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        layout = ((*code).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*layout).iso639s, &raw mut (*code).base.link) {
            return ::core::ptr::null_mut::<rxkb_iso639_code>();
        }
        next = ((*code).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        return next;
    }
}

pub unsafe fn rxkb_iso639_code_ref(mut object: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}

pub unsafe fn rxkb_iso639_code_unref(mut object: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_iso639_code>();
        }
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
unsafe fn rxkb_iso639_code_create(mut parent: *mut rxkb_object) -> *mut rxkb_iso639_code {
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
pub unsafe fn rxkb_iso639_code_get_code(mut object: *mut rxkb_iso639_code) -> *const i8 {
    unsafe {
        return (*object).code;
    }
}
unsafe fn rxkb_iso3166_code_destroy(mut code: *mut rxkb_iso3166_code) {
    unsafe {
        free((*code).code as *mut ::core::ffi::c_void);
    }
}
pub unsafe fn rxkb_layout_get_iso3166_first(
    mut layout: *mut rxkb_layout,
) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut code: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        if !list_empty(&raw mut (*layout).iso3166s) {
            code = ((*layout).iso3166s.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        }
        return code;
    }
}
pub unsafe fn rxkb_iso3166_code_next(mut code: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code {
    unsafe {
        let mut next: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        let mut layout: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        layout = ((*code).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*layout).iso3166s, &raw mut (*code).base.link) {
            return ::core::ptr::null_mut::<rxkb_iso3166_code>();
        }
        next = ((*code).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        return next;
    }
}

pub unsafe fn rxkb_iso3166_code_unref(
    mut object: *mut rxkb_iso3166_code,
) -> *mut rxkb_iso3166_code {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_iso3166_code>();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_iso3166_code_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_iso3166_code>();
    }
}

pub unsafe fn rxkb_iso3166_code_ref(mut object: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[inline]
unsafe fn rxkb_iso3166_code_create(mut parent: *mut rxkb_object) -> *mut rxkb_iso3166_code {
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
pub unsafe fn rxkb_iso3166_code_get_code(mut object: *mut rxkb_iso3166_code) -> *const i8 {
    unsafe {
        return (*object).code;
    }
}
unsafe fn rxkb_option_destroy(mut o: *mut rxkb_option) {
    unsafe {
        free((*o).name as *mut ::core::ffi::c_void);
        free((*o).brief as *mut ::core::ffi::c_void);
        free((*o).description as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn rxkb_option_unref(mut object: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_option>();
        }
        (*object).base.refcount = (*object).base.refcount.wrapping_sub(1);
        if (*object).base.refcount == 0 as u32 {
            rxkb_option_destroy(object);
            list_remove(&raw mut (*object).base.link);
            free(object as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<rxkb_option>();
    }
}

pub unsafe fn rxkb_option_ref(mut object: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
#[inline]
unsafe fn rxkb_option_create(mut parent: *mut rxkb_object) -> *mut rxkb_option {
    unsafe {
        let mut t: *mut rxkb_option =
            calloc(1 as usize, ::core::mem::size_of::<rxkb_option>() as usize) as *mut rxkb_option;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
pub unsafe fn rxkb_option_get_name(mut object: *mut rxkb_option) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_option_get_brief(mut object: *mut rxkb_option) -> *const i8 {
    unsafe {
        return (*object).brief;
    }
}
pub unsafe fn rxkb_option_get_description(mut object: *mut rxkb_option) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}

pub unsafe fn rxkb_option_get_popularity(mut object: *mut rxkb_option) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
pub unsafe fn rxkb_option_is_layout_specific(mut object: *mut rxkb_option) -> bool {
    unsafe {
        return (*object).layout_specific;
    }
}
pub unsafe fn rxkb_option_next(mut o: *mut rxkb_option) -> *mut rxkb_option {
    unsafe {
        let mut parent: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut next: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_option
            as *mut rxkb_option;
        if list_is_last(&raw mut (*parent).options, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_option>();
        }
        return next;
    }
}
pub unsafe fn rxkb_option_first(mut parent: *mut rxkb_option_group) -> *mut rxkb_option {
    unsafe {
        let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        if !list_empty(&raw mut (*parent).options) {
            o = ((*parent).options.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_option as *mut rxkb_option;
        }
        return o;
    }
}
unsafe fn rxkb_layout_destroy(mut l: *mut rxkb_layout) {
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
        iso639 = ((*l).iso639s.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        tmp_639 = ((*iso639).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        while &raw mut (*iso639).base.link != &raw mut (*l).iso639s {
            rxkb_iso639_code_unref(iso639);
            iso639 = tmp_639;
            tmp_639 = ((*iso639).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso639_code as *mut rxkb_iso639_code;
        }
        iso3166 = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        tmp_3166 = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        iso3166 = ((*l).iso3166s.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        tmp_3166 = ((*iso3166).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        while &raw mut (*iso3166).base.link != &raw mut (*l).iso3166s {
            rxkb_iso3166_code_unref(iso3166);
            iso3166 = tmp_3166;
            tmp_3166 = ((*iso3166).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_iso3166_code as *mut rxkb_iso3166_code;
        }
    }
}

pub unsafe fn rxkb_layout_ref(mut object: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}

pub unsafe fn rxkb_layout_unref(mut object: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_layout>();
        }
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
unsafe fn rxkb_layout_create(mut parent: *mut rxkb_object) -> *mut rxkb_layout {
    unsafe {
        let mut t: *mut rxkb_layout =
            calloc(1 as usize, ::core::mem::size_of::<rxkb_layout>() as usize) as *mut rxkb_layout;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
pub unsafe fn rxkb_layout_get_name(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_layout_get_brief(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).brief;
    }
}
pub unsafe fn rxkb_layout_get_description(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}
pub unsafe fn rxkb_layout_get_variant(mut object: *mut rxkb_layout) -> *const i8 {
    unsafe {
        return (*object).variant;
    }
}

pub unsafe fn rxkb_layout_get_popularity(mut object: *mut rxkb_layout) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
pub unsafe fn rxkb_layout_first(mut parent: *mut rxkb_context) -> *mut rxkb_layout {
    unsafe {
        let mut o: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        if !list_empty(&raw mut (*parent).layouts) {
            o = ((*parent).layouts.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_layout as *mut rxkb_layout;
        }
        return o;
    }
}
pub unsafe fn rxkb_layout_next(mut o: *mut rxkb_layout) -> *mut rxkb_layout {
    unsafe {
        let mut parent: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut next: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        if list_is_last(&raw mut (*parent).layouts, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_layout>();
        }
        return next;
    }
}
unsafe fn rxkb_model_destroy(mut m: *mut rxkb_model) {
    unsafe {
        free((*m).name as *mut ::core::ffi::c_void);
        free((*m).vendor as *mut ::core::ffi::c_void);
        free((*m).description as *mut ::core::ffi::c_void);
    }
}

pub unsafe fn rxkb_model_ref(mut object: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}

pub unsafe fn rxkb_model_unref(mut object: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_model>();
        }
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
unsafe fn rxkb_model_create(mut parent: *mut rxkb_object) -> *mut rxkb_model {
    unsafe {
        let mut t: *mut rxkb_model =
            calloc(1 as usize, ::core::mem::size_of::<rxkb_model>() as usize) as *mut rxkb_model;
        if !t.is_null() {
            rxkb_object_init(&raw mut (*t).base, parent);
        }
        return t;
    }
}
pub unsafe fn rxkb_model_get_name(mut object: *mut rxkb_model) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_model_get_vendor(mut object: *mut rxkb_model) -> *const i8 {
    unsafe {
        return (*object).vendor;
    }
}
pub unsafe fn rxkb_model_get_description(mut object: *mut rxkb_model) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}

pub unsafe fn rxkb_model_get_popularity(mut object: *mut rxkb_model) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
pub unsafe fn rxkb_model_next(mut o: *mut rxkb_model) -> *mut rxkb_model {
    unsafe {
        let mut parent: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut next: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        if list_is_last(&raw mut (*parent).models, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_model>();
        }
        return next;
    }
}
pub unsafe fn rxkb_model_first(mut parent: *mut rxkb_context) -> *mut rxkb_model {
    unsafe {
        let mut o: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        if !list_empty(&raw mut (*parent).models) {
            o = ((*parent).models.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
                as *mut rxkb_model;
        }
        return o;
    }
}
unsafe fn rxkb_option_group_destroy(mut og: *mut rxkb_option_group) {
    unsafe {
        let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        let mut otmp: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
        free((*og).name as *mut ::core::ffi::c_void);
        free((*og).description as *mut ::core::ffi::c_void);
        o = ::core::ptr::null_mut::<rxkb_option>();
        otmp = ::core::ptr::null_mut::<rxkb_option>();
        o = ((*og).options.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_option
            as *mut rxkb_option;
        otmp = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_option
            as *mut rxkb_option;
        while &raw mut (*o).base.link != &raw mut (*og).options {
            rxkb_option_unref(o);
            o = otmp;
            otmp = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_option as *mut rxkb_option;
        }
    }
}
pub unsafe fn rxkb_option_group_allows_multiple(mut g: *mut rxkb_option_group) -> bool {
    unsafe {
        return (*g).allow_multiple;
    }
}

pub unsafe fn rxkb_option_group_ref(mut object: *mut rxkb_option_group) -> *mut rxkb_option_group {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}

pub unsafe fn rxkb_option_group_unref(
    mut object: *mut rxkb_option_group,
) -> *mut rxkb_option_group {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_option_group>();
        }
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
unsafe fn rxkb_option_group_create(mut parent: *mut rxkb_object) -> *mut rxkb_option_group {
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
pub unsafe fn rxkb_option_group_get_name(mut object: *mut rxkb_option_group) -> *const i8 {
    unsafe {
        return (*object).name;
    }
}
pub unsafe fn rxkb_option_group_get_description(mut object: *mut rxkb_option_group) -> *const i8 {
    unsafe {
        return (*object).description;
    }
}

pub unsafe fn rxkb_option_group_get_popularity(
    mut object: *mut rxkb_option_group,
) -> rxkb_popularity {
    unsafe {
        return (*object).popularity;
    }
}
pub unsafe fn rxkb_option_group_first(mut parent: *mut rxkb_context) -> *mut rxkb_option_group {
    unsafe {
        let mut o: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        if !list_empty(&raw mut (*parent).option_groups) {
            o = ((*parent).option_groups.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_option_group as *mut rxkb_option_group;
        }
        return o;
    }
}
pub unsafe fn rxkb_option_group_next(mut o: *mut rxkb_option_group) -> *mut rxkb_option_group {
    unsafe {
        let mut parent: *mut rxkb_context = ::core::ptr::null_mut::<rxkb_context>();
        let mut next: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        parent = ((*o).base.parent as *mut i8).offset(-(0 as u64 as isize)) as *mut rxkb_context
            as *mut rxkb_context;
        next = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        if list_is_last(&raw mut (*parent).option_groups, &raw mut (*o).base.link) {
            return ::core::ptr::null_mut::<rxkb_option_group>();
        }
        return next;
    }
}
unsafe fn rxkb_context_destroy(mut ctx: *mut rxkb_context) {
    unsafe {
        let mut m: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        let mut mtmp: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
        let mut l: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        let mut ltmp: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
        let mut og: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut ogtmp: *mut rxkb_option_group = ::core::ptr::null_mut::<rxkb_option_group>();
        let mut path: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
        m = ::core::ptr::null_mut::<rxkb_model>();
        mtmp = ::core::ptr::null_mut::<rxkb_model>();
        m = ((*ctx).models.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        mtmp = ((*m).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
            as *mut rxkb_model;
        while &raw mut (*m).base.link != &raw mut (*ctx).models {
            rxkb_model_unref(m);
            m = mtmp;
            mtmp = ((*m).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
                as *mut rxkb_model;
        }
        l = ::core::ptr::null_mut::<rxkb_layout>();
        ltmp = ::core::ptr::null_mut::<rxkb_layout>();
        l = ((*ctx).layouts.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        ltmp = ((*l).base.link.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        while &raw mut (*l).base.link != &raw mut (*ctx).layouts {
            rxkb_layout_unref(l);
            l = ltmp;
            ltmp = ((*l).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_layout as *mut rxkb_layout;
        }
        og = ::core::ptr::null_mut::<rxkb_option_group>();
        ogtmp = ::core::ptr::null_mut::<rxkb_option_group>();
        og = ((*ctx).option_groups.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        ogtmp = ((*og).base.link.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        while &raw mut (*og).base.link != &raw mut (*ctx).option_groups {
            rxkb_option_group_unref(og);
            og = ogtmp;
            ogtmp = ((*og).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                as *mut rxkb_option_group as *mut rxkb_option_group;
        }
        if !(*ctx).includes.item.is_null() {
            path = (*ctx).includes.item.offset(0 as i32 as isize) as *mut *mut i8;
            while path < (*ctx).includes.item.offset((*ctx).includes.size as isize) as *mut *mut i8
            {
                free(*path as *mut ::core::ffi::c_void);
                path = path.offset(1);
            }
        }
        darray_free(
            &mut (*ctx).includes.item,
            &mut (*ctx).includes.size,
            &mut (*ctx).includes.alloc,
        );
    }
}

pub unsafe fn rxkb_context_ref(mut object: *mut rxkb_context) -> *mut rxkb_context {
    unsafe {
        rxkb_object_ref(&raw mut (*object).base);
        return object;
    }
}
pub unsafe fn rxkb_context_unref(mut object: *mut rxkb_context) -> *mut rxkb_context {
    unsafe {
        if object.is_null() {
            return ::core::ptr::null_mut::<rxkb_context>();
        }
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
unsafe fn rxkb_context_create(mut parent: *mut rxkb_object) -> *mut rxkb_context {
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

pub unsafe fn rxkb_context_get_log_level(mut object: *mut rxkb_context) -> rxkb_log_level {
    unsafe {
        return (*object).log_level;
    }
}
unsafe fn rxkb_context_getenv(mut ctx: *mut rxkb_context, mut name: *const i8) -> *mut i8 {
    unsafe {
        if (*ctx).use_secure_getenv {
            return secure_getenv(name);
        } else {
            return getenv(name);
        };
    }
}
pub unsafe fn rxkb_context_set_log_level(mut ctx: *mut rxkb_context, mut level: rxkb_log_level) {
    unsafe {
        (*ctx).log_level = level;
    }
}
unsafe fn log_level_to_prefix(mut level: rxkb_log_level) -> *const i8 {
    match level as u32 {
        50 => return b"xkbregistry: DEBUG: \0".as_ptr() as *const i8,
        40 => return b"xkbregistry: INFO: \0".as_ptr() as *const i8,
        30 => {
            return b"xkbregistry: WARNING: \0".as_ptr() as *const i8;
        }
        20 => return b"xkbregistry: ERROR: \0".as_ptr() as *const i8,
        10 => {
            return b"xkbregistry: CRITICAL: \0".as_ptr() as *const i8;
        }
        _ => return ::core::ptr::null::<i8>(),
    };
}
unsafe fn default_log_fn(
    mut ctx: *mut rxkb_context,
    mut level: rxkb_log_level,
    mut msg: *const i8,
) {
    unsafe {
        let mut prefix: *const i8 = log_level_to_prefix(level);
        if !prefix.is_null() {
            eprint!("{}", crate::xkb::utils::CStrDisplay(prefix));
        }
        eprint!("{}", crate::xkb::utils::CStrDisplay(msg));
    }
}
unsafe fn log_level(mut level: *const i8) -> rxkb_log_level {
    unsafe {
        let mut endptr: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut lvl: rxkb_log_level = 0 as rxkb_log_level;
        *__errno_location() = 0 as i32;
        lvl = strtol(level, &raw mut endptr, 10 as i32) as rxkb_log_level;
        if *__errno_location() == 0 as i32
            && (*endptr.offset(0 as i32 as isize) as i32 == '\0' as i32
                || is_space(*endptr.offset(0 as i32 as isize)) as i32 != 0)
        {
            return lvl;
        }
        if istrneq(
            b"crit\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_CRITICAL;
        }
        if istrneq(
            b"err\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_ERROR;
        }
        if istrneq(
            b"warn\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_WARNING;
        }
        if istrneq(
            b"info\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 5]>() as usize).wrapping_sub(1 as usize),
        ) {
            return RXKB_LOG_LEVEL_INFO;
        }
        if istrneq(
            b"debug\0".as_ptr() as *const i8,
            level,
            (::core::mem::size_of::<[i8; 6]>() as usize).wrapping_sub(1 as usize),
        ) as i32
            != 0
            || istrneq(
                b"dbg\0".as_ptr() as *const i8,
                level,
                (::core::mem::size_of::<[i8; 4]>() as usize).wrapping_sub(1 as usize),
            ) as i32
                != 0
        {
            return RXKB_LOG_LEVEL_DEBUG;
        }
        return RXKB_LOG_LEVEL_ERROR;
    }
}
pub unsafe fn rxkb_context_new(mut flags: rxkb_context_flags) -> *mut rxkb_context {
    unsafe {
        let mut ctx: *mut rxkb_context =
            rxkb_context_create(::core::ptr::null_mut::<rxkb_object>());
        let mut env: *const i8 = ::core::ptr::null::<i8>();
        if ctx.is_null() {
            return ::core::ptr::null_mut::<rxkb_context>();
        }
        (*ctx).context_state = CONTEXT_NEW;
        (*ctx).load_extra_rules_files =
            flags as u32 & RXKB_CONTEXT_LOAD_EXOTIC_RULES as i32 as u32 != 0;
        (*ctx).use_secure_getenv = flags as u32 & RXKB_CONTEXT_NO_SECURE_GETENV as i32 as u32 == 0;
        (*ctx).log_fn =
            Some(default_log_fn as unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ())
                as Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>;
        (*ctx).log_level = RXKB_LOG_LEVEL_ERROR;
        env = rxkb_context_getenv(ctx, b"RXKB_LOG_LEVEL\0".as_ptr() as *const i8);
        if !env.is_null() {
            rxkb_context_set_log_level(ctx, log_level(env));
        }
        static mut RXKB_CONTEXT_FLAGS: rxkb_context_flags =
            (RXKB_CONTEXT_NO_DEFAULT_INCLUDES as i32
                | RXKB_CONTEXT_LOAD_EXOTIC_RULES as i32
                | RXKB_CONTEXT_NO_SECURE_GETENV as i32) as rxkb_context_flags;
        if flags as u32 & !(RXKB_CONTEXT_FLAGS as u32) != 0 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "{}: Invalid context flags: 0x{:x}\n",
                crate::xkb::utils::CStrDisplay(b"rxkb_context_new\0".as_ptr() as *const i8),
                flags as u32 & !(RXKB_CONTEXT_FLAGS as u32),
            );
            free(ctx as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<rxkb_context>();
        }
        list_init(&raw mut (*ctx).models);
        list_init(&raw mut (*ctx).layouts);
        list_init(&raw mut (*ctx).option_groups);
        if flags as u32 & RXKB_CONTEXT_NO_DEFAULT_INCLUDES as i32 as u32 == 0
            && !rxkb_context_include_path_append_default(ctx)
        {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "[XKB-{:03}] Failed to add any default include path (default system path: {})\n",
                XKB_ERROR_NO_VALID_DEFAULT_INCLUDE_PATH as i32,
                crate::xkb::utils::CStrDisplay(
                    b"/usr/share/xkeyboard-config-2\0".as_ptr() as *const i8
                ),
            );
            rxkb_context_unref(ctx);
            return ::core::ptr::null_mut::<rxkb_context>();
        }
        return ctx;
    }
}

pub unsafe fn rxkb_context_set_log_fn(
    mut ctx: *mut rxkb_context,
    mut log_fn: Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>,
) {
    unsafe {
        (*ctx).log_fn = (if log_fn.is_some() {
            log_fn as Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>
        } else {
            Some(default_log_fn as unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ())
        })
            as Option<unsafe fn(*mut rxkb_context, rxkb_log_level, *const i8) -> ()>;
    }
}
pub unsafe fn rxkb_context_include_path_append(
    mut ctx: *mut rxkb_context,
    mut path: *const i8,
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
        let mut rules: [i8; 4096] = [0; 4096];
        let mut tmp: *mut i8 = ::core::ptr::null_mut::<i8>();
        let mut err: i32 = 0 as i32;
        if (*ctx).context_state as u32 != CONTEXT_NEW as i32 as u32 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
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
            err = xkb_stat(path, &raw mut stat_buf);
            if err != 0 as i32 {
                err = *__errno_location();
            } else if !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32) {
                err = ENOTDIR;
            } else if !check_eaccess(path, R_OK | X_OK) {
                err = EACCES;
            } else {
                rules = [0; 4096];
                let (_, _trunc) = crate::xkb::utils::snprintf_args(
                    &raw mut rules as *mut i8,
                    ::core::mem::size_of::<[i8; 4096]>() as usize,
                    format_args!(
                        "{}/rules/{}.xml",
                        crate::xkb::utils::CStrDisplay(path),
                        crate::xkb::utils::CStrDisplay(DEFAULT_XKB_RULES.as_ptr())
                    ),
                );
                if _trunc {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        "[XKB-{:03}] Path is too long: expected max length of {}, got: {}/rules/{}.xml\n",
                        XKB_ERROR_INVALID_PATH as i32,
                        ::core::mem::size_of::<[i8; 4096]>(),
                        crate::xkb::utils::CStrDisplay(path),
                        crate::xkb::utils::CStrDisplay(b"evdev\0".as_ptr() as *const i8),
                    );
                } else {
                    tmp = cstr_dup(path);
                    if tmp.is_null() {
                        err = ENOMEM;
                    } else {
                        darray_append(
                            &mut (*ctx).includes.item,
                            &mut (*ctx).includes.size,
                            &mut (*ctx).includes.alloc,
                            tmp,
                        );
                        rxkb_logf!(
                            ctx,
                            RXKB_LOG_LEVEL_INFO,
                            "Include path added: {}\n",
                            crate::xkb::utils::CStrDisplay(tmp),
                        );
                        return true;
                    }
                }
            }
        }
        rxkb_logf!(
            ctx,
            RXKB_LOG_LEVEL_INFO,
            "Include path failed: \"{}\" ({})\n",
            crate::xkb::utils::CStrDisplay(path),
            crate::xkb::utils::StrerrorDisplay(err),
        );
        return false;
    }
}
unsafe extern "C" fn compare_str(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> i32 {
    unsafe {
        return cstr_cmp(*(a as *mut *mut i8), *(b as *mut *mut i8));
    }
}
unsafe fn add_direct_subdirectories(
    mut ctx: *mut rxkb_context,
    mut path: *const i8,
    mut extensions: *mut darray_string,
    mut versioned_count: darray_size_t,
    mut versioned_path_length: usize,
) -> i32 {
    unsafe {
        let mut entry: *mut dirent = ::core::ptr::null_mut::<dirent>();
        let mut path_buf: [i8; 4096] = [0; 4096];
        let mut c2rust_current_block: u64;
        let mut ret: i32 = 0 as i32;
        let mut err: i32 = ENOMEM;
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
        err = xkb_stat(path, &raw mut stat_buf);
        if err != 0 as i32 {
            err = *__errno_location();
        } else if !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32) {
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
                    [i8; 4096],
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
                    let mut name: *const i8 = &raw mut (*entry).d_name as *mut i8;
                    if cstr_cmp(name, b".\0".as_ptr() as *const i8) == 0 as i32
                        || cstr_cmp(name, b"..\0".as_ptr() as *const i8) == 0 as i32
                    {
                        continue;
                    }
                    let (_, _trunc) = crate::xkb::utils::snprintf_args(
                        &raw mut path_buf as *mut i8,
                        ::core::mem::size_of::<[i8; 4096]>() as usize,
                        format_args!(
                            "{}/{}",
                            crate::xkb::utils::CStrDisplay(path),
                            crate::xkb::utils::CStrDisplay(name)
                        ),
                    );
                    if _trunc {
                        err = ENOMEM;
                        c2rust_current_block = 17009998909239196508;
                        break;
                    } else {
                        if xkb_stat(&raw mut path_buf as *mut i8, &raw mut stat_buf) != 0 as i32
                            || !(stat_buf.st_mode & __S_IFMT as u32 == 0o40000 as u32)
                        {
                            continue;
                        }
                        let mut i: darray_size_t = 0 as darray_size_t;
                        while i < versioned_count {
                            let prev_name: *const i8 = (*(*extensions).item.offset(i as isize))
                                .offset(versioned_path_length as isize);
                            if cstr_cmp(name, prev_name) == 0 as i32 {
                                continue 's_62;
                            }
                            i = i.wrapping_add(1);
                        }
                        let mut ext_path: *mut i8 = strdup_safe(&raw mut path_buf as *mut i8);
                        if ext_path.is_null() {
                            err = ENOMEM;
                            c2rust_current_block = 17009998909239196508;
                            break;
                        } else {
                            darray_append(
                                &mut (*extensions).item,
                                &mut (*extensions).size,
                                &mut (*extensions).alloc,
                                ext_path,
                            );
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
                                ::core::mem::size_of::<*mut i8>() as usize,
                                Some(
                                    compare_str
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_void,
                                            *const ::core::ffi::c_void,
                                        )
                                            -> i32,
                                ),
                            );
                            let mut ext_path_0: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
                            if !(*extensions).item.is_null() {
                                ext_path_0 = (*extensions).item.offset(versioned_count as isize)
                                    as *mut *mut i8;
                                while ext_path_0
                                    < (*extensions).item.offset((*extensions).size as isize)
                                        as *mut *mut i8
                                {
                                    ret |=
                                        rxkb_context_include_path_append(ctx, *ext_path_0) as i32;
                                    ext_path_0 = ext_path_0.offset(1);
                                }
                            }
                        }
                        return ret;
                    }
                }
            }
        }
        rxkb_logf!(
            ctx,
            RXKB_LOG_LEVEL_DEBUG,
            "Include extensions path failed: {} ({})\n",
            crate::xkb::utils::CStrDisplay(path),
            crate::xkb::utils::StrerrorDisplay(err),
        );
        if !dir.is_null() {
            closedir(dir);
        }
        return ret;
    }
}
pub unsafe fn rxkb_context_include_path_append_default(mut ctx: *mut rxkb_context) -> bool {
    unsafe {
        let mut user_path: [i8; 4096] = [0; 4096];
        let mut ret: i32 = if false { 1 } else { 0 };
        if (*ctx).context_state as u32 != CONTEXT_NEW as i32 as u32 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "include paths can only be appended to a new context\n",
            );
            return false;
        }
        let home: *const i8 = rxkb_context_getenv(ctx, b"HOME\0".as_ptr() as *const i8);
        let xdg: *const i8 = rxkb_context_getenv(ctx, b"XDG_CONFIG_HOME\0".as_ptr() as *const i8);
        if !xdg.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
                format_args!("{}/xkb", crate::xkb::utils::CStrDisplay(xdg)),
            );
            if !_trunc {
                ret = ret as i32
                    | rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        } else if !home.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
                format_args!("{}/.config/xkb", crate::xkb::utils::CStrDisplay(home)),
            );
            if !_trunc {
                ret = ret as i32
                    | rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        }
        if !home.is_null() {
            let (_, _trunc) = crate::xkb::utils::snprintf_args(
                &raw mut user_path as *mut i8,
                ::core::mem::size_of::<[i8; 4096]>() as usize,
                format_args!("{}/.xkb", crate::xkb::utils::CStrDisplay(home)),
            );
            if !_trunc {
                ret = ret as i32
                    | rxkb_context_include_path_append(ctx, &raw mut user_path as *mut i8) as i32;
            }
        }
        let extra: *const i8 =
            rxkb_context_getenv(ctx, b"XKB_CONFIG_EXTRA_PATH\0".as_ptr() as *const i8);
        ret = ret as i32
            | rxkb_context_include_path_append(
                ctx,
                if !extra.is_null() {
                    extra
                } else {
                    DFLT_XKB_CONFIG_EXTRA_PATH.as_ptr()
                },
            ) as i32;
        let mut extensions: darray_string = darray_string {
            size: 0 as darray_size_t,
            alloc: 0 as darray_size_t,
            item: ::core::ptr::null_mut::<*mut i8>(),
        };
        let mut extensions_path: *const i8 = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_VERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const i8,
        );
        if extensions_path.is_null() {
            extensions_path = DFLT_XKB_CONFIG_VERSIONED_EXTENSIONS_PATH.as_ptr();
        }
        let mut versioned_path_length: usize = 0 as usize;
        if !extensions_path.is_null() {
            ret = ret as i32
                | add_direct_subdirectories(
                    ctx,
                    extensions_path,
                    &raw mut extensions,
                    0 as darray_size_t,
                    0 as usize,
                );
            versioned_path_length = cstr_len(extensions_path);
        }
        extensions_path = rxkb_context_getenv(
            ctx,
            b"XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH\0".as_ptr() as *const i8,
        );
        if extensions_path.is_null() {
            extensions_path = DFLT_XKB_CONFIG_UNVERSIONED_EXTENSIONS_PATH.as_ptr();
        }
        if !extensions_path.is_null() {
            ret = ret as i32
                | add_direct_subdirectories(
                    ctx,
                    extensions_path,
                    &raw mut extensions,
                    extensions.size,
                    versioned_path_length,
                );
        }
        let mut ext_path: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
        if !extensions.item.is_null() {
            ext_path = extensions.item.offset(0 as i32 as isize) as *mut *mut i8;
            while ext_path < extensions.item.offset(extensions.size as isize) as *mut *mut i8 {
                free(*ext_path as *mut ::core::ffi::c_void);
                ext_path = ext_path.offset(1);
            }
        }
        darray_free(
            &mut extensions.item,
            &mut extensions.size,
            &mut extensions.alloc,
        );
        let root: *const i8 = rxkb_context_getenv(ctx, b"XKB_CONFIG_ROOT\0".as_ptr() as *const i8);
        let has_root: bool = rxkb_context_include_path_append(
            ctx,
            if !root.is_null() {
                root
            } else {
                DFLT_XKB_CONFIG_ROOT.as_ptr()
            },
        ) as bool;
        ret = ret as i32 | has_root as i32;
        if !has_root && (root.is_null() || *root.offset(0 as i32 as isize) as i32 != '\0' as i32) {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_WARNING,
                "Root include path failed; fallback to \"{}\". The setup is probably misconfigured. Please ensure that \"{}\" is available in the environment.\n",
                crate::xkb::utils::CStrDisplay(b"/usr/share/X11/xkb\0".as_ptr() as *const i8),
                crate::xkb::utils::CStrDisplay(if root.is_null() {
                    b"/usr/share/xkeyboard-config-2\0".as_ptr()
                        as *const i8
                } else {
                    root
                }),
            );
            ret = ret as i32
                | rxkb_context_include_path_append(ctx, DFLT_XKB_LEGACY_ROOT.as_ptr()) as i32;
        }
        return ret != 0;
    }
}

pub unsafe fn rxkb_context_parse_default_ruleset(mut ctx: *mut rxkb_context) -> bool {
    unsafe {
        return rxkb_context_parse(ctx, DEFAULT_XKB_RULES.as_ptr());
    }
}
pub unsafe fn rxkb_context_parse(mut ctx: *mut rxkb_context, mut ruleset: *const i8) -> bool {
    unsafe {
        let mut path: *mut *mut i8 = ::core::ptr::null_mut::<*mut i8>();
        let mut success: bool = false;
        if (*ctx).context_state as u32 != CONTEXT_NEW as i32 as u32 {
            rxkb_logf!(
                ctx,
                RXKB_LOG_LEVEL_ERROR,
                "parse must only be called on a new context\n",
            );
            return false;
        }
        if !(*ctx).includes.item.is_null() && (*ctx).includes.size != 0 {
            path = (*ctx)
                .includes
                .item
                .offset((*ctx).includes.size.wrapping_sub(1 as darray_size_t) as isize)
                as *mut *mut i8;
            while (*ctx).includes.size > 0 as darray_size_t
                && path >= (*ctx).includes.item.offset(0 as i32 as isize) as *mut *mut i8
            {
                let mut rules: [i8; 4096] = [0; 4096];
                let (_, _trunc) = crate::xkb::utils::snprintf_args(
                    &raw mut rules as *mut i8,
                    ::core::mem::size_of::<[i8; 4096]>() as usize,
                    format_args!(
                        "{}/rules/{}.xml",
                        crate::xkb::utils::CStrDisplay(*path),
                        crate::xkb::utils::CStrDisplay(ruleset)
                    ),
                );
                if !_trunc {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_DEBUG,
                        "Parsing {}\n",
                        crate::xkb::utils::CStrDisplay(&raw mut rules as *mut i8),
                    );
                    if parse(ctx, &raw mut rules as *mut i8, RXKB_POPULARITY_STANDARD) {
                        success = true;
                    }
                }
                if (*ctx).load_extra_rules_files as i32 != 0 {
                    let (_, _trunc) = crate::xkb::utils::snprintf_args(
                        &raw mut rules as *mut i8,
                        ::core::mem::size_of::<[i8; 4096]>() as usize,
                        format_args!(
                            "{}/rules/{}.extras.xml",
                            crate::xkb::utils::CStrDisplay(*path),
                            crate::xkb::utils::CStrDisplay(ruleset)
                        ),
                    );
                    if !_trunc {
                        rxkb_logf!(
                            ctx,
                            RXKB_LOG_LEVEL_DEBUG,
                            "Parsing {}\n",
                            crate::xkb::utils::CStrDisplay(&raw mut rules as *mut i8),
                        );
                        if parse(ctx, &raw mut rules as *mut i8, RXKB_POPULARITY_EXOTIC) {
                            success = true;
                        }
                    }
                }
                path = path.offset(-1);
            }
        }
        (*ctx).context_state = (if success as i32 != 0 {
            CONTEXT_PARSED as i32
        } else {
            CONTEXT_FAILED as i32
        }) as context_state;
        return success;
    }
}

pub unsafe fn rxkb_context_set_user_data(
    mut ctx: *mut rxkb_context,
    mut userdata: *mut ::core::ffi::c_void,
) {
    unsafe {
        (*ctx).userdata = userdata;
    }
}

pub unsafe fn rxkb_context_get_user_data(mut ctx: *mut rxkb_context) -> *mut ::core::ffi::c_void {
    unsafe {
        return (*ctx).userdata;
    }
}
#[inline]
unsafe fn is_node(mut node: *mut xmlNode, mut name: *const i8) -> bool {
    unsafe {
        return (*node).type_0 as u32 == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual((*node).name, name as *const xmlChar) != 0;
    }
}
unsafe fn extract_text(mut node: *mut xmlNode) -> *mut i8 {
    unsafe {
        let mut n: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        n = (*node).children as *mut xmlNode;
        while !n.is_null() {
            if (*n).type_0 as u32 == XML_TEXT_NODE as i32 as u32 {
                return xmlStrdup((*n).content) as *mut i8;
            }
            n = (*n).next as *mut xmlNode;
        }
        return ::core::ptr::null_mut::<i8>();
    }
}
unsafe fn config_item_free(mut config: *mut config_item) {
    unsafe {
        free((*config).name as *mut ::core::ffi::c_void);
        free((*config).description as *mut ::core::ffi::c_void);
        free((*config).brief as *mut ::core::ffi::c_void);
        free((*config).vendor as *mut ::core::ffi::c_void);
    }
}
unsafe fn parse_config_item(
    mut ctx: *mut rxkb_context,
    mut parent: *mut xmlNode,
    mut config: *mut config_item,
) -> bool {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut ci: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        ci = (*parent).children as *mut xmlNode;
        while !ci.is_null() {
            if is_node(ci, b"configItem\0".as_ptr() as *const i8) {
                let mut raw_popularity: *mut xmlChar =
                    xmlGetProp(ci, b"popularity\0".as_ptr() as *const i8 as *const xmlChar);
                if !raw_popularity.is_null() {
                    if xmlStrEqual(
                        raw_popularity,
                        b"standard\0".as_ptr() as *const i8 as *const xmlChar,
                    ) != 0
                    {
                        (*config).popularity = RXKB_POPULARITY_STANDARD;
                    } else if xmlStrEqual(
                        raw_popularity,
                        b"exotic\0".as_ptr() as *const i8 as *const xmlChar,
                    ) != 0
                    {
                        (*config).popularity = RXKB_POPULARITY_EXOTIC;
                    } else {
                        rxkb_logf!(
                            ctx,
                            RXKB_LOG_LEVEL_ERROR,
                            "xml:{}: invalid popularity attribute: expected 'standard' or 'exotic', got: '{}'\n",
                            (*ci).line as i32,
                            crate::xkb::utils::CStrDisplay(raw_popularity as *const i8),
                        );
                    }
                }
                xmlFree.expect("non-null function pointer")(
                    raw_popularity as *mut ::core::ffi::c_void,
                );
                let mut raw_layout_specific: *mut xmlChar = xmlGetProp(
                    ci,
                    b"layout-specific\0".as_ptr() as *const i8 as *const xmlChar,
                );
                if !raw_layout_specific.is_null()
                    && xmlStrEqual(
                        raw_layout_specific,
                        b"true\0".as_ptr() as *const i8 as *const xmlChar,
                    ) != 0
                {
                    (*config).layout_specific = true;
                }
                xmlFree.expect("non-null function pointer")(
                    raw_layout_specific as *mut ::core::ffi::c_void,
                );
                node = (*ci).children as *mut xmlNode;
                while !node.is_null() {
                    if is_node(node, b"name\0".as_ptr() as *const i8) {
                        (*config).name = extract_text(node);
                    } else if is_node(node, b"description\0".as_ptr() as *const i8) {
                        (*config).description = extract_text(node);
                    } else if is_node(node, b"shortDescription\0".as_ptr() as *const i8) {
                        (*config).brief = extract_text(node);
                    } else if is_node(node, b"vendor\0".as_ptr() as *const i8) {
                        (*config).vendor = extract_text(node);
                    }
                    node = (*node).next as *mut xmlNode;
                }
                if (*config).name.is_null() || cstr_len((*config).name) == 0 {
                    rxkb_logf!(
                        ctx,
                        RXKB_LOG_LEVEL_ERROR,
                        "xml:{}: missing required element 'name'\n",
                        (*ci).line as i32,
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
unsafe fn parse_model(
    mut ctx: *mut rxkb_context,
    mut model: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<i8>(),
            description: ::core::ptr::null_mut::<i8>(),
            brief: ::core::ptr::null_mut::<i8>(),
            vendor: ::core::ptr::null_mut::<i8>(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, model, &raw mut config) {
            let mut m: *mut rxkb_model = ::core::ptr::null_mut::<rxkb_model>();
            m = ::core::ptr::null_mut::<rxkb_model>();
            m = ((*ctx).models.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_model
                as *mut rxkb_model;
            while &raw mut (*m).base.link != &raw mut (*ctx).models {
                if streq((*m).name, config.name) {
                    config_item_free(&raw mut config);
                    return;
                }
                m = ((*m).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_model as *mut rxkb_model;
            }
            m = rxkb_model_create(&raw mut (*ctx).base);
            (*m).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*m).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*m).vendor =
                _steal(&raw mut config.vendor as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*m).popularity = config.popularity;
            list_append(&raw mut (*ctx).models, &raw mut (*m).base.link);
        }
    }
}
unsafe fn parse_model_list(
    mut ctx: *mut rxkb_context,
    mut model_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*model_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"model\0".as_ptr() as *const i8) {
                parse_model(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe fn parse_language_list(mut language_list: *mut xmlNode, mut layout: *mut rxkb_layout) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut code: *mut rxkb_iso639_code = ::core::ptr::null_mut::<rxkb_iso639_code>();
        node = (*language_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"iso639Id\0".as_ptr() as *const i8) {
                let mut str: *mut i8 = extract_text(node);
                let mut parent: *mut rxkb_object = ::core::ptr::null_mut::<rxkb_object>();
                if str.is_null() || cstr_len(str) != 3 as usize {
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
unsafe fn parse_country_list(mut country_list: *mut xmlNode, mut layout: *mut rxkb_layout) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut code: *mut rxkb_iso3166_code = ::core::ptr::null_mut::<rxkb_iso3166_code>();
        node = (*country_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"iso3166Id\0".as_ptr() as *const i8) {
                let mut str: *mut i8 = extract_text(node);
                let mut parent: *mut rxkb_object = ::core::ptr::null_mut::<rxkb_object>();
                if str.is_null() || cstr_len(str) != 2 as usize {
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
unsafe fn parse_variant(
    mut ctx: *mut rxkb_context,
    mut l: *mut rxkb_layout,
    mut variant: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut ci: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<i8>(),
            description: ::core::ptr::null_mut::<i8>(),
            brief: ::core::ptr::null_mut::<i8>(),
            vendor: ::core::ptr::null_mut::<i8>(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, variant, &raw mut config) {
            let mut v: *mut rxkb_layout = ::core::ptr::null_mut::<rxkb_layout>();
            let mut exists: bool = false;
            v = ::core::ptr::null_mut::<rxkb_layout>();
            v = ((*ctx).layouts.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
                as *mut rxkb_layout;
            while &raw mut (*v).base.link != &raw mut (*ctx).layouts {
                if streq_null((*v).variant, config.name) as i32 != 0
                    && streq((*v).name, (*l).name) as i32 != 0
                {
                    exists = true;
                    break;
                } else {
                    v = ((*v).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                        as *mut rxkb_layout as *mut rxkb_layout;
                }
            }
            if !exists {
                v = rxkb_layout_create(&raw mut (*ctx).base);
                list_init(&raw mut (*v).iso639s);
                list_init(&raw mut (*v).iso3166s);
                (*v).name = cstr_dup((*l).name);
                (*v).variant =
                    _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
                (*v).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                    as *mut i8 as *mut i8;
                (*v).brief = if config.brief.is_null() {
                    strdup_safe((*l).brief)
                } else {
                    _steal(&raw mut config.brief as *mut ::core::ffi::c_void) as *mut i8
                };
                (*v).popularity = config.popularity;
                list_append(&raw mut (*ctx).layouts, &raw mut (*v).base.link);
                ci = (*variant).children as *mut xmlNode;
                while !ci.is_null() {
                    let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
                    if is_node(ci, b"configItem\0".as_ptr() as *const i8) {
                        let mut found_language_list: bool = false;
                        let mut found_country_list: bool = false;
                        node = (*ci).children as *mut xmlNode;
                        while !node.is_null() {
                            if is_node(node, b"languageList\0".as_ptr() as *const i8) {
                                parse_language_list(node, v);
                                found_language_list = true;
                            }
                            if is_node(node, b"countryList\0".as_ptr() as *const i8) {
                                parse_country_list(node, v);
                                found_country_list = true;
                            }
                            node = (*node).next as *mut xmlNode;
                        }
                        if !found_language_list {
                            let mut x: *mut rxkb_iso639_code =
                                ::core::ptr::null_mut::<rxkb_iso639_code>();
                            x = ::core::ptr::null_mut::<rxkb_iso639_code>();
                            x = ((*l).iso639s.next as *mut i8).offset(-(16 as u64 as isize))
                                as *mut rxkb_iso639_code
                                as *mut rxkb_iso639_code;
                            while &raw mut (*x).base.link != &raw mut (*l).iso639s {
                                let mut code: *mut rxkb_iso639_code =
                                    rxkb_iso639_code_create(&raw mut (*v).base);
                                (*code).code = cstr_dup((*x).code);
                                list_append(&raw mut (*v).iso639s, &raw mut (*code).base.link);
                                x = ((*x).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                                    as *mut rxkb_iso639_code
                                    as *mut rxkb_iso639_code;
                            }
                        }
                        if !found_country_list {
                            let mut x_0: *mut rxkb_iso3166_code =
                                ::core::ptr::null_mut::<rxkb_iso3166_code>();
                            x_0 = ::core::ptr::null_mut::<rxkb_iso3166_code>();
                            x_0 = ((*l).iso3166s.next as *mut i8).offset(-(16 as u64 as isize))
                                as *mut rxkb_iso3166_code
                                as *mut rxkb_iso3166_code;
                            while &raw mut (*x_0).base.link != &raw mut (*l).iso3166s {
                                let mut code_0: *mut rxkb_iso3166_code =
                                    rxkb_iso3166_code_create(&raw mut (*v).base);
                                (*code_0).code = cstr_dup((*x_0).code);
                                list_append(&raw mut (*v).iso3166s, &raw mut (*code_0).base.link);
                                x_0 = ((*x_0).base.link.next as *mut i8)
                                    .offset(-(16 as u64 as isize))
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
unsafe fn parse_variant_list(
    mut ctx: *mut rxkb_context,
    mut l: *mut rxkb_layout,
    mut variant_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*variant_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"variant\0".as_ptr() as *const i8) {
                parse_variant(ctx, l, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe fn parse_layout(
    mut ctx: *mut rxkb_context,
    mut layout: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<i8>(),
            description: ::core::ptr::null_mut::<i8>(),
            brief: ::core::ptr::null_mut::<i8>(),
            vendor: ::core::ptr::null_mut::<i8>(),
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
        l = ((*ctx).layouts.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_layout
            as *mut rxkb_layout;
        while &raw mut (*l).base.link != &raw mut (*ctx).layouts {
            if streq((*l).name, config.name) as i32 != 0 && (*l).variant.is_null() {
                exists = true;
                break;
            } else {
                l = ((*l).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_layout as *mut rxkb_layout;
            }
        }
        if !exists {
            l = rxkb_layout_create(&raw mut (*ctx).base);
            list_init(&raw mut (*l).iso639s);
            list_init(&raw mut (*l).iso3166s);
            (*l).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*l).variant = ::core::ptr::null_mut::<i8>();
            (*l).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*l).brief =
                _steal(&raw mut config.brief as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*l).popularity = config.popularity;
            list_append(&raw mut (*ctx).layouts, &raw mut (*l).base.link);
        } else {
            config_item_free(&raw mut config);
        }
        node = (*layout).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"variantList\0".as_ptr() as *const i8) {
                parse_variant_list(ctx, l, node, popularity);
            }
            if !exists && is_node(node, b"configItem\0".as_ptr() as *const i8) as i32 != 0 {
                let mut ll: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
                ll = (*node).children as *mut xmlNode;
                while !ll.is_null() {
                    if is_node(ll, b"languageList\0".as_ptr() as *const i8) {
                        parse_language_list(ll, l);
                    }
                    if is_node(ll, b"countryList\0".as_ptr() as *const i8) {
                        parse_country_list(ll, l);
                    }
                    ll = (*ll).next as *mut xmlNode;
                }
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe fn parse_layout_list(
    mut ctx: *mut rxkb_context,
    mut layout_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*layout_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"layout\0".as_ptr() as *const i8) {
                parse_layout(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe fn parse_option(
    mut ctx: *mut rxkb_context,
    mut group: *mut rxkb_option_group,
    mut option: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<i8>(),
            description: ::core::ptr::null_mut::<i8>(),
            brief: ::core::ptr::null_mut::<i8>(),
            vendor: ::core::ptr::null_mut::<i8>(),
            popularity: popularity,
            layout_specific: false,
        };
        if parse_config_item(ctx, option, &raw mut config) {
            let mut o: *mut rxkb_option = ::core::ptr::null_mut::<rxkb_option>();
            o = ::core::ptr::null_mut::<rxkb_option>();
            o = ((*group).options.next as *mut i8).offset(-(16 as u64 as isize)) as *mut rxkb_option
                as *mut rxkb_option;
            while &raw mut (*o).base.link != &raw mut (*group).options {
                if streq((*o).name, config.name) {
                    config_item_free(&raw mut config);
                    return;
                }
                o = ((*o).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_option as *mut rxkb_option;
            }
            o = rxkb_option_create(&raw mut (*group).base);
            (*o).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*o).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*o).popularity = config.popularity;
            (*o).layout_specific = config.layout_specific;
            list_append(&raw mut (*group).options, &raw mut (*o).base.link);
        }
    }
}
unsafe fn parse_group(
    mut ctx: *mut rxkb_context,
    mut group: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut config: config_item = config_item {
            name: ::core::ptr::null_mut::<i8>(),
            description: ::core::ptr::null_mut::<i8>(),
            brief: ::core::ptr::null_mut::<i8>(),
            vendor: ::core::ptr::null_mut::<i8>(),
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
        g = ((*ctx).option_groups.next as *mut i8).offset(-(16 as u64 as isize))
            as *mut rxkb_option_group as *mut rxkb_option_group;
        while &raw mut (*g).base.link != &raw mut (*ctx).option_groups {
            if streq((*g).name, config.name) {
                exists = true;
                break;
            } else {
                g = ((*g).base.link.next as *mut i8).offset(-(16 as u64 as isize))
                    as *mut rxkb_option_group as *mut rxkb_option_group;
            }
        }
        if !exists {
            g = rxkb_option_group_create(&raw mut (*ctx).base);
            (*g).name =
                _steal(&raw mut config.name as *mut ::core::ffi::c_void) as *mut i8 as *mut i8;
            (*g).description = _steal(&raw mut config.description as *mut ::core::ffi::c_void)
                as *mut i8 as *mut i8;
            (*g).popularity = config.popularity;
            multiple = xmlGetProp(
                group,
                b"allowMultipleSelection\0".as_ptr() as *const i8 as *const xmlChar,
            );
            if !multiple.is_null()
                && xmlStrEqual(multiple, b"true\0".as_ptr() as *const i8 as *const xmlChar) != 0
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
            if is_node(node, b"option\0".as_ptr() as *const i8) {
                parse_option(ctx, g, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe fn parse_option_list(
    mut ctx: *mut rxkb_context,
    mut option_list: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*option_list).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"group\0".as_ptr() as *const i8) {
                parse_group(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe fn parse_rules_xml(
    mut ctx: *mut rxkb_context,
    mut root: *mut xmlNode,
    mut popularity: rxkb_popularity,
) {
    unsafe {
        let mut node: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        node = (*root).children as *mut xmlNode;
        while !node.is_null() {
            if is_node(node, b"modelList\0".as_ptr() as *const i8) {
                parse_model_list(ctx, node, popularity);
            } else if is_node(node, b"layoutList\0".as_ptr() as *const i8) {
                parse_layout_list(ctx, node, popularity);
            } else if is_node(node, b"optionList\0".as_ptr() as *const i8) {
                parse_option_list(ctx, node, popularity);
            }
            node = (*node).next as *mut xmlNode;
        }
    }
}
unsafe extern "C" fn xml_error_func(
    mut ctx: *mut ::core::ffi::c_void,
    mut msg: *const i8,
    mut c2rust_args: ...
) {
    unsafe {
        static mut buf: [i8; 4096] = [0; 4096];
        static mut slen: i32 = 0 as i32;
        let mut args: ::core::ffi::VaList;
        let mut rc: i32 = 0;
        args = c2rust_args.clone();
        rc = vsnprintf(
            (&raw mut buf as *mut i8).offset(slen as isize) as *mut i8,
            (::core::mem::size_of::<[i8; 4096]>() as usize).wrapping_sub(slen as usize),
            msg,
            args,
        );
        if rc < 0 as i32 {
            rxkb_logf!(
                ctx as *mut rxkb_context,
                RXKB_LOG_LEVEL_ERROR,
                "[XKB-{:03}] +++ out of cheese error. redo from start +++\n",
                XKB_ERROR_INSUFFICIENT_BUFFER_SIZE as i32,
            );
            slen = 0 as i32;
            std::ptr::write_bytes::<[i8; 4096]>(&raw mut buf as *mut i8 as *mut [i8; 4096], 0u8, 1);
            return;
        }
        slen += rc;
        if slen >= ::core::mem::size_of::<[i8; 4096]>() as i32 {
            buf[(::core::mem::size_of::<[i8; 4096]>() as usize).wrapping_sub(1 as usize)
                as usize] = '\n' as i32 as i8;
            slen = ::core::mem::size_of::<[i8; 4096]>() as i32;
        }
        if buf[(slen - 1 as i32) as usize] as i32 == '\n' as i32 {
            rxkb_logf!(
                ctx as *mut rxkb_context,
                RXKB_LOG_LEVEL_ERROR,
                "{}",
                crate::xkb::utils::CStrDisplay(&raw mut buf as *mut i8),
            );
            std::ptr::write_bytes::<[i8; 4096]>(&raw mut buf as *mut i8 as *mut [i8; 4096], 0u8, 1);
            slen = 0 as i32;
        }
    }
}
unsafe fn validate(mut ctx: *mut rxkb_context, mut doc: *mut xmlDoc) -> bool {
    unsafe {
        let mut dtd: *mut xmlDtd = ::core::ptr::null_mut::<xmlDtd>();
        let mut dtdvalid: *mut xmlValidCtxt = ::core::ptr::null_mut::<xmlValidCtxt>();
        let mut success: bool = false;
        let dtdstr: [i8; 1061] = ::core::mem::transmute::<
            [u8; 1061],
            [i8; 1061],
        >(
            *b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!ELEMENT xkbConfigRegistry (modelList?, layoutList?, optionList?)>\n<!ATTLIST xkbConfigRegistry version CDATA \"1.1\">\n<!ELEMENT modelList (model*)>\n<!ELEMENT model (configItem)>\n<!ELEMENT layoutList (layout*)>\n<!ELEMENT layout (configItem,  variantList?)>\n<!ELEMENT optionList (group*)>\n<!ELEMENT variantList (variant*)>\n<!ELEMENT variant (configItem)>\n<!ELEMENT group (configItem, option*)>\n<!ATTLIST group allowMultipleSelection (true|false) \"false\">\n<!ELEMENT option (configItem)>\n<!ELEMENT configItem (name, shortDescription?, description?, vendor?, countryList?, languageList?, hwList?)>\n<!ATTLIST configItem layout-specific (true|false) \"false\">\n<!ATTLIST configItem popularity (standard|exotic) #IMPLIED>\n<!ELEMENT name (#PCDATA)>\n<!ELEMENT shortDescription (#PCDATA)>\n<!ELEMENT description (#PCDATA)>\n<!ELEMENT vendor (#PCDATA)>\n<!ELEMENT countryList (iso3166Id+)>\n<!ELEMENT iso3166Id (#PCDATA)>\n<!ELEMENT languageList (iso639Id+)>\n<!ELEMENT iso639Id (#PCDATA)>\n<!ELEMENT hwList (hwId+)>\n<!ELEMENT hwId (#PCDATA)>\n\0",
        );
        let mut buf: xmlParserInputBufferPtr = xmlParserInputBufferCreateMem(
            &raw const dtdstr as *const i8,
            (::core::mem::size_of::<[i8; 1061]>() as usize)
                .wrapping_div(::core::mem::size_of::<i8>() as usize)
                .wrapping_sub(1 as usize) as i32,
            XML_CHAR_ENCODING_NONE,
        );
        if !buf.is_null() {
            dtd = xmlIOParseDTD(
                ::core::ptr::null_mut::<xmlSAXHandler>(),
                buf,
                XML_CHAR_ENCODING_UTF8,
            ) as *mut xmlDtd;
            if dtd.is_null() {
                rxkb_logf!(ctx, RXKB_LOG_LEVEL_ERROR, "Failed to load DTD\n",);
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
unsafe fn parse(
    mut ctx: *mut rxkb_context,
    mut path: *const i8,
    mut popularity: rxkb_popularity,
) -> bool {
    unsafe {
        let mut success: bool = false;
        let mut doc: *mut xmlDoc = ::core::ptr::null_mut::<xmlDoc>();
        let mut root: *mut xmlNode = ::core::ptr::null_mut::<xmlNode>();
        if !check_eaccess(path, R_OK) {
            return false;
        }
        xmlCheckVersion(21210 as i32);
        let mut xmlCtxt: xmlParserCtxtPtr = xmlNewParserCtxt();
        if xmlCtxt.is_null() {
            return false;
        }
        xmlCtxtUseOptions(xmlCtxt, XML_PARSE_NONET as i32);
        xmlSetGenericErrorFunc(
            ctx as *mut ::core::ffi::c_void,
            Some(
                xml_error_func
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const i8, ...) -> (),
            ),
        );
        doc = xmlCtxtReadFile(xmlCtxt, path, ::core::ptr::null::<i8>(), 0 as i32) as *mut xmlDoc;
        if !doc.is_null() {
            if !validate(ctx, doc) {
                rxkb_logf!(
                    ctx,
                    RXKB_LOG_LEVEL_ERROR,
                    "XML error: failed to validate document at {}\n",
                    crate::xkb::utils::CStrDisplay(path),
                );
            } else {
                root = xmlDocGetRootElement(doc) as *mut xmlNode;
                parse_rules_xml(ctx, root, popularity);
                success = true;
            }
            xmlFreeDoc(doc as xmlDocPtr);
        }
        xmlSetGenericErrorFunc(std::ptr::null_mut::<core::ffi::c_void>(), None);
        xmlFreeParserCtxt(xmlCtxt);
        return success;
    }
}
use crate::xkb::shared_types::*;
