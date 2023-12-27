pub fn level1(s: &str) -> usize {
    let num_cols = s.find('\n').unwrap_or(s.len() - 1);
    let (start_row, start_col) = s
        .lines()
        .enumerate()
        .find_map(|(i, line)| {
            line.chars()
                .position(|character| character == 'S')
                .map(|j| (i, j))
        })
        .expect("S must be present");

    let s = s.as_bytes();
    // now collect the 2 valid paths that connect to our position.

    let dirs = [(-1, 0, [b'|', b'7', b'F'].as_slice()), (0, -1, &[b'-', b'L', b'F']), (0, 1, &[b'-', b'J']), (1, 0, &[b'|', b'L', b'J'])]/* .filter_map(direction is valid && 
                                    the character at that pt is one that actually allows u to reach that pt) */;
    let two_valid_paths: Vec<(usize, usize)> = dirs
        .into_iter()
        .filter_map(|(dir_row, dir_col, allowed_paths)| {
            let neighbour_row = start_row.checked_add_signed(dir_row)?;
            let neighbour_col = start_col.checked_add_signed(dir_col)?;
            ((0..num_cols).contains(&neighbour_col)
                && allowed_paths.contains(s.get(neighbour_row * (num_cols + 1) + neighbour_col)?))
            .then_some((neighbour_row, neighbour_col))
        })
        .collect();

    dbg!(&two_valid_paths);
    // two_valid_paths should only have two pathss
    assert_eq!(two_valid_paths.len(), 2);

    // now just walk the paths together, count the steps, until they reach the same position.
    let mut prev_path_0 = (start_row, start_col);
    let mut prev_path_1 = prev_path_0.clone();
    let (mut path0, mut path1) = (two_valid_paths[0], two_valid_paths[1]);
    let mut steps = 0;

    while path0 != path1 {
        let temp = path0.clone();
        path0 = get_next_step(s, num_cols, path0, prev_path_0);
        prev_path_0 = temp;

        let temp = path1.clone();
        path1 = get_next_step(s, num_cols, path1, prev_path_1);
        prev_path_1 = temp;

        steps += 1;
    }

    steps + 1
}

fn get_next_step(
    s: &[u8],
    num_cols: usize,
    path: (usize, usize),
    prev_path: (usize, usize),
) -> (usize, usize) {
    // EXPECT: The two paths hence discovered must be a part of the loop, hence WILL NOT lead to out-of-bounds situations,
    // or non-pipe cells. So, s[_] indexing is panic-free (which can panic otherwise)
    match s[path.0 * (num_cols + 1) + path.1] {
        b'|' => {
            if prev_path.0 < path.0 {
                // on top of | previously
                (path.0 + 1, path.1)
            } else {
                (path.0 - 1, path.1)
            }
        }
        b'-' => {
            // on the left of '-' previously
            if prev_path.1 < path.1 {
                (path.0, path.1 + 1)
            } else {
                (path.0, path.1 - 1)
            }
        }
        b'L' => {
            if prev_path.1 == path.1 {
                // on top of L previously
                (path.0, path.1 + 1)
            } else {
                (path.0 - 1, path.1)
            }
        }
        b'J' => {
            if prev_path.1 == path.1 {
                // on top of J previously
                (path.0, path.1 - 1)
            } else {
                (path.0 - 1, path.1)
            }
        }
        b'7' => {
            if prev_path.1 == path.1 {
                // on bottom of 7 previously
                (path.0, path.1 - 1)
            } else {
                (path.0 + 1, path.1)
            }
        }
        b'F' => {
            if prev_path.1 == path.1 {
                // on bottom of F previously
                (path.0, path.1 + 1)
            } else {
                (path.0 + 1, path.1)
            }
        }
        b'S' => panic!("Starting position is ambiguous and shouldn't be passed into this function"),
        b'.' => panic!("The path should not have lead to a non-pipe"),
        _ => panic!("Encountered unexpected path in input stream"),
    }
}
