-- Create the materialized view with a deterministic ID
CREATE MATERIALIZED VIEW entities_caches AS
-- For each location of each parent, get a row with the parent and its location flattened
WITH transitive_locations AS (
    SELECT
        ee.child_id,
        e.id AS parent_id,
        e.display_name AS parent_display_name,
        parent_location.value,
        parent_location.ordinality AS location_index
    FROM entities_entities ee
    JOIN entities e ON ee.parent_id = e.id
    -- Join the locations from the array of locations
    LEFT JOIN LATERAL (
        SELECT value, ordinality
        FROM jsonb_array_elements(e.locations) WITH ORDINALITY AS location(value, ordinality)
    ) AS parent_location ON true
),
-- For each location of each entity, get a row with the entity and its location
direct_locations AS (
    SELECT
        e.id AS entity_id,
        e.category_id,
        e.display_name,
        c.family_id,
        location.value as location,
        location.ordinality AS location_index,
        array_remove(array_agg(DISTINCT et.tag_id) || array_agg(DISTINCT cet.tag_id), NULL) AS tags_ids,
        array_agg(DISTINCT e2.category_id) FILTER (WHERE e2.category_id IS NOT NULL) AS child_categories_ids
    FROM entities e
    JOIN categories c ON e.category_id = c.id
    LEFT JOIN entity_tags et ON e.id = et.entity_id
    LEFT JOIN entities_entities ee ON e.id = ee.parent_id
    LEFT JOIN entities e2 ON ee.child_id = e2.id
    LEFT JOIN entity_tags cet ON ee.child_id = cet.entity_id
    LEFT JOIN LATERAL (
        SELECT value, ordinality
        FROM jsonb_array_elements(e.locations) WITH ORDINALITY AS location(value, ordinality)
    ) AS location ON true
    WHERE e.moderated_at IS NOT NULL AND e.hide_from_map = FALSE
    GROUP BY e.id, c.family_id, e.display_name, e.category_id, location.value, location.ordinality
)
-- Add the entities with their locations to the materialized view
SELECT
    md5(dl.entity_id::text || COALESCE(dl.location_index, -1)::text || 'alone_loc')::uuid AS id,
    dl.entity_id,
    dl.category_id,
    dl.display_name,
    dl.family_id,
    dl.location_index,
    (dl.location ->> 'long')::double precision AS longitude,
    (dl.location ->> 'lat')::double precision AS latitude,
    ST_Transform(ST_SetSRID(ST_MakePoint((dl.location ->> 'long')::double precision, (dl.location ->> 'lat')::double precision), 4326), 3857) AS web_mercator_location,
    dl.location ->> 'plain_text' AS plain_text_location,
    dl.tags_ids,
    array_append(dl.child_categories_ids, dl.category_id) AS categories_ids,
    NULL AS parent_id,
    NULL AS parent_display_name,
    to_tsvector('english', dl.display_name) AS full_text_search_ts
FROM direct_locations dl

UNION

-- Add the entities with their parents locations to the materialized view
SELECT
    md5(tl.child_id::text || tl.parent_id::text || tl.location_index::text || 'with_parent')::uuid AS id,
    tl.child_id AS entity_id,
    dl.category_id,
    dl.display_name,
    dl.family_id,
    tl.location_index,
    (tl.value ->> 'long')::double precision AS longitude,
    (tl.value ->> 'lat')::double precision AS latitude,
    ST_Transform(ST_SetSRID(ST_MakePoint((tl.value ->> 'long')::double precision, (tl.value ->> 'lat')::double precision), 4326), 3857) AS web_mercator_location,
    tl.value ->> 'plain_text' AS plain_text_location,
    dl.tags_ids,
    array_append(dl.child_categories_ids, dl.category_id) AS categories_ids,
    tl.parent_id,
    tl.parent_display_name,
    to_tsvector('english', dl.display_name) AS full_text_search_ts
FROM transitive_locations tl
JOIN direct_locations dl ON tl.child_id = dl.entity_id;

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
