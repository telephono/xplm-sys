/* automatically generated by rust-bindgen */

pub type XPLMDataRef = *mut ::libc::c_void;
pub type Enum_Unnamed1 = ::libc::c_uint;
pub const xplmType_Unknown: ::libc::c_uint = 0;
pub const xplmType_Int: ::libc::c_uint = 1;
pub const xplmType_Float: ::libc::c_uint = 2;
pub const xplmType_Double: ::libc::c_uint = 4;
pub const xplmType_FloatArray: ::libc::c_uint = 8;
pub const xplmType_IntArray: ::libc::c_uint = 16;
pub const xplmType_Data: ::libc::c_uint = 32;
pub type XPLMDataTypeID = ::libc::c_int;
pub type XPLMGetDatai_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void)
                              -> ::libc::c_int>;
pub type XPLMSetDatai_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               inValue: ::libc::c_int) -> ()>;
pub type XPLMGetDataf_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void)
                              -> ::libc::c_float>;
pub type XPLMSetDataf_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               inValue: ::libc::c_float)
                              -> ()>;
pub type XPLMGetDatad_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void)
                              -> ::libc::c_double>;
pub type XPLMSetDatad_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               inValue: ::libc::c_double)
                              -> ()>;
pub type XPLMGetDatavi_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               outValues: *mut ::libc::c_int,
                                               inOffset: ::libc::c_int,
                                               inMax: ::libc::c_int)
                              -> ::libc::c_int>;
pub type XPLMSetDatavi_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               inValues: *mut ::libc::c_int,
                                               inOffset: ::libc::c_int,
                                               inCount: ::libc::c_int) -> ()>;
pub type XPLMGetDatavf_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               outValues:
                                                   *mut ::libc::c_float,
                                               inOffset: ::libc::c_int,
                                               inMax: ::libc::c_int)
                              -> ::libc::c_int>;
pub type XPLMSetDatavf_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               inValues: *mut ::libc::c_float,
                                               inOffset: ::libc::c_int,
                                               inCount: ::libc::c_int) -> ()>;
pub type XPLMGetDatab_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               outValue: *mut ::libc::c_void,
                                               inOffset: ::libc::c_int,
                                               inMaxLength: ::libc::c_int)
                              -> ::libc::c_int>;
pub type XPLMSetDatab_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void,
                                               inValue: *mut ::libc::c_void,
                                               inOffset: ::libc::c_int,
                                               inLength: ::libc::c_int)
                              -> ()>;
pub type XPLMDataChanged_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void)
                              -> ()>;
extern "C" {
    pub fn XPLMFindDataRef(inDataRefName: *const ::libc::c_char)
     -> XPLMDataRef;
    pub fn XPLMCanWriteDataRef(inDataRef: XPLMDataRef) -> ::libc::c_int;
    pub fn XPLMIsDataRefGood(inDataRef: XPLMDataRef) -> ::libc::c_int;
    pub fn XPLMGetDataRefTypes(inDataRef: XPLMDataRef) -> XPLMDataTypeID;
    pub fn XPLMGetDatai(inDataRef: XPLMDataRef) -> ::libc::c_int;
    pub fn XPLMSetDatai(inDataRef: XPLMDataRef, inValue: ::libc::c_int) -> ();
    pub fn XPLMGetDataf(inDataRef: XPLMDataRef) -> ::libc::c_float;
    pub fn XPLMSetDataf(inDataRef: XPLMDataRef, inValue: ::libc::c_float)
     -> ();
    pub fn XPLMGetDatad(inDataRef: XPLMDataRef) -> ::libc::c_double;
    pub fn XPLMSetDatad(inDataRef: XPLMDataRef, inValue: ::libc::c_double)
     -> ();
    pub fn XPLMGetDatavi(inDataRef: XPLMDataRef,
                         outValues: *mut ::libc::c_int,
                         inOffset: ::libc::c_int, inMax: ::libc::c_int)
     -> ::libc::c_int;
    pub fn XPLMSetDatavi(inDataRef: XPLMDataRef, inValues: *mut ::libc::c_int,
                         inoffset: ::libc::c_int, inCount: ::libc::c_int)
     -> ();
    pub fn XPLMGetDatavf(inDataRef: XPLMDataRef,
                         outValues: *mut ::libc::c_float,
                         inOffset: ::libc::c_int, inMax: ::libc::c_int)
     -> ::libc::c_int;
    pub fn XPLMSetDatavf(inDataRef: XPLMDataRef,
                         inValues: *mut ::libc::c_float,
                         inoffset: ::libc::c_int, inCount: ::libc::c_int)
     -> ();
    pub fn XPLMGetDatab(inDataRef: XPLMDataRef, outValue: *mut ::libc::c_void,
                        inOffset: ::libc::c_int, inMaxBytes: ::libc::c_int)
     -> ::libc::c_int;
    pub fn XPLMSetDatab(inDataRef: XPLMDataRef, inValue: *mut ::libc::c_void,
                        inOffset: ::libc::c_int, inLength: ::libc::c_int)
     -> ();
    pub fn XPLMRegisterDataAccessor(inDataName: *const ::libc::c_char,
                                    inDataType: XPLMDataTypeID,
                                    inIsWritable: ::libc::c_int,
                                    inReadInt: XPLMGetDatai_f,
                                    inWriteInt: XPLMSetDatai_f,
                                    inReadFloat: XPLMGetDataf_f,
                                    inWriteFloat: XPLMSetDataf_f,
                                    inReadDouble: XPLMGetDatad_f,
                                    inWriteDouble: XPLMSetDatad_f,
                                    inReadIntArray: XPLMGetDatavi_f,
                                    inWriteIntArray: XPLMSetDatavi_f,
                                    inReadFloatArray: XPLMGetDatavf_f,
                                    inWriteFloatArray: XPLMSetDatavf_f,
                                    inReadData: XPLMGetDatab_f,
                                    inWriteData: XPLMSetDatab_f,
                                    inReadRefcon: *mut ::libc::c_void,
                                    inWriteRefcon: *mut ::libc::c_void)
     -> XPLMDataRef;
    pub fn XPLMUnregisterDataAccessor(inDataRef: XPLMDataRef) -> ();
    pub fn XPLMShareData(inDataName: *const ::libc::c_char,
                         inDataType: XPLMDataTypeID,
                         inNotificationFunc: XPLMDataChanged_f,
                         inNotificationRefcon: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn XPLMUnshareData(inDataName: *const ::libc::c_char,
                           inDataType: XPLMDataTypeID,
                           inNotificationFunc: XPLMDataChanged_f,
                           inNotificationRefcon: *mut ::libc::c_void)
     -> ::libc::c_int;
}