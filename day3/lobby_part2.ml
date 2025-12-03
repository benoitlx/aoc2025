(* Ocaml is very elegant *)

let digits_of_line (s : string) : int list =
  s |> String.to_seq |> List.of_seq
  |> List.map (fun c -> Char.code c - Char.code '0')

let max_joltage_of_bank (digits : int list) (k : int) : int =
  let stack = Stack.create () in
  let remaining_drop = ref (List.length digits - k) in
  List.iter
    (fun h ->
      while
        !remaining_drop > 0
        && (not (Stack.is_empty stack))
        && Stack.top stack < h
      do
        ignore (Stack.pop stack);
        decr remaining_drop
      done;
      Stack.push h stack)
    digits;
  while Stack.length stack != k do
    ignore (Stack.pop stack)
  done;
  let chosen = Stack.to_seq stack |> List.of_seq |> List.rev in
  List.fold_left (fun acc d -> (acc * 10) + d) 0 chosen

let lobby () =
  let rec loop acc =
    try
      let line = read_line () in
      let digits = digits_of_line line in
      loop (acc + max_joltage_of_bank digits 12)
    with _ -> acc
  in
  loop 0
;;

print_endline (string_of_int (lobby ()))
