#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__Marker() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_msgs__msg__Marker__init(msg: *mut Marker) -> bool;
    fn mocap4r2_msgs__msg__Marker__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Marker>, size: usize) -> bool;
    fn mocap4r2_msgs__msg__Marker__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Marker>);
    fn mocap4r2_msgs__msg__Marker__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Marker>, out_seq: *mut rosidl_runtime_rs::Sequence<Marker>) -> bool;
}

// Corresponds to mocap4r2_msgs__msg__Marker
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Marker {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id_type: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub marker_index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub marker_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub translation: geometry_msgs::msg::rmw::Point,

}

impl Marker {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USE_NAME: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USE_INDEX: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const USE_BOTH: i8 = 2;

}


impl Default for Marker {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_msgs__msg__Marker__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_msgs__msg__Marker__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Marker {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__Marker__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__Marker__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__Marker__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Marker {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Marker where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_msgs/msg/Marker";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__Marker() }
  }
}


#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__Markers() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_msgs__msg__Markers__init(msg: *mut Markers) -> bool;
    fn mocap4r2_msgs__msg__Markers__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Markers>, size: usize) -> bool;
    fn mocap4r2_msgs__msg__Markers__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Markers>);
    fn mocap4r2_msgs__msg__Markers__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Markers>, out_seq: *mut rosidl_runtime_rs::Sequence<Markers>) -> bool;
}

// Corresponds to mocap4r2_msgs__msg__Markers
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Markers {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_number: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Marker>,

}



impl Default for Markers {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_msgs__msg__Markers__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_msgs__msg__Markers__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Markers {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__Markers__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__Markers__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__Markers__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Markers {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Markers where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_msgs/msg/Markers";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__Markers() }
  }
}


#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__ImusInfo() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_msgs__msg__ImusInfo__init(msg: *mut ImusInfo) -> bool;
    fn mocap4r2_msgs__msg__ImusInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ImusInfo>, size: usize) -> bool;
    fn mocap4r2_msgs__msg__ImusInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ImusInfo>);
    fn mocap4r2_msgs__msg__ImusInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ImusInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<ImusInfo>) -> bool;
}

// Corresponds to mocap4r2_msgs__msg__ImusInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ImusInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sensor_ids: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_level: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: f32,

}



impl Default for ImusInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_msgs__msg__ImusInfo__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_msgs__msg__ImusInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ImusInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__ImusInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__ImusInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__ImusInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ImusInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ImusInfo where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_msgs/msg/ImusInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__ImusInfo() }
  }
}


#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__RigidBody() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_msgs__msg__RigidBody__init(msg: *mut RigidBody) -> bool;
    fn mocap4r2_msgs__msg__RigidBody__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RigidBody>, size: usize) -> bool;
    fn mocap4r2_msgs__msg__RigidBody__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RigidBody>);
    fn mocap4r2_msgs__msg__RigidBody__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RigidBody>, out_seq: *mut rosidl_runtime_rs::Sequence<RigidBody>) -> bool;
}

// Corresponds to mocap4r2_msgs__msg__RigidBody
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RigidBody {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rigid_body_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Marker>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for RigidBody {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_msgs__msg__RigidBody__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_msgs__msg__RigidBody__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RigidBody {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__RigidBody__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__RigidBody__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__RigidBody__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RigidBody {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RigidBody where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_msgs/msg/RigidBody";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__RigidBody() }
  }
}


#[link(name = "mocap4r2_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__RigidBodies() -> *const std::ffi::c_void;
}

#[link(name = "mocap4r2_msgs__rosidl_generator_c")]
extern "C" {
    fn mocap4r2_msgs__msg__RigidBodies__init(msg: *mut RigidBodies) -> bool;
    fn mocap4r2_msgs__msg__RigidBodies__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RigidBodies>, size: usize) -> bool;
    fn mocap4r2_msgs__msg__RigidBodies__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RigidBodies>);
    fn mocap4r2_msgs__msg__RigidBodies__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RigidBodies>, out_seq: *mut rosidl_runtime_rs::Sequence<RigidBodies>) -> bool;
}

// Corresponds to mocap4r2_msgs__msg__RigidBodies
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RigidBodies {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_number: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rigidbodies: rosidl_runtime_rs::Sequence<super::super::msg::rmw::RigidBody>,

}



impl Default for RigidBodies {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !mocap4r2_msgs__msg__RigidBodies__init(&mut msg as *mut _) {
        panic!("Call to mocap4r2_msgs__msg__RigidBodies__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RigidBodies {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__RigidBodies__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__RigidBodies__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { mocap4r2_msgs__msg__RigidBodies__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RigidBodies {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RigidBodies where Self: Sized {
  const TYPE_NAME: &'static str = "mocap4r2_msgs/msg/RigidBodies";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__mocap4r2_msgs__msg__RigidBodies() }
  }
}


