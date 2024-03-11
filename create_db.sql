-- Database: full_local_app_development

-- DROP DATABASE IF EXISTS full_local_app_development;

CREATE DATABASE full_local_app_development
    WITH
    OWNER = loco
    ENCODING = 'UTF8'
    TABLESPACE = pg_default
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;

GRANT TEMPORARY, CONNECT ON DATABASE full_local_app_development TO PUBLIC;

GRANT ALL ON DATABASE full_local_app_development TO loco;

ALTER DEFAULT PRIVILEGES FOR ROLE postgres
GRANT ALL ON TABLES TO loco;