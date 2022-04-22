table! {
    import_history (id) {
        id -> Varchar,
        data_transacoes -> Varchar,
        data_importacao -> Varchar,
    }
}

table! {
    transactions (id) {
        id -> Varchar,
        banco_org -> Varchar,
        agencia_org -> Varchar,
        conta_org -> Varchar,
        banco_dest -> Varchar,
        agencia_dest -> Varchar,
        conta_dest -> Varchar,
        valor -> Float4,
        data -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        nome -> Varchar,
        senha -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    import_history,
    transactions,
    users,
);
