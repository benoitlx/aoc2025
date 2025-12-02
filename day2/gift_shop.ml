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
          let n = String.length as_str in
          let sub_str_left = String.sub as_str 0 (n / 2) in
          if String.ends_with ~suffix:sub_str_left as_str && n mod 2 = 0 then begin
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
