CREATE TABLE diffie_hellman_exchanges (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL
		REFERENCES users(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE,
	g BIGINT NOT NULL,
	n BIGINT NOT NULL,
	sk_server BIGINT NOT NULL,
	pk_user BIGINT,
	k BIGINT,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	completed_at TIMESTAMP WITH TIME ZONE,
	CONSTRAINT diffie_hellman_exchange_completed_check CHECK (pk_user IS NULL AND k IS NULL AND completed_at IS NULL OR pk_user IS NOT NULL AND k IS NOT NULL AND completed_at IS NOT NULL)
);
