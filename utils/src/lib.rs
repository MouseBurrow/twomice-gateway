pub mod errors;

#[macro_export]
macro_rules! db_call {
    // -----------------------------------
    // Handle no bindings, no errors
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
    ) => {{
        db_call!(
            pool   = $pool,
            query  = $query,
            binds  = [],
            errors = {}
        )
    }};

    // -----------------------------------
    // Handle no errors
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
        binds  = [$($param:expr),* $(,)?]
    ) => {{
        db_call!(
            pool   = $pool,
            query  = $query,
            binds  = [$($param),*],
            errors = {}
        )
    }};

    // -----------------------------------
    // Handle no bindings
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
        errors = { $( $code:expr => $variant:expr ),* $(,)? }
    ) => {{
        db_call!(
            pool   = $pool,
            query  = $query,
            binds  = [],
            errors = { $( $code => $variant ),* }
        )
    }};

    // -----------------------------------
    // Main solution with every parameter
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
        binds  = [$($param:expr),* $(,)?],
        errors = { $( $code:expr => $variant:expr ),* $(,)? }
    ) => {{
        // Handle the bindings
        let mut query = $query;
        $( query = query.bind($param); )*

        // Handle the fetching and error mapping
        query.fetch_one($pool)
            .await
            .map_err(|err: sqlx::Error| {
            if let sqlx::Error::Database(db_err) = &err {
                // If any of the input errors match return that
                match db_err.code().as_deref() {
                    $(
                        Some($code) => { return $variant },
                    )*
                    _ => {}
                }
            }
            // If nothing fits, return textual error
            AuthError::Db(err)
        })
    }};
}
