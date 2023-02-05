// @generated automatically by Diesel CLI.

diesel::table! {
    descendants (id) {
        id -> Text,
        parent_id -> Nullable<Text>,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Uuid,
        name -> Text,
        normalized_ingredient_id -> Nullable<Text>,
    }
}

diesel::table! {
    normalized_ingredients (id) {
        id -> Text,
        parent_id -> Nullable<Text>,
        plural -> Nullable<Text>,
        category -> Nullable<Text>,
        is_kitchen_staple -> Bool,
        is_dairy_free -> Bool,
        is_gluten_free -> Bool,
        is_vegan -> Bool,
        is_vegetarian -> Bool,
    }
}

diesel::table! {
    parents (id) {
        id -> Text,
    }
}

diesel::table! {
    recipe_ingredients (ingredient_id, recipe_id) {
        ingredient_id -> Uuid,
        recipe_id -> Uuid,
    }
}

diesel::table! {
    recipes (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        author_name -> Nullable<Text>,
        image -> Nullable<Text>,
        total_time -> Nullable<Text>,
        recipe_yield -> Nullable<Text>,
        recipe_cusine -> Nullable<Text>,
        recipe_category -> Nullable<Text>,
        keywords -> Nullable<Text>,
        aggregate_rating_value -> Nullable<Float4>,
        aggregate_rating_count -> Nullable<Int4>,
        url_hostname -> Text,
        url_path -> Text,
        crawler_fetched_at -> Timestamp,
    }
}

diesel::joinable!(ingredients -> normalized_ingredients (normalized_ingredient_id));
diesel::joinable!(recipe_ingredients -> ingredients (ingredient_id));
diesel::joinable!(recipe_ingredients -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    descendants,
    ingredients,
    normalized_ingredients,
    parents,
    recipe_ingredients,
    recipes,
);
