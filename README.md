
## API Endpoints
* /users/<id>/name
Request:
Reply:   Name associated with id

* /users/<id>/password
Request: Password hash (utf-8 string)
Reply:   Valid (true/false) 

* /users/<id>/password/change
Request: New Password hash (utf-8 string)
Reply:   Success (true/false)

