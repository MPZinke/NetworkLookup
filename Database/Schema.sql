
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2022.04.29                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


CREATE TYPE NetworkType AS ENUM ('Asus', 'NetGear');


-- SUMMARY:  List of Networkss that are tracked.
DROP TABLE IF EXISTS "Networks" CASCADE;
CREATE TABLE "Networks"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL DEFAULT '' UNIQUE,
	"credentials" TEXT DEFAULT NULL,
	"gateway" VARCHAR(15) NOT NULL,
	"netmask" VARCHAR(15) NOT NULL,
	"type" NetworkType DEFAULT NULL
);


CREATE TYPE Band AS ENUM ('Ethernet', '2.4GHz', '5GHz', 'All');


-- SUMMARY:  Devices Addresses for a Networks.
-- RELATION: <Devices>:<Networks> N:1.
DROP TABLE IF EXISTS "Devices" CASCADE;
CREATE TABLE "Devices"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"band" Band NOT NULL,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"mac" CHAR(17) NOT NULL,
	"Networks.id" INT NOT NULL REFERENCES "Networks"("id") ON DELETE CASCADE,
	"static_ip_address" VARCHAR(15) DEFAULT NULL,
	UNIQUE("label", "Networks.id")
);


CREATE UNIQUE INDEX ON "Devices"("static_ip_address")
	WHERE "static_ip_address" IS NOT NULL;


CREATE UNIQUE INDEX ON "Devices"("mac", "Networks.id")
	WHERE "mac" IS NOT NULL;


-- SUMMARY:  Servicess that runs on the device for a device.
-- RELATION: <Services>:<Devices> N:1.
DROP TABLE IF EXISTS "Services" CASCADE;
CREATE TABLE "Services"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL DEFAULT '',
	"domain" TEXT NOT NULL DEFAULT '',
	"port" SMALLINT NOT NULL DEFAULT 443,
	"auth_value" TEXT DEFAULT NULL,
	"Devices.id" INT NOT NULL REFERENCES "Devices"("id") ON DELETE CASCADE,
	"Networks.id" INT NOT NULL REFERENCES "Networks"("id") ON DELETE CASCADE,
	UNIQUE("label", "Devices.id")
);


-- SUMMARY:  Types of devices.
-- RELATION: <Groups>:<Devices> N:M.
-- REQUIRED VALUES: ['Other', 'Mixed']
DROP TABLE IF EXISTS "Groups" CASCADE;
CREATE TABLE "Groups"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"label" VARCHAR(32) NOT NULL UNIQUE
);


-- SUMMARY:  Associates Groupss with Devicess.
-- RELATION: <Groups>:<Devices> N:M.
DROP TABLE IF EXISTS "Groups-Devices" CASCADE;
CREATE TABLE "Groups-Devices"
(
	"id" SERIAL NOT NULL PRIMARY KEY,
	"Groups.id" INT NOT NULL REFERENCES "Groups"("id") ON DELETE CASCADE,
	"Devices.id" INT NOT NULL REFERENCES "Devices"("id") ON DELETE CASCADE,
	UNIQUE("Groups.id", "Devices.id")
);
