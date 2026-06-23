#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mocap4r2_control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_control_msgs__msg__Control() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_control_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_control_msgs__msg__Control__init(msg: *mut Control) -> bool;
    fn mocap4r2_control_msgs__msg__Control__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Control>, size: usize) -> bool;
    fn mocap4r2_control_msgs__msg__Control__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Control>);
    fn mocap4r2_control_msgs__msg__Control__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Control>, out_seq: *mut rosidl_runtime_rs::Sequence<Control>) -> bool;
}

// Corresponds to mocap4r2_control_msgs__msg__Control
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Control {

    // This member is not documented.
    #[allow(missing_docs)]
    pub control_type: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mocap4r2_source: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub session_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mocap4r2_systems: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}

impl Control {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const START: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ACK_START: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STOP: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ACK_STOP: i8 = 3;

}


impl Default for Control {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_control_msgs__msg__Control__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_control_msgs__msg__Control__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Control {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_control_msgs__msg__Control__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_control_msgs__msg__Control__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_control_msgs__msg__Control__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Control {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Control where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_control_msgs/msg/Control";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_control_msgs__msg__Control() }
  }
}


#[link(name = "mocap4r2_control_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_control_msgs__msg__MocapInfo() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_control_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_control_msgs__msg__MocapInfo__init(msg: *mut MocapInfo) -> bool;
    fn mocap4r2_control_msgs__msg__MocapInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MocapInfo>, size: usize) -> bool;
    fn mocap4r2_control_msgs__msg__MocapInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MocapInfo>);
    fn mocap4r2_control_msgs__msg__MocapInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MocapInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<MocapInfo>) -> bool;
}

// Corresponds to mocap4r2_control_msgs__msg__MocapInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MocapInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mocap4r2_source: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ros_version_source: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub topics: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}

impl MocapInfo {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROS2: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROS1: i8 = 1;

}


impl Default for MocapInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_control_msgs__msg__MocapInfo__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_control_msgs__msg__MocapInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MocapInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_control_msgs__msg__MocapInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_control_msgs__msg__MocapInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_control_msgs__msg__MocapInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MocapInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MocapInfo where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_control_msgs/msg/MocapInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_control_msgs__msg__MocapInfo() }
  }
}


