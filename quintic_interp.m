function smooth_path = quintic_interp(waypoints, velocities, n_interp)
% QUINTIC_INTERP  Fit a quintic spline through 3D waypoints with
%                 user-supplied velocity vectors at each point.
%
%   smooth_path = quintic_interp(waypoints, velocities)
%   smooth_path = quintic_interp(waypoints, velocities, n_interp)
%   smooth_path = quintic_interp(waypoints, [],         n_interp)  % auto velocities
%
%   INPUT
%     waypoints  : Nx3 matrix of [x, y, z] points (in order)
%     velocities : Nx3 matrix of [vx, vy, vz] at each waypoint
%                  Pass [] to auto-compute from central differences
%     n_interp   : interior points inserted between each pair (default 10)
%
%   OUTPUT
%     smooth_path : Mx3 matrix — all original waypoints + interpolated pts

    N = size(waypoints, 1);
    M = N - 1;

    if N < 2
        error('Need at least 2 waypoints.');
    end

    % --- defaults ---
    if nargin < 3 || isempty(n_interp)
        n_interp = 10;
    end

    % ------------------------------------------------------------------
    % 1. Arc-length-proportional time nodes
    % ------------------------------------------------------------------
    seg_len = zeros(M, 1);
    for i = 1:M
        seg_len(i) = norm(waypoints(i+1,:) - waypoints(i,:));
    end
    total_len = sum(seg_len);
    if total_len < 1e-12
        error('All waypoints are identical.');
    end
    T_nodes = [0; cumsum(seg_len / total_len * M)];

    % ------------------------------------------------------------------
    % 2. Velocities — use supplied or auto-compute
    % ------------------------------------------------------------------
    if nargin < 2 || isempty(velocities)
        % central differences, scaled
        vel_scale = 1.2;
        VEL = zeros(N, 3);
        VEL(1,:)   = (waypoints(2,:) - waypoints(1,:)) / (T_nodes(2) - T_nodes(1));
        VEL(end,:) = (waypoints(end,:) - waypoints(end-1,:)) / (T_nodes(end) - T_nodes(end-1));
        for i = 2:N-1
            VEL(i,:) = (waypoints(i+1,:) - waypoints(i-1,:)) / (T_nodes(i+1) - T_nodes(i-1));
        end
        VEL = VEL * vel_scale;
    else
        if ~isequal(size(velocities), [N, 3])
            error('velocities must be Nx3 — one [vx,vy,vz] row per waypoint.');
        end
        VEL = velocities;
    end

    % ------------------------------------------------------------------
    % 3. Accelerations (light second-difference estimate, fixed magnitude)
    % ------------------------------------------------------------------
    acc_scale = 0.1;
    ACC = zeros(N, 3);
    ACC(1,:)   = waypoints(3,:) - 2*waypoints(2,:) + waypoints(1,:);
    ACC(end,:) = waypoints(end,:) - 2*waypoints(end-1,:) + waypoints(end-2,:);
    for i = 2:N-1
        ACC(i,:) = waypoints(i+1,:) - 2*waypoints(i,:) + waypoints(i-1,:);
    end
    for i = 1:N
        nn = norm(ACC(i,:));
        if nn > 1e-9
            ACC(i,:) = ACC(i,:) / nn * acc_scale;
        end
    end

    % ------------------------------------------------------------------
    % 4. Solve quintic segments and evaluate
    % ------------------------------------------------------------------
    n_rows      = N + M * n_interp;
    smooth_path = zeros(n_rows, 3);
    row         = 0;

    for s = 1:M
        T = T_nodes(s+1) - T_nodes(s);

        cx = solve_quintic(waypoints(s,1), VEL(s,1), ACC(s,1), ...
                           waypoints(s+1,1), VEL(s+1,1), ACC(s+1,1), T);
        cy = solve_quintic(waypoints(s,2), VEL(s,2), ACC(s,2), ...
                           waypoints(s+1,2), VEL(s+1,2), ACC(s+1,2), T);
        cz = solve_quintic(waypoints(s,3), VEL(s,3), ACC(s,3), ...
                           waypoints(s+1,3), VEL(s+1,3), ACC(s+1,3), T);

        % start waypoint
        row = row + 1;
        smooth_path(row, :) = waypoints(s, :);

        % interior interpolated points
        t_all   = linspace(0, T, n_interp + 2);
        t_inner = t_all(2:end-1)';
        Tm = [ones(n_interp,1), t_inner, t_inner.^2, ...
              t_inner.^3, t_inner.^4, t_inner.^5];

        rows_inner = row + (1:n_interp);
        smooth_path(rows_inner, 1) = Tm * cx;
        smooth_path(rows_inner, 2) = Tm * cy;
        smooth_path(rows_inner, 3) = Tm * cz;
        row = row + n_interp;
    end

    % final waypoint
    row = row + 1;
    smooth_path(row, :) = waypoints(end, :);

end


function c = solve_quintic(p0,v0,a0, p1,v1,a1, T)
    A = [1  0   0      0       0        0;
         0  1   0      0       0        0;
         0  0   2      0       0        0;
         1  T   T^2    T^3     T^4      T^5;
         0  1   2*T    3*T^2   4*T^3    5*T^4;
         0  0   2      6*T     12*T^2   20*T^3];
    b = [p0; v0; a0; p1; v1; a1];
    c = A \ b;
end
