-- Create the materialized view with a deterministic ID
CREATE MATERIALIZED VIEW entities_caches AS
WITH parent_entities AS (
    SELECT
        ee.child_id,
        e.id AS parent_id,
        e.display_name AS parent_display_name,
        parent_location.value,
        parent_location.ordinality AS location_index
    FROM entities_entities ee
    JOIN entities e ON ee.parent_id = e.id,
    LATERAL jsonb_array_elements(e.locations) WITH ORDINALITY AS parent_location(value, ordinality)
),
entity_locations AS (
    SELECT
        e.id AS entity_id,
        e.category_id,
        e.display_name,
        c.family_id,
        location.value as location,
        location.ordinality AS location_index,
        array_remove(array_agg(DISTINCT et.tag_id), NULL) AS tags_ids,
        array_agg(DISTINCT e2.category_id) FILTER (WHERE e2.category_id IS NOT NULL) AS child_categories_ids
    FROM entities e
    JOIN categories c ON e.category_id = c.id
    LEFT JOIN entity_tags et ON e.id = et.entity_id
    LEFT JOIN entities_entities ee ON e.id = ee.parent_id
    LEFT JOIN entities e2 ON ee.child_id = e2.id,
    LATERAL jsonb_array_elements(e.locations) WITH ORDINALITY AS location(value, ordinality)
    WHERE e.moderated_at IS NOT NULL AND e.hide_from_map = FALSE
    GROUP BY e.id, c.family_id, e.display_name, e.category_id, location.value, location.ordinality
)
-- Add the entities with their locations to the materialized view
SELECT
    md5(el.entity_id::text || el.location_index::text || 'alone_loc')::uuid AS id,
    el.entity_id,
    el.category_id,
    el.display_name,
    el.family_id,
    el.location_index,
    (el.location ->> 'long')::double precision AS longitude,
    (el.location ->> 'lat')::double precision AS latitude,
    ST_Transform(ST_SetSRID(ST_MakePoint((el.location ->> 'long')::double precision, (el.location ->> 'lat')::double precision), 4326), 3857) AS web_mercator_location,
    el.location ->> 'plain_text' AS plain_text_location,
    el.tags_ids,
    array_append(el.child_categories_ids, el.category_id) AS categories_ids,
    NULL AS parent_id,
    NULL AS parent_display_name,
    to_tsvector('english', el.display_name || ' ' || array_to_string(array(
        SELECT t.title FROM tags t WHERE t.id = ANY(el.tags_ids) AND t.is_indexed
    ), ' ')) AS full_text_search_ts
FROM entity_locations el

UNION

-- Add the entities with their parents locations to the materialized view
SELECT
    md5(pe.child_id::text || pe.parent_id::text || pe.location_index::text || 'with_parent')::uuid AS id,
    pe.child_id AS entity_id,
    el.category_id,
    el.display_name,
    el.family_id,
    pe.location_index,
    (pe.value ->> 'long')::double precision AS longitude,
    (pe.value ->> 'lat')::double precision AS latitude,
    ST_Transform(ST_SetSRID(ST_MakePoint((pe.value ->> 'long')::double precision, (pe.value ->> 'lat')::double precision), 4326), 3857) AS web_mercator_location,
    pe.value ->> 'plain_text' AS plain_text_location,
    el.tags_ids,
    array_append(el.child_categories_ids, el.category_id) AS categories_ids,
    pe.parent_id,
    pe.parent_display_name,
    to_tsvector('english', el.display_name || ' ' || array_to_string(array(
        SELECT t.title FROM tags t WHERE t.id = ANY(el.tags_ids) AND t.is_indexed
    ), ' ')) AS full_text_search_ts
FROM parent_entities pe
JOIN entity_locations el ON pe.child_id = el.entity_id

UNION

-- Add the entities without locations to the materialized view
SELECT
    md5(e.id::text || 'alone_no_loc')::uuid AS id,
    e.id AS entity_id,
    e.category_id,
    e.display_name,
    c.family_id,
    0 AS location_index,
    NULL AS longitude,
    NULL AS latitude,
    NULL AS web_mercator_location,
    NULL AS plain_text_location,
    array_remove(array_agg(DISTINCT et.tag_id), NULL) AS tags_ids,
    array_append(array_agg(DISTINCT e2.category_id) FILTER (WHERE e2.category_id IS NOT NULL), e.category_id) AS categories_ids,
    NULL AS parent_id,
    NULL AS parent_display_name,
    to_tsvector('english', e.display_name || ' ' || array_to_string(array(
        SELECT t.title FROM tags t WHERE t.id = ANY(array_remove(array_agg(et.tag_id), NULL)) AND t.is_indexed
    ), ' ')) AS full_text_search_ts
FROM entities e
JOIN categories c ON e.category_id = c.id
LEFT JOIN entity_tags et ON e.id = et.entity_id
LEFT JOIN entities_entities ee ON e.id = ee.parent_id
LEFT JOIN entities e2 ON ee.child_id = e2.id
WHERE e.moderated_at IS NOT NULL AND e.hide_from_map = FALSE AND jsonb_array_length(e.locations) = 0
GROUP BY e.id, c.family_id, e.display_name, e.category_id;

-- Create unique index on ID for concurrency refresh
CREATE UNIQUE INDEX entities_caches_id_idx ON entities_caches(id);

-- Create indexes on the materialized view
CREATE INDEX entities_caches_entity_id_idx ON entities_caches(entity_id);
CREATE INDEX entities_caches_category_id_idx ON entities_caches(category_id);
CREATE INDEX entities_caches_gps_location_idx ON entities_caches USING GIST((ST_SetSRID(ST_MakePoint(longitude, latitude), 4326)));
CREATE INDEX entities_caches_web_mercator_location_idx ON entities_caches USING GIST(web_mercator_location);
CREATE INDEX entities_caches_full_text_search_idx ON entities_caches USING GIN(full_text_search_ts);

-- Function to refresh the materialized view
CREATE OR REPLACE FUNCTION refresh_entities_caches() RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY entities_caches;
END;
$$ LANGUAGE plpgsql;

-- Trigger to refresh the materialized view
CREATE OR REPLACE FUNCTION trigger_refresh_entities_caches()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM refresh_entities_caches();
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- `entities` table triggers
CREATE TRIGGER refresh_entities_caches_on_insert
AFTER INSERT ON entities
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_update
AFTER UPDATE ON entities
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_delete
AFTER DELETE ON entities
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

-- `tags` table triggers
CREATE TRIGGER refresh_entities_caches_on_tags_insert
AFTER INSERT ON tags
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_tags_update
AFTER UPDATE ON tags
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_tags_delete
AFTER DELETE ON tags
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

-- `families` table triggers
CREATE TRIGGER refresh_entities_caches_on_families_insert
AFTER INSERT ON families
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_families_update
AFTER UPDATE ON families
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_families_delete
AFTER DELETE ON families
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

-- `categories` table triggers
CREATE TRIGGER refresh_entities_caches_on_categories_insert
AFTER INSERT ON categories
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_categories_update
AFTER UPDATE ON categories
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_categories_delete
AFTER DELETE ON categories
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

-- `entity_tags` table triggers
CREATE TRIGGER refresh_entities_caches_on_entity_tags_insert
AFTER INSERT ON entity_tags
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_entity_tags_delete
AFTER DELETE ON entity_tags
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

-- `entities_entities` table triggers
CREATE TRIGGER refresh_entities_caches_on_entities_entities_insert
AFTER INSERT ON entities_entities
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();

CREATE TRIGGER refresh_entities_caches_on_entities_entities_delete
AFTER DELETE ON entities_entities
FOR EACH STATEMENT
EXECUTE FUNCTION trigger_refresh_entities_caches();
