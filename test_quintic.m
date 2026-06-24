clear; clc; close all;

% --- your input points (Nx3) ---
% waypoints = [0, 0, 0.35;
%     0, 2.10, 0.35;
%     -0.70, 2.10, 0.35;
%     -0.70, 1.40, 0.35;
%     0,1.40,1.05;
%     0,2.10,1.05;
%     -0.70,2.10,1.05;
%     -0.70,0.70,1.05;
%     -1.40,0.70,1.05;
%     -1.40,0.70,0.35;
%     -1.40,1.40,0.35;
%     -0.70,1.40,0.35;
%     -0.70,0.70,0.35;
%     -1.40,0.70,0.35;
%     -1.40,0,0.35;
%     0,0,0.35];

% --- your input points (Nx3) ---
base_waypoints = [0, 0, 0.35;
    0, 0.35, 0.35;
    0, 1.05, 0.35;
    0, 1.75, 0.35;
    -0.35, 2.10, 0.35;
    -0.70, 1.75, 0.35;
    -0.35, 1.40, 0.70;
    0, 1.75, 1.05;
    -0.35, 2.10, 1.05;
    -0.70, 1.75, 1.05;
    -0.70, 1.05, 1.05;
    -1.05, 0.70, 1.05;
    -1.40, 1.05, 0.70;
    -1.05, 1.40, 0.35;
    -0.70, 1.05, 0.35;
    -1.05, 0.70, 0.35;
    -1.40, 0.35, 0.35;
    -1.05, 0.00, 0.35;
    -0.35, 0.00, 0.35;
    0.00, 0.00, 0.35];

vel = 0.5;
base_velocities = [0, vel, 0;
    0, vel, 0;
    0, vel, 0;
    0, vel, 0;
    -vel, 0, 0;
    0, -vel, 0;
    vel, 0, vel;
    0, vel, 0;
    -vel, 0, 0;
    0, -vel, 0;
    0, -vel, 0;
    -vel, 0, 0;
    0, vel, -vel;
    vel, 0, 0;
    0, -vel, 0;
    -vel, 0, 0;
    0, -vel, 0;
    vel, 0, 0;
    vel, 0, 0;
    0, 0, 0];

% --- repeat the route without copy/pasting the matrices ---
num_loops = 3;
base_n = size(base_waypoints, 1);
loop_n = base_n - 1;
total_n = base_n + (num_loops - 1) * loop_n;

waypoints = zeros(total_n, 3);
velocities = zeros(total_n, 3);

waypoints(1:base_n,:) = base_waypoints;
velocities(1:base_n,:) = base_velocities;

for loop_idx = 2:num_loops
    row_start = base_n + (loop_idx - 2) * loop_n + 1;
    row_end = row_start + loop_n - 1;

    waypoints(row_start:row_end,:) = base_waypoints(2:end,:);
    velocities(row_start:row_end,:) = base_velocities(2:end,:);
end

% --- call the function ---
% smooth_path = quintic_interp(waypoints, 10);
smooth_path = quintic_interp(waypoints, velocities, 10);
% N = size(waypoints, 1);
% w_uniform = ones(N, 1);
% smooth_path = bspline_interp(waypoints, w_uniform, 10);

% --- print sizes ---
fprintf('Input  waypoints : %d points\n', size(waypoints,1));
fprintf('Output smooth_path: %d points\n', size(smooth_path,1));

% --- 3D plot ---
figure('Color','k','Position',[100 100 900 750]);
ax = axes;
hold(ax,'on'); grid(ax,'on'); box(ax,'on');
set(ax,'Color','k','GridColor',[0.35 0.35 0.35],...
    'XColor','w','YColor','w','ZColor','w','FontSize',11);

plot3(ax, smooth_path(:,1), smooth_path(:,2), smooth_path(:,3), ...
    'b', 'LineWidth', 2.5);

scatter3(ax, waypoints(:,1), waypoints(:,2), waypoints(:,3), ...
    100, 'r', 'filled', 'MarkerEdgeColor','w','LineWidth',2.5);

for i = 1:size(waypoints,1)
    text(ax, waypoints(i,1)+0.1, waypoints(i,2)+0.1, waypoints(i,3)+0.15, ...
        sprintf('P%d',i-1),'Color','w','FontSize',10,'FontWeight','bold');
end

legend(ax, {'Quintic path','Waypoints'}, ...
    'TextColor','w','Color',[0.1 0.1 0.1],'EdgeColor',[0.5 0.5 0.5]);
xlabel(ax,'X','Color','w');
ylabel(ax,'Y','Color','w');
zlabel(ax,'Z','Color','w');
title(ax,'quintic\_interp demo','Color','w','FontSize',13);
view(ax, 35, 25);
axis(ax, 'equal'); 
rotate3d(ax,'on');
