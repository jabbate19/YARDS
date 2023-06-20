-- Add migration script here
ALTER TABLE IF EXISTS public.dnszone
    ADD COLUMN dnsroot character varying(255) NOT NULL;