
<!----------------------------------------------------------------------------------------------------------------------
-                                                                                                                      -
-   Created by: MPZinke                                                                                                -
-   on 2022.04.28                                                                                                      -
-                                                                                                                      -
-   DESCRIPTION:                                                                                                       -
-   BUGS:                                                                                                              -
-   FUTURE:                                                                                                            -
-                                                                                                                      -
----------------------------------------------------------------------------------------------------------------------->


# Network Lookup
A JSON REST API service for tracking known network devices. For Asus router, gets current devices, sets static IPs, and sets allowed devices.

Author: MPZinke
Created on: 2022.04.28


## Features
- Tracks and tags known devices & services.
- On Asus router
	- Gets currently connected devices.
	- Sets static IP addresses for tracked devices.
	- Sets allowed devices (MAC filtering).


### Additional Features
- Currently connected devices
- Hosts' names sugar (use the device.label as a way of getting device info)


### Desired Integrations
- Who Is Home
- Smart Curtain


### Behavior
A HTTP request will access the REST API, which will read from the Postgres DB.
If the request is for device specific info (information containing device address specifics) then the router will make an HTTP get request to the router and parse its HTML to see information like whether the address currently has a device connected to it (if no DB values exists for it).
Additionally, the ping method can be used to determine if a device is active/responsive on an address.



## Compiling
```bash
export NETWORKLOOKUP_BEARERTOKEN=""
export NETWORKLOOKUP_ROUTER_DOMAIN=""

cargo build
```



## Appendix

- Router Attached Devices URL: `http://<ROUTER GATEWAY>/DEV_device_dev_id.htm`
