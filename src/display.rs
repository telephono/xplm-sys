/* automatically generated by rust-bindgen */

use defs::{XPLMKeyFlags, XPLMPluginID};

pub type Enum_Unnamed1 = ::libc::c_uint;
pub const xplm_Phase_FirstScene: ::libc::c_uint = 0;
pub const xplm_Phase_Terrain: ::libc::c_uint = 5;
pub const xplm_Phase_Airports: ::libc::c_uint = 10;
pub const xplm_Phase_Vectors: ::libc::c_uint = 15;
pub const xplm_Phase_Objects: ::libc::c_uint = 20;
pub const xplm_Phase_Airplanes: ::libc::c_uint = 25;
pub const xplm_Phase_LastScene: ::libc::c_uint = 30;
pub const xplm_Phase_FirstCockpit: ::libc::c_uint = 35;
pub const xplm_Phase_Panel: ::libc::c_uint = 40;
pub const xplm_Phase_Gauges: ::libc::c_uint = 45;
pub const xplm_Phase_Window: ::libc::c_uint = 50;
pub const xplm_Phase_LastCockpit: ::libc::c_uint = 55;
pub const xplm_Phase_LocalMap3D: ::libc::c_uint = 100;
pub const xplm_Phase_LocalMap2D: ::libc::c_uint = 101;
pub const xplm_Phase_LocalMapProfile: ::libc::c_uint = 102;
pub type XPLMDrawingPhase = ::libc::c_int;
pub type XPLMDrawCallback_f =
    ::std::option::Option<unsafe extern "C" fn(inPhase: XPLMDrawingPhase,
                                               inIsBefore: ::libc::c_int,
                                               inRefcon: *mut ::libc::c_void)
                              -> ::libc::c_int>;
pub type XPLMKeySniffer_f =
    ::std::option::Option<unsafe extern "C" fn(inChar: ::libc::c_char,
                                               inFlags: XPLMKeyFlags,
                                               inVirtualKey: ::libc::c_char,
                                               inRefcon: *mut ::libc::c_void)
                              -> ::libc::c_int>;
pub type Enum_Unnamed2 = ::libc::c_uint;
pub const xplm_MouseDown: ::libc::c_uint = 1;
pub const xplm_MouseDrag: ::libc::c_uint = 2;
pub const xplm_MouseUp: ::libc::c_uint = 3;
pub type XPLMMouseStatus = ::libc::c_int;
pub type Enum_Unnamed3 = ::libc::c_uint;
pub const xplm_CursorDefault: ::libc::c_uint = 0;
pub const xplm_CursorHidden: ::libc::c_uint = 1;
pub const xplm_CursorArrow: ::libc::c_uint = 2;
pub const xplm_CursorCustom: ::libc::c_uint = 3;
pub type XPLMCursorStatus = ::libc::c_int;
pub type XPLMWindowID = *mut ::libc::c_void;
pub type XPLMDrawWindow_f =
    ::std::option::Option<unsafe extern "C" fn(inWindowID: XPLMWindowID,
                                               inRefcon: *mut ::libc::c_void)
                              -> ()>;
pub type XPLMHandleKey_f =
    ::std::option::Option<unsafe extern "C" fn(inWindowID: XPLMWindowID,
                                               inKey: ::libc::c_char,
                                               inFlags: XPLMKeyFlags,
                                               inVirtualKey: ::libc::c_char,
                                               inRefcon: *mut ::libc::c_void,
                                               losingFocus: ::libc::c_int)
                              -> ()>;
pub type XPLMHandleMouseClick_f =
    ::std::option::Option<unsafe extern "C" fn(inWindowID: XPLMWindowID,
                                               x: ::libc::c_int,
                                               y: ::libc::c_int,
                                               inMouse: XPLMMouseStatus,
                                               inRefcon: *mut ::libc::c_void)
                              -> ::libc::c_int>;
pub type XPLMHandleCursor_f =
    ::std::option::Option<unsafe extern "C" fn(inWindowID: XPLMWindowID,
                                               x: ::libc::c_int,
                                               y: ::libc::c_int,
                                               inRefcon: *mut ::libc::c_void)
                              -> XPLMCursorStatus>;
pub type XPLMHandleMouseWheel_f =
    ::std::option::Option<unsafe extern "C" fn(inWindowID: XPLMWindowID,
                                               x: ::libc::c_int,
                                               y: ::libc::c_int,
                                               wheel: ::libc::c_int,
                                               clicks: ::libc::c_int,
                                               inRefcon: *mut ::libc::c_void)
                              -> ::libc::c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub structSize: ::libc::c_int,
    pub left: ::libc::c_int,
    pub top: ::libc::c_int,
    pub right: ::libc::c_int,
    pub bottom: ::libc::c_int,
    pub visible: ::libc::c_int,
    pub drawWindowFunc: XPLMDrawWindow_f,
    pub handleMouseClickFunc: XPLMHandleMouseClick_f,
    pub handleKeyFunc: XPLMHandleKey_f,
    pub handleCursorFunc: XPLMHandleCursor_f,
    pub handleMouseWheelFunc: XPLMHandleMouseWheel_f,
    pub refcon: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XPLMCreateWindow_t = Struct_Unnamed4;
pub type XPLMHotKey_f =
    ::std::option::Option<unsafe extern "C" fn(inRefcon: *mut ::libc::c_void)
                              -> ()>;
pub type XPLMHotKeyID = *mut ::libc::c_void;
extern "C" {
    pub fn XPLMRegisterDrawCallback(inCallback: XPLMDrawCallback_f,
                                    inPhase: XPLMDrawingPhase,
                                    inWantsBefore: ::libc::c_int,
                                    inRefcon: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn XPLMUnregisterDrawCallback(inCallback: XPLMDrawCallback_f,
                                      inPhase: XPLMDrawingPhase,
                                      inWantsBefore: ::libc::c_int,
                                      inRefcon: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn XPLMRegisterKeySniffer(inCallback: XPLMKeySniffer_f,
                                  inBeforeWindows: ::libc::c_int,
                                  inRefcon: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn XPLMUnregisterKeySniffer(inCallback: XPLMKeySniffer_f,
                                    inBeforeWindows: ::libc::c_int,
                                    inRefcon: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn XPLMGetScreenSize(outWidth: *mut ::libc::c_int,
                             outHeight: *mut ::libc::c_int) -> ();
    pub fn XPLMGetMouseLocation(outX: *mut ::libc::c_int,
                                outY: *mut ::libc::c_int) -> ();
    pub fn XPLMCreateWindow(inLeft: ::libc::c_int, inTop: ::libc::c_int,
                            inRight: ::libc::c_int, inBottom: ::libc::c_int,
                            inIsVisible: ::libc::c_int,
                            inDrawCallback: XPLMDrawWindow_f,
                            inKeyCallback: XPLMHandleKey_f,
                            inMouseCallback: XPLMHandleMouseClick_f,
                            inRefcon: *mut ::libc::c_void) -> XPLMWindowID;
    pub fn XPLMCreateWindowEx(inParams: *mut XPLMCreateWindow_t)
     -> XPLMWindowID;
    pub fn XPLMDestroyWindow(inWindowID: XPLMWindowID) -> ();
    pub fn XPLMGetWindowGeometry(inWindowID: XPLMWindowID,
                                 outLeft: *mut ::libc::c_int,
                                 outTop: *mut ::libc::c_int,
                                 outRight: *mut ::libc::c_int,
                                 outBottom: *mut ::libc::c_int) -> ();
    pub fn XPLMSetWindowGeometry(inWindowID: XPLMWindowID,
                                 inLeft: ::libc::c_int, inTop: ::libc::c_int,
                                 inRight: ::libc::c_int,
                                 inBottom: ::libc::c_int) -> ();
    pub fn XPLMGetWindowIsVisible(inWindowID: XPLMWindowID) -> ::libc::c_int;
    pub fn XPLMSetWindowIsVisible(inWindowID: XPLMWindowID,
                                  inIsVisible: ::libc::c_int) -> ();
    pub fn XPLMGetWindowRefCon(inWindowID: XPLMWindowID)
     -> *mut ::libc::c_void;
    pub fn XPLMSetWindowRefCon(inWindowID: XPLMWindowID,
                               inRefcon: *mut ::libc::c_void) -> ();
    pub fn XPLMTakeKeyboardFocus(inWindow: XPLMWindowID) -> ();
    pub fn XPLMBringWindowToFront(inWindow: XPLMWindowID) -> ();
    pub fn XPLMIsWindowInFront(inWindow: XPLMWindowID) -> ::libc::c_int;
    pub fn XPLMRegisterHotKey(inVirtualKey: ::libc::c_char,
                              inFlags: XPLMKeyFlags,
                              inDescription: *const ::libc::c_char,
                              inCallback: XPLMHotKey_f,
                              inRefcon: *mut ::libc::c_void) -> XPLMHotKeyID;
    pub fn XPLMUnregisterHotKey(inHotKey: XPLMHotKeyID) -> ();
    pub fn XPLMCountHotKeys() -> ::libc::c_int;
    pub fn XPLMGetNthHotKey(inIndex: ::libc::c_int) -> XPLMHotKeyID;
    pub fn XPLMGetHotKeyInfo(inHotKey: XPLMHotKeyID,
                             outVirtualKey: *mut ::libc::c_char,
                             outFlags: *mut XPLMKeyFlags,
                             outDescription: *mut ::libc::c_char,
                             outPlugin: *mut XPLMPluginID) -> ();
    pub fn XPLMSetHotKeyCombination(inHotKey: XPLMHotKeyID,
                                    inVirtualKey: ::libc::c_char,
                                    inFlags: XPLMKeyFlags) -> ();
}