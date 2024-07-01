UPDATE access_tokens
SET permissions = permissions || '{"can_list_entities": true, "can_access_entity": true, "can_add_entity": true, "can_add_comment": true}'::jsonb;
