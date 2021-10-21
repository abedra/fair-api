CREATE TABLE IF NOT EXISTS scenarios
(
    id                         SERIAL PRIMARY KEY,
    name                       TEXT NOT NULL,
    qualified_probable_risk    TEXT NOT NULL,
    qualified_worst_case_risk  TEXT NOT NULL,
    quantified_probable_loss   REAL NOT NULL,
    quantified_worst_case_loss REAL NOT NULL,
    created_at                 TIMESTAMP DEFAULT now()
);
