
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


INSERT INTO "Network" ("auth_value", "label", "gateway", "netmask") VALUES
('Basic SGVsbG86V29ybGQK', 'Home', '192.168.1.1', '255.255.255.0');


INSERT INTO "Device" ("address", "label", "is_reservation", "is_static", "mac", "Network.id")
SELECT "Temp"."address", "Temp"."label", "Temp"."is_reservation", "Temp"."is_static", "Temp"."mac", "Network"."id"
FROM
(
	VALUES
	('192.168.1.2', 'Device-2', TRUE, FALSE, NULL),
	('192.168.1.3', 'Device-3', TRUE, FALSE, NULL),
	('192.168.1.4', 'Device-4', TRUE, FALSE, NULL),
	('192.168.1.5', 'Device-5', TRUE, FALSE, NULL),
	-- Livingroom
	('192.168.1.6', 'Device-6', TRUE, FALSE, NULL),
	('192.168.1.7', 'Device-7', TRUE, FALSE, NULL),
	('192.168.1.8', 'Device-8', TRUE, FALSE, NULL),
	('192.168.1.9', 'Device-9', TRUE, FALSE, NULL),
	('192.168.1.10', 'Device-10', TRUE, FALSE, NULL),
	-- Bedroom
	('192.168.1.11', 'Device-11', TRUE, FALSE, NULL),
	('192.168.1.12', 'Device-12', TRUE, FALSE, NULL),
	('192.168.1.13', 'Device-13', TRUE, FALSE, NULL),
	('192.168.1.14', 'Device-14', TRUE, FALSE, NULL),
	('192.168.1.15', 'Resevered-15', TRUE, TRUE, NULL),
	-- Kitchen
	('192.168.1.16', 'Device-16', TRUE, FALSE, NULL),
	('192.168.1.17', 'Device-17', TRUE, FALSE, NULL),
	('192.168.1.18', 'Device-18', TRUE, FALSE, NULL),
	('192.168.1.19', 'Device-19', TRUE, FALSE, NULL),
	('192.168.1.20', 'Device-20', TRUE, FALSE, NULL),
	-- Computers
	('192.168.1.21', 'Device-21', TRUE, FALSE, NULL),
	('192.168.1.22', 'Device-22', TRUE, FALSE, NULL),
	('192.168.1.23', 'Device-23', TRUE, FALSE, NULL),
	('192.168.1.24', 'Device-24', TRUE, FALSE, NULL),
	('192.168.1.25', 'Device-25', TRUE, FALSE, NULL),
	-- Mobile
	('192.168.1.26', 'Device-26', TRUE, FALSE, NULL),
	('192.168.1.27', 'Device-27', TRUE, FALSE, NULL),
	('192.168.1.28', 'Device-28', TRUE, FALSE, NULL),
	('192.168.1.29', 'Device-29', TRUE, FALSE, NULL),
	('192.168.1.30', 'Device-30', TRUE, FALSE, NULL)
) AS "Temp"("address", "label", "is_reservation", "is_static", "mac")
JOIN "Network" ON "Network"."label" = 'Home';


-- ————————————————————————————————————————————————————— GROUPS ————————————————————————————————————————————————————— --
-- —————————————————————————————————————————————————————————————————————————————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('House'),  -- 2–10
-- Whole rooms
('Livingroom'),  -- 11–20
('Bedroom'),  -- 21–30
-- Half rooms
('Kitchen'),  -- 31–35
-- Other
('Computer'),  -- 36–40
('Mobile');  -- 41–45


-- ————————————————————————————————————————————————— GROUPS::ROOMS  ————————————————————————————————————————————————— --

-- House
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'House'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-2', 'Home'),
	('Device-3', 'Home'),
	('Device-4', 'Home'),
	('Device-5', 'Home')
);


-- Bedroom
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Livingroom'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-6', 'Home'),
	('Device-7', 'Home'),
	('Device-8', 'Home'),
	('Device-9', 'Home'),
	('Device-10', 'Home')
);


-- Livingroom
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Bedroom'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-11', 'Home'),
	('Device-12', 'Home'),
	('Device-13', 'Home'),
	('Device-14', 'Home'),
	'Resevered-15'
);


-- Kitchen
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Kitchen'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-16', 'Home'),
	('Device-17', 'Home'),
	('Device-18', 'Home'),
	('Device-19', 'Home'),
	('Device-20', 'Home')
);


-- ———————————————————————————————————————————————— GROUPS::COMPUTER ———————————————————————————————————————————————— --

-- Computers
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Computer'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-21', 'Home'),
	('Device-22', 'Home'),
	('Device-23', 'Home'),
	('Device-24', 'Home'),
	('Device-25', 'Home')
);


-- ———————————————————————————————————————————————— GROUPS::DEVICES  ———————————————————————————————————————————————— --

-- Mobile
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Phone'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-26', 'Home'),
	('Device-27', 'Home'),
	('Device-28', 'Home'),
	('Device-29', 'Home'),
	('Device-30', 'Home')
);

-- ———————————————————————————————————————————— GROUPS::ENTERTAINMENT ———————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('Entertainment');


INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Entertainment'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Bedroom-TV', 'Home'),
	('Livingroom-TV', 'Home')
);


-- ———————————————————————————————————————————— GROUPS::SMART ———————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('Smart');


INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Smart'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Bedroom-TV', 'Home'),
	('Livingroom-TV', 'Home')
);


-- ———————————————————————————————————————————— GROUPS::CURTAIN ———————————————————————————————————————————— --

INSERT INTO "Group" ("label") VALUES
('Curtain');


INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Curtain'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Bedroom-Curtain', 'Home'),
	('Livingroom-Curtain', 'Home')
);
