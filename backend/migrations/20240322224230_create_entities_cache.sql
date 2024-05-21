-- Add migration script here
-- CACHE CREATION AND LIFECYCLE --

-- Denormalized table for faster access
-- Each entry of this table is a point on a map
CREATE TABLE entities_caches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_id UUID NOT NULL,
    category_id UUID NOT NULL,
    categories_ids UUID[] NOT NULL,
    tags_ids UUID[] NOT NULL,
    family_id UUID NOT NULL,
    display_name TEXT NOT NULL,
    parent_id UUID,
    parent_display_name TEXT,
    gps_location GEOGRAPHY(POINT, 4326) NOT NULL,
    web_mercator_location GEOMETRY(POINT, 3857) NOT NULL,
    plain_text_location TEXT NOT NULL,
    full_text_search TEXT NOT NULL,
    full_text_search_ts TSVECTOR GENERATED ALWAYS AS (to_tsvector('english', full_text_search)) STORED,

    FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE
);
CREATE INDEX entities_caches_gps_location_idx ON entities_caches USING GIST(gps_location);
CREATE INDEX entities_caches_web_mercator_location__idx ON entities_caches USING GIST(web_mercator_location);
CREATE INDEX entities_caches_full_text_search_idx ON entities_caches USING GIN(full_text_search_ts);

-- Refresh the cache for a given entity
-- Parameters:
--   entity_id: the id of the entity to refresh
--   recurse: whether to refresh the cache of the parents of the entity
-- The cache is generated using the following elements:
--   - The entity's data
--   - The entity's category
--   - The category's family
--   - The entity's tags
--   - The entity's children
-- Since we depend on children and we could be a child of multiple entities,
-- we need to refresh the cache of all parents
CREATE OR REPLACE FUNCTION refresh_entity_cache(given_entity_id UUID, recurse BOOL) RETURNS void AS $$
DECLARE
    refreshed_entity RECORD;
    family_id UUID;
    family_entity_form JSONB;
    field JSONB;
    tag_title TEXT;
    indexed_values TEXT := '';
    categories_array UUID[];
    tags_array UUID[];
    parent_entity RECORD;
    parent_location JSONB;
BEGIN
    SELECT e.* INTO refreshed_entity FROM entities e WHERE e.id = given_entity_id;

    -- Delete existing cache entries for the entity
    DELETE FROM entities_caches WHERE entity_id = given_entity_id;

     -- Check if moderated_at is null or hide_from_map is true
    IF refreshed_entity.moderated_at IS NULL OR refreshed_entity.hide_from_map THEN
        IF recurse THEN
            PERFORM refresh_parents_cache(given_entity_id);
        END IF;
        RETURN;
    END IF;

    -- Get the family data for the given entity
    SELECT c.family_id, f.entity_form
        INTO family_id, family_entity_form
        FROM categories c
        JOIN families f on f.id = c.family_id
        WHERE c.id = refreshed_entity.category_id;

    -- Populate categories array
    SELECT array_agg(e.category_id)
        INTO categories_array
        FROM entities_entities ee
        JOIN entities e ON ee.child_id = e.id
        WHERE ee.parent_id = refreshed_entity.id
            AND e.moderated_at IS NOT NULL
            AND e.hide_from_map IS FALSE
        GROUP BY ee.parent_id;
    categories_array := array_prepend(refreshed_entity.category_id, categories_array);

    -- Parse the entity_form to get the indexed values
    FOR field IN SELECT * FROM jsonb_array_elements(family_entity_form->'fields') field
    LOOP
        IF (field->>'indexed')::boolean AND refreshed_entity.data ? (field->>'key') THEN
            indexed_values := indexed_values || ' ' || (refreshed_entity.data->>(field->>'key'))::text;
        END IF;
    END LOOP;

    -- Fetch and concatenate tag ids associated with the entity
    SELECT array_agg(et.tag_id)
        INTO tags_array
        FROM entity_tags et
        WHERE et.entity_id = refreshed_entity.id;
    IF tags_array IS NULL THEN
        tags_array := '{}';
    END IF;

    -- Fetch and concatenate tag titles associated with the entity
    FOR tag_title IN
        SELECT t.title
        FROM tags t
        JOIN entity_tags et ON t.id = et.tag_id
        WHERE t.is_indexed AND et.entity_id = refreshed_entity.id
    LOOP
        indexed_values := indexed_values || ' ' || tag_title;
    END LOOP;

    -- Concatenate display_name at the beginning
    indexed_values := refreshed_entity.display_name || ' ' || indexed_values;

    -- Insert new entries for each location in the locations array
    FOR i IN 0..jsonb_array_length(refreshed_entity.locations)-1 LOOP
        INSERT INTO entities_caches (
            id,
            entity_id,
            category_id,
            categories_ids,
            tags_ids,
            family_id,
            display_name,
            gps_location,
            web_mercator_location,
            plain_text_location,
            full_text_search
        )
        VALUES (
            uuid_generate_v4(),
            refreshed_entity.id,
            refreshed_entity.category_id,
            categories_array,
            tags_array,
            family_id,
            refreshed_entity.display_name,
            ST_SetSRID(
                ST_MakePoint(
                    (refreshed_entity.locations->i->>'long')::double precision,
                    (refreshed_entity.locations->i->>'lat')::double precision
                ),
                4326
            ),
            ST_Transform(
                ST_SetSRID(
                    ST_MakePoint(
                        (refreshed_entity.locations->i->>'long')::double precision,
                        (refreshed_entity.locations->i->>'lat')::double precision
                    ),
                    4326
                ),
                3857
            ),
            refreshed_entity.locations->i->>'plain_text',
            indexed_values
        );
    END LOOP;

    -- Insert new entries for each location of each parent of the entity
    FOR parent_entity IN
        SELECT e.*
        FROM entities_entities ee
        JOIN entities e ON ee.parent_id = e.id
        WHERE ee.child_id = refreshed_entity.id
    LOOP
        FOR i IN 0..jsonb_array_length(parent_entity.locations)-1 LOOP
            INSERT INTO entities_caches (
                id,
                entity_id,
                category_id,
                categories_ids,
                tags_ids,
                family_id,
                display_name,
                parent_id,
                parent_display_name,
                gps_location,
                web_mercator_location,
                plain_text_location,
                full_text_search
            )
            VALUES (
                uuid_generate_v4(),
                refreshed_entity.id,
                refreshed_entity.category_id,
                categories_array,
                tags_array,
                family_id,
                refreshed_entity.display_name,
                parent_entity.id,
                parent_entity.display_name,
                ST_SetSRID(
                    ST_MakePoint(
                        (parent_entity.locations->i->>'long')::double precision,
                        (parent_entity.locations->i->>'lat')::double precision
                    ),
                    4326
                ),
                ST_Transform(
                    ST_SetSRID(
                        ST_MakePoint(
                            (parent_entity.locations->i->>'long')::double precision,
                            (parent_entity.locations->i->>'lat')::double precision
                        ),
                        4326
                    ),
                    3857
                ),
                parent_entity.locations->i->>'plain_text',
                indexed_values
            );
        END LOOP;
    END LOOP;

    -- For each entity that refreshed_entity is a child of, update the cache
    IF recurse THEN
        PERFORM refresh_parents_cache(given_entity_id);
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Refresh all parents of a given entity
CREATE OR REPLACE FUNCTION refresh_parents_cache(entity_id UUID) RETURNS void AS $$
DECLARE
    found_parent_id UUID;
BEGIN
    FOR found_parent_id IN SELECT parent_id FROM entities_entities WHERE child_id = entity_id LOOP
        PERFORM refresh_entity_cache(found_parent_id, false);
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Force refresh the cache for all entities
CREATE OR REPLACE FUNCTION refresh_all_entities_cache() RETURNS void AS $$
DECLARE
    entity_record RECORD;
BEGIN
    DELETE FROM entities_caches;
    FOR entity_record IN SELECT id FROM entities LOOP
        PERFORM refresh_entity_cache(entity_record.id, true);
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- ENTITIES TRIGGERS SECTION --

-- When an entity is created or updated, we should update the cache table
CREATE OR REPLACE FUNCTION insert_or_update_entity_cache()
RETURNS TRIGGER AS $$
BEGIN
    PERFORM refresh_entity_cache(NEW.id, true);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_entities_insert_or_update
AFTER INSERT OR UPDATE ON entities
FOR EACH ROW EXECUTE FUNCTION insert_or_update_entity_cache();

-- ENTITIES_ENTITIES TRIGGERS SECTION --

-- Update the cache for a an entity pair from entities_entities when it is created or updated
CREATE OR REPLACE FUNCTION insert_entities_entities_trigger_function() RETURNS TRIGGER AS $$
BEGIN
    PERFORM refresh_entity_cache(NEW.child_id, true);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_entities_entities_insert
AFTER INSERT ON entities_entities
FOR EACH ROW EXECUTE FUNCTION insert_entities_entities_trigger_function();

-- Update the cache for a an entity pair from entities_entities when one is deleted
CREATE OR REPLACE FUNCTION delete_entities_entities_trigger_function() RETURNS TRIGGER AS $$
BEGIN
    PERFORM refresh_entity_cache(OLD.parent_id, false);
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_entities_entities_delete
AFTER DELETE ON entities_entities
FOR EACH ROW EXECUTE FUNCTION delete_entities_entities_trigger_function();

-- FAMILIES TRIGGERS SECTION --

-- Update the cache for all entities that belong to a family when the family is updated
CREATE OR REPLACE FUNCTION refresh_family_entities_cache() RETURNS TRIGGER AS $$
DECLARE
    entity_id UUID;
BEGIN
    FOR entity_id IN
        SELECT e.id FROM entities e
        JOIN categories c ON e.category_id = c.id
        WHERE c.family_id = NEW.id
    LOOP
        PERFORM refresh_entity_cache(entity_id, false);
    END LOOP;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_family_update
AFTER UPDATE ON families
FOR EACH ROW EXECUTE FUNCTION refresh_family_entities_cache();

-- TAGS TRIGGERS SECTION --

-- Update the cache for all entities that have a given tag when the tag is updated
CREATE OR REPLACE FUNCTION refresh_tag_entities_cache() RETURNS TRIGGER AS $$
DECLARE
    found_entity_id UUID;
BEGIN
    FOR found_entity_id IN
        SELECT entity_id FROM entity_tags
        WHERE tag_id = NEW.id
    LOOP
        PERFORM refresh_entity_cache(found_entity_id, false);
    END LOOP;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_tag_update
AFTER UPDATE ON tags
FOR EACH ROW EXECUTE FUNCTION refresh_tag_entities_cache();

-- Update the cache for the entity related to a tag when the tag is linked to the entity
CREATE OR REPLACE FUNCTION refresh_entity_cache_on_tag_link_insertion() RETURNS TRIGGER AS $$
BEGIN
    PERFORM refresh_entity_cache(NEW.entity_id, false);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_entity_tag_link_insertion
AFTER INSERT ON entity_tags
FOR EACH ROW EXECUTE FUNCTION refresh_entity_cache_on_tag_link_insertion();

-- Update the cache for the entity related to a tag when the tag is removed from the entity
CREATE OR REPLACE FUNCTION refresh_entity_cache_on_tag_link_deletion() RETURNS TRIGGER AS $$
BEGIN
    PERFORM refresh_entity_cache(OLD.entity_id, false);
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_entity_tag_link_deletion
AFTER DELETE ON entity_tags
FOR EACH ROW EXECUTE FUNCTION refresh_entity_cache_on_tag_link_deletion();
