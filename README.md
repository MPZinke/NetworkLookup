
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


# Device Lookup
API to detail and lookup local network IPs and devices.

Author: MPZinke
Created on: 2022.04.28


## Description
As a Home Automator,
I want a system that organizes my attached devices, and provides a way to access them by IP, Type, current connection, and network,
So that I can easily and dynamically gather information and expand my network and automations.


### Additional Features
- Currently connected devices
- Hosts' names sugar (use the device.label as a way of getting device info)


### Desired Implementations
- Who Is Home
- Smart Curtain


### Behavior
A HTTP request will access the REST API, which will read from the Postgres DB.
If the request is for device specific info (information containing device address specifics) then the router will make an HTTP get request to the router and parse its HTML to see information like whether the address currently has a device connected to it (if no DB values exists for it).
Additionally, the ping method can be used to determine if a device is active/responsive on an address.


#### Desired Endpoints
- `/api`
- `/api`
- `/api/group`: "Queries for groups"
- `/api/group/all`: "List all groups"
- `/api/group/id`: "Queries for group based on id"
- `/api/group/id/{id}`: "Get a group by id"
- `/api/group/label`: "Queries for group based on label"
- `/api/group/label/{label}`: "Get a group by label"
- `/api/network`: "Queries for networks"
- `/api/networks/all`: "List all networks"
- `/api/network`: "Queries for a network based on network id"
- `/api/networks/{id}`: "Get a network by id"
- `/api/networks/{id}/device`: "Queries for device based on network id"
- `/api/networks/{id}/devices/address`: "Queries for device based on device address and network id"
- `/api/networks/{id}/devices/address/{address}`: "Get a device by device address and network id"
- `/api/networks/{id}/devices/id`: "Queries for device based on device id and network id"
- `/api/networks/{id}/devices/id/{id}`: "Get a device by device id and network id"
- `/api/networks/{id}/devices/label`: "Queries for device based on device label and network id"
- `/api/networks/{id}/devices/label/{label}`: "Get a device by device label and network id"
- `/api/networks/{id}/devices`: "Queries for devices based one network id"
- `/api/networks/{id}/devices/all`: "List all devices for network id"
- `/api/networks/{id}/devices/group`: "Queries for devices based on group and network id"
- `/api/networks/{id}/devices/group/id`: "Queries for devices based on group id and network id"
- `/api/networks/{id}/devices/group/id/{id}`: "List all devices based on group id and network id"
- `/api/networks/{id}/devices/group/label`: "Queries for devices based on group label and network id"
- `/api/networks/{id}/devices/group/label/{label}`: "List all devices based on group label and network id"
- `/api/networks/{id}/services`: "Queries for services based on service and network id"
- `/api/networks/{id}/services/label`: "Queries for services based on service label and network id"
- `/api/networks/{id}/services/label/{label}`: "List all services based on service label and network id"
- `/api/networks/label`: "Queries for a network based on network label"
- `/api/networks/label/{label}`: "Get a network by label"
- `/api/networks/label/{label}/device`: "Queries for device based on network label"
- `/api/networks/label/{label}/devices/address`: "Queries for device based on device address and network label"
- `/api/networks/lable/{label}/devices/address/{address}`: "Get a device by device address and network label"
- `/api/networks/label/{label}/devices/id`: "Queries for device based on device id and network label"
- `/api/networks/label/{label}/devices/id/{id}`: "Get a device by device id and network label"
- `/api/networks/label/{label}/devices/label`: "Queries for device based on device label and network label"
- `/api/networks/label/{label}/devices/label/{label}`: "Get a device by device label and network label"
- `/api/networks/label/{label}/devices`: "Queries for devices based one network label"
- `/api/networks/label/{label}/devices/all`: "List all devices for network label"
- `/api/networks/label/{label}/devices/group`: "Queries for devices based on group and network label"
- `/api/networks/label/{label}/devices/group/id`: "Queries for devices based on group id and network label"
- `/api/networks/label/{label}/devices/group/id/{id}`: "List all devices based on group id and network label"
- `/api/networks/label/{label}/devices/group/label`: "Queries for devices based on group label and network label"
- `/api/networks/label/{label}/devices/group/label/{label}`: "List all devices based on group label and network label"
- `/api/networks/label/{label}/services`: "Queries for services based on service and network label"
- `/api/networks/label/{label}/services/label`: "Queries for services based on service label and network label"
- `/api/networks/label/{label}/services/label/{label}`: "List all services based on service label and network label"


## Compiling
```bash
export NETWORKLOOKUP_BEARERTOKEN=""
export NETWORKLOOKUP_ROUTER_DOMAIN=""

cargo build
```



## Appendix

- Router Attached Devices URL: `http://<ROUTER GATEWAY>/DEV_device_dev_id.htm`
