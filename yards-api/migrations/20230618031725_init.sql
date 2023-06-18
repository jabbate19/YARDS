--
-- PostgreSQL database dump
--

CREATE TYPE public.dnsrecordtype AS ENUM (
    'A',
    'AAAA',
    'NS',
    'MX',
    'CNAME',
    'SOA',
    'SRV',
    'PTR'
);


--
-- TOC entry 663 (class 1247 OID 29367203)
-- Name: iptype; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.iptype AS ENUM (
    'Static',
    'Dynamic'
);


--
-- TOC entry 667 (class 1247 OID 29367281)
-- Name: ipversion; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.ipversion AS ENUM (
    'v4',
    'v6',
    'V4',
    'V6'
);

--
-- TOC entry 207 (class 1259 OID 29367183)
-- Name: address; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.address (
    id integer NOT NULL,
    interfaceid integer NOT NULL,
    iprangeid integer NOT NULL,
    iptype public.iptype
);


--
-- TOC entry 217 (class 1259 OID 29633475)
-- Name: address_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.address ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.address_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 202 (class 1259 OID 29367060)
-- Name: apikey; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.apikey (
    id integer NOT NULL,
    name character varying(64) NOT NULL,
    keyhash character(256) NOT NULL
);


--
-- TOC entry 218 (class 1259 OID 29633477)
-- Name: apikey_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.apikey ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.apikey_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 212 (class 1259 OID 29367357)
-- Name: ddns; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.ddns (
    iprangeid integer NOT NULL,
    zoneid integer NOT NULL
);


--
-- TOC entry 205 (class 1259 OID 29367119)
-- Name: device; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.device (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    owner character varying(255) NOT NULL,
    comments character varying(255)
);


--
-- TOC entry 219 (class 1259 OID 29633488)
-- Name: device_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.device ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.device_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 214 (class 1259 OID 29367471)
-- Name: dhcprange; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.dhcprange (
    id integer NOT NULL,
    iprangeid integer NOT NULL,
    name character varying NOT NULL,
    dhcpstart character varying(255) NOT NULL,
    dhcpend character varying(255) NOT NULL,
    gateway character varying(255) NOT NULL,
    default_dns character varying(255) NOT NULL,
    lease_time integer NOT NULL,
    serverid integer NOT NULL
);


--
-- TOC entry 220 (class 1259 OID 29633490)
-- Name: dhcprange_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.dhcprange ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.dhcprange_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 213 (class 1259 OID 29367445)
-- Name: dnsrecord; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.dnsrecord (
    id integer NOT NULL,
    zoneid integer NOT NULL,
    key character varying(255) NOT NULL,
    ttl integer NOT NULL,
    value character varying(255) NOT NULL,
    recordtype public.dnsrecordtype NOT NULL
);


--
-- TOC entry 221 (class 1259 OID 29633492)
-- Name: dnsrecord_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.dnsrecord ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.dnsrecord_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 211 (class 1259 OID 29367329)
-- Name: dnszone; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.dnszone (
    id integer NOT NULL,
    zonename character varying(255) NOT NULL,
    serverid integer NOT NULL
);


--
-- TOC entry 222 (class 1259 OID 29633494)
-- Name: dnszone_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.dnszone ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.dnszone_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 206 (class 1259 OID 29367146)
-- Name: interface; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.interface (
    id integer NOT NULL,
    macaddr character(17) NOT NULL,
    deviceid integer NOT NULL,
    name character varying(255) NOT NULL,
    comments character varying(255)
);


--
-- TOC entry 223 (class 1259 OID 29633496)
-- Name: interface_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.interface ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.interface_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 209 (class 1259 OID 29367295)
-- Name: iprange; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.iprange (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    ipversion public.ipversion NOT NULL,
    networkid character varying(15) NOT NULL,
    cidr integer NOT NULL,
    description character varying(255) NOT NULL
);


--
-- TOC entry 216 (class 1259 OID 29631780)
-- Name: iprange_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.iprange ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.iprange_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 203 (class 1259 OID 29367074)
-- Name: keypermissions; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.keypermissions (
    id integer NOT NULL,
    keyid integer NOT NULL,
    permission character varying NOT NULL
);


--
-- TOC entry 224 (class 1259 OID 29633507)
-- Name: keypermissions_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.keypermissions ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.keypermissions_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 204 (class 1259 OID 29367096)
-- Name: logs; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.logs (
    id integer NOT NULL,
    "timestamp" timestamp with time zone NOT NULL,
    message character varying(255)
);


--
-- TOC entry 225 (class 1259 OID 29633509)
-- Name: logs_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.logs ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.logs_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 210 (class 1259 OID 29367312)
-- Name: server; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.server (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    tokenhash character varying(256),
    lastcheckin timestamp with time zone
);


--
-- TOC entry 215 (class 1259 OID 29629994)
-- Name: server_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

ALTER TABLE public.server ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.server_id_seq
    START WITH 0
    INCREMENT BY 1
    MINVALUE 0
    NO MAXVALUE
    CACHE 1
);


--
-- TOC entry 208 (class 1259 OID 29367252)
-- Name: staticaddress; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.staticaddress (
    addressid integer NOT NULL,
    ipaddr character varying(15) NOT NULL
);


--
-- TOC entry 3644 (class 2606 OID 29367064)
-- Name: apikey APIKey_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.apikey
    ADD CONSTRAINT "APIKey_pkey" PRIMARY KEY (id);


--
-- TOC entry 3646 (class 2606 OID 29367081)
-- Name: keypermissions KeyPermissions_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.keypermissions
    ADD CONSTRAINT "KeyPermissions_pkey" PRIMARY KEY (id);


--
-- TOC entry 3648 (class 2606 OID 29367100)
-- Name: logs Logs_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.logs
    ADD CONSTRAINT "Logs_pkey" PRIMARY KEY (id);


--
-- TOC entry 3654 (class 2606 OID 29367187)
-- Name: address address_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.address
    ADD CONSTRAINT address_pkey PRIMARY KEY (id);


--
-- TOC entry 3664 (class 2606 OID 29367361)
-- Name: ddns ddns_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ddns
    ADD CONSTRAINT ddns_pkey PRIMARY KEY (iprangeid, zoneid);


--
-- TOC entry 3650 (class 2606 OID 29367126)
-- Name: device device_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.device
    ADD CONSTRAINT device_pkey PRIMARY KEY (id);


--
-- TOC entry 3668 (class 2606 OID 29367478)
-- Name: dhcprange dhcprange_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dhcprange
    ADD CONSTRAINT dhcprange_pkey PRIMARY KEY (id);


--
-- TOC entry 3666 (class 2606 OID 29367452)
-- Name: dnsrecord dnsrecord_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dnsrecord
    ADD CONSTRAINT dnsrecord_pkey PRIMARY KEY (id);


--
-- TOC entry 3662 (class 2606 OID 29367333)
-- Name: dnszone dnszone_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dnszone
    ADD CONSTRAINT dnszone_pkey PRIMARY KEY (id);


--
-- TOC entry 3652 (class 2606 OID 29367153)
-- Name: interface interface_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.interface
    ADD CONSTRAINT interface_pkey PRIMARY KEY (id);


--
-- TOC entry 3658 (class 2606 OID 29367302)
-- Name: iprange iprange_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.iprange
    ADD CONSTRAINT iprange_pkey PRIMARY KEY (id);


--
-- TOC entry 3660 (class 2606 OID 29367319)
-- Name: server server_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.server
    ADD CONSTRAINT server_pkey PRIMARY KEY (id);


--
-- TOC entry 3656 (class 2606 OID 29367256)
-- Name: staticaddress staticaddress_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.staticaddress
    ADD CONSTRAINT staticaddress_pkey PRIMARY KEY (addressid);


--
-- TOC entry 3672 (class 2606 OID 29367257)
-- Name: staticaddress fk_address; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.staticaddress
    ADD CONSTRAINT fk_address FOREIGN KEY (addressid) REFERENCES public.address(id) NOT VALID;


--
-- TOC entry 3670 (class 2606 OID 29367154)
-- Name: interface fk_device; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.interface
    ADD CONSTRAINT fk_device FOREIGN KEY (deviceid) REFERENCES public.device(id);


--
-- TOC entry 3675 (class 2606 OID 29367367)
-- Name: ddns fk_dnszone; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ddns
    ADD CONSTRAINT fk_dnszone FOREIGN KEY (zoneid) REFERENCES public.dnszone(id);


--
-- TOC entry 3676 (class 2606 OID 29367489)
-- Name: dnsrecord fk_dnszone; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dnsrecord
    ADD CONSTRAINT fk_dnszone FOREIGN KEY (zoneid) REFERENCES public.dnszone(id) NOT VALID;


--
-- TOC entry 3671 (class 2606 OID 29367188)
-- Name: address fk_interface; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.address
    ADD CONSTRAINT fk_interface FOREIGN KEY (interfaceid) REFERENCES public.interface(id);


--
-- TOC entry 3674 (class 2606 OID 29367362)
-- Name: ddns fk_iprange; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.ddns
    ADD CONSTRAINT fk_iprange FOREIGN KEY (iprangeid) REFERENCES public.iprange(id);


--
-- TOC entry 3677 (class 2606 OID 29367479)
-- Name: dhcprange fk_iprange; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dhcprange
    ADD CONSTRAINT fk_iprange FOREIGN KEY (iprangeid) REFERENCES public.iprange(id);


--
-- TOC entry 3669 (class 2606 OID 29367082)
-- Name: keypermissions fk_keyid_apikey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.keypermissions
    ADD CONSTRAINT fk_keyid_apikey FOREIGN KEY (keyid) REFERENCES public.apikey(id);


--
-- TOC entry 3673 (class 2606 OID 29367343)
-- Name: dnszone fk_server; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dnszone
    ADD CONSTRAINT fk_server FOREIGN KEY (serverid) REFERENCES public.server(id) NOT VALID;


--
-- TOC entry 3678 (class 2606 OID 29367484)
-- Name: dhcprange fk_server; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.dhcprange
    ADD CONSTRAINT fk_server FOREIGN KEY (serverid) REFERENCES public.server(id);


-- Completed on 2023-06-17 23:18:46

--
-- PostgreSQL database dump complete
--

