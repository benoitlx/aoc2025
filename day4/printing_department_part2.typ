#set text(font: "Maple Mono NF")

= Day4

#{
  let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
  // input
  [#input]
  let neighbors(i, x) = {
    if i == 0 {
      (0, 1)
    } else if i == (x - 1) {
      (x - 2, x - 1)
    } else {
      (i - 1, i, i + 1)
    }
  }

  let cartesian(l1, l2) = {
    // Attention ça flatten même les couples
    l1.map(x => l2.map(y => (x: x, y: y))).flatten()
  }


  let t1 = neighbors(0, 5);
  let t2 = neighbors(2, 6);
  [#t1 #t2 #cartesian(t1, t2)]

  let mat = ();
  let sub_array = ();
  for c in input {
    if c == "\n" {
      mat.push(sub_array);
      sub_array = ();
    } else {
      sub_array.push(c);
    }
  }
  [#mat]
  let y = mat.len();
  let x = mat.at(0).len();
}
