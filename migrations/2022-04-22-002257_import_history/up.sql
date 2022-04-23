-- Your SQL goes here
CREATE TABLE import_history (
    id varchar NOT NULL,
    user_id varchar NOT NULL,
    data_transacoes varchar NOT NULL,
    data_importacao varchar NOT NULL,
    PRIMARY KEY(id),
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);