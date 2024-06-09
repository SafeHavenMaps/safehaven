-- Icons upsert function
CREATE OR REPLACE FUNCTION upsert_row_icon(
    p_row_id UUID,
    p_data BYTEA,
    p_http_mime_type TEXT,
    p_table_name TEXT
) RETURNS VOID AS $$
DECLARE
    v_icon_id UUID;
    v_query TEXT;
BEGIN
    -- Check if the row already has an icon
    EXECUTE format('SELECT icon_id FROM %I WHERE id = $1', p_table_name) INTO v_icon_id USING p_row_id;

    IF v_icon_id IS NOT NULL THEN
        -- Update the existing icon
        UPDATE icons
        SET data = p_data, http_mime_type = p_http_mime_type
        WHERE id = v_icon_id;
    ELSE
        -- Insert a new icon
        INSERT INTO icons (data, http_mime_type)
        VALUES (p_data, p_http_mime_type)
        RETURNING id INTO v_icon_id;

        -- Update the row with the new icon_id
        EXECUTE format('UPDATE %I SET icon_id = $1 WHERE id = $2', p_table_name) USING v_icon_id, p_row_id;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Replace tags for entity
CREATE OR REPLACE FUNCTION replace_tags_for_entity(
    p_entity_id UUID,
    p_tags_ids UUID[]
) RETURNS VOID AS $$
BEGIN
    DELETE FROM entity_tags WHERE entity_id = p_entity_id;
    INSERT INTO entity_tags (entity_id, tag_id)
        SELECT p_entity_id, UNNEST(p_tags_ids);
END;
$$ LANGUAGE plpgsql;