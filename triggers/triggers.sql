CREATE OR REPLACE FUNCTION ledger_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'ledger_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER ledger_notify_update
AFTER INSERT OR UPDATE
ON pmm_new."storage.ledger_map"
FOR EACH ROW
EXECUTE PROCEDURE ledger_notify_update();