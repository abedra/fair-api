table! {
    scenarios (id) {
        id -> Int4,
        name -> Text,
        qualified_probable_risk -> Text,
        qualified_worst_case_risk -> Text,
        quantified_probable_loss -> Float4,
        quantified_worst_case_loss -> Float4,
        created_at -> Nullable<Timestamp>,
    }
}
