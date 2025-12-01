let int_of_bool = function true -> 1 | false -> -1

let dialanalyzer () =
  let rec counter count dial_pos =
    try
      let line = read_line () in
      begin
        let direction = int_of_bool (String.get line 0 = 'R') in
        let shift =
          int_of_string (String.sub line 1 (String.length line - 1))
        in
        let new_dial_pos = (dial_pos + (direction * shift)) mod 100 in
        if new_dial_pos = 0 then counter (count + 1) new_dial_pos
        else counter count new_dial_pos
      end
    with _ -> count
  in
  counter 0 50
;;

print_endline (string_of_int (dialanalyzer ()))
