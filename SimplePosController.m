classdef SimplePosController < matlab.System & matlab.system.mixin.Propagates
    % SimplePosController
    %
    % Simple hand-tuned PD position controller.
    %
    % No force calculations.
    % No mass model.
    % No gravity model.
    %
    % Control idea:
    %   X error -> pitch command
    %   Y error -> roll command
    %   Z error -> thrust command
    %
    % Assumption:
    %   thrust closer to 0   = more upward thrust
    %   thrust more negative = less upward thrust / downward

    properties
        % ============================
        % Limits
        % ============================

        % Max roll/pitch command in radians
        max_tilt = 3.14;        % 15 degrees

        % Thrust hover/trim value
        % Tune this first with all gains set to 0.
        gravityGain = -0.3;

        % Since 0 makes the drone shoot upward, keep thrust away from 0.
        thrust_upper_limit = 1;
        thrust_lower_limit = -1;

        % Limit derivative damping so noisy position estimates cannot rail thrust.
        max_thrust_damping = 0.50;

        % Clamp accumulated integral error in each axis.
        max_i_error = 0.50;

        % Controller sample time in seconds. Match this to the Simulink block rate.
        sample_time = 0.04;

        % ============================
        % Direct PD gains
        % ============================

        % Pitch controller:
        % X position error -> pitch
        Kp_pitch = 0.16;
        Kd_pitch = 0.2;
        Ki_pitch = 0.010;
        bias_pitch = 0.0091;

        % Roll controller:
        % Y position error -> roll
        Kp_roll = 0.16;
        Kd_roll = 0.2;
        Ki_roll = 0.010;
        bias_roll = -0.0363;

        % Thrust controller:
        % world-frame Z position error -> thrust correction
        Kp_thrust = 0.29;
        Kd_thrust = 0.34;
        Ki_thrust = 0.020;

        % ============================
        % Manual trims
        % ============================

        % Use these if the drone has small constant bias.
        roll_trim = 0.0;
        pitch_trim = 0.0;
        thrust_trim = 0.0;

        % ============================
        % Sign flips
        % ============================
        %
        % If X correction goes the wrong way, set pitch_sign = -1.
        % If Y correction goes the wrong way, set roll_sign = -1.
        % If Z correction goes the wrong way, set thrust_sign = -1.

        pitch_sign = 1.0;
        roll_sign = 1.0;
        thrust_sign = 1.0;
    end

    properties (DiscreteState)
        prev_e_pos
        has_prev_error
        i_error
    end

    methods (Access = protected)

        function setupImpl(obj)
            obj.prev_e_pos = zeros(3,1);
            obj.has_prev_error = false;
            obj.i_error = zeros(3,1);
        end

        function [thrust, roll, pitch, F_d, e_pos] = stepImpl(obj, pos, measured_yaw, sp_pos, sp_vel, sp_acc, setpoint_yaw)
            %#ok<INUSD>
            % measured_yaw, sp_vel, sp_acc and setpoint_yaw are unused in this simple controller.

            % Force column vectors
            pos    = pos(:);
            sp_pos = sp_pos(:);

            % ============================
            % Position error and internal derivative estimate
            % ============================

            e_pos = sp_pos - pos;

            if obj.has_prev_error
                e_dot = (e_pos - obj.prev_e_pos) / obj.sample_time;
            else
                e_dot = zeros(3,1);
                obj.has_prev_error = true;
            end

            obj.prev_e_pos = e_pos;

            obj.i_error = obj.i_error + e_pos * obj.sample_time;
            obj.i_error = max(min(obj.i_error, obj.max_i_error), -obj.max_i_error);

            % Keep XY control direct:
            %   X error -> pitch
            %   Y error -> roll
            % D terms use internally estimated error derivative, like a PD block.
            % I terms are clamped to prevent windup.

            % ============================
            % Pitch PD controller
            % ============================

            pitch_raw = obj.Kp_pitch * e_pos(1) ...
                      + obj.Kd_pitch * e_dot(1) ...
                      + obj.Ki_pitch * obj.i_error(1);

            pitch = obj.pitch_sign * pitch_raw + obj.pitch_trim;

            % Clamp pitch
            pitch = max(min(pitch, obj.max_tilt), -obj.max_tilt) + obj.bias_pitch;

            % ============================
            % Roll PD controller
            % ============================

            roll_raw = obj.Kp_roll * e_pos(2) ...
                     + obj.Kd_roll * e_dot(2) ...
                     + obj.Ki_roll * obj.i_error(2);

            roll = obj.roll_sign * roll_raw + obj.roll_trim  ;

            % Clamp roll
            roll = max(min(roll, obj.max_tilt), -obj.max_tilt) + obj.bias_roll;%- 0.0610;

            % ============================
            % Thrust PD controller
            % ============================

            thrust_pos_correction = obj.Kp_thrust * e_pos(3) ...
                                  + obj.Ki_thrust * obj.i_error(3);

            z_error_rate = e_dot(3);
            thrust_damping_size = obj.Kd_thrust * abs(z_error_rate);
            thrust_damping_size = min(thrust_damping_size, obj.max_thrust_damping);

            % Size comes from error-rate magnitude. Sign follows the PD derivative.
            thrust_vel_damping = sign(z_error_rate) * thrust_damping_size;

            thrust_correction = thrust_pos_correction + thrust_vel_damping;

            thrust = obj.gravityGain ...
                   + obj.thrust_trim ...
                   + obj.thrust_sign * thrust_correction;

            % Clamp thrust
            thrust = max(min(thrust, obj.thrust_upper_limit), obj.thrust_lower_limit);

            % ============================
            % Debug output
            % ============================
            %
            % F_d is no longer force.
            % It is just a debug vector:
            %   F_d(1) = raw pitch command before sign/trim/clamp
            %   F_d(2) = raw roll command before sign/trim/clamp
            %   F_d(3) = raw thrust correction before hover/trim/clamp

            F_d = zeros(3,1);
            F_d(1) = pitch_raw;
            F_d(2) = roll_raw;
            F_d(3) = thrust_correction;
        end

        function resetImpl(obj)
            obj.prev_e_pos = zeros(3,1);
            obj.has_prev_error = false;
            obj.i_error = zeros(3,1);
        end

        function [sz, dt, cp] = getDiscreteStateSpecificationImpl(~, name)
            switch name
                case 'prev_e_pos'
                    sz = [3 1];
                    dt = 'double';
                    cp = false;
                case 'has_prev_error'
                    sz = [1 1];
                    dt = 'logical';
                    cp = false;
                case 'i_error'
                    sz = [3 1];
                    dt = 'double';
                    cp = false;
            end
        end

        function [sz1, sz2, sz3, sz4, sz5] = getOutputSizeImpl(~)
            sz1 = [1 1];
            sz2 = [1 1];
            sz3 = [1 1];
            sz4 = [3 1];
            sz5 = [3 1];
        end

        function [dt1, dt2, dt3, dt4, dt5] = getOutputDataTypeImpl(~)
            dt1 = 'double';
            dt2 = 'double';
            dt3 = 'double';
            dt4 = 'double';
            dt5 = 'double';
        end

        function [cp1, cp2, cp3, cp4, cp5] = isOutputComplexImpl(~)
            cp1 = false;
            cp2 = false;
            cp3 = false;
            cp4 = false;
            cp5 = false;
        end

        function [f1, f2, f3, f4, f5] = isOutputFixedSizeImpl(~)
            f1 = true;
            f2 = true;
            f3 = true;
            f4 = true;
            f5 = true;
        end
    end
end
