classdef PreprocessSystem < matlab.System & matlab.system.mixin.Propagates
    % PreprocessSystem  Low-pass filter for position, derivative-based velocity estimate, and yaw extraction
    %
    % Inputs:
    %   raw_pos  (3x1)             -- position [m]
    %   raw_att  (3x1 [r p y] or 4x1 quaternion [w x y z])
    %
    % Outputs:
    %   fpos (3x1)                 -- filtered position
    %   fvel (3x1)                 -- filtered velocity estimate
    %   yaw  (1x1)                 -- yaw (rad)
    
    properties (Access = private)
        % Timing (set this to your ROS message rate)
        dt = 0.04;           % seconds, e.g. 0.02 for 50 Hz
        
        % Filter design
        fc_vel = 5.0;        % cutoff frequency for velocity filter (Hz)
        alpha_pos = 0.8;     % smoothing for position (0..1)
        alpha_vel = [];      % computed from fc_vel and dt if empty
        
        % Internal states (pre-initialized for codegen to fixed-size values)
        prev_pos  = zeros(3,1);
        prev_fpos = zeros(3,1);
        prev_fvel = zeros(3,1);
    end
    
    methods
        function obj = PreprocessSystem(varargin)
            % Allow override of dt and fc_vel by name-value if desired
            if nargin > 0
                for k=1:2:length(varargin)
                    if strcmpi(varargin{k}, 'dt'), obj.dt = varargin{k+1}; end
                    if strcmpi(varargin{k}, 'fc_vel'), obj.fc_vel = varargin{k+1}; end
                    if strcmpi(varargin{k}, 'alpha_pos'), obj.alpha_pos = varargin{k+1}; end
                end
            end
            % Note: compute alpha_vel in setupImpl for codegen friendliness
        end
    end
    
    methods (Access = protected)
        function setupImpl(obj)
            % Ensure alpha_vel is computed deterministically here for codegen
            if isempty(obj.alpha_vel)
                tau = 1 / (2*pi*obj.fc_vel);
                obj.alpha_vel = obj.dt / (tau + obj.dt);
                % alternative equivalent: alpha = (2*pi*fc*dt) / (1 + 2*pi*fc*dt)
            end
            % Initialize internal states to fixed-size values (already set in properties,
            % but keep this to be explicit in setup)
            obj.prev_pos  = zeros(3,1);
            obj.prev_fpos = zeros(3,1);
            obj.prev_fvel = zeros(3,1);
        end
        
        function [fpos, fvel, yaw] = stepImpl(obj, raw_pos, raw_att)
            % Ensure column vectors
            raw_pos = raw_pos(:);
            
            % Defensive first-call initialization:
            % If prev_pos is empty (shouldn't be when preinitialized) OR it is
            % all zeros and we get a non-zero raw_pos, initialize to raw_pos to
            % avoid large transient derivative estimates.
            if isempty(obj.prev_pos) || (all(obj.prev_pos == 0) && any(raw_pos ~= 0))
                obj.prev_pos  = raw_pos;
                obj.prev_fpos = raw_pos;
                obj.prev_fvel = zeros(3,1);
            end
            
            % 1) Position low-pass (exponential smoothing)
            fpos = obj.alpha_pos * raw_pos + (1 - obj.alpha_pos) * obj.prev_fpos;
            
            % 2) Velocity estimate: backward difference then low-pass
            raw_vel_est = (raw_pos - obj.prev_pos) / obj.dt;
            fvel = obj.alpha_vel * raw_vel_est + (1 - obj.alpha_vel) * obj.prev_fvel;
            
            % 3) Update internal states
            obj.prev_pos = raw_pos;
            obj.prev_fpos = fpos;
            obj.prev_fvel = fvel;
            
            % 4) Extract yaw from attitude input
            if numel(raw_att) == 3
                % roll, pitch, yaw vector
                yaw = raw_att(3);
            elseif numel(raw_att) == 4
                % quaternion [w x y z]
                q = raw_att(:);
                w = q(1); x = q(2); yq = q(3); z = q(4);
                % yaw extraction (ZYX convention)
                yaw = atan2(2*(w*z + x*yq), 1 - 2*(yq^2 + z^2));
            else
                yaw = 0;
            end
        end
        
        function resetImpl(obj)
            obj.prev_pos = zeros(3,1);
            obj.prev_fpos = zeros(3,1);
            obj.prev_fvel = zeros(3,1);
        end
        
        % Propagation helpers for Simulink
        function num = getNumInputsImpl(~), num = 2; end
        function num = getNumOutputsImpl(~), num = 3; end
        function [out1,out2,out3] = getOutputSizeImpl(~)
            out1 = [3 1]; out2 = [3 1]; out3 = [1 1];
        end
        function [out1,out2,out3] = getOutputDataTypeImpl(~)
            out1 = 'double'; out2 = 'double'; out3 = 'double';
        end
        function [o1,o2,o3] = isOutputComplexImpl(~), o1=false; o2=false; o3=false; end
        function [o1,o2,o3] = isOutputFixedSizeImpl(~), o1=true; o2=true; o3=true; end
        
        % (Optional) Explicit input propagation to avoid inference issues
        function [in1,in2] = getInputSizeImpl(~)
            in1 = [3 1]; in2 = [4 1]; % allow 3x1 or 4x1; Simulink will check actual sizes
        end
        function [in1,in2] = getInputDataTypeImpl(~)
            in1 = 'double'; in2 = 'double';
        end
        function [in1,in2] = isInputComplexImpl(~), in1=false; in2=false; end
        function [in1,in2] = isInputFixedSizeImpl(~), in1=true; in2=false; end
    end
end
