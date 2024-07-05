-- Add migration script here
UPDATE access_tokens
SET permissions = permissions || '{"can_list_without_query": true, "can_list_with_filters": true, "can_list_with_enum_constraints": true}'::jsonb;
