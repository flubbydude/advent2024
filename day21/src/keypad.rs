use array2d::Array2D;

pub fn build_keypad_from_string<T>(keypad_str: &str) -> Array2D<Option<T>>
where
    T: TryFrom<char>,
    <T as TryFrom<char>>::Error: std::fmt::Debug,
{
    let num_rows = keypad_str.lines().count();
    let num_columns = keypad_str.lines().next().unwrap().len();
    Array2D::from_iter_row_major(
        keypad_str.lines().flat_map(str::chars).map(|c| {
            if c == ' ' {
                None
            } else {
                Some(c.try_into().unwrap())
            }
        }),
        num_rows,
        num_columns,
    )
    .unwrap()
}
