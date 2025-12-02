let is_invalid_id id =
  let len = String.length id in
  let rec check n =
    if n > len / 2 then false
    else
      let pattern = String.sub id 0 n in
      let rec repeat_check i =
        if i >= len then true
        else if String.sub id i n = pattern then repeat_check (i + n)
        else false
      in
      if len mod n = 0 && repeat_check n then true else check (n + 1)
  in
  check 1

let sum_good_id () =
  let line = read_line () in
  let ranges = String.split_on_char ',' line in
  let rec process_range acc ranges =
    match ranges with
    | [] -> acc
    | h :: t -> begin
        Printf.printf "ranges: %s\n" h;
        let parsed_range = String.split_on_char '-' h in
        let left = int_of_string (List.hd parsed_range) in
        let right = int_of_string (List.nth parsed_range 1) in
        let sum = ref 0 in
        for i = left to right do
          let as_str = string_of_int i in
          if is_invalid_id as_str then begin
            sum := !sum + i;
            Printf.printf "%d\n" i
          end
        done;
        process_range (acc + !sum) t
      end
  in
  process_range 0 ranges
;;

print_endline (string_of_int (sum_good_id ()))
