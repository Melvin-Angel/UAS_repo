#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "mocap4r2_marker_viz_srvs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_marker_viz_srvs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__init(msg: *mut SetMarkerColor_Request) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMarkerColor_Request>, size: usize) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMarkerColor_Request>);
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMarkerColor_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMarkerColor_Request>) -> bool;
}

// Corresponds to mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMarkerColor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std_msgs::msg::rmw::Int32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub color: std_msgs::msg::rmw::ColorRGBA,

}



impl Default for SetMarkerColor_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMarkerColor_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMarkerColor_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMarkerColor_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_marker_viz_srvs/srv/SetMarkerColor_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request() }
  }
}


#[link(name = "mocap4r2_marker_viz_srvs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_marker_viz_srvs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__init(msg: *mut SetMarkerColor_Response) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetMarkerColor_Response>, size: usize) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetMarkerColor_Response>);
    fn mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetMarkerColor_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetMarkerColor_Response>) -> bool;
}

// Corresponds to mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMarkerColor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetMarkerColor_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetMarkerColor_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetMarkerColor_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetMarkerColor_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_marker_viz_srvs/srv/SetMarkerColor_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response() }
  }
}


#[link(name = "mocap4r2_marker_viz_srvs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_marker_viz_srvs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__init(msg: *mut ResetMarkerColor_Request) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetMarkerColor_Request>, size: usize) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetMarkerColor_Request>);
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetMarkerColor_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetMarkerColor_Request>) -> bool;
}

// Corresponds to mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetMarkerColor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std_msgs::msg::rmw::Int32,

}



impl Default for ResetMarkerColor_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetMarkerColor_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetMarkerColor_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetMarkerColor_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_marker_viz_srvs/srv/ResetMarkerColor_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request() }
  }
}


#[link(name = "mocap4r2_marker_viz_srvs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_marker_viz_srvs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__init(msg: *mut ResetMarkerColor_Response) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetMarkerColor_Response>, size: usize) -> bool;
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetMarkerColor_Response>);
    fn mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetMarkerColor_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetMarkerColor_Response>) -> bool;
}

// Corresponds to mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetMarkerColor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ResetMarkerColor_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetMarkerColor_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetMarkerColor_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetMarkerColor_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_marker_viz_srvs/srv/ResetMarkerColor_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response() }
  }
}






#[link(name = "mocap4r2_marker_viz_srvs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_marker_viz_srvs__srv__SetMarkerColor() -> *const std::ffi::c_void;
}

// Corresponds to mocap4r2_marker_viz_srvs__srv__SetMarkerColor
#[allow(missing_docs, non_camel_case_types)]
pub struct SetMarkerColor;

impl rosidl_runtime_rs::Service for SetMarkerColor {
    type Request = SetMarkerColor_Request;
    type Response = SetMarkerColor_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_marker_viz_srvs__srv__SetMarkerColor() }
    }
}




#[link(name = "mocap4r2_marker_viz_srvs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_marker_viz_srvs__srv__ResetMarkerColor() -> *const std::ffi::c_void;
}

// Corresponds to mocap4r2_marker_viz_srvs__srv__ResetMarkerColor
#[allow(missing_docs, non_camel_case_types)]
pub struct ResetMarkerColor;

impl rosidl_runtime_rs::Service for ResetMarkerColor {
    type Request = ResetMarkerColor_Request;
    type Response = ResetMarkerColor_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_marker_viz_srvs__srv__ResetMarkerColor() }
    }
}


