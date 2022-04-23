-- Your SQL goes here
CREATE TABLE transactions (
    id varchar NOT NULL,
    user_id varchar NOT NULL,
    banco_org varchar NOT NULL,
    agencia_org varchar NOT NULL,
    conta_org varchar NOT NULL,
    banco_dest varchar NOT NULL,
    agencia_dest varchar NOT NULL,
    conta_dest varchar NOT NULL,
    valor real NOT NULL,
    data varchar NOT NULL,
    PRIMARY KEY (id),
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
            REFERENCES users(id)
);