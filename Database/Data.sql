
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


INSERT INTO "Device" ("static_ip_address", "label", "is_reservation", "is_static", "mac", "Network.id")
SELECT "Temp"."static_ip_address", "Temp"."label", "Temp"."is_reservation", "Temp"."is_static", "Temp"."mac", "Network"."id"
FROM
(
	VALUES
	('192.168.1.2', 'Device-2', FALSE, NULL),
	('192.168.1.3', 'Device-3', FALSE, NULL),
	('192.168.1.4', 'Device-4', FALSE, NULL),
	('192.168.1.5', 'Device-5', FALSE, NULL),
	-- Livingroom
	('192.168.1.6', 'Device-6', FALSE, NULL),
	('192.168.1.7', 'Device-7', FALSE, NULL),
	('192.168.1.8', 'Device-8', FALSE, NULL),
	('192.168.1.9', 'Device-9', FALSE, NULL),
	('192.168.1.10', 'Device-10', FALSE, NULL),
	-- Bedroom
	('192.168.1.11', 'Device-11', FALSE, NULL),
	('192.168.1.12', 'Device-12', FALSE, NULL),
	('192.168.1.13', 'Device-13', FALSE, NULL),
	('192.168.1.14', 'Device-14', FALSE, NULL),
	('192.168.1.15', 'Resevered-15', TRUE, TRUE, NULL),
	-- Kitchen
	('192.168.1.16', 'Device-16', FALSE, NULL),
	('192.168.1.17', 'Device-17', FALSE, NULL),
	('192.168.1.18', 'Device-18', FALSE, NULL),
	('192.168.1.19', 'Device-19', FALSE, NULL),
	('192.168.1.20', 'Device-20', FALSE, NULL),
	-- Computers
	('192.168.1.21', 'Device-21', FALSE, NULL),
	('192.168.1.22', 'Device-22', FALSE, NULL),
	('192.168.1.23', 'Device-23', FALSE, NULL),
	('192.168.1.24', 'Device-24', FALSE, NULL),
	('192.168.1.25', 'Device-25', FALSE, NULL),
	-- Mobile
	('192.168.1.26', 'Device-26', FALSE, NULL),
	('192.168.1.27', 'Device-27', FALSE, NULL),
	('192.168.1.28', 'Device-28', FALSE, NULL),
	('192.168.1.29', 'Device-29', FALSE, NULL),
	('192.168.1.30', 'Device-30', FALSE, NULL)
) AS "Temp"("static_ip_address", "label", "is_reservation", "mac")
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
	('Device-6', 'Livingroom'),
	('Device-7', 'Livingroom'),
	('Device-8', 'Livingroom'),
	('Device-9', 'Livingroom'),
	('Device-10', 'Livingroom')
);


-- Livingroom
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Bedroom'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-11', 'Bedroom'),
	('Device-12', 'Bedroom'),
	('Device-13', 'Bedroom'),
	('Device-14', 'Bedroom'),
	'Resevered-15'
);


-- Kitchen
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Kitchen'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-16', 'Kitchen'),
	('Device-17', 'Kitchen'),
	('Device-18', 'Kitchen'),
	('Device-19', 'Kitchen'),
	('Device-20', 'Kitchen')
);


-- ———————————————————————————————————————————————— GROUPS::COMPUTER ———————————————————————————————————————————————— --

-- Computers
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Computer'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-21', 'Computer'),
	('Device-22', 'Computer'),
	('Device-23', 'Computer'),
	('Device-24', 'Computer'),
	('Device-25', 'Computer')
);


-- ———————————————————————————————————————————————— GROUPS::DEVICES  ———————————————————————————————————————————————— --

-- Mobile
INSERT INTO "Group-Device" ("Group.id", "Device.id")
SELECT "Group"."id", "Device"."id" FROM "Device"
JOIN "Group" ON "Group"."label" = 'Phone'
JOIN "Network" ON "Device"."Network.id" = "Network"."id"
WHERE ("Device"."label", "Network"."label") IN
(
	('Device-26', 'Phone'),
	('Device-27', 'Phone'),
	('Device-28', 'Phone'),
	('Device-29', 'Phone'),
	('Device-30', 'Phone')
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
	('Bedroom-TV', 'Entertainment'),
	('Livingroom-TV', 'Entertainment')
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
	('Bedroom-TV', 'Smart'),
	('Livingroom-TV', 'Smart')
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
	('Bedroom-Curtain', 'Curtain'),
	('Livingroom-Curtain', 'Curtain')
);
