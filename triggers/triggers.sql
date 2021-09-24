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

CREATE OR REPLACE FUNCTION market_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'market_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER market_notify_update
AFTER INSERT OR UPDATE
ON pmm_new."storage.market_map"
FOR EACH ROW
EXECUTE PROCEDURE market_notify_update();


CREATE OR REPLACE FUNCTION liquidity_provider_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'liquidity_provider_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER liquidity_provider_notify_update
AFTER INSERT OR UPDATE
ON pmm_new."storage.liquidity_provider_map"
FOR EACH ROW
EXECUTE PROCEDURE liquidity_provider_notify_update();


CREATE OR REPLACE FUNCTION token_supply_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'token_supplies_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER token_supply_notify_update
AFTER INSERT OR UPDATE
ON pmm_new."storage.supply_map"
FOR EACH ROW
EXECUTE PROCEDURE token_supply_notify_update();