let print_char_array_array arr =
  Array.iter
    (fun inner ->
      Array.iter (fun c -> print_char c) inner;
      print_newline ())
    arr

let cartesian l l' =
  List.concat (List.map (fun e -> List.map (fun e' -> (e, e')) l') l)

let neighbors i x =
  match i with
  | 0 -> i :: [ i + 1 ]
  | i when i = x - 1 -> (i - 1) :: [ i ]
  | _ -> (i - 1) :: i :: [ i + 1 ]

let forklifts () =
  let mat = ref [||] in
  let rec init_array () =
    try
      let line = read_line () in
      mat := Array.append !mat [| line |> String.to_seq |> Array.of_seq |];
      init_array ()
    with _ -> ()
  in
  init_array ();
  print_char_array_array !mat;
  let y = Array.length !mat in
  let x = Array.length !mat.(0) in
  Printf.printf "%d %d\n" x y;
  let check_roll i j =
    let coordinates = cartesian (neighbors i x) (neighbors j y) in
    List.fold_left
      (fun acc (a, b) -> acc + if !mat.(a).(b) = '@' then 1 else 0)
      (-1) coordinates
    < 4
  in
  let counter = ref 0 in
  Array.iteri
    (fun i line ->
      Array.iteri
        (fun j roll -> if roll = '@' && check_roll i j then incr counter)
        line)
    !mat;
  !counter
;;

print_endline (string_of_int (forklifts ()))
