-- Your SQL goes here
CREATE TABLE cash_inflow (
    inflow_id SERIAL PRIMARY KEY,
    amount DECIMAL,
    description TEXT,
    user_id INT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
