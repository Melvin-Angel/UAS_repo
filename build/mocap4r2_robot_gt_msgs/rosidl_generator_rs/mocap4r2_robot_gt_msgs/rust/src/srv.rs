#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGTOrigin_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_is_origin: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub origin_pose: geometry_msgs::msg::Pose,

}



impl Default for SetGTOrigin_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGTOrigin_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetGTOrigin_Request {
  type RmwMsg = super::srv::rmw::SetGTOrigin_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_is_origin: msg.current_is_origin,
        origin_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.origin_pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      current_is_origin: msg.current_is_origin,
        origin_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.origin_pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_is_origin: msg.current_is_origin,
      origin_pose: geometry_msgs::msg::Pose::from_rmw_message(msg.origin_pose),
    }
  }
}


// Corresponds to mocap4r2_robot_gt_msgs__srv__SetGTOrigin_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetGTOrigin_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_msg: std::string::String,

}



impl Default for SetGTOrigin_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetGTOrigin_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetGTOrigin_Response {
  type RmwMsg = super::srv::rmw::SetGTOrigin_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        error_msg: msg.error_msg.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        error_msg: msg.error_msg.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      error_msg: msg.error_msg.to_string(),
    }
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


