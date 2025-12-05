let cafeteria () =
  let rec create_ranges (acc : (int * int) list) =
    let line = read_line () in
    if line = "" then acc
    else
      let range_list =
        String.split_on_char '-' line |> List.map int_of_string
      in
      let tuple = match range_list with [ a; b ] -> (a, b) in
      create_ranges (tuple :: acc)
  in
  let ranges = create_ranges [] in
  let rec process_ingredient acc =
    let is_fresh id (ranges : (int * int) list) =
      List.exists (fun (a, b) -> a <= id && id <= b) ranges
    in
    try
      let id = read_line () |> int_of_string in
      if is_fresh id ranges then process_ingredient (acc + 1)
      else process_ingredient acc
    with _ -> acc
  in
  process_ingredient 0
;;

print_endline (string_of_int (cafeteria ()))
