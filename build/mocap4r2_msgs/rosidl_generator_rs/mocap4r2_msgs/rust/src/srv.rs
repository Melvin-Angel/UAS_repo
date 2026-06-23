#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to mocap4r2_msgs__srv__CreateRigidBody_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CreateRigidBody_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rigid_body_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub link_parent: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: Vec<i32>,

}



impl Default for CreateRigidBody_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CreateRigidBody_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CreateRigidBody_Request {
  type RmwMsg = super::srv::rmw::CreateRigidBody_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rigid_body_name: msg.rigid_body_name.as_str().into(),
        link_parent: msg.link_parent.as_str().into(),
        markers: msg.markers.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rigid_body_name: msg.rigid_body_name.as_str().into(),
        link_parent: msg.link_parent.as_str().into(),
        markers: msg.markers.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      rigid_body_name: msg.rigid_body_name.to_string(),
      link_parent: msg.link_parent.to_string(),
      markers: msg.markers
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to mocap4r2_msgs__srv__CreateRigidBody_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CreateRigidBody_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for CreateRigidBody_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CreateRigidBody_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CreateRigidBody_Response {
  type RmwMsg = super::srv::rmw::CreateRigidBody_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
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


