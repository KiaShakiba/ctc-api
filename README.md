# ctc-api

# Database

```
CREATE TABLE public.users
(
	datetime timestamp with time zone NOT NULL DEFAULT now(),
	username character varying(16) COLLATE pg_catalog."default" NOT NULL,
	password character(1024) COLLATE pg_catalog."default" NOT NULL,
	salt character(128) COLLATE pg_catalog."default" NOT NULL,
	type text COLLATE pg_catalog."default" NOT NULL DEFAULT 'student'::text,
	CONSTRAINT users_pkey PRIMARY KEY (username)
);

CREATE TABLE public.sessions
(
	datetime timestamp with time zone NOT NULL DEFAULT now(),
	username character varying(16) COLLATE pg_catalog."default" NOT NULL,
	token character(512) COLLATE pg_catalog."default" NOT NULL,
	CONSTRAINT sessions_pkey PRIMARY KEY (token),
	CONSTRAINT sessions_fkey_username FOREIGN KEY (username)
	REFERENCES public.users (username) MATCH SIMPLE
		ON UPDATE CASCADE
		ON DELETE CASCADE
);

CREATE TABLE public.caesar
(
	datetime_created timestamp with time zone NOT NULL DEFAULT now(),
	datetime_submitted timestamp with time zone,
	username character varying(16) COLLATE pg_catalog."default" NOT NULL,
	key numeric,
	message text COLLATE pg_catalog."default",
	cipher text COLLATE pg_catalog."default",
	type text COLLATE pg_catalog."default" NOT NULL,
	CONSTRAINT caesar_fkey_username FOREIGN KEY (username)
	REFERENCES public.users (username) MATCH SIMPLE
		ON UPDATE CASCADE
		ON DELETE CASCADE
);

CREATE TABLE public.rsa
(
    datetime_created timestamp with time zone NOT NULL DEFAULT now(),
    datetime_submitted timestamp with time zone,
    username character varying(16) COLLATE pg_catalog."default" NOT NULL,
    p numeric NOT NULL,
    q numeric NOT NULL,
    e numeric NOT NULL,
    d numeric NOT NULL,
    m numeric,
    c numeric,
    type text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT rsa_fkey_username FOREIGN KEY (username)
        REFERENCES public.users (username) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE public.dss
(
    datetime_created timestamp with time zone NOT NULL DEFAULT now(),
    datetime_submitted timestamp with time zone,
    username character varying(16) COLLATE pg_catalog."default" NOT NULL,
    p numeric NOT NULL,
    q numeric NOT NULL,
    g numeric NOT NULL,
    h text,
    r numeric,
    s numeric,
    sk numeric,
    pk numeric,
    k numeric,
    u numeric,
    v numeric,
    w numeric,
    m numeric NOT NULL,
    type text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT rsa_fkey_username FOREIGN KEY (username)
        REFERENCES public.users (username) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
```
