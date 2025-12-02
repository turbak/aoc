input_filename = "inputs/d01_example"

{_, total_times_seen_zero} = File.read!(input_filename)
|> String.splitter("\n", trim: true)
|> Enum.reduce({50, 0}, fn line, {dial_value, total_zero}->
    <<dial_dir, rest::binary>> = line
    {num, _} = Integer.parse(rest)

    dial_value =
        case dial_dir do
            ?L -> dial_value-num
            ?R -> dial_value+num
        end

    dial_value =
        cond do
            dial_value >= 100 -> rem(dial_value, 100)
            dial_value <= 100 -> rem(100 + dial_value, 100)
        end

    total_zero =
        if dial_value == 0 do
            total_zero + 1
        else
            total_zero
        end

    {dial_value, total_zero}
end)

IO.puts("1. total times seen 0: #{total_times_seen_zero}")

{_, total_times_seen_zero} = File.read!(input_filename)
|> String.splitter("\n", trim: true)
|> Enum.reduce({50, 0}, fn line, {initial_dial_value, total_zero}->
    <<dial_dir, rest::binary>> = line
    {num, _} = Integer.parse(rest)

    dial_value =
        case dial_dir do
            ?L -> initial_dial_value-num
            ?R -> initial_dial_value+num
        end

    {dial_value, times_seen_zero} =
        cond do
            dial_value == 100 || dial_value == 0 -> {0, 1}
            dial_value > 100 -> {rem(dial_value, 100), div(dial_value, 100)}
            dial_value < 0 -> {rem(100 + dial_value, 100), abs(div(dial_value, 100))+if(initial_dial_value != 0, do: 1, else: 0)}
            dial_value < 100 -> {dial_value, 0}
        end

    IO.puts("#{line}: new_val: #{dial_value} times_seen_zero: #{times_seen_zero}")

    {dial_value, total_zero+times_seen_zero}
end)

IO.puts("2. total times seen 0: #{total_times_seen_zero}")
