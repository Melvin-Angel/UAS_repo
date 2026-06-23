#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "mocap4r2_robot_gt_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_robot_gt_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__init(msg: *mut SetGTOrigin_Request) -> bool;
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGTOrigin_Request>, size: usize) -> bool;
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGTOrigin_Request>);
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGTOrigin_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGTOrigin_Request>) -> bool;
}

// Corresponds to mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGTOrigin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_is_origin: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub origin_pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for SetGTOrigin_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGTOrigin_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGTOrigin_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGTOrigin_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_robot_gt_msgs/srv/SetGTOrigin_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request() }
  }
}


#[link(name = "mocap4r2_robot_gt_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_robot_gt_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__init(msg: *mut SetGTOrigin_Response) -> bool;
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SetGTOrigin_Response>, size: usize) -> bool;
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SetGTOrigin_Response>);
    fn mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SetGTOrigin_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SetGTOrigin_Response>) -> bool;
}

// Corresponds to mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGTOrigin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_msg: rosidl_runtime_rs::String,

}



impl Default for SetGTOrigin_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SetGTOrigin_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SetGTOrigin_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SetGTOrigin_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_robot_gt_msgs/srv/SetGTOrigin_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response() }
  }
}






#[link(name = "mocap4r2_robot_gt_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_robot_gt_msgs__srv__SetGTOrigin() -> *const std::ffi::c_void;
}

// Corresponds to mocap4r2_robot_gt_msgs__srv__SetGTOrigin
#[allow(missing_docs, non_camel_case_types)]
pub struct SetGTOrigin;

impl rosidl_runtime_rs::Service for SetGTOrigin {
    type Request = SetGTOrigin_Request;
    type Response = SetGTOrigin_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_robot_gt_msgs__srv__SetGTOrigin() }
    }
}


