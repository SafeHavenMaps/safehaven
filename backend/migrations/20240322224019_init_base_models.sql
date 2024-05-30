-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "postgis";

-- Icons management
CREATE TABLE icons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    data BYTEA NOT NULL,
    hash CHAR(32) GENERATED ALWAYS AS (md5(data)) STORED,
    http_mime_type TEXT NOT NULL
);
CREATE INDEX icons_hash ON icons(hash);

-- Icons upsert function
CREATE OR REPLACE FUNCTION upsert_entity_icon(
    p_entity_id UUID,
    p_data BYTEA,
    p_http_mime_type TEXT,
    p_table_name TEXT
) RETURNS VOID AS $$
DECLARE
    v_icon_id UUID;
    v_query TEXT;
BEGIN
    -- Check if the entity already has an icon
    EXECUTE format('SELECT icon_id FROM %I WHERE id = $1', p_table_name) INTO v_icon_id USING p_entity_id;

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

        -- Update the entity with the new icon_id
        EXECUTE format('UPDATE %I SET icon_id = $1 WHERE id = $2', p_table_name) USING v_icon_id, p_entity_id;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    last_login TIMESTAMP
);
CREATE UNIQUE INDEX users_name_idx ON users(name);

CREATE TABLE options (
    name VARCHAR(255) NOT NULL,
    value JSONB NOT NULL
);
CREATE UNIQUE INDEX options_name_uindex ON options (name);

CREATE TABLE families (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    icon_id UUID,
    entity_form JSONB NOT NULL,
    comment_form JSONB NOT NULL,
    sort_order INT NOT NULL DEFAULT 0,

    FOREIGN KEY (icon_id) REFERENCES icons(id) ON DELETE SET NULL
);

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    family_id UUID NOT NULL,
    default_status BOOLEAN NOT NULL DEFAULT TRUE,
    icon_id UUID,
    fill_color VARCHAR(7) NOT NULL DEFAULT '#FFFFFF',
    border_color VARCHAR(7) NOT NULL DEFAULT '#000000',

    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (icon_id) REFERENCES icons(id) ON DELETE SET NULL
);
CREATE INDEX categories_family_id_idx ON categories(family_id);

CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    is_filter BOOLEAN NOT NULL DEFAULT FALSE,
    default_filter_status BOOLEAN NOT NULL DEFAULT TRUE,
    filter_description TEXT
);

CREATE TABLE entities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    display_name TEXT NOT NULL,
    category_id UUID NOT NULL,
    locations JSONB NOT NULL DEFAULT '[]',
    data JSONB NOT NULL,
    hide_from_map BOOLEAN NOT NULL DEFAULT FALSE,
    moderation_notes TEXT,
    moderated_at TIMESTAMP,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_update_by UUID,

    full_text_search_ts TSVECTOR GENERATED ALWAYS AS (to_tsvector('english', display_name)) STORED,

    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE,
    FOREIGN KEY (last_update_by) REFERENCES users(id) ON DELETE SET NULL
);
CREATE INDEX entities_category_id_idx ON entities(category_id);
CREATE INDEX entities_moderated_at_hide_from_map_idx ON entities(moderated_at, hide_from_map);
CREATE INDEX entities_full_text_search_idx ON entities USING GIN(full_text_search_ts);

CREATE TABLE entity_tags (
    entity_id UUID NOT NULL,
    tag_id UUID NOT NULL,

    PRIMARY KEY (entity_id, tag_id),

    FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
CREATE INDEX entity_tags_entity_id_idx ON entity_tags(entity_id);
CREATE INDEX entity_tags_tag_id_idx ON entity_tags(tag_id);
CREATE INDEX entity_tags_entity_tag_idx ON entity_tags(entity_id, tag_id);

CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_id UUID NOT NULL,
    author TEXT NOT NULL,
    text TEXT NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    moderated_at TIMESTAMP,
    moderated_by UUID,

    FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE,
    FOREIGN KEY (moderated_by) REFERENCES users(id) ON DELETE SET NULL
);
CREATE INDEX comments_entity_id_idx ON comments(entity_id);

CREATE TABLE access_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title TEXT NOT NULL,
    token VARCHAR(64) NOT NULL,
    permissions JSONB NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Table to track visits to access tokens
CREATE TABLE access_tokens_visits (
    token_id UUID NOT NULL,
    visited_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    referrer TEXT,

    FOREIGN KEY (token_id) REFERENCES access_tokens(id) ON DELETE CASCADE
);
CREATE INDEX ON access_tokens_visits (token_id);
CREATE INDEX ON access_tokens_visits (visited_at);

-- Many-to-many table for entities
-- since an entity can have multiple parents and children
CREATE TABLE entities_entities (
    parent_id UUID NOT NULL,
    child_id UUID NOT NULL,

    PRIMARY KEY (parent_id, child_id),

    FOREIGN KEY (parent_id) REFERENCES entities(id) ON DELETE CASCADE,
    FOREIGN KEY (child_id) REFERENCES entities(id) ON DELETE CASCADE
);
CREATE INDEX entities_entities_parent_id_idx ON entities_entities(parent_id);
CREATE INDEX entities_entities_child_id_idx ON entities_entities(child_id);

CREATE OR REPLACE FUNCTION prevent_parent_as_child()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if the entity to be added as a child is already a parent
    IF EXISTS (
        SELECT 1 FROM entities_entities
        WHERE parent_id = NEW.child_id
    ) THEN
        RAISE EXCEPTION 'An entity that is already a parent cannot be a child.';
    END IF;

    -- Check if the entity to be added as a parent is already a child
    IF EXISTS (
        SELECT 1 FROM entities_entities
        WHERE child_id = NEW.parent_id
    ) THEN
        RAISE EXCEPTION 'An entity that is already a child cannot be a parent.';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER prevent_parent_as_child_trigger
BEFORE INSERT OR UPDATE ON entities_entities
FOR EACH ROW EXECUTE FUNCTION prevent_parent_as_child();

--- Trigger to update the last_update_at column

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_entities_updated_at
BEFORE UPDATE ON entities
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_comments_updated_at
BEFORE UPDATE ON comments
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();
