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

  let pretty_print(mat) = {
    for sub_array in mat {
      for c in sub_array [#c]
      linebreak();
    }
  }

  let cartesian(l1, l2) = {
    l1.map(x => l2.map(y => (x: x, y: y))).flatten()
  }

  let check_roll(i, j, mat, x, y) = {
    let t1 = neighbors(i, x);
    let t2 = neighbors(j, y);
    let coordinates = cartesian(t1, t2);
    // [#coordinates]
    let count = - 1;
    for coordinate in coordinates {
      if mat.at(coordinate.x).at(coordinate.y) == "@" {
        count = count + 1;
      }
      // [#count #coordinate]
      // [#count ]
    }
    // linebreak()
    count < 4
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
  let y = mat.len();
  let x = mat.at(0).len();

  let counter = 0;
  let flag = true;
  while flag {
    flag = false;
    for (i, sub_array) in mat.enumerate() {
      for (j, c) in sub_array.enumerate() {
        if c == "@" and check_roll(i, j, mat, x, y) {
          counter = counter + 1;
          mat.at(i).at(j) = "x";
          flag = true;
        }
        // [#i #j      ]
        // check_roll(i, j, mat, x, y)
      }
    }
    pretty_print(mat);
  }

  [= #counter]
}
