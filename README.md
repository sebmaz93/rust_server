# rust_server

The app will run and you can add/update items by using HTTP request you can use postman or curl using this url: `http://localhost:3030/v1/groceries`

#### postman
methods : post / get / put / delete

body: raw (json)

post / put
```json
  {
    "name": "apple",
    "quantity": 5
  }
```

delete
```json
  {
    "val": "id"
  }
```

### ToDo

    Add a 404 route
    Add error handling for malformatted JSON
    Adjust the return messages for each route
    Add test for each route with curls
