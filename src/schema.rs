// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "brewing_method"))]
    pub struct BrewingMethod;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "grinder_type"))]
    pub struct GrinderType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BrewingMethod;

    brewing_sessions (id) {
        id -> Int4,
        bean_id -> Int4,
        grinder_id -> Int4,
        grind_setting -> Int4,
        brewing_method -> BrewingMethod,
        water_temp_celsius -> Nullable<Int4>,
        brew_time_seconds -> Nullable<Int4>,
        coffee_grams -> Float4,
        water_grams -> Float4,
        tasting_notes -> Nullable<Text>,
        rating -> Nullable<Float4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    coffee_beans (id) {
        id -> Int4,
        name -> Varchar,
        origin -> Varchar,
        roast_date -> Date,
        purchase_date -> Date,
        remaining_grams -> Nullable<Float4>,
        variety -> Nullable<Varchar>,
        processing_method -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GrinderType;

    grinders (id) {
        id -> Int4,
        brand -> Varchar,
        model -> Varchar,
        grinder_type -> GrinderType,
        min_setting -> Int4,
        max_setting -> Int4,
        created_at -> Timestamp,
    }
}

diesel::joinable!(brewing_sessions -> coffee_beans (bean_id));
diesel::joinable!(brewing_sessions -> grinders (grinder_id));

diesel::allow_tables_to_appear_in_same_query!(brewing_sessions, coffee_beans, grinders,);
