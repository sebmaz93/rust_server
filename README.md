# rust_server

The app will run and you can add/update items by using HTTP request you can use postman or curl using this url: `http://localhost:3030/v1/groceries`

#### postman
methods : post / get / put / delete

body: raw (json)

post -> http://localhost:3030/v1/groceries
```json
  {
    "name": "apple",
    "quantity": 5
  }
```
put -> http://localhost:3030/v1/groceries/{id}
```json
  {
    "name": "appleeee",
    "quantity": 10
  }
```

delete -> http://localhost:3030/v1/groceries/{id}

header: { "authorization" : "Bearer admin" }

-----

### ToDo

    Add a 404 route
    Add error handling for malformatted JSON
    Add test for each route with curls
