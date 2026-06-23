#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mocap4r2_control_msgs__msg__Control

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Control {

    // This member is not documented.
    #[allow(missing_docs)]
    pub control_type: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mocap4r2_source: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub session_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mocap4r2_systems: Vec<std::string::String>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Control::default())
  }
}

impl rosidl_runtime_rs::Message for Control {
  type RmwMsg = super::msg::rmw::Control;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        control_type: msg.control_type,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
        mocap4r2_source: msg.mocap4r2_source.as_str().into(),
        session_id: msg.session_id.as_str().into(),
        mocap4r2_systems: msg.mocap4r2_systems
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      control_type: msg.control_type,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
        mocap4r2_source: msg.mocap4r2_source.as_str().into(),
        session_id: msg.session_id.as_str().into(),
        mocap4r2_systems: msg.mocap4r2_systems
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      control_type: msg.control_type,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
      mocap4r2_source: msg.mocap4r2_source.to_string(),
      session_id: msg.session_id.to_string(),
      mocap4r2_systems: msg.mocap4r2_systems
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


// Corresponds to mocap4r2_control_msgs__msg__MocapInfo

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MocapInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mocap4r2_source: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ros_version_source: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub topics: Vec<std::string::String>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MocapInfo::default())
  }
}

impl rosidl_runtime_rs::Message for MocapInfo {
  type RmwMsg = super::msg::rmw::MocapInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mocap4r2_source: msg.mocap4r2_source.as_str().into(),
        ros_version_source: msg.ros_version_source,
        topics: msg.topics
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mocap4r2_source: msg.mocap4r2_source.as_str().into(),
      ros_version_source: msg.ros_version_source,
        topics: msg.topics
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mocap4r2_source: msg.mocap4r2_source.to_string(),
      ros_version_source: msg.ros_version_source,
      topics: msg.topics
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}


