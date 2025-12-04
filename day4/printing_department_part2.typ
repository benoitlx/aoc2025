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
    l1.map(x => l2.map(y => (x: x, y: y))).flatten()
  }


  let t1 = neighbors(0, 5);
  let t2 = neighbors(2, 6);
  [#t1 #t2 #cartesian(t1, t2)]

  let check_roll(i, j, mat) = {
    let coordinates = cartesian(t1, t2);
    let count = -1;
    for coordinate in coordinates {
      if mat.at(coordinate.y).at(coordinate.x) == "@" {
        count = count + 1;
      }
      [#count #coordinate]
    }
    linebreak()
    // count < 4
  }

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

  let counter = 0;
  for (i, sub_array) in mat.enumerate() {
    for (j, c) in sub_array.enumerate() {
      // if c == "@" and check_roll(i, j, mat) {
      //   counter = counter + 1;
      //   mat.at(i).at(j) = "x";
      // }
      [#i #j]
      check_roll(i, j, mat)
    }
  }
  [#mat]

  [#counter]
}
