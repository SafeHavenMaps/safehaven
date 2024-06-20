CREATE OR REPLACE FUNCTION fetch_entities_within_view(
    input_xmin DOUBLE PRECISION,
    input_ymin DOUBLE PRECISION,
    input_xmax DOUBLE PRECISION,
    input_ymax DOUBLE PRECISION,
    input_family_id UUID,

    allow_all_categories BOOL,
    allow_all_tags BOOL,
    categories_list UUID[],
    tags_list UUID[],
    exclude_categories_list UUID[],
    exclude_tags_list UUID[],

    cluster_eps DOUBLE PRECISION,
    cluster_min_points INT,

    active_categories_ids UUID[],
    required_tags_ids UUID[],
    excluded_tags_ids UUID[]
) RETURNS TABLE (
    id UUID,
    entity_id UUID,
    category_id UUID,
    tags_ids UUID[],
    family_id UUID,
    display_name TEXT,
    parent_id UUID,
    parent_display_name TEXT,
    web_mercator_x DOUBLE PRECISION,
    web_mercator_y DOUBLE PRECISION,
    plain_text_location TEXT,
    cluster_id INT,
    cluster_center_x DOUBLE PRECISION,
    cluster_center_y DOUBLE PRECISION
) AS $$
BEGIN
    RETURN QUERY
    WITH filtered_entities AS (
        SELECT ec.id,
            ec.entity_id,
            ec.category_id,
            ec.tags_ids,
            ec.family_id,
            ec.display_name,
            ec.parent_id,
            ec.parent_display_name,
            ec.web_mercator_location,
            ec.plain_text_location
        FROM entities_caches ec
        WHERE
            ST_Intersects(
                ec.web_mercator_location,
                ST_MakeEnvelope(input_xmin, input_ymin, input_xmax, input_ymax, 3857)
            )
            AND NOT ec.hidden
            AND ec.family_id = input_family_id
            -- Categories filter
            AND (allow_all_categories OR ec.category_id = ANY(categories_list))
            AND NOT (ec.category_id = ANY(exclude_categories_list))
            -- Tags filter
            AND (allow_all_tags OR ec.tags_ids && tags_list)
            AND NOT (ec.tags_ids && exclude_tags_list)
            -- User filters
            AND (ec.category_id = ANY(active_categories_ids))
            AND (array_length(required_tags_ids, 1) = 0 OR required_tags_ids <@ ec.tags_ids)
            AND NOT (ec.tags_ids && excluded_tags_ids)
    ),
    parent_entities AS (
        SELECT
            DISTINCT e.id,
            e.entity_id,
            e.category_id,
            e.tags_ids,
            e.family_id,
            e.display_name,
            e.parent_id,
            e.parent_display_name,
            e.web_mercator_location,
            e.plain_text_location
        FROM entities_caches e
        WHERE e.entity_id IN (SELECT DISTINCT fe.parent_id FROM filtered_entities fe)
    ),
    combined_entities AS (
        SELECT * FROM filtered_entities fe WHERE fe.parent_id IS NULL
        UNION
        SELECT * FROM parent_entities
    ),
    clustered_entities AS (
        SELECT
            ce.*,
            CASE WHEN cluster_eps > 0 AND cluster_min_points > 0 THEN
                ST_ClusterDBSCAN(ce.web_mercator_location, cluster_eps, cluster_min_points) OVER()
            END AS cluster_id
        FROM combined_entities ce
    ),
    clusters AS (
        SELECT
            ce.cluster_id,
            AVG(ST_X(ce.web_mercator_location)) AS cluster_center_x,
            AVG(ST_Y(ce.web_mercator_location)) AS cluster_center_y
        FROM clustered_entities ce
        WHERE ce.cluster_id IS NOT NULL
        GROUP BY ce.cluster_id
    )
    SELECT
        ce.id,
        ce.entity_id,
        ce.category_id,
        ce.tags_ids,
        ce.family_id,
        ce.display_name,
        ce.parent_id,
        ce.parent_display_name,
        ST_X(ce.web_mercator_location) AS web_mercator_x,
        ST_Y(ce.web_mercator_location) AS web_mercator_y,
        ce.plain_text_location,
        ce.cluster_id,
        cl.cluster_center_x,
        cl.cluster_center_y
    FROM clustered_entities ce
    LEFT JOIN clusters cl ON ce.cluster_id = cl.cluster_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION search_entities(
    search_query TEXT,
    input_family_id UUID,

    allow_all_categories BOOL,
    allow_all_tags BOOL,

    categories_list UUID[],
    tags_list UUID[],

    exclude_categories_list UUID[],
    exclude_tags_list UUID[],

    current_page BIGINT,
    page_size BIGINT,

    active_categories_ids UUID[],
    required_tags_ids UUID[],
    excluded_tags_ids UUID[],

    require_locations BOOL
) RETURNS TABLE (
    id UUID,
    entity_id UUID,
    category_id UUID,
    tags_ids UUID[],
    family_id UUID,
    display_name TEXT,
    parent_id UUID,
    parent_display_name TEXT,
    web_mercator_x DOUBLE PRECISION,
    web_mercator_y DOUBLE PRECISION,
    plain_text_location TEXT,
    total_results BIGINT,
    total_pages BIGINT,
    response_current_page BIGINT
) AS $$
BEGIN
    RETURN QUERY
    WITH filtered_entities AS (
        SELECT ec.*
        FROM entities_caches ec
        WHERE
            (
                search_query IS NULL
                OR search_query = ''
                OR (full_text_search_ts @@ plainto_tsquery(search_query))
            )
            AND ec.family_id = input_family_id
            AND NOT ec.hidden
            -- Categories
            AND (allow_all_categories OR ec.category_id = ANY(categories_list))
            AND NOT (ec.category_id = ANY(exclude_categories_list))
            -- Tags
            AND (allow_all_tags OR (ec.tags_ids && tags_list))
            AND NOT (ec.tags_ids && exclude_tags_list)
            -- User filters
            AND (ec.category_id = ANY(active_categories_ids))
            AND (array_length(required_tags_ids, 1) = 0 OR required_tags_ids <@ ec.tags_ids)
            AND NOT (ec.tags_ids && excluded_tags_ids)
            -- If we do not require locations, we only return entities with locations
            AND (NOT require_locations OR ec.web_mercator_location IS NOT NULL)
        ORDER BY
            ts_rank(full_text_search_ts, plainto_tsquery(search_query)) DESC,
            (ec.web_mercator_location IS NOT NULL) DESC -- prioritize entities with locations
    ),
    distinct_entities AS (
        SELECT DISTINCT ON (ec.entity_id) ec.*
        FROM filtered_entities ec
    ),
    total_count AS (
        SELECT COUNT(*) AS total_results FROM distinct_entities
    ),
    paginated_results AS (
        SELECT
            ec.id,
            ec.entity_id,
            ec.category_id,
            ec.tags_ids,
            ec.family_id,
            ec.display_name,
            ec.parent_id,
            ec.parent_display_name,
            ST_X(ec.web_mercator_location) AS web_mercator_x,
            ST_Y(ec.web_mercator_location) AS web_mercator_y,
            ec.plain_text_location,
            tc.total_results,
            CEIL(tc.total_results / page_size::FLOAT)::BIGINT AS total_pages,
            current_page as response_current_page
        FROM distinct_entities ec, total_count tc
        LIMIT page_size
        OFFSET (current_page - 1) * page_size
    )
    SELECT * FROM paginated_results;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION search_entities_admin(
    search_query TEXT,
    input_family_id UUID,

    current_page BIGINT,
    page_size BIGINT,

    active_categories_ids UUID[],
    required_tags_ids UUID[],
    excluded_tags_ids UUID[]
) RETURNS TABLE (
    id UUID,
    entity_id UUID,
    category_id UUID,
    tags_ids UUID[],
    family_id UUID,
    display_name TEXT,
    hidden BOOL,

    total_results BIGINT,
    total_pages BIGINT,
    response_current_page BIGINT
) AS $$
BEGIN
    RETURN QUERY
    WITH filtered_entities AS (
        SELECT ec.*
        FROM entities_caches ec
        WHERE
            (
                search_query IS NULL
                OR search_query = ''
                OR (full_text_search_ts @@ plainto_tsquery(search_query))
            )
            AND ec.family_id = input_family_id
            AND (ec.category_id = ANY(active_categories_ids))
            AND (array_length(required_tags_ids, 1) = 0 OR required_tags_ids <@ ec.tags_ids)
            AND NOT (ec.tags_ids && excluded_tags_ids)
        ORDER BY
            ts_rank(full_text_search_ts, plainto_tsquery(search_query)) DESC
    ),
    distinct_entities AS (
        SELECT DISTINCT ON (ec.entity_id) ec.*
        FROM filtered_entities ec
    ),
    total_count AS (
        SELECT COUNT(*) AS total_results FROM distinct_entities
    ),
    paginated_results AS (
        SELECT
            ec.id,
            ec.entity_id,
            ec.category_id,
            ec.tags_ids,
            ec.family_id,
            ec.display_name,
            ec.hidden,

            tc.total_results,
            CEIL(tc.total_results / page_size::FLOAT)::BIGINT AS total_pages,
            current_page as response_current_page
        FROM distinct_entities ec, total_count tc
        LIMIT page_size
        OFFSET (current_page - 1) * page_size
    )
    SELECT * FROM paginated_results;
END;
$$ LANGUAGE plpgsql;
