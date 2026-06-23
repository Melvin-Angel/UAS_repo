function rosmsgOut = Joy(slBusIn, rosmsgOut)
%#codegen
%   Copyright 2021 The MathWorks, Inc.
    rosmsgOut.header = bus_conv_fcns.ros2.busToMsg.std_msgs.Header(slBusIn.header,rosmsgOut.header(1));
    rosmsgOut.axes = single(slBusIn.axes(1:slBusIn.axes_SL_Info.CurrentLength));
    rosmsgOut.buttons = int32(slBusIn.buttons(1:slBusIn.buttons_SL_Info.CurrentLength));
end
