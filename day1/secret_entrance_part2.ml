(* This solution is not efficient at all xD *)

let int_of_bool = function true -> 1 | false -> -1

let positive_mod x n =
  let r = x mod n in
  if r < 0 then r + n else r

let dialanalyzer () =
  let rec counter count dial_pos =
    try
      let line = read_line () in
      begin
        let direction = String.get line 0 = 'R' in
        let shift =
          int_of_string (String.sub line 1 (String.length line - 1))
        in
        let clicks = ref 0 in
        let new_dial_pos = ref dial_pos in
        let i = ref 0 in
        while !i < shift do
          new_dial_pos := !new_dial_pos + int_of_bool direction;
          if !new_dial_pos mod 100 = 0 then clicks := !clicks + 1;
          i := !i + 1
        done;
        counter (count + !clicks) (positive_mod !new_dial_pos 100)
      end
    with _ -> count
  in
  counter 0 50
;;

print_endline (string_of_int (dialanalyzer ()))
