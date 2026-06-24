function is_live = PositionLiveCheck(current_position, numPosToCheck)
% PositionLiveCheck returns 0 when position samples are stale and 1 when live.
%
% Stale means the last numPosToCheck position samples are exactly identical.

%#codegen

    max_positions_to_check = 100;

    persistent position_history sample_count write_index

    current_position = current_position(:);
    n = round(numPosToCheck);
    n = max(min(n, max_positions_to_check), 1);

    if isempty(position_history)
        position_history = zeros(3, max_positions_to_check);
        sample_count = 0;
        write_index = 1;
    end

    position_history(:, write_index) = current_position;
    write_index = write_index + 1;
    if write_index > max_positions_to_check
        write_index = 1;
    end

    sample_count = min(sample_count + 1, max_positions_to_check);

    if sample_count < n
        is_live = int32(1);
        return;
    end

    newest_index = write_index - 1;
    if newest_index < 1
        newest_index = max_positions_to_check;
    end

    newest_position = position_history(:, newest_index);
    all_same = true;

    for k = 1:n
        history_index = newest_index - k + 1;
        while history_index < 1
            history_index = history_index + max_positions_to_check;
        end

        if any(position_history(:, history_index) ~= newest_position)
            all_same = false;
            break;
        end
    end

    if all_same
        is_live = int32(0);
    else
        is_live = int32(1);
    end
end
