function slBusOut = Vector3Stamped(msgIn, slBusOut, varargin)
%#codegen
%   Copyright 2021-2022 The MathWorks, Inc.
    currentlength = length(slBusOut.header);
    for iter=1:currentlength
        slBusOut.header(iter) = bus_conv_fcns.ros2.msgToBus.std_msgs.Header(msgIn.header(iter),slBusOut(1).header(iter),varargin{:});
    end
    slBusOut.header = bus_conv_fcns.ros2.msgToBus.std_msgs.Header(msgIn.header,slBusOut(1).header,varargin{:});
    currentlength = length(slBusOut.vector);
    for iter=1:currentlength
        slBusOut.vector(iter) = bus_conv_fcns.ros2.msgToBus.geometry_msgs.Vector3(msgIn.vector(iter),slBusOut(1).vector(iter),varargin{:});
    end
    slBusOut.vector = bus_conv_fcns.ros2.msgToBus.geometry_msgs.Vector3(msgIn.vector,slBusOut(1).vector,varargin{:});
end
