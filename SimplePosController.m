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
        max_tilt = 0.2618;        % 15 degrees

        % Thrust hover/trim value
        % Tune this first with all gains set to 0.
        gravityGain = -1;

        % Since 0 makes the drone shoot upward, keep thrust away from 0.
        thrust_upper_limit = -0.05;
        thrust_lower_limit = -0.80;

        % ============================
        % Direct PD gains
        % ============================

        % Pitch controller:
        % body-frame X position error -> pitch
        Kp_pitch = 0.000005;
        Kd_pitch = 0.000;

        % Roll controller:
        % body-frame Y position error -> roll
        Kp_roll = 0.00000;
        Kd_roll = 0.000;

        % Thrust controller:
        % world-frame Z position error -> thrust correction
        Kp_thrust = 0.070;
        Kd_thrust = 7.0;

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

    methods (Access = protected)

        function setupImpl(~)
        end

        function [thrust, roll, pitch, F_d, e_pos] = stepImpl(obj, pos, vel, measured_yaw, sp_pos, sp_vel, sp_acc, setpoint_yaw)
            %#ok<INUSD>
            % sp_acc and setpoint_yaw are unused in this simple controller.

            % Force column vectors
            pos    = pos(:);
            vel    = vel(:);
            sp_pos = sp_pos(:);
            sp_vel = sp_vel(:);

            % ============================
            % Position and velocity errors
            % ============================

            e_pos = sp_pos - pos;
            e_vel = sp_vel - vel;

            % ============================
            % Convert XY error into body frame
            % ============================
            %
            % This means:
            %   body X error -> pitch
            %   body Y error -> roll
            %
            % If your yaw is always zero, this does almost nothing.
            % If the drone yaws, this keeps pitch/roll aligned with the drone body.

            psi = measured_yaw;

            R_world_to_body = [ cos(psi)  sin(psi);
                               -sin(psi)  cos(psi)];

            e_xy_body  = R_world_to_body * e_pos(1:2);
            ev_xy_body = R_world_to_body * e_vel(1:2);

            % ============================
            % Pitch PD controller
            % ============================

            pitch_raw = obj.Kp_pitch * e_xy_body(1) ...
                      + obj.Kd_pitch * ev_xy_body(1);

            pitch = obj.pitch_sign * pitch_raw + obj.pitch_trim;

            % Clamp pitch
            pitch = max(min(pitch, obj.max_tilt), -obj.max_tilt);

            % ============================
            % Roll PD controller
            % ============================

            roll_raw = obj.Kp_roll * e_xy_body(2) ...
                     + obj.Kd_roll * ev_xy_body(2);

            roll = obj.roll_sign * roll_raw + obj.roll_trim;

            % Clamp roll
            roll = max(min(roll, obj.max_tilt), -obj.max_tilt);

            % ============================
            % Thrust PD controller
            % ============================

            z_error = e_pos(3);
            z_vel_error = e_vel(3);

            thrust_correction = obj.Kp_thrust * z_error ...
                              + obj.Kd_thrust * z_vel_error;

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

        function resetImpl(~)
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