#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMarkerColor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std_msgs::msg::Int32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub color: std_msgs::msg::ColorRGBA,

}



impl Default for SetMarkerColor_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetMarkerColor_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SetMarkerColor_Request {
  type RmwMsg = super::srv::rmw::SetMarkerColor_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: std_msgs::msg::Int32::into_rmw_message(std::borrow::Cow::Owned(msg.id)).into_owned(),
        color: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Owned(msg.color)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: std_msgs::msg::Int32::into_rmw_message(std::borrow::Cow::Borrowed(&msg.id)).into_owned(),
        color: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Borrowed(&msg.color)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: std_msgs::msg::Int32::from_rmw_message(msg.id),
      color: std_msgs::msg::ColorRGBA::from_rmw_message(msg.color),
    }
  }
}


// Corresponds to mocap4r2_marker_viz_srvs__srv__SetMarkerColor_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetMarkerColor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for SetMarkerColor_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SetMarkerColor_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SetMarkerColor_Response {
  type RmwMsg = super::srv::rmw::SetMarkerColor_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetMarkerColor_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std_msgs::msg::Int32,

}



impl Default for ResetMarkerColor_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ResetMarkerColor_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ResetMarkerColor_Request {
  type RmwMsg = super::srv::rmw::ResetMarkerColor_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: std_msgs::msg::Int32::into_rmw_message(std::borrow::Cow::Owned(msg.id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: std_msgs::msg::Int32::into_rmw_message(std::borrow::Cow::Borrowed(&msg.id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: std_msgs::msg::Int32::from_rmw_message(msg.id),
    }
  }
}


// Corresponds to mocap4r2_marker_viz_srvs__srv__ResetMarkerColor_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetMarkerColor_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ResetMarkerColor_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ResetMarkerColor_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ResetMarkerColor_Response {
  type RmwMsg = super::srv::rmw::ResetMarkerColor_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
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


