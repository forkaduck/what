
## API Endpoints
* Strings are all UTF-8

### POST /users/password
#### Request:
* Type: application/json
* Data: {"id": "<id>", "pass": "<password>"}

#### Reply:
* Data: {"valid": "<true/false>", "token": "<hash>"}



### POST /users/password/change
#### Request:
* Type: application/json
* Data: {"token": "<hash>", "oldpass": "<password>", "newpass": "<password>"}

#### Reply:
* Data: {"success": "<true/false>"}



### POST /users/register
#### Request:
* Type: application/json
* Data: {"name": "<id>", "pass": "<password>"}

#### Reply:
* Data: {"success": "<true/false>"}
