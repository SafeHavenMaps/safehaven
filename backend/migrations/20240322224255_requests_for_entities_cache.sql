CREATE OR REPLACE FUNCTION fetch_entities_within_view(
    upper_left_lat DOUBLE PRECISION,
    upper_left_lon DOUBLE PRECISION,
    lower_right_lat DOUBLE PRECISION,
    lower_right_lon DOUBLE PRECISION,

    allow_all_families BOOL,
    allow_all_categories BOOL,
    allow_all_tags BOOL,

    families_list UUID[],
    categories_list UUID[],
    tags_list UUID[],

    exclude_families_list UUID[],
    exclude_categories_list UUID[],
    exclude_tags_list UUID[]
) RETURNS TABLE (
    id UUID,
    entity_id UUID,
    category_id UUID,
    categories_ids UUID[],
    tags_ids UUID[],
    family_id UUID,
    display_name TEXT,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    plain_text_location TEXT
) AS $$
BEGIN
    RETURN QUERY
        SELECT ec.id,
            ec.entity_id,
            ec.category_id,
            ec.categories_ids,
            ec.tags_ids,
            ec.family_id,
            ec.display_name,
            ST_Y(ec.location::geometry) AS latitude,
            ST_X(ec.location::geometry) AS longitude,
            ec.plain_text_location
        FROM entities_caches ec
        WHERE
            ST_Intersects(
                "location",
                ST_MakeEnvelope(
                    upper_left_lat,
                    upper_left_lon,
                    lower_right_lat,
                    lower_right_lon,
                    4326
                )
            )
            -- Families
            AND
            (allow_all_families OR (ec.family_id = ANY(families_list)))
            AND NOT (ec.family_id = ANY(exclude_families_list))
            -- Categories
            AND
            (allow_all_categories OR (ec.categories_ids && categories_list))
            AND NOT (ec.categories_ids && exclude_categories_list)
            -- Tags
            AND
            (allow_all_tags OR (ec.tags_ids && tags_list))
            AND NOT (ec.tags_ids && exclude_tags_list);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION search_entities(
    search_query TEXT,

    allow_all_families BOOL,
    allow_all_categories BOOL,
    allow_all_tags BOOL,

    families_list UUID[],
    categories_list UUID[],
    tags_list UUID[],

    exclude_families_list UUID[],
    exclude_categories_list UUID[],
    exclude_tags_list UUID[]
) RETURNS TABLE (
    id UUID,
    entity_id UUID,
    category_id UUID,
    categories_ids UUID[],
    tags_ids UUID[],
    family_id UUID,
    display_name TEXT,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    plain_text_location TEXT
) AS $$
BEGIN
    RETURN QUERY
        SELECT ec.id,
            ec.entity_id,
            ec.category_id,
            ec.categories_ids,
            ec.tags_ids,
            ec.family_id,
            ec.display_name,
            ST_Y(ec.location::geometry) AS latitude,
            ST_X(ec.location::geometry) AS longitude,
            ec.plain_text_location
        FROM entities_caches ec
        WHERE
            (full_text_search_ts @@ to_tsquery(search_query))
            -- Families
            AND
            (allow_all_families OR (ec.family_id = ANY(families_list)))
            AND NOT (ec.family_id = ANY(exclude_families_list))
            -- Categories
            AND
            (allow_all_categories OR (ec.categories_ids && categories_list))
            AND NOT (ec.categories_ids && exclude_categories_list)
            -- Tags
            AND
            (allow_all_tags OR (ec.tags_ids && tags_list))
            AND NOT (ec.tags_ids && exclude_tags_list);
END;
$$ LANGUAGE plpgsql;
