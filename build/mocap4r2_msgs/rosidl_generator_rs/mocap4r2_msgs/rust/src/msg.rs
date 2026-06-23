#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to mocap4r2_msgs__msg__Marker

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub marker_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub translation: geometry_msgs::msg::Point,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Marker::default())
  }
}

impl rosidl_runtime_rs::Message for Marker {
  type RmwMsg = super::msg::rmw::Marker;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id_type: msg.id_type,
        marker_index: msg.marker_index,
        marker_name: msg.marker_name.as_str().into(),
        translation: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Owned(msg.translation)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id_type: msg.id_type,
      marker_index: msg.marker_index,
        marker_name: msg.marker_name.as_str().into(),
        translation: geometry_msgs::msg::Point::into_rmw_message(std::borrow::Cow::Borrowed(&msg.translation)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id_type: msg.id_type,
      marker_index: msg.marker_index,
      marker_name: msg.marker_name.to_string(),
      translation: geometry_msgs::msg::Point::from_rmw_message(msg.translation),
    }
  }
}


// Corresponds to mocap4r2_msgs__msg__Markers

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Markers {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_number: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: Vec<super::msg::Marker>,

}



impl Default for Markers {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Markers::default())
  }
}

impl rosidl_runtime_rs::Message for Markers {
  type RmwMsg = super::msg::rmw::Markers;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        frame_number: msg.frame_number,
        markers: msg.markers
          .into_iter()
          .map(|elem| super::msg::Marker::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      frame_number: msg.frame_number,
        markers: msg.markers
          .iter()
          .map(|elem| super::msg::Marker::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      frame_number: msg.frame_number,
      markers: msg.markers
          .into_iter()
          .map(super::msg::Marker::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to mocap4r2_msgs__msg__ImusInfo

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ImusInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sensor_ids: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub battery_level: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: f32,

}



impl Default for ImusInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ImusInfo::default())
  }
}

impl rosidl_runtime_rs::Message for ImusInfo {
  type RmwMsg = super::msg::rmw::ImusInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sensor_ids: msg.sensor_ids
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        battery_level: msg.battery_level,
        temperature: msg.temperature,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sensor_ids: msg.sensor_ids
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      battery_level: msg.battery_level,
      temperature: msg.temperature,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      sensor_ids: msg.sensor_ids
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      battery_level: msg.battery_level,
      temperature: msg.temperature,
    }
  }
}


// Corresponds to mocap4r2_msgs__msg__RigidBody

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RigidBody {

    // This member is not documented.
    #[allow(missing_docs)]
    pub rigid_body_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub markers: Vec<super::msg::Marker>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::Pose,

}



impl Default for RigidBody {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RigidBody::default())
  }
}

impl rosidl_runtime_rs::Message for RigidBody {
  type RmwMsg = super::msg::rmw::RigidBody;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rigid_body_name: msg.rigid_body_name.as_str().into(),
        markers: msg.markers
          .into_iter()
          .map(|elem| super::msg::Marker::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        rigid_body_name: msg.rigid_body_name.as_str().into(),
        markers: msg.markers
          .iter()
          .map(|elem| super::msg::Marker::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      rigid_body_name: msg.rigid_body_name.to_string(),
      markers: msg.markers
          .into_iter()
          .map(super::msg::Marker::from_rmw_message)
          .collect(),
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
    }
  }
}


// Corresponds to mocap4r2_msgs__msg__RigidBodies

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RigidBodies {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frame_number: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rigidbodies: Vec<super::msg::RigidBody>,

}



impl Default for RigidBodies {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RigidBodies::default())
  }
}

impl rosidl_runtime_rs::Message for RigidBodies {
  type RmwMsg = super::msg::rmw::RigidBodies;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        frame_number: msg.frame_number,
        rigidbodies: msg.rigidbodies
          .into_iter()
          .map(|elem| super::msg::RigidBody::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      frame_number: msg.frame_number,
        rigidbodies: msg.rigidbodies
          .iter()
          .map(|elem| super::msg::RigidBody::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      frame_number: msg.frame_number,
      rigidbodies: msg.rigidbodies
          .into_iter()
          .map(super::msg::RigidBody::from_rmw_message)
          .collect(),
    }
  }
}


