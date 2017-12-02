let process_line line =
    let nums = List.map int_of_string (Str.split (Str.regexp "[ \t]+") line) in

    let rec loop l1 l2 : int =

        match (l1, l2) with

        | (l1_h :: l1_t, l2_h :: l2_t) ->
            let max = max l1_h l2_h in
            let min = min l1_h l2_h in
            if max mod min = 0 then
                max / min
            else
                loop l1 l2_t

        | (_ :: l1_h :: l1_t, []) ->
            loop (l1_h :: l1_t) l1_t

        | _ ->
            0
    in

    loop nums (List.tl nums)
in

let rec process_lines acc : int =
    try
        let line = read_line () in
        process_lines (acc + (process_line line))
    with
    | End_of_file -> acc
in

Printf.printf "%d\n" (process_lines 0)
