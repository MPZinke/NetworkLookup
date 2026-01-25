import requests


def request(URL) -> None:
	print(URL)

	auth = {"Authorization": "Bearer Hello"}
	response = requests.get(URL, headers=auth)
	print(response)
	print(response.json())

	print("——————————————————————————————")


request("http://localhost:443/")
request("http://localhost:443/api")
request("http://localhost:443/api/groups")
request("http://localhost:443/api/groups/1")
request("http://localhost:443/api/networks")
request("http://localhost:443/api/networks/1")
request("http://localhost:443/api/networks/1/devices")
request("http://localhost:443/api/networks/1/devices/1")
request("http://localhost:443/api/services")
request("http://localhost:443/api/services/1")
