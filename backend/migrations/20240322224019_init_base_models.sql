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
    version INT NOT NULL DEFAULT 1,

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
    version INT NOT NULL DEFAULT 1,

    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (icon_id) REFERENCES icons(id) ON DELETE SET NULL
);
CREATE INDEX categories_family_id_idx ON categories(family_id);

CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    is_filter BOOLEAN NOT NULL DEFAULT FALSE,
    default_filter_status BOOLEAN NOT NULL DEFAULT TRUE,
    filter_description TEXT,
    fill_color VARCHAR(7) NOT NULL DEFAULT '#E86BA7',
    border_color VARCHAR(7) NOT NULL DEFAULT '#E696BA',

    version INT NOT NULL DEFAULT 1
);

CREATE TABLE entities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    display_name TEXT NOT NULL,
    category_id UUID NOT NULL,
    locations JSONB NOT NULL DEFAULT '[]',
    data JSONB NOT NULL,
    hidden BOOLEAN NOT NULL DEFAULT FALSE,
    moderation_notes TEXT,
    moderated BOOLEAN NOT NULL DEFAULT FALSE,
    version INT NOT NULL DEFAULT 1,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);
CREATE INDEX entities_category_id_idx ON entities(category_id);
CREATE INDEX entities_moderated_hidden_idx ON entities(moderated, hidden);

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
    moderated BOOLEAN NOT NULL DEFAULT FALSE,
    version INT NOT NULL DEFAULT 1,

    FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE
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

-- Prevent adding a parent as a child and vice versa
CREATE OR REPLACE FUNCTION prevent_parent_as_child()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if the entity to be added as a child is already a parent
    IF EXISTS (
        SELECT 1 FROM entities_entities
        WHERE parent_id = NEW.child_id
    ) THEN
        RAISE EXCEPTION 'sh_code_parent_as_child_error';
    END IF;

    -- Check if the entity to be added as a parent is already a child
    IF EXISTS (
        SELECT 1 FROM entities_entities
        WHERE child_id = NEW.parent_id
    ) THEN
        RAISE EXCEPTION 'sh_code_child_as_parent_error';
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

-- Trigger to check and increment the version number
CREATE OR REPLACE FUNCTION check_and_increment_version()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.version != OLD.version THEN
        RAISE EXCEPTION 'sh_code_version_mismatch';
    END IF;
    NEW.version := OLD.version + 1;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for entities
CREATE TRIGGER check_and_increment_version_entities
BEFORE UPDATE ON entities
FOR EACH ROW
EXECUTE FUNCTION check_and_increment_version();

-- Trigger for categories
CREATE TRIGGER check_and_increment_version_categories
BEFORE UPDATE ON categories
FOR EACH ROW
EXECUTE FUNCTION check_and_increment_version();

-- Trigger for tags
CREATE TRIGGER check_and_increment_version_tags
BEFORE UPDATE ON tags
FOR EACH ROW
EXECUTE FUNCTION check_and_increment_version();

-- Trigger for families
CREATE TRIGGER check_and_increment_version_families
BEFORE UPDATE ON families
FOR EACH ROW
EXECUTE FUNCTION check_and_increment_version();
