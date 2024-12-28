pub fn print_aligned(rows: Vec<Vec<String>>) {
    let num_cols = rows.iter().map(|row| row.len()).max().unwrap();

    let col_widths = (0..num_cols)
        .map(|j| {
            rows.iter()
                .flat_map(|row| row.get(j).map(|item| item.len()))
                .max()
                .unwrap_or(0)
        })
        .collect::<Vec<_>>();

    for row in rows {
        for (i, value) in row.iter().enumerate() {
            print!("{:<width$} ", value, width = col_widths[i]);
        }
        println!();
    }
}

/// ## Aligned printing.
/// ```rust,ignore
/// paligned! ([
///     ["expression","value", "description"],
///     ["1 + 1 =", 1 + 1],
///     ["foo:", @debug foo, "`@debug` example"],
/// ]);
/// ```
/// Features:
/// - Use `@debug` to print the [`std::fmt::Debug`] version.
#[macro_export]
macro_rules! paligned {
    ([$($rest:tt)*]) => {{
        let mut v = Vec::new();
        paligned!(
            @build
            @v v
            $($rest)*
        );
    }};
    (
        @build
        @v $v:ident
    ) => {
        paligned::print_aligned($v);
    };
    (
        @build
        @v $v:ident
        [],
        $($rest:tt)*
    ) => {
        $v.push(vec![]);
        paligned!(
            @build
            @v $v
            $($rest)*
        );
    };
    (
        @build
        @v $v:ident
        [@vec $row:expr],
        $($rest:tt)*
    ) => {
        for item in ($row).into_iter() {
            $v.push(vec![item.to_string()]);
        }
        paligned!(
            @build
            @v $v
            $($rest)*
        );
    };
    (
        @build
        @v $v:ident
        [$($(@$label:tt)? $row:expr),* $(,)?],
        $($rest:tt)*
    ) => {
        paligned!(
            @consume
            @v $v
            @item [ $( $(@$label)? $row ;)* ]
            @row []
            @rest [$($rest)*]
        );
    };
    (
        @consume
        @v $v:ident
        @item [@debug $item:expr ; $($item_rest:tt)*]
        @row [$($row:tt)*]
        @rest [$($rest:tt)*]
    ) => {
        paligned!(
            @consume
            @v $v
            @item [ $($item_rest)* ]
            @row [$($row)* format!("{:?}", $item),]
            @rest [$($rest)*]
        );
    };
    (
        @consume
        @v $v:ident
        @item [$item:expr ; $($item_rest:tt)*]
        @row [$($row:tt)*]
        @rest [$($rest:tt)*]
    ) => {
        paligned!(
            @consume
            @v $v
            @item [ $($item_rest)* ]
            @row [$($row)* format!("{}", $item),]
            @rest [$($rest)*]
        );
    };
    (
        @consume
        @v $v:ident
        @item []
        @row [$($row:tt)*]
        @rest [$($rest:tt)*]
    ) => {
        $v.push(vec![$($row)*]);
        paligned!(
            @build
            @v $v
            $($rest)*
        );
    };
    (
        @build
        @v $v:ident
        $row:expr,
        $($rest:tt)*
    ) => {
        $v.push(vec![($row).to_string()]);
        paligned!(
            @build
            @v $v
            $($rest)*
        );
    };
}
