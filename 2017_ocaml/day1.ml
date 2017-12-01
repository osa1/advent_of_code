let input = read_line () in
let input_len = String.length input in

let parse_digit c = Char.code c - Char.code '0' in

let rec loop ?(acc = 0) n =
    if n == input_len then
        acc
    else
        let i1 = parse_digit (String.get input n) in
        let i2 = parse_digit (String.get input ((n + input_len / 2) mod input_len)) in
        loop ~acc:(acc + if i1 == i2 then i1 else 0) (n + 1)
in

Printf.printf "%d\n" (loop 0)
