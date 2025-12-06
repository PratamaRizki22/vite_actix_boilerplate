--
-- PostgreSQL database cluster dump
--

SET default_transaction_read_only = off;

SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;

--
-- Drop databases (except postgres and template1)
--

DROP DATABASE wallet_db;




--
-- Drop roles
--

DROP ROLE admin;


--
-- Roles
--

CREATE ROLE admin;
ALTER ROLE admin WITH SUPERUSER INHERIT CREATEROLE CREATEDB LOGIN REPLICATION BYPASSRLS PASSWORD 'SCRAM-SHA-256$4096:2LUjBHM/ZJTi5YdiH1ZBHg==$DGzX062v7pooxmF+tYExftQZB5hjfaIAbLL5WGjIDSo=:dm1RSQQcYJ8qfwDGFMX8V4BwvHS3xALLiZZYwOsyJxs=';

--
-- User Configurations
--








--
-- Databases
--

--
-- Database "template1" dump
--

--
-- PostgreSQL database dump
--

-- Dumped from database version 15.13
-- Dumped by pg_dump version 15.13

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

UPDATE pg_catalog.pg_database SET datistemplate = false WHERE datname = 'template1';
DROP DATABASE template1;
--
-- Name: template1; Type: DATABASE; Schema: -; Owner: admin
--

CREATE DATABASE template1 WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'en_US.utf8';


ALTER DATABASE template1 OWNER TO admin;

\connect template1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: DATABASE template1; Type: COMMENT; Schema: -; Owner: admin
--

COMMENT ON DATABASE template1 IS 'default template for new databases';


--
-- Name: template1; Type: DATABASE PROPERTIES; Schema: -; Owner: admin
--

ALTER DATABASE template1 IS_TEMPLATE = true;


\connect template1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: DATABASE template1; Type: ACL; Schema: -; Owner: admin
--

REVOKE CONNECT,TEMPORARY ON DATABASE template1 FROM PUBLIC;
GRANT CONNECT ON DATABASE template1 TO PUBLIC;


--
-- PostgreSQL database dump complete
--

--
-- Database "postgres" dump
--

--
-- PostgreSQL database dump
--

-- Dumped from database version 15.13
-- Dumped by pg_dump version 15.13

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

DROP DATABASE postgres;
--
-- Name: postgres; Type: DATABASE; Schema: -; Owner: admin
--

CREATE DATABASE postgres WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'en_US.utf8';


ALTER DATABASE postgres OWNER TO admin;

\connect postgres

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: DATABASE postgres; Type: COMMENT; Schema: -; Owner: admin
--

COMMENT ON DATABASE postgres IS 'default administrative connection database';


--
-- PostgreSQL database dump complete
--

--
-- Database "wallet_db" dump
--

--
-- PostgreSQL database dump
--

-- Dumped from database version 15.13
-- Dumped by pg_dump version 15.13

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: wallet_db; Type: DATABASE; Schema: -; Owner: admin
--

CREATE DATABASE wallet_db WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'en_US.utf8';


ALTER DATABASE wallet_db OWNER TO admin;

\connect wallet_db

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO admin;

--
-- Name: account_lockout; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.account_lockout (
    id integer NOT NULL,
    user_id integer NOT NULL,
    failed_attempts integer DEFAULT 0,
    locked_until timestamp without time zone,
    last_attempt timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.account_lockout OWNER TO admin;

--
-- Name: account_lockout_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.account_lockout_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.account_lockout_id_seq OWNER TO admin;

--
-- Name: account_lockout_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.account_lockout_id_seq OWNED BY public.account_lockout.id;


--
-- Name: audit_logs; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.audit_logs (
    id integer NOT NULL,
    user_id integer,
    event_type character varying(50) NOT NULL,
    event_action character varying(100) NOT NULL,
    ip_address character varying(50),
    user_agent text,
    status character varying(20) DEFAULT 'success'::character varying NOT NULL,
    details jsonb,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.audit_logs OWNER TO admin;

--
-- Name: audit_logs_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.audit_logs_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.audit_logs_id_seq OWNER TO admin;

--
-- Name: audit_logs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.audit_logs_id_seq OWNED BY public.audit_logs.id;


--
-- Name: comments; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.comments (
    id integer NOT NULL,
    post_id integer NOT NULL,
    user_id integer NOT NULL,
    parent_comment_id integer,
    content text NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.comments OWNER TO admin;

--
-- Name: comments_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.comments_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.comments_id_seq OWNER TO admin;

--
-- Name: comments_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.comments_id_seq OWNED BY public.comments.id;


--
-- Name: likes; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.likes (
    id integer NOT NULL,
    post_id integer NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.likes OWNER TO admin;

--
-- Name: likes_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.likes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.likes_id_seq OWNER TO admin;

--
-- Name: likes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.likes_id_seq OWNED BY public.likes.id;


--
-- Name: posts; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.posts (
    id integer NOT NULL,
    title character varying(255) NOT NULL,
    content text NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    likes_count integer DEFAULT 0,
    comments_count integer DEFAULT 0
);


ALTER TABLE public.posts OWNER TO admin;

--
-- Name: posts_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.posts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.posts_id_seq OWNER TO admin;

--
-- Name: posts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.posts_id_seq OWNED BY public.posts.id;


--
-- Name: refresh_tokens; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.refresh_tokens (
    id integer NOT NULL,
    user_id integer NOT NULL,
    token_hash character varying(255) NOT NULL,
    token_family character varying(255) NOT NULL,
    parent_token_hash character varying(255),
    is_revoked boolean DEFAULT false,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    expires_at timestamp without time zone NOT NULL,
    rotated_at timestamp without time zone,
    reuse_detected boolean DEFAULT false
);


ALTER TABLE public.refresh_tokens OWNER TO admin;

--
-- Name: refresh_tokens_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.refresh_tokens_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.refresh_tokens_id_seq OWNER TO admin;

--
-- Name: refresh_tokens_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.refresh_tokens_id_seq OWNED BY public.refresh_tokens.id;


--
-- Name: sessions; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.sessions (
    id integer NOT NULL,
    user_id integer NOT NULL,
    token_hash character varying(255) NOT NULL,
    device_name character varying(255),
    ip_address character varying(45),
    user_agent character varying(500),
    last_activity timestamp without time zone DEFAULT now() NOT NULL,
    expires_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.sessions OWNER TO admin;

--
-- Name: sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.sessions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.sessions_id_seq OWNER TO admin;

--
-- Name: sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.sessions_id_seq OWNED BY public.sessions.id;


--
-- Name: token_blacklist; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.token_blacklist (
    id integer NOT NULL,
    token_hash character varying(255) NOT NULL,
    user_id integer NOT NULL,
    reason character varying(255),
    expires_at timestamp without time zone NOT NULL,
    blacklisted_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.token_blacklist OWNER TO admin;

--
-- Name: token_blacklist_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.token_blacklist_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.token_blacklist_id_seq OWNER TO admin;

--
-- Name: token_blacklist_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.token_blacklist_id_seq OWNED BY public.token_blacklist.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.users (
    id integer NOT NULL,
    username character varying(255) NOT NULL,
    email character varying(255),
    password character varying(255) NOT NULL,
    role character varying(20) DEFAULT 'user'::character varying NOT NULL,
    totp_secret text,
    totp_enabled boolean DEFAULT false,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    wallet_address character varying(255),
    email_verified boolean DEFAULT false NOT NULL,
    recovery_codes text[],
    is_banned boolean DEFAULT false,
    banned_until timestamp without time zone,
    last_login timestamp without time zone
);


ALTER TABLE public.users OWNER TO admin;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO admin;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: web3_challenges; Type: TABLE; Schema: public; Owner: admin
--

CREATE TABLE public.web3_challenges (
    id integer NOT NULL,
    address character varying(42) NOT NULL,
    challenge character varying(255) NOT NULL,
    expires_at timestamp without time zone NOT NULL,
    used_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.web3_challenges OWNER TO admin;

--
-- Name: web3_challenges_id_seq; Type: SEQUENCE; Schema: public; Owner: admin
--

CREATE SEQUENCE public.web3_challenges_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.web3_challenges_id_seq OWNER TO admin;

--
-- Name: web3_challenges_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: admin
--

ALTER SEQUENCE public.web3_challenges_id_seq OWNED BY public.web3_challenges.id;


--
-- Name: account_lockout id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.account_lockout ALTER COLUMN id SET DEFAULT nextval('public.account_lockout_id_seq'::regclass);


--
-- Name: audit_logs id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.audit_logs ALTER COLUMN id SET DEFAULT nextval('public.audit_logs_id_seq'::regclass);


--
-- Name: comments id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.comments ALTER COLUMN id SET DEFAULT nextval('public.comments_id_seq'::regclass);


--
-- Name: likes id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.likes ALTER COLUMN id SET DEFAULT nextval('public.likes_id_seq'::regclass);


--
-- Name: posts id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.posts ALTER COLUMN id SET DEFAULT nextval('public.posts_id_seq'::regclass);


--
-- Name: refresh_tokens id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.refresh_tokens ALTER COLUMN id SET DEFAULT nextval('public.refresh_tokens_id_seq'::regclass);


--
-- Name: sessions id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.sessions ALTER COLUMN id SET DEFAULT nextval('public.sessions_id_seq'::regclass);


--
-- Name: token_blacklist id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.token_blacklist ALTER COLUMN id SET DEFAULT nextval('public.token_blacklist_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: web3_challenges id; Type: DEFAULT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.web3_challenges ALTER COLUMN id SET DEFAULT nextval('public.web3_challenges_id_seq'::regclass);


--
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
20251117060319	create users table	2025-11-17 18:39:43.050283+00	t	\\x3fb2b8350f5e455f5c8d2d860c5ba59cfab50561fab9d2184004a86c0a2dcda26e02d1bcbd093f409402599f7ffdc4d2	47036462
20251117064048	create posts table	2025-11-17 18:39:43.098574+00	t	\\xb1ac5b372c95c85d6a1254cdc1d7402c06d4925d85df203dc2c6dd32e1c622df5b16f9e165d05ea943b52de5146c0e6c	10231088
20251117183836	add wallet address column	2025-11-17 18:39:43.1095+00	t	\\x1ff44a1f03587936b327f448785c5d8e5ca1ec4e11227bb9580b8c376683d70daf561c91bf1767604daeae888cf5ab2a	3660304
20251118011945	add 2fa columns	2025-11-17 18:39:43.113831+00	t	\\x299a6941539f1273eda8e63fead4c5d2e8ffc1ab36555e796f12619d704d7b91a997f95da9f1742b1746ab1d1459ea8d	1327723
20251118003223	add email verified column	2025-11-18 00:55:14.005849+00	t	\\x00290381a2d0d69b730efed4a39e9024cb8fbb6e85e86eebaf8b903532658ecaae00185ecf067b6992b641d4ace3764d	10832621
20251118020000	make email nullable	2025-11-18 00:55:14.018002+00	t	\\xf0cf214aaf6276629d826e943b1e48c463a81623df6d8a098dd3af9c5778471336bb82d49268a210c60ca4a0f9005589	1715246
20251118120000	create sessions table	2025-11-18 08:21:04.702751+00	t	\\x115f3de88e3fa0c830ae7022a3b9fff82b5020c43ee63014390ae9e6bf6e2c00b741cf67cf76b545ec1d7d3c8f7570b7	34800896
20251118140000	create token blacklist table	2025-11-18 08:48:56.347368+00	t	\\x158fa12d3b3b37b60f24bcd3a8e6b1e09e50de71d81f0ecdfd326f77410344d1b797a317bb16ab7ed0e114438e331cc7	35394190
20251119000000	create account lockout table	2025-11-19 14:43:09.280176+00	t	\\x03a5bd7f28f56a5fd8e988eaf9a76b9850c601f5bf453d188def918571420942cd87f2e7ef5f747a391638ccd41705cf	21740274
20251119010000	create audit logs table	2025-11-19 15:05:16.6506+00	t	\\x81a1db294b58315b682aff75dff0a5053b16661af2caf788525000585958a3785f8ec63031f84fff06db3d5eacbb683f	14358762
20251119020000	create refresh tokens table	2025-11-19 15:23:14.183788+00	t	\\xb4bb738bb5ff1734f04da076da2d1c3420eb45c17fa8faf40bf02d951d5bb944061c3972569bf29abed75f339f354ab7	18382975
20251119030000	create web3 challenges table	2025-11-19 15:53:03.42929+00	t	\\x9746a7a485562bd100dcbea827a358591e9d15fb55d7df281538e18586adcb48a86c4e67da1676f6a2b6ec3c024a8dd0	28947893
20251120	add search indexes	2025-11-20 18:16:38.573044+00	t	\\x8160d292ae1806f136a1f333b3650d6fcc7269e39a334a6595475b3dada1f7ca1e889385b7bb5ce85e8a282aae4bf872	29811084
20251121000000	create likes table	2025-11-20 18:16:38.603669+00	t	\\x58903cf3faa6454e64fd205efe20d5ef894472f38809ae89879b4405392b87a90f486354030229f8f1b222f41dab07ee	13334179
20251121000001	create comments table	2025-11-20 18:16:38.617645+00	t	\\xa6382130326c6d07a8c483711c7691f40b18476a65615132df0f270537fcd0cfa2ddbf990e54287dd2c58ba7fc01915c	10629677
20251121000002	add counts to posts	2025-11-20 18:16:38.628947+00	t	\\xbc23ae1d2e4f65364a7aeab5aa0e3053ad9155594e802e080448b1e7ca6d978b4ae38e1651c40c86b7f0c09f9e0593bf	1598821
20251122000000	add recovery codes	2025-11-21 12:54:26.960011+00	t	\\x8510cf5c01dfc081499513fd6041bf82d26033bb1694f7133decd3c4534ceca0201102059e0982027cef984819617993	4872075
\.


--
-- Data for Name: account_lockout; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.account_lockout (id, user_id, failed_attempts, locked_until, last_attempt, created_at, updated_at) FROM stdin;
1	1	0	\N	2025-12-06 09:52:36.425542	2025-12-06 08:37:33.516552	2025-12-06 13:39:50.162334
\.


--
-- Data for Name: audit_logs; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.audit_logs (id, user_id, event_type, event_action, ip_address, user_agent, status, details, created_at) FROM stdin;
1	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:37:16.090078
2	95	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:37:17.604798
3	95	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:37:17.606701
4	100	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:37:17.607909
5	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:37:27.103122
6	111	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:37:49.011582
7	112	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:37:49.074583
8	114	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:37:49.183142
9	115	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:37:49.228295
10	115	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:37:49.230446
11	115	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:37:49.233292
12	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:37:49.247349
13	116	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:37:49.249463
14	117	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:37:49.336201
15	118	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:37:49.361088
16	119	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:37:50.136755
17	120	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:37:50.196782
18	122	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:38:27.745602
19	123	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:38:27.832326
20	124	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:38:27.86425
21	124	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:38:27.866622
22	124	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:38:27.867909
23	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:38:27.966915
24	125	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:38:28.008368
25	127	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:38:28.076283
26	128	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:38:28.113611
27	129	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:38:28.12458
28	130	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:38:28.733536
29	131	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:38:28.767087
30	135	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:39:10.360804
31	136	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:39:10.37463
32	137	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:39:10.470291
33	138	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:39:10.510725
34	138	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:39:10.515386
35	138	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:39:10.51781
36	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:39:10.53369
37	139	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:39:10.540375
38	140	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:39:10.548107
39	142	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:39:10.603021
40	143	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:39:11.277055
41	144	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:39:11.520857
42	141	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:39:11.523115
43	141	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:39:11.524359
44	145	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:39:11.525483
45	147	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:39:20.313633
46	148	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:39:20.379233
47	149	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:39:20.477732
48	151	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:39:20.517278
49	150	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:39:20.521432
50	150	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:39:20.524809
51	150	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:39:20.530258
52	152	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:39:20.539263
53	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:39:20.541997
54	153	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:39:20.572417
55	146	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:39:21.205943
56	146	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:39:21.207642
57	154	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:39:21.208853
58	155	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:39:21.261178
59	156	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:39:21.298081
60	158	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:39:52.68841
61	157	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:39:52.702082
62	159	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:39:52.887186
63	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:39:52.956229
64	160	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:39:52.977891
65	161	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:39:53.106095
66	163	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:39:53.189204
67	164	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:39:53.221412
68	164	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:39:53.224482
69	164	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:39:53.226475
70	165	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:39:53.77001
71	166	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:39:53.815857
72	162	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:39:53.895164
73	162	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:39:53.896454
74	167	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:39:53.897552
75	179	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:40:06.090155
76	180	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:40:06.13854
77	181	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:40:06.284766
78	183	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:40:06.385099
79	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:40:06.424168
80	184	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:40:06.465798
81	185	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:40:06.673574
82	186	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:40:06.719205
83	186	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:40:06.720829
84	186	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:40:06.721873
85	187	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:40:07.231737
86	182	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:40:07.297262
87	182	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:40:07.298573
88	189	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:40:07.299641
89	188	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:40:07.308042
90	202	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:40:23.389912
91	201	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:40:23.426845
92	203	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:40:23.504087
93	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:40:23.583512
94	204	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:40:23.630071
95	206	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:40:23.790293
96	205	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:40:23.799723
97	207	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:40:23.806529
98	207	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:40:23.808222
99	207	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:40:23.809525
100	209	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:40:24.449216
101	210	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:40:24.49754
102	208	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:40:24.657205
103	208	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:40:24.658519
104	211	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:40:24.659532
105	223	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:42:37.707287
106	224	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:42:37.832346
107	224	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:42:37.834395
108	224	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:42:37.835708
109	226	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:42:38.063196
110	227	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:42:38.080717
111	228	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:42:38.122087
112	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:42:38.136652
113	229	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:42:38.25129
114	230	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:42:38.269985
115	225	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:42:38.84448
116	231	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:42:38.848844
117	225	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:42:38.849495
118	232	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:42:38.850461
119	233	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:42:38.897297
120	245	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:51:00.850356
121	246	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:51:00.90741
122	247	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:51:00.957337
123	248	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:51:00.981813
124	249	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:51:01.012392
125	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:51:01.025051
126	251	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:51:01.039725
127	252	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:51:01.110497
128	252	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:51:01.112013
129	252	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:51:01.113034
130	250	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:51:01.77733
131	253	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:51:01.777619
132	250	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:51:01.778817
133	254	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:51:01.77988
134	255	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:51:01.840553
135	267	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:51:14.71474
136	269	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:51:14.735558
137	270	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:51:14.786131
138	271	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:51:14.816624
139	272	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:51:14.858135
140	273	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:51:14.8588
141	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:51:14.859017
142	273	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:51:14.861673
143	273	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:51:14.862982
144	274	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:51:14.954918
145	268	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:51:15.609988
146	268	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:51:15.611325
147	275	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:51:15.612357
148	276	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:51:15.889089
149	277	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:51:15.906089
150	289	LOGIN	User login	192.168.1.1	Chrome/120	success	\N	2025-11-19 15:51:28.450155
151	290	PASSWORD_RESET	Password reset	\N	\N	success	\N	2025-11-19 15:51:28.464557
152	291	ACCOUNT_LOCKOUT	Account locked due to failed login attempts	8.8.8.8	Edge/120	blocked	{"lockout_minutes": 30}	2025-11-19 15:51:28.49671
153	292	FAILED_LOGIN	Failed login attempt: Invalid password	10.0.0.1	Firefox/122	failed	{"reason": "invalid_password"}	2025-11-19 15:51:28.529437
154	\N	FAILED_LOGIN	Failed login: unknown user	10.20.30.40	Unknown Agent	failed	\N	2025-11-19 15:51:28.57713
155	295	LOGIN	Test login event	127.0.0.1	Mozilla/5.0	success	\N	2025-11-19 15:51:28.607517
156	294	LOGOUT	User logout	172.16.0.1	Safari/537	success	\N	2025-11-19 15:51:28.607607
157	296	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:51:28.673285
158	296	LOGOUT	Test logout	\N	\N	success	\N	2025-11-19 15:51:28.675379
159	296	LOGIN	Second login	\N	\N	success	\N	2025-11-19 15:51:28.676374
160	297	FAILED_LOGIN	Multiple failed attempts	\N	\N	failed	{"reason": "incorrect_password", "attempts": 3}	2025-11-19 15:51:29.779381
161	293	LOGIN	User1 login	\N	\N	success	\N	2025-11-19 15:51:29.802816
162	293	LOGOUT	User1 logout	\N	\N	success	\N	2025-11-19 15:51:29.806071
163	299	LOGIN	User2 login	\N	\N	success	\N	2025-11-19 15:51:29.807949
164	298	LOGIN	Test login	\N	\N	success	\N	2025-11-19 15:51:29.809605
165	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "kokok"}	2025-11-20 02:46:59.48596
166	316	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-20 02:47:21.39343
167	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "cek"}	2025-11-20 03:55:00.350725
168	320	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:20:23.21934
169	320	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:20:51.023821
170	320	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:21:11.48793
171	327	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:33:01.202851
172	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "testadmin_ush_330"}	2025-11-20 05:51:22.056814
173	333	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:51:34.593761
174	333	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-20 05:52:03.18156
175	333	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:57:01.41887
176	333	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:57:55.176561
177	333	LOGIN	User login successful	127.0.0.1	curl/7.81.0	success	{"event": "login"}	2025-11-20 05:58:05.012382
178	333	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 05:58:26.0942
179	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "pratamarizki22"}	2025-11-20 06:12:38.732064
180	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "pratamarizki22"}	2025-11-20 06:12:56.829566
181	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "pratamarizki22"}	2025-11-20 06:13:13.768432
182	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "pratamarizki22"}	2025-11-20 06:13:52.805495
183	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "pratamarizki22"}	2025-11-20 06:15:48.153699
184	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "pratamarizki22"}	2025-11-20 06:21:43.996738
185	334	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-20 06:22:07.976584
186	334	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 06:36:28.44758
187	335	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 06:46:39.220833
188	335	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 06:46:52.027008
189	335	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 06:51:30.523273
190	335	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 07:06:42.948304
191	335	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 07:09:41.533111
192	336	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 07:38:09.086295
193	336	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 11:45:21.106386
194	336	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 12:07:53.982331
195	336	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 12:19:47.698749
236	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "testuser"}	2025-12-04 19:55:11.670943
196	336	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 12:31:39.274894
197	336	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 18:19:00.967984
198	336	LOGOUT	User logout	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "logout"}	2025-11-20 20:01:07.214225
199	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 03:43:26.824116
200	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 04:44:55.148407
201	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 04:49:26.987652
202	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 04:50:06.083686
203	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 04:50:33.04681
204	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 04:51:08.900858
205	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 05:32:05.76772
206	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 05:33:11.790168
207	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "test"}	2025-11-21 09:08:03.273689
208	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 09:28:53.657569
209	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 09:33:54.504515
210	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "jiooi"}	2025-11-21 09:35:16.320021
211	345	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 09:36:18.90798
212	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 09:42:32.118821
213	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 09:56:03.429626
214	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 10:04:21.127996
215	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 10:08:15.537029
216	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "jojo"}	2025-11-21 10:21:12.462634
217	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 11:42:56.699453
218	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 11:43:10.620436
219	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 12:15:13.946426
220	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 12:18:44.030739
221	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 12:26:45.012183
222	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 12:30:48.792307
223	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 12:31:45.353558
224	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 12:31:50.683717
225	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 12:32:03.662318
226	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-11-21 12:39:06.126068
227	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-11-21 14:08:26.842765
228	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "wronguser"}	2025-12-04 18:54:38.142538
229	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "wronguser"}	2025-12-04 18:54:38.162023
230	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "wronguser"}	2025-12-04 18:54:38.178579
231	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "apitest@example.com"}	2025-12-04 18:57:04.078045
232	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "ratelimit"}	2025-12-04 18:57:04.172347
233	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "ratelimit"}	2025-12-04 18:57:04.191072
234	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "ratelimit"}	2025-12-04 18:57:04.210036
235	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	curl/7.81.0	failed	{"identifier": "ratelimit"}	2025-12-04 18:57:04.226883
237	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 06:15:05.70634
238	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 06:29:46.693407
239	352	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 06:54:52.506592
240	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 06:55:11.012629
241	352	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 06:57:57.392213
242	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 07:15:24.172828
243	352	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 07:34:30.37551
244	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 07:35:02.669103
245	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 08:04:53.264462
246	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 08:24:50.093863
247	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 08:30:13.056547
248	352	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 08:49:00.636533
249	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 08:49:42.496303
250	352	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 08:56:06.579557
251	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 08:56:40.125115
252	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-05 08:57:33.106994
253	336	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 10:21:04.156655
254	361	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 10:32:16.096664
255	361	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 10:33:15.253722
256	361	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-05 10:36:07.037806
257	336	LOGIN	User login successful	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 06:01:10.94949
258	336	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 06:03:04.499972
259	361	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 06:04:08.551515
260	363	LOGIN	User login successful	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 07:49:19.81001
261	363	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 08:04:17.996658
262	361	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 08:04:50.494502
263	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "prizkipurnomo914@gmail.com"}	2025-12-06 08:05:08.097466
264	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "prizkipurnomo914@gmail.com"}	2025-12-06 08:06:03.937912
265	363	LOGIN	User login successful	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 08:12:58.780613
266	363	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 08:13:08.89606
267	363	LOGIN	User login successful	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 08:31:48.569628
268	363	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 08:32:04.283728
269	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "asd@gmail.com"}	2025-12-06 08:35:49.656269
270	1	FAILED_LOGIN	Failed login attempt: Invalid password	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"reason": "Invalid password"}	2025-12-06 08:37:33.498927
271	363	LOGIN	User login successful	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 08:38:22.346794
272	363	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 08:38:28.323172
273	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "joko@gmail.com"}	2025-12-06 08:40:45.549475
274	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "dkokoeako@gmail.com"}	2025-12-06 08:41:09.168447
276	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "rwre@gmail.com"}	2025-12-06 08:45:45.280413
279	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "ioio@gmail.com"}	2025-12-06 08:49:36.856693
280	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 08:50:41.814981
283	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "324234234@gmail.com"}	2025-12-06 08:53:13.273872
286	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "joko@gmail.com"}	2025-12-06 08:58:10.365601
275	363	FAILED_LOGIN	Failed login attempt: Invalid password	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"reason": "Invalid password"}	2025-12-06 08:42:53.753237
277	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "qweqwewq@gmail.com"}	2025-12-06 08:46:02.369739
278	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 08:47:20.850231
281	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 08:51:30.092775
282	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "fr@gmail.com"}	2025-12-06 08:52:58.314852
284	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "joko@gmail.com"}	2025-12-06 08:56:08.266767
285	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "231213@gmail.com"}	2025-12-06 08:56:23.208987
287	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 08:58:31.209125
288	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "test@test.com"}	2025-12-06 09:02:45.650927
289	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "joko@gmail.com"}	2025-12-06 09:12:38.467636
290	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 09:12:52.957383
291	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 09:15:47.631805
292	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 09:15:57.686983
293	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "koko@gmail.com"}	2025-12-06 09:17:13.279215
294	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 09:26:40.109866
295	352	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 09:26:48.579188
296	\N	FAILED_LOGIN	Failed login attempt - invalid username/email	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"identifier": "nonoa@gmail.com"}	2025-12-06 09:27:01.734726
297	1	FAILED_LOGIN	Failed login attempt: Invalid password	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"reason": "Invalid password"}	2025-12-06 09:28:29.116347
298	1	FAILED_LOGIN	Failed login attempt: Invalid password	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"reason": "Invalid password"}	2025-12-06 09:28:32.19422
299	1	FAILED_LOGIN	Failed login attempt: Invalid password	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"reason": "Invalid password"}	2025-12-06 09:28:54.080959
300	1	FAILED_LOGIN	Failed login attempt: Invalid password	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"reason": "Invalid password"}	2025-12-06 09:29:26.900537
301	1	FAILED_LOGIN	Login attempt on locked account	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	blocked	{"remaining_seconds": 743}	2025-12-06 09:32:03.532925
302	1	FAILED_LOGIN	Login attempt on locked account	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	blocked	{"remaining_seconds": 709}	2025-12-06 09:32:37.627043
303	1	FAILED_LOGIN	Failed login attempt: Invalid password	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	failed	{"reason": "Invalid password"}	2025-12-06 09:52:36.409599
304	1	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 09:55:04.319443
305	1	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 10:12:35.75335
306	1	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 10:13:16.901927
307	1	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 10:39:28.91439
308	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 10:40:19.255807
309	345	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 10:59:46.827551
310	336	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 11:01:34.378306
311	352	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 11:05:17.550979
312	352	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 11:05:29.25729
313	1	LOGIN	User login successful	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 11:06:08.993756
314	1	LOGOUT	User logged out	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 11:40:48.046331
315	352	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 13:36:07.295352
316	352	LOGOUT	User logged out	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	\N	2025-12-06 13:39:32.501645
317	1	LOGIN	User login successful	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	success	{"event": "login"}	2025-12-06 13:40:40.16546
\.


--
-- Data for Name: comments; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.comments (id, post_id, user_id, parent_comment_id, content, created_at, updated_at) FROM stdin;
15	14	1	\N	mantap wok	2025-12-06 09:58:15.945234	2025-12-06 09:58:15.945234
\.


--
-- Data for Name: likes; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.likes (id, post_id, user_id, created_at) FROM stdin;
27	9	336	2025-11-21 12:16:10.209705
32	9	352	2025-12-05 08:25:18.34496
\.


--
-- Data for Name: posts; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.posts (id, title, content, user_id, created_at, updated_at, likes_count, comments_count) FROM stdin;
9	wala wak	aduhai	345	2025-11-21 09:40:59.751317	2025-11-21 09:40:59.751317	2	0
14	aku raja 	aduhai\n	1	2025-12-06 09:57:58.103444	2025-12-06 09:57:58.103444	0	1
16	cek	dee	345	2025-12-06 11:31:07.218695	2025-12-06 11:31:07.218695	0	0
\.


--
-- Data for Name: refresh_tokens; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.refresh_tokens (id, user_id, token_hash, token_family, parent_token_hash, is_revoked, created_at, expires_at, rotated_at, reuse_detected) FROM stdin;
172	1	cb107868e2bae35183b15b9f0ef68ddf930d70a234d7a360f29fcc06f4ffa362	37dba19c-307a-4296-8fc9-c83dfeba0c52	\N	f	2025-12-06 13:40:40.160345	2025-12-13 13:40:40.159271	\N	f
126	333	a6774502dcbdcf2410864e069bfa58eee8b08e4762bfe1423df9a6a7064c730a	6cbae01b-8574-46b9-a895-3bf38db8c8d2	\N	f	2025-11-20 05:51:34.595039	2025-11-27 05:51:34.594595	\N	f
127	333	b166714d1baf62728fcc66545820f112ac502570a3e2fe52506b9d5799f7cf0e	f5e015dc-8953-4c73-a2d2-15c4aad0b174	\N	f	2025-11-20 05:52:03.18272	2025-11-27 05:52:03.182454	\N	f
128	333	6bf196006b65329a701cac8a4fe2cbca489bf665961e81a36a15103a1a97910e	0e6c243a-5bc7-41bd-93b6-f5b33b049931	\N	f	2025-11-20 05:57:01.432614	2025-11-27 05:57:01.432263	\N	f
129	333	18712944b5e8050a57ab58fa6312ea308c88bc562a7ed1e258d5081c499da7de	c5ed21a2-e06f-4f9e-b06f-27fa07c5a341	\N	f	2025-11-20 05:57:55.177996	2025-11-27 05:57:55.177528	\N	f
130	333	dc36c5eed287df28f7dffb68d947116751f0f602f034cb8853984508543b65c3	6e6af0b7-0cc9-4b53-8b3b-84d6b428adb9	\N	f	2025-11-20 05:58:05.013374	2025-11-27 05:58:05.01321	\N	f
132	336	705efc9fe18deb665a7098cf345da11a2ffa4519910043fe1a08230b2556e601	b2f5fcae-99b5-4ae3-80c5-774b471fe4e2	\N	f	2025-11-21 04:44:55.142947	2025-11-28 04:44:55.141928	\N	f
133	336	bf09e7c2c15d945e10d1658b9bdd66d2cc908cb542da900f0cb78eda96eac379	80f6d719-bd12-46e3-8021-25660494c16f	\N	f	2025-11-21 04:50:06.080523	2025-11-28 04:50:06.080172	\N	f
134	336	5e9c86227113cb1b89fdeb38f5da016c0bd014285ace65fb16536c10ad67b17a	056fe009-f101-4bcb-ae47-f44efd68854f	\N	f	2025-11-21 04:51:08.89781	2025-11-28 04:51:08.896763	\N	f
135	336	5c9f5ef93ea35719a66f3644cac88993fcb58a316c093f74ddf914695d2d2408	3e3c891c-fb56-41ac-aa7c-b731038728ca	\N	f	2025-11-21 05:33:11.788459	2025-11-28 05:33:11.788083	\N	f
136	336	c531afb1320e70b20fca34ced8c1726fe6477f08874846faa9cca4e159f55bfe	853500ea-45e6-4135-adb7-414a009207e8	\N	f	2025-11-21 09:28:53.651194	2025-11-28 09:28:53.65014	\N	f
137	345	ef5098391c6e11846e2ba90d8bc1e12c3e68932b207c718e6e3bb78a858b52f8	a16d14e5-df51-4377-964d-956d8a2713f5	\N	f	2025-11-21 09:36:18.905328	2025-11-28 09:36:18.904707	\N	f
138	336	635a3be0dc51f1974be49ce66f573e19b17ae0d56dc77742a0bb91713dfef0db	85e78dd3-1758-4a90-8fc5-b1720826c0a1	\N	f	2025-11-21 09:42:32.111537	2025-11-28 09:42:32.109889	\N	f
139	336	644a8b961c6e3a21ccdac33d30c91e92412e1ab17dd322ee394217541cd60852	dd2afed7-570d-42c5-9738-377d178e1465	\N	f	2025-11-21 09:56:03.423697	2025-11-28 09:56:03.41835	\N	f
140	336	77fd88fc78a3601bb1c2c3d3d80cd285e9892de25e82e361f17a37d687b5ee9e	dc7c543e-1081-4a9a-a976-0690083134c0	\N	f	2025-11-21 10:04:21.12255	2025-11-28 10:04:21.121609	\N	f
141	336	fb1be9a74d502458625cb0cceee86619c462616a5a9d681cb0b59a21af524c50	de502f0d-ae49-420a-8268-00f00044f9c0	\N	f	2025-11-21 11:42:56.693494	2025-11-28 11:42:56.692102	\N	f
142	336	70c50385bffe18fb59420c45c896b40add8171685298cd2e8e14b17d1777a5d7	6e491fa3-635f-4e69-95f1-7d7a8b3d832d	\N	f	2025-11-21 12:15:13.939564	2025-11-28 12:15:13.938252	\N	f
143	336	9982c0fe5566be3f26ee009b88eda69a7586b8253196858b74fcf5b10fbbe047	4d37c860-d8c8-4d9d-93db-d55f4551514d	\N	f	2025-11-21 12:26:45.009602	2025-11-28 12:26:45.009148	\N	f
144	336	f2a2bba824ea188067e18a2ce8a651015f9947b97fce7f37b8b8fe05bbd57aab	bf86ec16-2e4a-4712-be25-0247775dcef7	\N	f	2025-11-21 12:31:45.350498	2025-11-28 12:31:45.349935	\N	f
145	336	41a22066a49eb0ac9aa13d1bcb61376d4b5ecb61d0ebd8d2d6660fa9f7f11e69	0d5f2c40-5e8f-437c-9a7a-d40161fd85ba	\N	f	2025-11-21 12:32:03.658843	2025-11-28 12:32:03.657462	\N	f
146	336	3c6ea57d4ab21605526979cef6cf42a771a101c36c6d6d0db7bf13b50738716d	869847d5-046b-4524-bdce-c993dfccb494	\N	f	2025-11-21 14:08:26.836334	2025-11-28 14:08:26.834955	\N	f
147	352	2c737181622b6eec88429b2d207b4f99f540bee79f0fd843722772d5d822bb79	a7e6cc71-480b-421a-a107-64692e669ae5	\N	f	2025-12-05 06:15:05.701826	2025-12-12 06:15:05.701182	\N	f
148	352	cb0aad6e8cbc90fd6ba7e3a84c9adadea2e429305692d3852716275b8b2094f6	c02dc0c0-1637-4305-b371-81d1a3f3970f	\N	f	2025-12-05 06:29:46.688672	2025-12-12 06:29:46.688046	\N	f
149	352	b175031803ebf0699df1d67addf0e6f16d3e348f2830a7c3134aed79fadd8c62	8ecdcbc0-8d20-4285-9ba6-f7ae1d4a235c	\N	f	2025-12-05 06:55:11.009496	2025-12-12 06:55:11.009118	\N	f
150	352	1b5d2a1075bdbffe0d545df5aaed7a55ac691884c9fc2f070664a9214d71cd7e	7bb76aec-323e-435a-a59e-9dfdd542b61a	\N	f	2025-12-05 07:15:24.166911	2025-12-12 07:15:24.165694	\N	f
151	352	f3cccb8e8ee61efcc9f6d9b6c6e62136abf6d29aa780df6133e86f0158277685	d6cc9a2c-43ea-4494-ad42-5cd71aa8ae6e	\N	f	2025-12-05 07:35:02.667323	2025-12-12 07:35:02.667089	\N	f
152	352	6926697cbbf2711f939d52bad3dd2040dd81c322e281c75b4f07fefd7330f02c	2a1117bf-dc00-4554-9667-00d4748700b4	\N	f	2025-12-05 08:04:53.258011	2025-12-12 08:04:53.256868	\N	f
153	352	6c0e2c5adff145677761c35d181dd6b4bdcf8e4e4204819cdc59911e5e7739db	49a06016-1169-4df4-a636-db1f29b276f7	\N	f	2025-12-05 08:24:50.089902	2025-12-12 08:24:50.089156	\N	f
154	352	ddc1875488d6540e86071362a997a066d2ec6373fdf8bf3cc6f8550bd80c674f	f940760b-0fd8-4c13-95b4-f7a8c68af150	\N	f	2025-12-05 08:30:13.051836	2025-12-12 08:30:13.051004	\N	f
155	352	825b62ee9505539044bdf5faa720bd6b8744d91addd20cae004a1ab890adc528	57332dde-5591-456a-9be8-cf1d2a78365b	\N	f	2025-12-05 08:49:42.492406	2025-12-12 08:49:42.491118	\N	f
156	352	541e273ae8394b832cdac79b1c293fc583998b8fc7b42a852c9253b954e56160	11854b3c-4092-418a-a248-3559fb0b5d45	\N	f	2025-12-05 08:56:40.119968	2025-12-12 08:56:40.119135	\N	f
157	336	de7f3dbc594b626263ff385fbcb06e680b39aab4e87b228b6b58a94897da3389	f5f445b6-9a45-475b-bf26-8ff7a9671ade	\N	f	2025-12-05 08:57:33.104807	2025-12-12 08:57:33.104151	\N	f
158	336	b387f7ee3686f7fbf725721677d6b251c8623b012209b14b621307cfab4b3c40	71f228fe-db4f-4230-b712-58eab394e3e7	\N	f	2025-12-06 06:01:10.942362	2025-12-13 06:01:10.941152	\N	f
163	352	8741f0ff108c476666d97477f8b7c8289397228ec1b33514572b2a785d984c50	a98664ab-c7a0-48f6-a0ae-8c02928156e3	\N	f	2025-12-06 09:26:40.105281	2025-12-13 09:26:40.10437	\N	f
164	1	c9d702663c5cf6368814cf215a82c0ca2dec3152ea6bf7874e4e1867c2c1b199	57912cd8-fe5d-4932-8f71-a1477a2f330b	\N	f	2025-12-06 09:55:04.317582	2025-12-13 09:55:04.317187	\N	f
165	1	5c8417c7141d8ee4e943d68bf83f0d7a57dfac44527467b73e6eb327f29d7e0e	b5f940d8-1ea3-4fc7-81fd-4903c322afb5	\N	f	2025-12-06 10:13:16.89881	2025-12-13 10:13:16.898243	\N	f
166	1	8af4b9e05bd8f5a58eee21577d292a254bbfc38eebc0d560fd34a134d96d115b	4882941e-9bc6-41dc-be3a-3600a424eae1	\N	f	2025-12-06 10:39:28.908003	2025-12-13 10:39:28.90738	\N	f
167	336	d14bef1a2d2dbb92b95d52a09fbf85f4bb9cea96a88a1150ce464bb4e1607b0a	e56b5f66-ff9f-4294-bdf2-dc976496d7b8	\N	f	2025-12-06 10:40:19.253153	2025-12-13 10:40:19.251355	\N	f
168	345	e3d2384e172c96ea1490ff29d1245b09982792dc451e7be83b8484335cf90bc5	28d03b84-ff97-4782-a307-2fce638419df	\N	f	2025-12-06 10:59:46.822595	2025-12-13 10:59:46.82167	\N	f
169	336	337932302ae53d32a89b8c57668b077a8eb201905a5d56de6cb05a929732b8a0	d661a613-9420-4c1a-8d76-ce3e2344105f	\N	f	2025-12-06 11:01:34.374085	2025-12-13 11:01:34.373639	\N	f
170	352	b3e2091028011fa13b8bffcb575860f56ac9a5369eef1916bbd2a19e07a13638	799c490d-6d0c-4ac3-83e1-a09bf9f22f81	\N	f	2025-12-06 11:05:17.546347	2025-12-13 11:05:17.545325	\N	f
171	1	1d70b86e8c154818ad7d620996177137db40e08b0be0f77b68ce4bd8b2d98e59	8b63a176-e5c9-43a8-a7bd-a68c059e2d81	\N	f	2025-12-06 11:06:08.990161	2025-12-13 11:06:08.989862	\N	f
\.


--
-- Data for Name: sessions; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.sessions (id, user_id, token_hash, device_name, ip_address, user_agent, last_activity, expires_at, created_at, updated_at) FROM stdin;
64	336	a02fb76f15bfce0211e9e524b3cab111fd53c372f81e88b7bbd83a5c7b4ef9d9	OAuth	\N	\N	2025-11-21 03:54:40.109887	2025-11-22 03:54:40.07017	2025-11-21 03:54:40.109887	2025-11-21 03:54:40.109887
38	333	b2b89bc5f048b39e5d23715f2934167ff907e0d67a011ec8e76c10b4b4785022	Desktop	127.0.0.1	curl/7.81.0	2025-11-20 05:51:34.591675	2025-11-21 05:51:34.591172	2025-11-20 05:51:34.591675	2025-11-20 05:51:34.591675
40	333	7575d0cec116321fc93ece6c852f69d492e9791a3d66143784f8b15b995adf18	Desktop	127.0.0.1	curl/7.81.0	2025-11-20 05:57:01.416531	2025-11-21 05:57:01.415851	2025-11-20 05:57:01.416531	2025-11-20 05:57:01.416531
41	333	2dd97e81f8fc5026c1036035ec77ecf1d1133c3acdac34587a226e6f77ea62d9	Desktop	127.0.0.1	curl/7.81.0	2025-11-20 05:57:55.167131	2025-11-21 05:57:55.166908	2025-11-20 05:57:55.167131	2025-11-20 05:57:55.167131
42	333	65672406035babfbd883cffc8d3645611f3add71ed7d5fdd160e4e1a1417a690	Desktop	127.0.0.1	curl/7.81.0	2025-11-20 05:58:05.003311	2025-11-21 05:58:05.003076	2025-11-20 05:58:05.003311	2025-11-20 05:58:05.003311
39	333	5276771600ae1f1ea8ff15d4b19871f69856240f673451cb4d507f6cbb639fc9	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-20 05:52:03.172333	2025-11-20 05:58:26.07974	2025-11-20 05:52:03.172333	2025-11-20 05:52:03.172333
51	336	10f966281a06afef937511da1c4c1982543b4f46a2516dfb20bcae7785232544	OAuth	\N	\N	2025-11-20 07:09:51.29129	2025-11-20 07:38:09.072675	2025-11-20 07:09:51.29129	2025-11-20 07:09:51.29129
52	336	76943061db506aed425dfe462681a2d63ef1af2d1aa6340e712236dd8711248b	OAuth	\N	\N	2025-11-20 07:38:16.941049	2025-11-20 11:45:21.088654	2025-11-20 07:38:16.941049	2025-11-20 07:38:16.941049
53	336	9d6639565ecb4d91f7dde38584cc022df5882167631a5ad1e000a2bab9f5f522	OAuth	\N	\N	2025-11-20 11:45:28.093232	2025-11-20 12:07:53.967298	2025-11-20 11:45:28.093232	2025-11-20 11:45:28.093232
54	336	7dab0bb27aab7941cc6b42b49dac3b4ada5b97e8086897c2ca7436b1b1764f85	OAuth	\N	\N	2025-11-20 12:08:01.84904	2025-11-20 12:19:47.683049	2025-11-20 12:08:01.84904	2025-11-20 12:08:01.84904
55	336	94abeee18ac81cbc91c774501c85c97e198c8da7edb2b331a3c6797b40c22383	OAuth	\N	\N	2025-11-20 12:25:45.675449	2025-11-20 12:31:39.266764	2025-11-20 12:25:45.675449	2025-11-20 12:25:45.675449
56	336	91308ae38197624ad88c67c372c49b1280054852d1db3fb3fede3b691555e3bb	OAuth	\N	\N	2025-11-20 12:31:45.176963	2025-11-21 12:31:45.175594	2025-11-20 12:31:45.176963	2025-11-20 12:31:45.176963
65	336	492f9a3f0d0f37cdac557a90112a639d5acced7c62fc003f062c1a24a64006e4	OAuth	\N	\N	2025-11-21 03:58:00.054782	2025-11-22 03:58:00.026806	2025-11-21 03:58:00.054782	2025-11-21 03:58:00.054782
57	336	e8110dd877ae530c3bde99f668331424b8a31341e7867250a9b4ef5b13582ebc	OAuth	\N	\N	2025-11-20 12:39:23.54164	2025-11-20 18:19:00.956864	2025-11-20 12:39:23.54164	2025-11-20 12:39:23.54164
58	336	6cf6e368c18ba342a8ce1e20db8e26687956037c2eb0f35c15b9202821b458fc	OAuth	\N	\N	2025-11-20 18:19:07.781857	2025-11-21 18:19:07.78118	2025-11-20 18:19:07.781857	2025-11-20 18:19:07.781857
66	336	85bc12c451a1d95d4d25344dae161d7b5a1b7da0bc5029bb04bbd3bb8b380b36	OAuth	\N	\N	2025-11-21 04:06:36.722367	2025-11-22 04:06:36.682298	2025-11-21 04:06:36.722367	2025-11-21 04:06:36.722367
59	336	e80d55f21453cf6431b383e03acbc427215d6f791c94e2a15c516ecd6c7c8a6e	OAuth	\N	\N	2025-11-20 19:12:20.552755	2025-11-20 20:01:07.197462	2025-11-20 19:12:20.552755	2025-11-20 19:12:20.552755
60	336	66640ec941f39c6f351f1863fdb2feef34680781964c090e1d6fe8b3b71349a8	OAuth	\N	\N	2025-11-21 03:08:35.390541	2025-11-22 03:08:35.351925	2025-11-21 03:08:35.390541	2025-11-21 03:08:35.390541
67	336	60202a90892d24b739a06fe32250fc09ec7b328a0baafcfb72977f6aba022a91	OAuth	\N	\N	2025-11-21 04:10:06.574777	2025-11-22 04:10:06.521968	2025-11-21 04:10:06.574777	2025-11-21 04:10:06.574777
61	336	2e356f2f23b9101e62076bb6e7970a3b94ed9d794736045712d6f656144feaa4	OAuth	\N	\N	2025-11-21 03:43:20.130054	2025-11-21 03:43:26.816737	2025-11-21 03:43:20.130054	2025-11-21 03:43:20.130054
62	336	614edbe12233b8d8810afef2568c6c95560a35b844017fc0fb097f5dbb1f0d1d	OAuth	\N	\N	2025-11-21 03:47:13.531261	2025-11-22 03:47:13.499546	2025-11-21 03:47:13.531261	2025-11-21 03:47:13.531261
63	336	bbb415217f78c9f4caf5162c2ac3f8f2419501c730969f56efa8b8b62a8e8a1a	OAuth	\N	\N	2025-11-21 03:50:53.238516	2025-11-22 03:50:53.198348	2025-11-21 03:50:53.238516	2025-11-21 03:50:53.238516
68	336	dc09af21777e0d0bee8581399207b81403fed14477af82243057aae927b6e9d0	OAuth	\N	\N	2025-11-21 04:21:01.619429	2025-11-22 04:21:01.61611	2025-11-21 04:21:01.619429	2025-11-21 04:21:01.619429
69	336	cee5da996bc0eb94f67d1e66497843445b3e6821d6096717049c4cc744861ecd	OAuth	\N	\N	2025-11-21 04:29:30.662109	2025-11-22 04:29:30.620412	2025-11-21 04:29:30.662109	2025-11-21 04:29:30.662109
70	336	465235d42364fcf1a00c201f85fabad267a1a8912557dbe31a94c9d8066c5926	OAuth	\N	\N	2025-11-21 04:44:30.804778	2025-11-22 04:44:30.763474	2025-11-21 04:44:30.804778	2025-11-21 04:44:30.804778
71	336	3d4de2f2a22ab1d8a1f8bba95606a19adb9b746e7897eaaa6d390b21b1c33371	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 04:44:55.140245	2025-11-21 04:49:26.980921	2025-11-21 04:44:55.140245	2025-11-21 04:44:55.140245
72	336	a0fb0fffb3059f57b0c5b5de1acff6908e89bc81006563f4a6efc3d86ec17c82	OAuth	\N	\N	2025-11-21 04:49:33.438998	2025-11-22 04:49:33.437744	2025-11-21 04:49:33.438998	2025-11-21 04:49:33.438998
73	336	3b10528e877cc3ff1c3ba6c7343b1c1edfc1933b228551d5fae00bb226a41cb3	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 04:50:06.078616	2025-11-21 04:50:33.032333	2025-11-21 04:50:06.078616	2025-11-21 04:50:06.078616
74	336	55d4395acf6c736292c26f9e7536e1943898e121b209141f3181cf2fbb0a58d0	OAuth	\N	\N	2025-11-21 04:50:43.303752	2025-11-22 04:50:43.30331	2025-11-21 04:50:43.303752	2025-11-21 04:50:43.303752
75	336	8f987e11679eaa608609252133218e7f4f9309de435c1933dbf5f3ae64309e68	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 04:51:08.895083	2025-11-21 05:32:05.745162	2025-11-21 04:51:08.895083	2025-11-21 04:51:08.895083
76	336	e589d6a02f9ac38b169bf3e0e2aa6a682e2eb07d0aa37b051b174c36a4ff67f6	OAuth	\N	\N	2025-11-21 05:32:47.932406	2025-11-22 05:32:47.932022	2025-11-21 05:32:47.932406	2025-11-21 05:32:47.932406
77	336	8ea5f32c3bfba8f966a74fd8c98b816dba9862bc0ed3dda279c28d47f03a4138	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 05:33:11.787055	2025-11-22 05:33:11.78661	2025-11-21 05:33:11.787055	2025-11-21 05:33:11.787055
78	336	4bede974edbf54b187fe51fc6ea5860b5f289b06b89adda49aedf1b201b30a83	OAuth	\N	\N	2025-11-21 09:27:48.08744	2025-11-22 09:27:48.050268	2025-11-21 09:27:48.08744	2025-11-21 09:27:48.08744
79	336	9e6ad892726d404a5d827b022d10b5eb7f2b77298b6ad324887909377817ba90	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 09:28:53.640719	2025-11-21 09:33:54.490841	2025-11-21 09:28:53.640719	2025-11-21 09:28:53.640719
80	345	5fa6a3f487530e92f177159b6c43ce0318d5a7e0ce3198180ceaa8e836020c85	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 09:36:18.898098	2025-11-22 09:36:18.897664	2025-11-21 09:36:18.898098	2025-11-21 09:36:18.898098
81	336	a55cb4d3f6037108872c51afc3e486d61888516495f696f459232df37e917aa9	OAuth	\N	\N	2025-11-21 09:42:05.03917	2025-11-22 09:42:05.000099	2025-11-21 09:42:05.03917	2025-11-21 09:42:05.03917
82	336	44ea74e9de7510e3a06dccb6902665461be99374e88a2d09354bf61996206912	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 09:42:32.108043	2025-11-22 09:42:32.107227	2025-11-21 09:42:32.108043	2025-11-21 09:42:32.108043
83	336	7dafe60f5dc24dc2873939c551a9e473af12bded4a3bab42f148f11b3529a102	OAuth	\N	\N	2025-11-21 09:55:38.446395	2025-11-22 09:55:38.444946	2025-11-21 09:55:38.446395	2025-11-21 09:55:38.446395
84	336	939be1a14629ba32a2683f20b6f6bbf331386d184d3a12eb79ba893b6fb17e19	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 09:56:03.41599	2025-11-22 09:56:03.414306	2025-11-21 09:56:03.41599	2025-11-21 09:56:03.41599
85	336	a781a593492c38bd37fd223ce62bd9d4d30fee86ed15db21a6ffbc3cbf7780c5	OAuth	\N	\N	2025-11-21 10:03:53.643609	2025-11-22 10:03:53.615782	2025-11-21 10:03:53.643609	2025-11-21 10:03:53.643609
86	336	3dd7c94b5f11f0e97fb69a5f1ae427b381027b2a7ee8e1cb2bc3173b6275d82b	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 10:04:21.118615	2025-11-21 10:08:15.531193	2025-11-21 10:04:21.118615	2025-11-21 10:04:21.118615
87	336	16fa9a9c3d685636d17aba33ae339c05c5a06df05c34f84a88c0905c2ed3a698	OAuth	\N	\N	2025-11-21 10:25:10.153173	2025-11-22 10:25:10.109907	2025-11-21 10:25:10.153173	2025-11-21 10:25:10.153173
88	336	082d4c87ed2416f68de8b5d62317bee2a33328aa370656037e380677c782b9c2	OAuth	\N	\N	2025-11-21 10:28:06.238159	2025-11-22 10:28:06.237465	2025-11-21 10:28:06.238159	2025-11-21 10:28:06.238159
89	336	8251da6234e42365189e858447c94d6d7a3519445d31aa5ba7812bf5a8c81355	OAuth	\N	\N	2025-11-21 10:29:37.29564	2025-11-22 10:29:37.295024	2025-11-21 10:29:37.29564	2025-11-21 10:29:37.29564
90	336	b111c9b1cf8eb65ae57d865c86c95847cdb6a7a76586134b911e2d2bcc388915	OAuth	\N	\N	2025-11-21 10:36:37.865251	2025-11-22 10:36:37.824053	2025-11-21 10:36:37.865251	2025-11-21 10:36:37.865251
91	336	0b6253db5aebc588a75035ade86c22653e3830e909d775220d87c2b0820b61b8	OAuth	\N	\N	2025-11-21 10:39:52.956054	2025-11-22 10:39:52.917359	2025-11-21 10:39:52.956054	2025-11-21 10:39:52.956054
92	336	ae720ec3a444b0a7d7033b509d36d91e11d985d704ae755d5ea5702a3fc14481	OAuth	\N	\N	2025-11-21 10:42:16.843064	2025-11-22 10:42:16.842101	2025-11-21 10:42:16.843064	2025-11-21 10:42:16.843064
93	336	11826ca9a8385debdd152fc3aeb8e329b301559f3b35ddd6ee9bbcf9f9f36a3b	OAuth	\N	\N	2025-11-21 10:43:45.655238	2025-11-22 10:43:45.654575	2025-11-21 10:43:45.655238	2025-11-21 10:43:45.655238
94	336	f32812aef875b575e17f9b884234c60c6e8b86f492547d1b568142479d38ffa4	OAuth	\N	\N	2025-11-21 10:44:00.969171	2025-11-22 10:44:00.968165	2025-11-21 10:44:00.969171	2025-11-21 10:44:00.969171
95	336	d9c4bc55087bf823e1bf441880df6d9faaedb0623d8a814c80f0cb14bd57bfbb	OAuth	\N	\N	2025-11-21 10:46:20.602856	2025-11-22 10:46:20.601909	2025-11-21 10:46:20.602856	2025-11-21 10:46:20.602856
96	336	db8a8b5e104b837639d9046f32b97aac834a1624e802f33974f3b5b653194b35	OAuth	\N	\N	2025-11-21 10:49:21.250991	2025-11-22 10:49:21.212239	2025-11-21 10:49:21.250991	2025-11-21 10:49:21.250991
97	336	715d303ea30221d17ec9adfdaf4ae62ba93c502db2fd97f8b90abe8f491ed500	OAuth	\N	\N	2025-11-21 10:52:03.52956	2025-11-22 10:52:03.489993	2025-11-21 10:52:03.52956	2025-11-21 10:52:03.52956
98	336	8bfacf591c3b28b3330781ede058996c2abcb183eec3c412c27b0f0fa1a7ff84	OAuth	\N	\N	2025-11-21 11:00:37.544216	2025-11-22 11:00:37.514999	2025-11-21 11:00:37.544216	2025-11-21 11:00:37.544216
99	336	a255fd9d906edb27b1d11c5c51f74b929e22cca2d4bc431cda20ec20c835975b	OAuth	\N	\N	2025-11-21 11:02:21.134224	2025-11-22 11:02:21.133022	2025-11-21 11:02:21.134224	2025-11-21 11:02:21.134224
100	336	e8684a85c7705ba539a75f1c82630ef2c7b2ca04d00d169235b3397f029074c0	OAuth	\N	\N	2025-11-21 11:03:47.75261	2025-11-22 11:03:47.750675	2025-11-21 11:03:47.75261	2025-11-21 11:03:47.75261
101	336	ae7a6765ee24dc72ab269394e71b0c2889021a91bc945e86f44e8f34e0480f20	OAuth	\N	\N	2025-11-21 11:17:37.137568	2025-11-22 11:17:37.094953	2025-11-21 11:17:37.137568	2025-11-21 11:17:37.137568
103	336	20cdc34c677afd241abf85959789de9dce6ecb9ce6e068ebfaf7d27ca0adf581	OAuth	\N	\N	2025-11-21 11:42:30.769574	2025-11-22 11:42:30.731015	2025-11-21 11:42:30.769574	2025-11-21 11:42:30.769574
104	336	60e54ece1f653cb2fe05a6aa6a9c4502d69062595b44374c4094d3bcb018c650	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 11:42:56.690055	2025-11-21 11:43:10.612587	2025-11-21 11:42:56.690055	2025-11-21 11:42:56.690055
105	336	f4e382c9f74c1f17593de7dd839bf8eb3dd9d782e9a3fabbfd16e5bb33f733a5	OAuth	\N	\N	2025-11-21 11:43:17.06492	2025-11-22 11:43:17.063476	2025-11-21 11:43:17.06492	2025-11-21 11:43:17.06492
106	336	1700708da3bc844daf2327732738d0eb1e471c11e4da28c4fae412d2efe46f9e	OAuth	\N	\N	2025-11-21 11:57:55.268684	2025-11-22 11:57:55.227394	2025-11-21 11:57:55.268684	2025-11-21 11:57:55.268684
107	336	5c299152d6b374fc2f69c9490a71d494a981efd0736e370b0c2d3d0cfc7fcfc7	OAuth	\N	\N	2025-11-21 11:59:45.091525	2025-11-22 11:59:45.090708	2025-11-21 11:59:45.091525	2025-11-21 11:59:45.091525
108	336	5f2c97e1cc9ccdf2b178c3483b54a32e47740ca63ea785b367aa2d4ae909253d	OAuth	\N	\N	2025-11-21 12:02:43.209574	2025-11-22 12:02:43.200567	2025-11-21 12:02:43.209574	2025-11-21 12:02:43.209574
109	336	5fb0ca46de1fc666a54cd3f1eaa64780402e5631060dfaddad32d505e21934bb	OAuth	\N	\N	2025-11-21 12:11:54.77723	2025-11-22 12:11:54.737835	2025-11-21 12:11:54.77723	2025-11-21 12:11:54.77723
110	336	ff1e18879e522061440b5f25644352303ea10b8e65494406885a91cd9be816f7	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 12:15:13.93596	2025-11-21 12:18:44.015879	2025-11-21 12:15:13.93596	2025-11-21 12:15:13.93596
111	336	3b736e96fe0a86f9371888ae8a1f57b799dda94dc8d5561ce42c8906ecdf5242	OAuth	\N	\N	2025-11-21 12:18:52.763115	2025-11-22 12:18:52.762389	2025-11-21 12:18:52.763115	2025-11-21 12:18:52.763115
112	336	e86bc19fd09afb3356e93f2b3c49f94fbf0414008fa1b07977fd7469c48046ee	OAuth	\N	\N	2025-11-21 12:20:08.096703	2025-11-22 12:20:08.096124	2025-11-21 12:20:08.096703	2025-11-21 12:20:08.096703
113	336	e5d1bd9f42d860bc95dd48cb44e904d9fdfdf54f05b561e5113182ef08eb24bc	OAuth	\N	\N	2025-11-21 12:25:19.381325	2025-11-22 12:25:19.380617	2025-11-21 12:25:19.381325	2025-11-21 12:25:19.381325
114	336	50661cd7f93d3b839413308a878dec44e63cf95c934d94b9a5d5949d343bb4de	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 12:26:45.008191	2025-11-21 12:30:48.778327	2025-11-21 12:26:45.008191	2025-11-21 12:26:45.008191
115	336	71d02e2932b0dc0771309eae837268f84733c29981fca00f1f15a24a4260f232	OAuth	\N	\N	2025-11-21 12:31:27.126354	2025-11-22 12:31:27.125745	2025-11-21 12:31:27.126354	2025-11-21 12:31:27.126354
116	336	eb17d7b9873730f3ebcf70c76e2e844f5f03457f6d502ce4b3fc4eb0d592129c	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 12:31:45.340101	2025-11-21 12:31:50.670677	2025-11-21 12:31:45.340101	2025-11-21 12:31:45.340101
117	336	b4cc1a3958fbe81a421ce81d9de3ebcbd21d093b4f4d0fdd89dc5c1fb2012d8c	OAuth	\N	\N	2025-11-21 12:31:57.862194	2025-11-22 12:31:57.861316	2025-11-21 12:31:57.862194	2025-11-21 12:31:57.862194
118	336	1acbf5463294bfa06297550b7a216346e80c31508914cca009fe7b265f3e0db4	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 12:32:03.648133	2025-11-21 12:39:06.11498	2025-11-21 12:32:03.648133	2025-11-21 12:32:03.648133
119	336	7be8aeb704dc038d1e6bf4e8d7ffe3070426237df50c6385072b54d7aa033a25	OAuth	\N	\N	2025-11-21 12:39:15.132805	2025-11-22 12:39:15.131348	2025-11-21 12:39:15.132805	2025-11-21 12:39:15.132805
120	336	64456cf2bda7ab8d6f480ddae4656183b9effbe36a29b3003e26689bb61f9d08	OAuth	\N	\N	2025-11-21 12:45:44.149131	2025-11-22 12:45:44.10946	2025-11-21 12:45:44.149131	2025-11-21 12:45:44.149131
121	336	6cf9e5c3da86a3ff9b6017ae952f2606264362e58a3c209f52991bf7c9165f80	OAuth	\N	\N	2025-11-21 12:47:08.211369	2025-11-22 12:47:08.210822	2025-11-21 12:47:08.211369	2025-11-21 12:47:08.211369
122	336	220c7a08c9e9eea8c1c14791d693e0d30885fa17b07b915164f54b41a5cc316c	OAuth	\N	\N	2025-11-21 13:11:12.962844	2025-11-22 13:11:12.922759	2025-11-21 13:11:12.962844	2025-11-21 13:11:12.962844
123	336	b6077d7e77bf03d200834eaca57d3830e0cfc9f5fc0219248599bb2c5032563e	OAuth	\N	\N	2025-11-21 13:18:05.969529	2025-11-22 13:18:05.968489	2025-11-21 13:18:05.969529	2025-11-21 13:18:05.969529
124	336	960d644eff5d34e416c30cfae108e67888b297295d187744161673eb82a26267	OAuth	\N	\N	2025-11-21 13:25:47.604136	2025-11-22 13:25:47.603475	2025-11-21 13:25:47.604136	2025-11-21 13:25:47.604136
125	336	822e5103533c5195326a55041a524999fc716ae767bf8309fe5f74f3f1202eb2	OAuth	\N	\N	2025-11-21 13:26:10.467409	2025-11-22 13:26:10.467097	2025-11-21 13:26:10.467409	2025-11-21 13:26:10.467409
126	336	c22c13e017d0cbae1f15d48d9e9f428c866d38ba376bc37213c9743978ccbc61	OAuth	\N	\N	2025-11-21 14:06:24.59374	2025-11-22 14:06:24.556462	2025-11-21 14:06:24.59374	2025-11-21 14:06:24.59374
127	336	fc395ef360977981d12bf78a8b891dbc1adcf9f4a4211f9550fb40a3d787da7a	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-11-21 14:08:26.833154	2025-11-22 14:08:26.832361	2025-11-21 14:08:26.833154	2025-11-21 14:08:26.833154
128	352	5aefebc98c3eedeaed07f9245836207ce884b5139caac720d35971770f89e9da	OAuth	\N	\N	2025-12-04 19:10:23.773438	2025-12-05 19:10:23.772968	2025-12-04 19:10:23.773438	2025-12-04 19:10:23.773438
129	352	adfa6eb34afa55df1e690cef4a2f565e43138998d5ab8be3986ed65b60ac8561	OAuth	\N	\N	2025-12-04 19:10:59.960765	2025-12-05 19:10:59.960486	2025-12-04 19:10:59.960765	2025-12-04 19:10:59.960765
130	352	8d999f3aca3eea9f1d8768a8584be7566aaaccef9055133d37d4b8d335da2d1b	OAuth	\N	\N	2025-12-04 19:15:19.266162	2025-12-05 19:15:19.265494	2025-12-04 19:15:19.266162	2025-12-04 19:15:19.266162
131	352	e401eec9efde114fb09d9b0e08d1881b9d857761603f1b548ad73d3bdeb70c45	OAuth	\N	\N	2025-12-04 19:21:17.345954	2025-12-05 19:21:17.345838	2025-12-04 19:21:17.345954	2025-12-04 19:21:17.345954
132	352	05854eee8116bac29a65829fd182e61c5737b1c55306818cac8f3a5150f76e9b	OAuth	\N	\N	2025-12-04 19:23:02.927304	2025-12-05 19:23:02.911677	2025-12-04 19:23:02.927304	2025-12-04 19:23:02.927304
133	352	7ccc5227059f12a115663605ebd2e7f58716fcd941c302499fb1048f58282f94	OAuth	\N	\N	2025-12-05 03:58:58.721953	2025-12-06 03:58:58.721451	2025-12-05 03:58:58.721953	2025-12-05 03:58:58.721953
134	352	2ff95ab98f34b31178dfeb76243fea14f03281aba494eecc3b9f530ce5f1c7f1	OAuth	\N	\N	2025-12-05 04:20:00.947004	2025-12-06 04:20:00.916699	2025-12-05 04:20:00.947004	2025-12-05 04:20:00.947004
135	352	f914eff7ad423c058178adc7efffbd4e0f8c947e7976e9398437456e2338535d	OAuth	\N	\N	2025-12-05 04:26:36.880044	2025-12-06 04:26:36.879505	2025-12-05 04:26:36.880044	2025-12-05 04:26:36.880044
136	352	ce80206bec9254c10fe84ed6846afdcedeb3d8003cbf25d58ef9ffdc0e1a980c	OAuth	\N	\N	2025-12-05 04:31:50.719947	2025-12-06 04:31:50.687612	2025-12-05 04:31:50.719947	2025-12-05 04:31:50.719947
137	352	dbe06d18cb82a56865047154218751e8aa9ed7fb52086e70c9812e84c8f56163	OAuth	\N	\N	2025-12-05 04:37:48.377831	2025-12-06 04:37:48.345644	2025-12-05 04:37:48.377831	2025-12-05 04:37:48.377831
138	352	dd4287c3caa8b4bed557c0cbb6287971b7976b95db3881f245aa4c89079a68db	OAuth	\N	\N	2025-12-05 04:39:06.005409	2025-12-06 04:39:06.004675	2025-12-05 04:39:06.005409	2025-12-05 04:39:06.005409
139	352	db518776f46ee4d60bed78bf37edf80ebadfc5d5b1738e79ba90a0d88008ebb8	OAuth	\N	\N	2025-12-05 04:43:35.193311	2025-12-06 04:43:35.15572	2025-12-05 04:43:35.193311	2025-12-05 04:43:35.193311
140	352	2faf6b6261f559494a0a3b3ce3a7e4945b4d1c7275e49f645548b9d0f3f77db4	OAuth	\N	\N	2025-12-05 04:46:09.732232	2025-12-06 04:46:09.731182	2025-12-05 04:46:09.732232	2025-12-05 04:46:09.732232
141	352	f49365662c7b1334e4918b06ad7e3b60ac7d5decc1ecad28a791823e76074cfd	OAuth	\N	\N	2025-12-05 05:33:24.788357	2025-12-06 05:33:24.760569	2025-12-05 05:33:24.788357	2025-12-05 05:33:24.788357
142	352	d7c8d324c47427be735698ce74c58de53a5b134a407fb6a3234167ce7eef1b7a	OAuth	\N	\N	2025-12-05 05:34:01.275856	2025-12-06 05:34:01.275442	2025-12-05 05:34:01.275856	2025-12-05 05:34:01.275856
143	352	fff7de984971d2e443755bdfb8b9a7bbfba49c070d70845181578bb1b89e6e69	OAuth	\N	\N	2025-12-05 05:34:28.159497	2025-12-06 05:34:28.159325	2025-12-05 05:34:28.159497	2025-12-05 05:34:28.159497
144	352	651b6a0d4472944dc4f44d8c88104b0863b7160f648d7765b273651624920636	OAuth	\N	\N	2025-12-05 05:36:33.758192	2025-12-06 05:36:33.729255	2025-12-05 05:36:33.758192	2025-12-05 05:36:33.758192
145	352	88662ba44b0faa0830e98ae022741568fe4425313ac946a1b388918ae2dd1acd	OAuth	\N	\N	2025-12-05 05:47:27.929084	2025-12-06 05:47:27.889215	2025-12-05 05:47:27.929084	2025-12-05 05:47:27.929084
146	352	547fce0a90a93f6ce9632b0dfa1e50729fbd61f144bceebe85da1f4cddfd1c9a	OAuth	\N	\N	2025-12-05 05:55:44.099664	2025-12-06 05:55:44.099453	2025-12-05 05:55:44.099664	2025-12-05 05:55:44.099664
147	352	066f3c90ce647d6aa38d2d2ed5290bb0a9a3981ecaa5d4a4647a32b16bc553f4	OAuth	\N	\N	2025-12-05 06:01:55.437778	2025-12-06 06:01:55.43708	2025-12-05 06:01:55.437778	2025-12-05 06:01:55.437778
148	352	7fdb0054bea4774f41c2c1b6bfcea6d3b97f3351e346da32871cf84910a782db	OAuth	\N	\N	2025-12-05 06:06:18.762016	2025-12-06 06:06:18.761791	2025-12-05 06:06:18.762016	2025-12-05 06:06:18.762016
149	352	5d083c1693d257a3f0754b6a65312921803fe008cb18ce5abc153c3e61d36c23	OAuth	\N	\N	2025-12-05 06:12:06.34837	2025-12-06 06:12:06.310567	2025-12-05 06:12:06.34837	2025-12-05 06:12:06.34837
150	352	171fd2dfedcc8ec0b0b592c0ac43b9216f8bd21158440508aa3e3d5a3b7128d7	OAuth	\N	\N	2025-12-05 06:14:48.447653	2025-12-06 06:14:48.420212	2025-12-05 06:14:48.447653	2025-12-05 06:14:48.447653
151	352	23cbe64a4210f8e417795dcaf3dc209577f1e2cb6f020dba4fa6a0449beea930	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 06:15:05.691214	2025-12-06 06:15:05.690968	2025-12-05 06:15:05.691214	2025-12-05 06:15:05.691214
152	352	8a3cca296a42b0e9418deb4d19093e8d2a49a98a5129b2605e36cc88576c1334	OAuth	\N	\N	2025-12-05 06:29:29.507101	2025-12-06 06:29:29.478958	2025-12-05 06:29:29.507101	2025-12-05 06:29:29.507101
153	352	366257e14718a4c6e3d6fca4a346b3b4ac7845f6f5a6fc8e588ee63224481ddf	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 06:29:46.6773	2025-12-05 06:54:52.498303	2025-12-05 06:29:46.6773	2025-12-05 06:29:46.6773
154	352	13b5ae51e0ac28fb2d990718674dafe31fa77b3809a0e0216cb1fbc72d19524d	OAuth	\N	\N	2025-12-05 06:54:59.987573	2025-12-06 06:54:59.986856	2025-12-05 06:54:59.987573	2025-12-05 06:54:59.987573
155	352	cf161beb3819ea8d8a2b019a1f52dd42e9a8b232ca87f3d3f86380c88bcc27f2	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 06:55:11.008003	2025-12-05 06:57:57.378011	2025-12-05 06:55:11.008003	2025-12-05 06:55:11.008003
156	352	a5fd93133549868b810661435eef6a794ddd416c24d44f872efa4f4aa8762314	OAuth	\N	\N	2025-12-05 07:14:46.370734	2025-12-06 07:14:46.328567	2025-12-05 07:14:46.370734	2025-12-05 07:14:46.370734
157	352	c5f59bdf66d56cf9479bb6cae42e9f0daeea116d74a39118a514dd2a12b3ef03	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 07:15:24.163581	2025-12-05 07:34:30.366897	2025-12-05 07:15:24.163581	2025-12-05 07:15:24.163581
158	352	a1265d373903adee9efa0bec3ae1a298a8904cc03250efde4a196ac74d57a0ad	OAuth	\N	\N	2025-12-05 07:34:36.723608	2025-12-06 07:34:36.722392	2025-12-05 07:34:36.723608	2025-12-05 07:34:36.723608
159	352	e3ff9df5a86e93631c1425860b1f17c550d25f7f0471a81c6734a854f0584ec9	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 07:35:02.664736	2025-12-06 07:35:02.664337	2025-12-05 07:35:02.664736	2025-12-05 07:35:02.664736
160	352	fffd9e92f9233d493c302de720f505a0b6e9cc979ae7d2b95a845db094b665c5	OAuth	\N	\N	2025-12-05 08:04:22.088826	2025-12-06 08:04:22.059962	2025-12-05 08:04:22.088826	2025-12-05 08:04:22.088826
161	352	a07ec607f8ac9cadc00b7865dd7641ebb3bdaf17d91f81f28d94251ef6b9dde3	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 08:04:53.253058	2025-12-06 08:04:53.251012	2025-12-05 08:04:53.253058	2025-12-05 08:04:53.253058
162	352	ba4a3e1a4d51ba4c5807b1da7542fbfa4f2febed86779bce6a6c47e0d24f130b	OAuth	\N	\N	2025-12-05 08:24:25.4407	2025-12-06 08:24:25.411506	2025-12-05 08:24:25.4407	2025-12-05 08:24:25.4407
163	352	7f6dfccaea634bc2f6b24b971e1363256d020c9214f5c61cb76f1a3c75c4622e	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 08:24:50.086753	2025-12-06 08:24:50.085601	2025-12-05 08:24:50.086753	2025-12-05 08:24:50.086753
164	352	99dcee815e729e71db30f71751028501750fba0a72016a8c3653134523a7f9a3	OAuth	\N	\N	2025-12-05 08:29:55.326776	2025-12-06 08:29:55.298332	2025-12-05 08:29:55.326776	2025-12-05 08:29:55.326776
165	352	c4b75432b0ab1576db307f171adb6ecc0292baf9335a21ea8a422bec37e11299	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 08:30:13.048165	2025-12-05 08:49:00.628074	2025-12-05 08:30:13.048165	2025-12-05 08:30:13.048165
166	352	6d6e1d7a0f3b2aaeb0ffc144b95f76034bfa158fb29b3ef147722925f5d83a7d	OAuth	\N	\N	2025-12-05 08:49:06.427332	2025-12-06 08:49:06.426564	2025-12-05 08:49:06.427332	2025-12-05 08:49:06.427332
167	352	6e2dfa0345fb06ef65b6ba94d4c2d4c823b5dbe545e05883f9cc17412e839663	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 08:49:42.481496	2025-12-05 08:56:06.559863	2025-12-05 08:49:42.481496	2025-12-05 08:49:42.481496
168	352	b962156aaae9f0dafef2a32b67642aaa34f49105cf8b237b7c39b317ea92731b	OAuth	\N	\N	2025-12-05 08:56:12.221373	2025-12-06 08:56:12.22073	2025-12-05 08:56:12.221373	2025-12-05 08:56:12.221373
169	352	a4e18b349c2f78a45eeab7dea63b0ba0ab26d9fb0aa57adfd92fbba9b55c4ea1	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 08:56:40.117908	2025-12-06 08:56:40.11751	2025-12-05 08:56:40.117908	2025-12-05 08:56:40.117908
170	336	59c1e16a8d264055ac9e2468a6d0a5d19a464080fcc125c9118448b986af3766	OAuth	\N	\N	2025-12-05 08:57:14.917571	2025-12-06 08:57:14.917381	2025-12-05 08:57:14.917571	2025-12-05 08:57:14.917571
171	336	5d45f5452d8021027f7b6c69907d0f5eb4dcacdada2581361306fb03b986cd1d	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-05 08:57:33.10129	2025-12-05 10:21:04.148719	2025-12-05 08:57:33.10129	2025-12-05 08:57:33.10129
172	336	0fb52435d0f9d2bb8a9e20e958007fc27f9b228c5e2d28c37fa3b3af01efefe2	OAuth	\N	\N	2025-12-06 06:00:50.725745	2025-12-07 06:00:50.72064	2025-12-06 06:00:50.725745	2025-12-06 06:00:50.725745
173	336	521cb4881a4d3f7f37cf6b35741791a32874fb9f07dc266883cca04dd0d694d7	Desktop	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 06:01:10.939544	2025-12-06 06:03:04.486998	2025-12-06 06:01:10.939544	2025-12-06 06:01:10.939544
174	336	829ba0a6a463302586cff4c6f7b1b9ba3f65601b545c3c20b951ffdd527e0174	OAuth	\N	\N	2025-12-06 07:48:31.913816	2025-12-07 07:48:31.908569	2025-12-06 07:48:31.913816	2025-12-06 07:48:31.913816
177	336	5231f2cf68c5380d25b27c484a6d80a25fddd60634f7112017eefeaae068a6a0	OAuth	\N	\N	2025-12-06 08:13:19.57868	2025-12-07 08:13:19.578575	2025-12-06 08:13:19.57868	2025-12-06 08:13:19.57868
180	352	54442d3d28bba50371f1b4b84da098137a863b34a9f7edad579b867e419b132b	OAuth	\N	\N	2025-12-06 09:25:55.256573	2025-12-07 09:25:55.229433	2025-12-06 09:25:55.256573	2025-12-06 09:25:55.256573
181	352	439600331f789e94bca1c42da0048b5095fe672784bf0a173e0128ab99830ba2	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 09:26:40.102256	2025-12-06 09:26:48.562558	2025-12-06 09:26:40.102256	2025-12-06 09:26:40.102256
182	352	4b410265447e76bea5b0f8cc36e14432ee9c5c8907d5fb931e0502e2a9c5c089	OAuth	\N	\N	2025-12-06 09:27:15.440537	2025-12-07 09:27:15.440327	2025-12-06 09:27:15.440537	2025-12-06 09:27:15.440537
183	1	3f8136f3690a9aa4983864b8c728c72780d679191633c3e27186ad1230c22e09	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 09:55:04.315366	2025-12-06 10:12:35.738394	2025-12-06 09:55:04.315366	2025-12-06 09:55:04.315366
184	1	e219b371d1f462440339b767e02fd94e5dbaab6e8741330ce87bba332c46e9c1	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 10:13:16.893339	2025-12-07 10:13:16.892732	2025-12-06 10:13:16.893339	2025-12-06 10:13:16.893339
185	1	3c32ad1866f1db3c2a0e8aff53b5a28c66b25fbd83bb5ef94732c72413f3f036	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 10:39:28.901119	2025-12-07 10:39:28.900406	2025-12-06 10:39:28.901119	2025-12-06 10:39:28.901119
186	336	777ba0bdeb76698b346f77a65e693e225418d6e90597d4f003b170e2f03a276d	OAuth	\N	\N	2025-12-06 10:40:01.299104	2025-12-07 10:40:01.29876	2025-12-06 10:40:01.299104	2025-12-06 10:40:01.299104
187	336	bea631faf33ddfb66d74cf9642f337ae9f57e043d4bf0252ddc4cead8d777134	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 10:40:19.249092	2025-12-07 10:40:19.24842	2025-12-06 10:40:19.249092	2025-12-06 10:40:19.249092
188	345	c0f6e9269801167f87dcbff026f7813e93641bdaa2997d03034ff0e6b087bb68	OAuth	\N	\N	2025-12-06 10:59:31.985256	2025-12-07 10:59:31.984373	2025-12-06 10:59:31.985256	2025-12-06 10:59:31.985256
189	345	3f7c80a6de7fca83cacf21ec523ca06d4afbc70d5d163949a1476f89999fb2dc	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 10:59:46.818483	2025-12-07 10:59:46.818269	2025-12-06 10:59:46.818483	2025-12-06 10:59:46.818483
190	336	f99bd0735a6dcd98511195c0ca27ebe34bde3bfb617817d2f6c7dd022b070e84	OAuth	\N	\N	2025-12-06 11:01:18.148298	2025-12-07 11:01:18.118049	2025-12-06 11:01:18.148298	2025-12-06 11:01:18.148298
191	336	b4d916f9e38166e55cd9bb4464f7e3315018b4b816b4a784c0308fe7f678f62a	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 11:01:34.371936	2025-12-07 11:01:34.371559	2025-12-06 11:01:34.371936	2025-12-06 11:01:34.371936
192	352	9dc4487098e8b089f3189c2556f9c937cd7e33be21011fcee3501784604439c4	OAuth	\N	\N	2025-12-06 11:05:06.650934	2025-12-07 11:05:06.650008	2025-12-06 11:05:06.650934	2025-12-06 11:05:06.650934
193	352	2b435ec10c01e4e86d5fbd52779d632b6f5203eb5b1ced4adc33e93fba58db94	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 11:05:17.535484	2025-12-06 11:05:29.241396	2025-12-06 11:05:17.535484	2025-12-06 11:05:17.535484
194	1	20be6edd0f210856b23907939ca7575b53eea462f80cb621c3171a9bf2eb775a	Desktop	127.0.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 11:06:08.983323	2025-12-06 11:40:48.037344	2025-12-06 11:06:08.983323	2025-12-06 11:06:08.983323
195	352	502c9d744bf099e0549243f2573748d28b4d5123b576bea900d8d11d82745503	OAuth	\N	\N	2025-12-06 13:08:45.87371	2025-12-07 13:08:45.869478	2025-12-06 13:08:45.87371	2025-12-06 13:08:45.87371
196	352	ce67782343064d1298b91802bccb0dc0cdf6f0e8652319e245d56ee2cccd9760	OAuth	\N	\N	2025-12-06 13:34:43.928822	2025-12-07 13:34:43.92451	2025-12-06 13:34:43.928822	2025-12-06 13:34:43.928822
197	1	e50e53285e0c85dcdc780ca83a950397dd719ac57cba4d3d7457eb036408bd56	Desktop	172.24.0.1	Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36	2025-12-06 13:40:40.131502	2025-12-07 13:40:40.131404	2025-12-06 13:40:40.131502	2025-12-06 13:40:40.131502
\.


--
-- Data for Name: token_blacklist; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.token_blacklist (id, token_hash, user_id, reason, expires_at, blacklisted_at) FROM stdin;
6	5276771600ae1f1ea8ff15d4b19871f69856240f673451cb4d507f6cbb639fc9	333	User logout	2025-11-21 05:52:03	2025-11-20 05:58:26.090603
13	10f966281a06afef937511da1c4c1982543b4f46a2516dfb20bcae7785232544	336	User logout	2025-11-21 07:09:51	2025-11-20 07:38:09.079369
14	76943061db506aed425dfe462681a2d63ef1af2d1aa6340e712236dd8711248b	336	User logout	2025-11-21 07:38:16	2025-11-20 11:45:21.103062
15	9d6639565ecb4d91f7dde38584cc022df5882167631a5ad1e000a2bab9f5f522	336	User logout	2025-11-21 11:45:28	2025-11-20 12:07:53.979005
16	7dab0bb27aab7941cc6b42b49dac3b4ada5b97e8086897c2ca7436b1b1764f85	336	User logout	2025-11-21 12:08:01	2025-11-20 12:19:47.695133
17	94abeee18ac81cbc91c774501c85c97e198c8da7edb2b331a3c6797b40c22383	336	User logout	2025-11-21 12:25:45	2025-11-20 12:31:39.271307
18	e8110dd877ae530c3bde99f668331424b8a31341e7867250a9b4ef5b13582ebc	336	User logout	2025-11-21 12:39:23	2025-11-20 18:19:00.963895
19	e80d55f21453cf6431b383e03acbc427215d6f791c94e2a15c516ecd6c7c8a6e	336	User logout	2025-11-21 19:12:20	2025-11-20 20:01:07.202572
20	2e356f2f23b9101e62076bb6e7970a3b94ed9d794736045712d6f656144feaa4	336	User logout	2025-11-22 03:43:20	2025-11-21 03:43:26.819873
21	3d4de2f2a22ab1d8a1f8bba95606a19adb9b746e7897eaaa6d390b21b1c33371	336	User logout	2025-11-22 04:44:55	2025-11-21 04:49:26.984467
22	3b10528e877cc3ff1c3ba6c7343b1c1edfc1933b228551d5fae00bb226a41cb3	336	User logout	2025-11-22 04:50:06	2025-11-21 04:50:33.043523
23	8f987e11679eaa608609252133218e7f4f9309de435c1933dbf5f3ae64309e68	336	User logout	2025-11-22 04:51:08	2025-11-21 05:32:05.751117
24	9e6ad892726d404a5d827b022d10b5eb7f2b77298b6ad324887909377817ba90	336	User logout	2025-11-22 09:28:53	2025-11-21 09:33:54.500137
25	3dd7c94b5f11f0e97fb69a5f1ae427b381027b2a7ee8e1cb2bc3173b6275d82b	336	User logout	2025-11-22 10:04:21	2025-11-21 10:08:15.533455
26	60e54ece1f653cb2fe05a6aa6a9c4502d69062595b44374c4094d3bcb018c650	336	User logout	2025-11-22 11:42:56	2025-11-21 11:43:10.616536
27	ff1e18879e522061440b5f25644352303ea10b8e65494406885a91cd9be816f7	336	User logout	2025-11-22 12:15:13	2025-11-21 12:18:44.027622
28	50661cd7f93d3b839413308a878dec44e63cf95c934d94b9a5d5949d343bb4de	336	User logout	2025-11-22 12:26:45	2025-11-21 12:30:48.789558
29	eb17d7b9873730f3ebcf70c76e2e844f5f03457f6d502ce4b3fc4eb0d592129c	336	User logout	2025-11-22 12:31:45	2025-11-21 12:31:50.681095
30	1acbf5463294bfa06297550b7a216346e80c31508914cca009fe7b265f3e0db4	336	User logout	2025-11-22 12:32:03	2025-11-21 12:39:06.121184
31	366257e14718a4c6e3d6fca4a346b3b4ac7845f6f5a6fc8e588ee63224481ddf	352	User logout	2025-12-06 06:29:46	2025-12-05 06:54:52.503797
32	cf161beb3819ea8d8a2b019a1f52dd42e9a8b232ca87f3d3f86380c88bcc27f2	352	User logout	2025-12-06 06:55:11	2025-12-05 06:57:57.389619
33	c5f59bdf66d56cf9479bb6cae42e9f0daeea116d74a39118a514dd2a12b3ef03	352	User logout	2025-12-06 07:15:24	2025-12-05 07:34:30.372005
34	c4b75432b0ab1576db307f171adb6ecc0292baf9335a21ea8a422bec37e11299	352	User logout	2025-12-06 08:30:13	2025-12-05 08:49:00.632758
35	6e2dfa0345fb06ef65b6ba94d4c2d4c823b5dbe545e05883f9cc17412e839663	352	User logout	2025-12-06 08:49:42	2025-12-05 08:56:06.577445
36	5d45f5452d8021027f7b6c69907d0f5eb4dcacdada2581361306fb03b986cd1d	336	User logout	2025-12-06 08:57:33	2025-12-05 10:21:04.153861
37	95f55d157af772c62ba38bae471c1af5ebf346fc12e64384a4da1fa55eb59530	361	User logout	2025-12-06 10:31:40	2025-12-05 10:32:16.09462
38	db7a11a4c4923d991da6719216564aadfefb6704ef567ea7fb8f4f6b5d4df3fc	361	User logout	2025-12-06 10:32:55	2025-12-05 10:33:15.244163
39	a3c566ef178772ffbea3bda14489f78862a1e6ba2d627b6d975895b59dc417e6	361	User logout	2025-12-06 10:33:23	2025-12-05 10:36:07.027663
40	521cb4881a4d3f7f37cf6b35741791a32874fb9f07dc266883cca04dd0d694d7	336	User logout	2025-12-07 06:01:10	2025-12-06 06:03:04.497832
41	ad2ca647f1363f88803057cd80d1582434c3148ef886131a3de2336c742745e8	361	User logout	2025-12-07 06:03:24	2025-12-06 06:04:08.546524
43	3443114eb3c249b69906e4bb6e6cf702cfbb3dfc134afc4b47c87144ef037ea1	361	User logout	2025-12-07 08:04:38	2025-12-06 08:04:50.490646
47	439600331f789e94bca1c42da0048b5095fe672784bf0a173e0128ab99830ba2	352	User logout	2025-12-07 09:26:40	2025-12-06 09:26:48.575952
48	3f8136f3690a9aa4983864b8c728c72780d679191633c3e27186ad1230c22e09	1	User logout	2025-12-07 09:55:04	2025-12-06 10:12:35.749214
49	2b435ec10c01e4e86d5fbd52779d632b6f5203eb5b1ced4adc33e93fba58db94	352	User logout	2025-12-07 11:05:17	2025-12-06 11:05:29.254162
50	20be6edd0f210856b23907939ca7575b53eea462f80cb621c3171a9bf2eb775a	1	User logout	2025-12-07 11:06:08	2025-12-06 11:40:48.043136
51	2524c1ea2ac89ebd93fa5df3c85df70418e8f5cde3ab35cf47bb7ccdb0e5eb49	352	User logout	2025-12-07 13:35:34	2025-12-06 13:36:07.291665
52	5311e1b3fb07c85a8aaa4c6e799aa2ed5b362a7a0fcae5c9dbe3596f2987b469	352	User logout	2025-12-07 13:36:50	2025-12-06 13:39:32.498863
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.users (id, username, email, password, role, totp_secret, totp_enabled, created_at, updated_at, wallet_address, email_verified, recovery_codes, is_banned, banned_until, last_login) FROM stdin;
1	admin	admin@example.com	$2b$12$Njeiw7bT3ET7M7I9PpOL3OjbSzT114Ykb9WZmWDnTL.CPhj8gNMBe	admin	XW2WIPD7ZF4MUGG7SJWGQOA3T67BHAOJ	t	2025-11-17 18:39:43.050283	2025-12-06 10:30:24.795792	\N	t	{2MSW-VBKE,6TMY-WSMM,L56O-SIHW,V3GN-X6FD,WQIR-6UFK,KR6D-57RE,DO6Q-DGAR,73US-E2IX}	f	\N	2025-12-06 13:40:42.310433
361	user_627d8d62	\N	web3_auth	user	\N	f	2025-12-05 10:31:40.196764	2025-12-05 10:31:40.196764	0x95eed6d9be7e772ac21f815d35e1a970627d8d62	t	\N	f	\N	\N
336	rizkipurno	rizkipurnomo914@gmail.com	google_oauth	user	CDJELYO2QVKAPEQOGOTIBFG56UCCBKDG	t	2025-11-20 07:09:51.288298	2025-12-06 10:39:40.735806	\N	t	{65WB-L62D,NTH0-CX8O,KGBR-BAVP,FL1P-9DN2,OUTW-0MG3,5SSP-RR3L,5CDP-Y124,FI46-6KFE}	t	2025-12-08 10:39:40.735255	2025-12-06 11:34:05.890249
333	testadmin_ush_333	testadmin@test.com	$2b$12$TXZLu8hXHQcKyhqPrexBven355oUJXMppO//q3qFFF2CJXILVmRYy	admin	CXOIJ42RMKUO6SLPREEH3ORBRGN5GWYL	f	2025-11-20 05:50:38.659168	2025-11-20 05:50:38.659168	\N	t	\N	f	\N	\N
345	jiooi_ush_345	pr001280@gmail.com	$2b$12$vusSd0oPWJPqsHTY2nU24OdIXVVNgSlve/LT/3LUFsK5jHK1ziDie	user	\N	f	2025-11-21 09:34:43.262191	2025-12-06 11:30:44.142751	\N	t	\N	f	\N	2025-12-06 11:53:43.847309
352	jokowi	rizkipurnomopratama@gmail.com	google_oauth	user	AHPSQKSKKNCMDYMAQIM5SEFGR2YLNII3	t	2025-12-04 19:10:23.771525	2025-12-05 06:36:50.323026	0x097edb6f9de3c8066fafc59b9ce9cd9f9713b40c	t	{9MQI-78MD,RGOE-OPLV,41MG-3Y9H,5FZS-4M27,0T4Z-JBBZ,YP6D-J9MX,SETC-CPGM,TM5A-Z680}	f	\N	2025-12-06 13:39:32.497797
\.


--
-- Data for Name: web3_challenges; Type: TABLE DATA; Schema: public; Owner: admin
--

COPY public.web3_challenges (id, address, challenge, expires_at, used_at, created_at) FROM stdin;
17	0x097edb6f9de3c8066fafc59b9ce9cd9f9713b40c	Welcome to USH!\n\nPlease sign this message to authenticate with your wallet.\n\nAddress: 0x097edb6f9de3c8066fafc59b9ce9cd9f9713b40c\n\nChallenge: 096e7f8a39444b513670e95214a633ff14fbe16f594439ebddf807b75145e613\n\nTimestamp: 1765028092	2025-12-06 13:39:52.460061	2025-12-06 13:35:34.065141	2025-12-06 13:34:52.460507
18	0x097edb6f9de3c8066fafc59b9ce9cd9f9713b40c	Welcome to USH!\n\nPlease sign this message to authenticate with your wallet.\n\nAddress: 0x097edb6f9de3c8066fafc59b9ce9cd9f9713b40c\n\nChallenge: 89ffc9fb2eb4aee15fcf4efff9d1bfc599d6a3827580354299e5a8aa2ca4a6cc\n\nTimestamp: 1765028171	2025-12-06 13:41:11.247933	2025-12-06 13:36:50.338645	2025-12-06 13:36:11.248259
\.


--
-- Name: account_lockout_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.account_lockout_id_seq', 2, true);


--
-- Name: audit_logs_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.audit_logs_id_seq', 317, true);


--
-- Name: comments_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.comments_id_seq', 15, true);


--
-- Name: likes_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.likes_id_seq', 36, true);


--
-- Name: posts_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.posts_id_seq', 16, true);


--
-- Name: refresh_tokens_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.refresh_tokens_id_seq', 172, true);


--
-- Name: sessions_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.sessions_id_seq', 197, true);


--
-- Name: token_blacklist_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.token_blacklist_id_seq', 52, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.users_id_seq', 364, true);


--
-- Name: web3_challenges_id_seq; Type: SEQUENCE SET; Schema: public; Owner: admin
--

SELECT pg_catalog.setval('public.web3_challenges_id_seq', 18, true);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: account_lockout account_lockout_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.account_lockout
    ADD CONSTRAINT account_lockout_pkey PRIMARY KEY (id);


--
-- Name: account_lockout account_lockout_user_id_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.account_lockout
    ADD CONSTRAINT account_lockout_user_id_key UNIQUE (user_id);


--
-- Name: audit_logs audit_logs_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.audit_logs
    ADD CONSTRAINT audit_logs_pkey PRIMARY KEY (id);


--
-- Name: comments comments_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_pkey PRIMARY KEY (id);


--
-- Name: likes likes_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.likes
    ADD CONSTRAINT likes_pkey PRIMARY KEY (id);


--
-- Name: likes likes_post_id_user_id_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.likes
    ADD CONSTRAINT likes_post_id_user_id_key UNIQUE (post_id, user_id);


--
-- Name: posts posts_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_pkey PRIMARY KEY (id);


--
-- Name: refresh_tokens refresh_tokens_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.refresh_tokens
    ADD CONSTRAINT refresh_tokens_pkey PRIMARY KEY (id);


--
-- Name: refresh_tokens refresh_tokens_token_hash_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.refresh_tokens
    ADD CONSTRAINT refresh_tokens_token_hash_key UNIQUE (token_hash);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- Name: sessions sessions_token_hash_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_token_hash_key UNIQUE (token_hash);


--
-- Name: token_blacklist token_blacklist_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.token_blacklist
    ADD CONSTRAINT token_blacklist_pkey PRIMARY KEY (id);


--
-- Name: token_blacklist token_blacklist_token_hash_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.token_blacklist
    ADD CONSTRAINT token_blacklist_token_hash_key UNIQUE (token_hash);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: users users_wallet_address_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_wallet_address_key UNIQUE (wallet_address);


--
-- Name: web3_challenges web3_challenges_challenge_key; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.web3_challenges
    ADD CONSTRAINT web3_challenges_challenge_key UNIQUE (challenge);


--
-- Name: web3_challenges web3_challenges_pkey; Type: CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.web3_challenges
    ADD CONSTRAINT web3_challenges_pkey PRIMARY KEY (id);


--
-- Name: idx_account_lockout_locked_until; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_account_lockout_locked_until ON public.account_lockout USING btree (locked_until);


--
-- Name: idx_account_lockout_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_account_lockout_user_id ON public.account_lockout USING btree (user_id);


--
-- Name: idx_audit_logs_created_at; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_audit_logs_created_at ON public.audit_logs USING btree (created_at);


--
-- Name: idx_audit_logs_event_type; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_audit_logs_event_type ON public.audit_logs USING btree (event_type);


--
-- Name: idx_audit_logs_user_event; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_audit_logs_user_event ON public.audit_logs USING btree (user_id, event_type, created_at DESC);


--
-- Name: idx_audit_logs_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_audit_logs_user_id ON public.audit_logs USING btree (user_id);


--
-- Name: idx_comments_parent_comment_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_comments_parent_comment_id ON public.comments USING btree (parent_comment_id);


--
-- Name: idx_comments_post_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_comments_post_id ON public.comments USING btree (post_id);


--
-- Name: idx_comments_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_comments_user_id ON public.comments USING btree (user_id);


--
-- Name: idx_likes_post_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_likes_post_id ON public.likes USING btree (post_id);


--
-- Name: idx_likes_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_likes_user_id ON public.likes USING btree (user_id);


--
-- Name: idx_posts_content_tsvector; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_posts_content_tsvector ON public.posts USING gin (to_tsvector('english'::regconfig, (((title)::text || ' '::text) || content)));


--
-- Name: idx_posts_created_at; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_posts_created_at ON public.posts USING btree (created_at);


--
-- Name: idx_posts_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_posts_user_id ON public.posts USING btree (user_id);


--
-- Name: idx_refresh_tokens_expires_at; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_refresh_tokens_expires_at ON public.refresh_tokens USING btree (expires_at);


--
-- Name: idx_refresh_tokens_token_family; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_refresh_tokens_token_family ON public.refresh_tokens USING btree (token_family);


--
-- Name: idx_refresh_tokens_token_hash; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_refresh_tokens_token_hash ON public.refresh_tokens USING btree (token_hash);


--
-- Name: idx_refresh_tokens_user_family; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_refresh_tokens_user_family ON public.refresh_tokens USING btree (user_id, token_family);


--
-- Name: idx_refresh_tokens_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_refresh_tokens_user_id ON public.refresh_tokens USING btree (user_id);


--
-- Name: idx_sessions_expires_at; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_sessions_expires_at ON public.sessions USING btree (expires_at);


--
-- Name: idx_sessions_token_hash; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_sessions_token_hash ON public.sessions USING btree (token_hash);


--
-- Name: idx_sessions_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_sessions_user_id ON public.sessions USING btree (user_id);


--
-- Name: idx_token_blacklist_expires_at; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_token_blacklist_expires_at ON public.token_blacklist USING btree (expires_at);


--
-- Name: idx_token_blacklist_token_hash; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_token_blacklist_token_hash ON public.token_blacklist USING btree (token_hash);


--
-- Name: idx_token_blacklist_user_id; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_token_blacklist_user_id ON public.token_blacklist USING btree (user_id);


--
-- Name: idx_users_email; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_users_email ON public.users USING btree (email);


--
-- Name: idx_users_role; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_users_role ON public.users USING btree (role);


--
-- Name: idx_users_username_tsvector; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_users_username_tsvector ON public.users USING gin (to_tsvector('english'::regconfig, (username)::text));


--
-- Name: idx_web3_challenges_address; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_web3_challenges_address ON public.web3_challenges USING btree (address);


--
-- Name: idx_web3_challenges_challenge; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_web3_challenges_challenge ON public.web3_challenges USING btree (challenge);


--
-- Name: idx_web3_challenges_expires; Type: INDEX; Schema: public; Owner: admin
--

CREATE INDEX idx_web3_challenges_expires ON public.web3_challenges USING btree (expires_at);


--
-- Name: account_lockout account_lockout_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.account_lockout
    ADD CONSTRAINT account_lockout_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: comments comments_parent_comment_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_parent_comment_id_fkey FOREIGN KEY (parent_comment_id) REFERENCES public.comments(id) ON DELETE CASCADE;


--
-- Name: comments comments_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.posts(id) ON DELETE CASCADE;


--
-- Name: comments comments_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.comments
    ADD CONSTRAINT comments_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: likes likes_post_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.likes
    ADD CONSTRAINT likes_post_id_fkey FOREIGN KEY (post_id) REFERENCES public.posts(id) ON DELETE CASCADE;


--
-- Name: likes likes_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.likes
    ADD CONSTRAINT likes_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: posts posts_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.posts
    ADD CONSTRAINT posts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: refresh_tokens refresh_tokens_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.refresh_tokens
    ADD CONSTRAINT refresh_tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sessions sessions_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: token_blacklist token_blacklist_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: admin
--

ALTER TABLE ONLY public.token_blacklist
    ADD CONSTRAINT token_blacklist_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

--
-- PostgreSQL database cluster dump complete
--

