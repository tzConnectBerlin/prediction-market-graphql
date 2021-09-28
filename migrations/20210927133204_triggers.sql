CREATE OR REPLACE FUNCTION ledger_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'ledger_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION market_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'market_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION liquidity_provider_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'liquidity_provider_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION token_supply_notify_update()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify(
    'token_supplies_notify', 'payload'
  );
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


DROP TRIGGER IF EXISTS ledger_notify_update_trigger on pmm_new."storage.ledger_map";
DROP TRIGGER IF EXISTS market_notify_update_trigger on pmm_new."storage.market_map";
DROP TRIGGER IF EXISTS liquidity_provider_notify_update_trigger on pmm_new."storage.liquidity_provider_map";
DROP TRIGGER IF EXISTS token_supply_notify_update_trigger on pmm_new."storage.supply_map";


CREATE TRIGGER ledger_notify_update_trigger
AFTER INSERT OR UPDATE
ON pmm_new."storage.ledger_map"
FOR EACH ROW
EXECUTE PROCEDURE ledger_notify_update();

CREATE TRIGGER market_notify_update_trigger
AFTER INSERT OR UPDATE
ON pmm_new."storage.market_map"
FOR EACH ROW
EXECUTE PROCEDURE market_notify_update();

CREATE TRIGGER liquidity_provider_notify_update_trigger
AFTER INSERT OR UPDATE
ON pmm_new."storage.liquidity_provider_map"
FOR EACH ROW
EXECUTE PROCEDURE liquidity_provider_notify_update();


CREATE TRIGGER token_supply_notify_update_trigger
AFTER INSERT OR UPDATE
ON pmm_new."storage.supply_map"
FOR EACH ROW
EXECUTE PROCEDURE token_supply_notify_update();