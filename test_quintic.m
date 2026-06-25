base_waypoints = [0, 0, 0.35;
    0, 0.55, 0.35;
    -0.15, 1.75, 0.45;
    -0.55, 1.75, 0.55;
    -0.15, 1.75, 0.75;
    -0.55, 1.75, 0.95;
    -0.85, 0.85, 0.85;
    -1.25, 1.05, 0.70;
    -0.85, 1.05, 0.55;
    -1.25, 0.35, 0.45;
    -0.6, 0.00, 0.35;
    % 0.00, 0.00, 0.35;
    % 
    % 0, 0, 0.35;
    0, 0.55, 0.35;
    -0.15, 1.75, 0.45;
    -0.55, 1.75, 0.55;
    -0.15, 1.75, 0.75;
    -0.55, 1.75, 0.85;
    -0.85, 0.85, 0.95;
    -1.25, 1.05, 0.70;
    -0.85, 1.05, 0.55;
    -1.25, 0.35, 0.45;
    -0.6, 0.00, 0.35;
    % 0.00, 0.00, 0.35;
    % 
    % 0, 0, 0.35;
    0, 0.55, 0.35;
    -0.15, 1.75, 0.45;
    -0.55, 1.75, 0.55;
    -0.15, 1.75, 0.75;
    -0.55, 1.75, 0.85;
    -0.85, 0.85, 0.95;
    -1.25, 1.05, 0.70;
    -0.85, 1.05, 0.55;
    -1.25, 0.35, 0.45;
    -0.6, 0.00, 0.35;
    0.00, 0.00, 0.35];

vel = 0.5
base_velocities = [0, vel, 0;
    0, vel*2, 0;
    -vel, vel*2, vel/10;
    0, -vel*2, vel/10;
    0, vel*2, 0;
    -vel/3, -vel*2, 0;
    -vel, -vel*1.5, 0;
    -vel/2, vel*1.5, -vel;
    -vel/5, -vel*1.5, 0;
    -vel/2, -vel*1.5, 0;
    vel, 0, 0;
    % 0, 0, 0;
    % 
    % 0, vel, 0;
    0, vel*2, 0;
    -vel, vel*2, vel/10;
    0, -vel*2, vel/10;
    0, vel*2, 0;
    -vel/3, -vel*2, 0;
    -vel, -vel*1.5, 0;
    -vel/2, vel*1.5, -vel;
    -vel/5, -vel*1.5, 0;
    -vel/2, -vel*1.5, 0;
    vel, 0, 0;
    % 0, 0, 0;
    % 
    % 0, vel, 0;
    0, vel*2, 0;
    -vel, vel*2, vel/10;
    0, -vel*2, vel/10;
    0, vel*2, 0;
    -vel/3, -vel*2, 0;
    -vel, -vel*1.5, 0;
    -vel/2, vel*1.5, -vel;
    -vel/5, -vel*1.5, 0;
    -vel/2, -vel*1.5, 0;
    vel, 0, 0;
    0, 0, 0];
% --- repeat the route without copy/pasting the matrices ---
% num_loops = 3;
% base_n = size(base_waypoints, 1);
% loop_n = base_n - 1;
% total_n = base_n + (num_loops - 1) * loop_n;
% 
% waypoints = zeros(total_n, 3);
% velocities = zeros(total_n, 3);
% 
% waypoints(1:base_n,:) = base_waypoints;
% velocities(1:base_n,:) = base_velocities;

% for loop_idx = 2:num_loops
%     row_start = base_n + (loop_idx - 2) * loop_n + 1;
%     row_end = row_start + loop_n - 1;
% 
%     waypoints(row_start:row_end,:) = base_waypoints(2:end,:);
%     velocities(row_start:row_end,:) = base_velocities(2:end,:);
% end
% --- call the function ---
y = quintic_interp(base_waypoints, base_velocities, 12);