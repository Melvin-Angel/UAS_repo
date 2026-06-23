#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__srv__CreateRigidBody_Request() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_msgs__srv__CreateRigidBody_Request__init(msg: *mut CreateRigidBody_Request) -> bool;
    fn mocap4r2_msgs__srv__CreateRigidBody_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CreateRigidBody_Request>, size: usize) -> bool;
    fn mocap4r2_msgs__srv__CreateRigidBody_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CreateRigidBody_Request>);
    fn mocap4r2_msgs__srv__CreateRigidBody_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CreateRigidBody_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CreateRigidBody_Request>) -> bool;
}

// Corresponds to mocap4r2_msgs__srv__CreateRigidBody_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CreateRigidBody_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rigid_body_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub link_parent: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: rosidl_runtime_rs::Sequence<i32>,

}



impl Default for CreateRigidBody_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_msgs__srv__CreateRigidBody_Request__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_msgs__srv__CreateRigidBody_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CreateRigidBody_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__srv__CreateRigidBody_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__srv__CreateRigidBody_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__srv__CreateRigidBody_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CreateRigidBody_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CreateRigidBody_Request where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_msgs/srv/CreateRigidBody_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__srv__CreateRigidBody_Request() }
  }
}


#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__srv__CreateRigidBody_Response() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_msgs__srv__CreateRigidBody_Response__init(msg: *mut CreateRigidBody_Response) -> bool;
    fn mocap4r2_msgs__srv__CreateRigidBody_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CreateRigidBody_Response>, size: usize) -> bool;
    fn mocap4r2_msgs__srv__CreateRigidBody_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CreateRigidBody_Response>);
    fn mocap4r2_msgs__srv__CreateRigidBody_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CreateRigidBody_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CreateRigidBody_Response>) -> bool;
}

// Corresponds to mocap4r2_msgs__srv__CreateRigidBody_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CreateRigidBody_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for CreateRigidBody_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_msgs__srv__CreateRigidBody_Response__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_msgs__srv__CreateRigidBody_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CreateRigidBody_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__srv__CreateRigidBody_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__srv__CreateRigidBody_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__srv__CreateRigidBody_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CreateRigidBody_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CreateRigidBody_Response where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_msgs/srv/CreateRigidBody_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__srv__CreateRigidBody_Response() }
  }
}






#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_msgs__srv__CreateRigidBody() -> *const std::ffi::c_void;
}

// Corresponds to mocap4r2_msgs__srv__CreateRigidBody
#[allow(missing_docs, non_camel_case_types)]
pub struct CreateRigidBody;

impl rosidl_runtime_rs::Service for CreateRigidBody {
    type Request = CreateRigidBody_Request;
    type Response = CreateRigidBody_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__mocap4r2_msgs__srv__CreateRigidBody() }
    }
}


